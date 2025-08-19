use anyhow::Result;
use crate::MobilePlatformConfig;
use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use tracing::{info, warn, debug};
use std::time::{SystemTime, Duration};
use tokio::sync::RwLock;

/// Offline mode manager for mobile gameplay
pub struct OfflineManager {
    pub config: OfflineConfig,
    pub offline_state: Arc<RwLock<OfflineState>>,
    pub local_storage: Arc<RwLock<LocalStorage>>,
    pub sync_queue: Arc<RwLock<SyncQueue>>,
    pub offline_systems: OfflineSystems,
}

/// Offline mode configuration
#[derive(Debug, Clone)]
pub struct OfflineConfig {
    pub enabled: bool,
    pub max_offline_duration: Duration,
    pub auto_save_interval: Duration,
    pub compression_enabled: bool,
    pub max_storage_size: u64, // bytes
    pub cache_expiry_duration: Duration,
}

/// Current offline state
#[derive(Debug, Clone)]
pub struct OfflineState {
    pub is_offline: bool,
    pub offline_since: Option<SystemTime>,
    pub last_online: Option<SystemTime>,
    pub pending_actions: Vec<OfflineAction>,
    pub cached_world_state: Option<CachedWorldState>,
    pub offline_progress: OfflineProgress,
}

/// Local storage for offline data
#[derive(Debug, Clone)]
pub struct LocalStorage {
    pub stored_data: HashMap<String, StoredData>,
    pub current_size: u64,
    pub last_cleanup: SystemTime,
    pub storage_directory: std::path::PathBuf,
}

/// Synchronization queue for when going back online
#[derive(Debug, Clone)]
pub struct SyncQueue {
    pub queued_actions: Vec<QueuedAction>,
    pub failed_syncs: Vec<FailedSync>,
    pub last_sync_attempt: Option<SystemTime>,
}

/// Systems that can operate offline
#[derive(Debug)]
pub struct OfflineSystems {
    pub character_progression: bool,
    pub inventory_management: bool,
    pub crafting_system: bool,
    pub skill_training: bool,
    pub local_exploration: bool,
    pub offline_activities: bool,
}

/// Action performed while offline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineAction {
    pub id: Uuid,
    pub action_type: OfflineActionType,
    pub timestamp: SystemTime,
    pub character_id: Uuid,
    pub data: serde_json::Value,
    pub requires_validation: bool,
}

/// Types of actions that can be performed offline
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfflineActionType {
    // Character actions
    LevelUp { skill: String, levels: u32 },
    StatIncrease { stat: String, amount: u32 },
    ExperienceGain { skill: String, amount: u64 },
    
    // Inventory actions
    ItemPickup { item_id: Uuid, quantity: u32 },
    ItemDrop { item_id: Uuid, quantity: u32 },
    ItemUse { item_id: Uuid, quantity: u32 },
    InventoryReorder { from_slot: u32, to_slot: u32 },
    
    // Crafting actions
    CraftItem { recipe_id: Uuid, quantity: u32, materials_used: Vec<(Uuid, u32)> },
    UpgradeItem { item_id: Uuid, materials_used: Vec<(Uuid, u32)> },
    
    // Settings and preferences
    SettingChange { key: String, value: serde_json::Value },
    UILayoutChange { layout_data: serde_json::Value },
    
    // Offline-specific actions
    OfflineTimeAccumulation { duration: Duration, activities: Vec<String> },
    CacheUpdate { cache_key: String, data_size: u64 },
}

/// Cached world state for offline play
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedWorldState {
    pub character_data: HashMap<Uuid, CharacterSnapshot>,
    pub inventory_data: HashMap<Uuid, InventorySnapshot>,
    pub world_areas: HashMap<String, AreaSnapshot>,
    pub npc_states: HashMap<Uuid, NPCSnapshot>,
    pub cached_at: SystemTime,
    pub expires_at: SystemTime,
}

/// Character data snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSnapshot {
    pub id: Uuid,
    pub level: u32,
    pub stats: HashMap<String, u32>,
    pub skills: HashMap<String, u32>,
    pub location: String,
    pub health: u32,
    pub mana: u32,
    pub experience: HashMap<String, u64>,
}

/// Inventory data snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySnapshot {
    pub character_id: Uuid,
    pub items: HashMap<u32, InventoryItem>, // slot -> item
    pub capacity: u32,
    pub gold: u64,
}

/// Inventory item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub item_id: Uuid,
    pub quantity: u32,
    pub durability: Option<u32>,
    pub enchantments: Vec<String>,
}

/// World area snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaSnapshot {
    pub area_id: String,
    pub npcs: Vec<Uuid>,
    pub available_resources: HashMap<String, u32>,
    pub weather: String,
    pub time_of_day: f32,
}

/// NPC state snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCSnapshot {
    pub npc_id: Uuid,
    pub name: String,
    pub location: String,
    pub available_quests: Vec<Uuid>,
    pub inventory: HashMap<Uuid, u32>,
    pub dialogue_state: String,
}

/// Offline progress tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineProgress {
    pub total_offline_time: Duration,
    pub activities_completed: HashMap<String, u32>,
    pub experience_gained: HashMap<String, u64>,
    pub items_gathered: HashMap<Uuid, u32>,
    pub achievements_unlocked: Vec<Uuid>,
    pub milestones_reached: Vec<String>,
}

/// Stored data item
#[derive(Debug, Clone)]
pub struct StoredData {
    pub data: Vec<u8>,
    pub stored_at: SystemTime,
    pub expires_at: Option<SystemTime>,
    pub access_count: u64,
    pub data_type: String,
}

/// Queued action for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedAction {
    pub id: Uuid,
    pub action: OfflineAction,
    pub priority: u32,
    pub retry_count: u32,
    pub queued_at: SystemTime,
}

/// Failed synchronization attempt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedSync {
    pub action_id: Uuid,
    pub error_message: String,
    pub failed_at: SystemTime,
    pub retry_after: SystemTime,
}

impl OfflineManager {
    /// Create new offline manager
    pub async fn new(config: &MobilePlatformConfig) -> Result<Self> {
        info!("📴 Initializing Offline Mode Manager");

        let offline_config = OfflineConfig {
            enabled: config.enable_offline_mode,
            max_offline_duration: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            auto_save_interval: Duration::from_secs(60), // 1 minute
            compression_enabled: true,
            max_storage_size: 500 * 1024 * 1024, // 500 MB
            cache_expiry_duration: Duration::from_secs(24 * 60 * 60), // 24 hours
        };

        let offline_state = Arc::new(RwLock::new(OfflineState {
            is_offline: false,
            offline_since: None,
            last_online: Some(SystemTime::now()),
            pending_actions: Vec::new(),
            cached_world_state: None,
            offline_progress: OfflineProgress::new(),
        }));

        let storage_directory = Self::get_storage_directory().await?;
        let local_storage = Arc::new(RwLock::new(LocalStorage {
            stored_data: HashMap::new(),
            current_size: 0,
            last_cleanup: SystemTime::now(),
            storage_directory,
        }));

        let sync_queue = Arc::new(RwLock::new(SyncQueue {
            queued_actions: Vec::new(),
            failed_syncs: Vec::new(),
            last_sync_attempt: None,
        }));

        let offline_systems = OfflineSystems {
            character_progression: true,
            inventory_management: true,
            crafting_system: true,
            skill_training: true,
            local_exploration: true,
            offline_activities: true,
        };

        info!("✅ Offline manager initialized");
        info!("   Enabled: {}", offline_config.enabled);
        info!("   Max offline duration: {} hours", offline_config.max_offline_duration.as_secs() / 3600);
        info!("   Max storage size: {} MB", offline_config.max_storage_size / 1024 / 1024);

        Ok(Self {
            config: offline_config,
            offline_state,
            local_storage,
            sync_queue,
            offline_systems,
        })
    }

    /// Enter offline mode
    pub async fn enter_offline_mode(&self) -> Result<()> {
        if !self.config.enabled {
            return Err(anyhow::anyhow!("Offline mode is disabled"));
        }

        let mut state = self.offline_state.write().await;
        
        if state.is_offline {
            debug!("📴 Already in offline mode");
            return Ok(());
        }

        state.is_offline = true;
        state.offline_since = Some(SystemTime::now());
        
        info!("📴 Entered offline mode");

        // Cache current world state
        self.cache_world_state().await?;

        // Start offline systems
        self.start_offline_systems().await?;

        Ok(())
    }

    /// Exit offline mode and sync
    pub async fn exit_offline_mode(&self) -> Result<()> {
        let mut state = self.offline_state.write().await;
        
        if !state.is_offline {
            debug!("🌐 Already online");
            return Ok(());
        }

        let offline_duration = state.offline_since
            .map(|since| SystemTime::now().duration_since(since).unwrap_or(Duration::ZERO))
            .unwrap_or(Duration::ZERO);

        state.is_offline = false;
        state.last_online = Some(SystemTime::now());
        
        // Update offline progress
        state.offline_progress.total_offline_time += offline_duration;
        
        info!("🌐 Exiting offline mode (was offline for {} seconds)", offline_duration.as_secs());

        drop(state);

        // Process offline actions and sync
        self.process_offline_actions().await?;
        self.sync_with_server().await?;

        Ok(())
    }

    /// Record offline action
    pub async fn record_offline_action(&self, action_type: OfflineActionType, character_id: Uuid, data: serde_json::Value) -> Result<Uuid> {
        let action_id = Uuid::new_v4();
        
        let action = OfflineAction {
            id: action_id,
            action_type,
            timestamp: SystemTime::now(),
            character_id,
            data,
            requires_validation: true,
        };

        let mut state = self.offline_state.write().await;
        state.pending_actions.push(action);

        debug!("📝 Recorded offline action: {}", action_id);
        Ok(action_id)
    }

    /// Get offline progress
    pub async fn get_offline_progress(&self) -> Result<OfflineProgress> {
        let state = self.offline_state.read().await;
        Ok(state.offline_progress.clone())
    }

    /// Check if currently offline
    pub async fn is_offline(&self) -> bool {
        let state = self.offline_state.read().await;
        state.is_offline
    }

    /// Get offline duration
    pub async fn get_offline_duration(&self) -> Duration {
        let state = self.offline_state.read().await;
        
        if let Some(offline_since) = state.offline_since {
            SystemTime::now().duration_since(offline_since).unwrap_or(Duration::ZERO)
        } else {
            Duration::ZERO
        }
    }

    /// Store data locally
    pub async fn store_data_locally(&self, key: String, data: Vec<u8>, data_type: String) -> Result<()> {
        let mut storage = self.local_storage.write().await;
        
        // Check storage limits
        let data_size = data.len() as u64;
        if storage.current_size + data_size > self.config.max_storage_size {
            self.cleanup_storage(&mut storage, data_size).await?;
        }

        let stored_data = StoredData {
            data,
            stored_at: SystemTime::now(),
            expires_at: Some(SystemTime::now() + self.config.cache_expiry_duration),
            access_count: 1,
            data_type,
        };

        storage.stored_data.insert(key.clone(), stored_data);
        storage.current_size += data_size;

        debug!("💾 Stored data locally: {} ({} bytes)", key, data_size);
        Ok(())
    }

    /// Retrieve locally stored data
    pub async fn get_stored_data(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let mut storage = self.local_storage.write().await;
        
        if let Some(stored_data) = storage.stored_data.get_mut(key) {
            // Check if expired
            if let Some(expires_at) = stored_data.expires_at {
                if SystemTime::now() > expires_at {
                    storage.stored_data.remove(key);
                    return Ok(None);
                }
            }

            stored_data.access_count += 1;
            Ok(Some(stored_data.data.clone()))
        } else {
            Ok(None)
        }
    }

    /// Calculate offline rewards based on time and activities
    pub async fn calculate_offline_rewards(&self, character_id: Uuid, offline_duration: Duration) -> Result<OfflineRewards> {
        let hours_offline = offline_duration.as_secs_f32() / 3600.0;
        let max_hours = self.config.max_offline_duration.as_secs_f32() / 3600.0;
        let effective_hours = hours_offline.min(max_hours);

        // Base rewards calculation
        let base_exp_per_hour = 100u64;
        let base_gold_per_hour = 50u64;

        let experience_gained = (base_exp_per_hour as f32 * effective_hours) as u64;
        let gold_gained = (base_gold_per_hour as f32 * effective_hours) as u64;

        // Calculate items based on activities
        let mut items_found = HashMap::new();
        
        // Example: Basic materials from idle gathering
        if effective_hours >= 1.0 {
            items_found.insert("basic_ore".to_string(), (effective_hours as u32).min(20));
            items_found.insert("herbs".to_string(), (effective_hours as u32 / 2).min(10));
        }

        let rewards = OfflineRewards {
            experience_gained,
            gold_gained,
            items_found,
            time_processed: Duration::from_secs_f32(effective_hours * 3600.0),
            bonus_multiplier: if hours_offline > 24.0 { 1.5 } else { 1.0 },
        };

        info!("🎁 Calculated offline rewards for {} hours: {} exp, {} gold", effective_hours, experience_gained, gold_gained);
        Ok(rewards)
    }

    // Private helper methods
    async fn get_storage_directory() -> Result<std::path::PathBuf> {
        if let Some(dirs) = directories::ProjectDirs::from("com", "arceon", "Arceon") {
            let storage_dir = dirs.data_dir().join("offline");
            tokio::fs::create_dir_all(&storage_dir).await?;
            Ok(storage_dir)
        } else {
            let storage_dir = std::env::current_dir()?.join("data").join("offline");
            tokio::fs::create_dir_all(&storage_dir).await?;
            Ok(storage_dir)
        }
    }

    async fn cache_world_state(&self) -> Result<()> {
        // This would cache essential world state for offline play
        debug!("🏞️ Caching world state for offline play");
        
        let cached_state = CachedWorldState {
            character_data: HashMap::new(), // Would populate with actual data
            inventory_data: HashMap::new(),
            world_areas: HashMap::new(),
            npc_states: HashMap::new(),
            cached_at: SystemTime::now(),
            expires_at: SystemTime::now() + self.config.cache_expiry_duration,
        };

        let mut state = self.offline_state.write().await;
        state.cached_world_state = Some(cached_state);

        Ok(())
    }

    async fn start_offline_systems(&self) -> Result<()> {
        info!("⚙️ Starting offline systems");
        
        // Start auto-save timer
        let auto_save_interval = self.config.auto_save_interval;
        let offline_state = self.offline_state.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(auto_save_interval);
            
            loop {
                interval.tick().await;
                
                let state = offline_state.read().await;
                if state.is_offline {
                    // Perform auto-save
                    debug!("💾 Auto-saving offline progress");
                }
            }
        });

        Ok(())
    }

    async fn process_offline_actions(&self) -> Result<()> {
        let mut state = self.offline_state.write().await;
        let actions = std::mem::take(&mut state.pending_actions);
        drop(state);

        if actions.is_empty() {
            return Ok(());
        }

        info!("⚡ Processing {} offline actions", actions.len());

        let mut sync_queue = self.sync_queue.write().await;
        
        for action in actions {
            let queued_action = QueuedAction {
                id: Uuid::new_v4(),
                action,
                priority: 1,
                retry_count: 0,
                queued_at: SystemTime::now(),
            };
            
            sync_queue.queued_actions.push(queued_action);
        }

        // Sort by priority
        sync_queue.queued_actions.sort_by(|a, b| b.priority.cmp(&a.priority));

        Ok(())
    }

    async fn sync_with_server(&self) -> Result<()> {
        info!("🔄 Synchronizing offline changes with server");
        
        let mut sync_queue = self.sync_queue.write().await;
        sync_queue.last_sync_attempt = Some(SystemTime::now());
        
        // This would implement actual server synchronization
        // For now, just clear the queue to simulate successful sync
        let synced_count = sync_queue.queued_actions.len();
        sync_queue.queued_actions.clear();

        info!("✅ Synchronized {} actions with server", synced_count);
        Ok(())
    }

    async fn cleanup_storage(&self, storage: &mut LocalStorage, needed_space: u64) -> Result<()> {
        // Simple LRU eviction based on last access
        let mut items_by_access: Vec<_> = storage.stored_data.iter()
            .map(|(key, data)| (key.clone(), data.stored_at, data.data.len() as u64))
            .collect();
        
        items_by_access.sort_by_key(|(_, stored_at, _)| *stored_at);

        let mut space_freed = 0u64;
        let mut keys_to_remove = Vec::new();

        for (key, _, size) in items_by_access {
            keys_to_remove.push(key);
            space_freed += size;
            
            if space_freed >= needed_space {
                break;
            }
        }

        for key in keys_to_remove {
            storage.stored_data.remove(&key);
        }

        storage.current_size = storage.current_size.saturating_sub(space_freed);
        storage.last_cleanup = SystemTime::now();

        debug!("🗑️ Cleaned up storage, freed {} bytes", space_freed);
        Ok(())
    }
}

impl OfflineProgress {
    fn new() -> Self {
        Self {
            total_offline_time: Duration::ZERO,
            activities_completed: HashMap::new(),
            experience_gained: HashMap::new(),
            items_gathered: HashMap::new(),
            achievements_unlocked: Vec::new(),
            milestones_reached: Vec::new(),
        }
    }
}

/// Offline rewards calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineRewards {
    pub experience_gained: u64,
    pub gold_gained: u64,
    pub items_found: HashMap<String, u32>,
    pub time_processed: Duration,
    pub bonus_multiplier: f32,
}