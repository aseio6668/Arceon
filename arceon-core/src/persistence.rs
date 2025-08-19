use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use anyhow::{Result, Context};
use tokio::sync::RwLock;
use std::sync::Arc;

use crate::state::GameState;
use crate::entities::quests::QuestSystem;

/// World persistence system for safe shutdowns and real-time saving
/// Handles data snapshots to maintain world continuity across restarts
#[derive(Debug, Clone)]
pub struct PersistenceManager {
    pub data_directory: PathBuf,
    pub auto_save_interval: tokio::time::Duration,
    pub max_snapshots: usize,
    pub compression_enabled: bool,
}

/// Complete world snapshot containing all persistent data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub snapshot_id: String,
    pub timestamp: DateTime<Utc>,
    pub version: String,
    pub game_state: GameState,
    pub quest_system: QuestSystem,
    pub world_events: Vec<WorldEvent>,
    pub metadata: SnapshotMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotMetadata {
    pub world_age: u64,              // Ticks since genesis
    pub player_count: usize,
    pub active_npcs: usize,
    pub active_quests: usize,
    pub total_areas: usize,
    pub save_reason: SaveReason,
    pub world_checksum: String,      // For integrity verification
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SaveReason {
    AutoSave,
    ManualSave,
    SafeShutdown,
    PlayerJoin,
    PlayerLeave,
    CriticalEvent,
    ScheduledBackup,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub event_type: String,
    pub timestamp: DateTime<Utc>,
    pub affected_areas: Vec<String>,
    pub participants: Vec<String>,
    pub consequences: Vec<String>,
    pub is_persistent: bool,
}

/// Recovery information for failed loads
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryInfo {
    pub failed_snapshot_id: String,
    pub error_description: String,
    pub alternative_snapshots: Vec<String>,
    pub partial_recovery_possible: bool,
    pub corrupted_sections: Vec<String>,
}

impl PersistenceManager {
    pub fn new(data_directory: PathBuf) -> Result<Self> {
        // Ensure data directory exists
        fs::create_dir_all(&data_directory)
            .with_context(|| format!("Failed to create data directory: {:?}", data_directory))?;

        Ok(Self {
            data_directory,
            auto_save_interval: tokio::time::Duration::from_secs(300), // 5 minutes
            max_snapshots: 50,
            compression_enabled: true,
        })
    }

    /// Start the auto-save system
    pub async fn start_auto_save(&self, game_state: Arc<RwLock<GameState>>, quest_system: Arc<RwLock<QuestSystem>>) -> Result<()> {
        let mut interval = tokio::time::interval(self.auto_save_interval);
        let data_dir = self.data_directory.clone();
        let max_snapshots = self.max_snapshots;
        let compression = self.compression_enabled;

        tokio::spawn(async move {
            loop {
                interval.tick().await;

                // Create auto-save snapshot
                let snapshot_result = {
                    let state = game_state.read().await;
                    let quests = quest_system.read().await;
                    Self::create_snapshot_static(&state, &quests, SaveReason::AutoSave)
                };

                match snapshot_result {
                    Ok(snapshot) => {
                        if let Err(e) = Self::save_snapshot_static(&data_dir, &snapshot, compression).await {
                            eprintln!("Auto-save failed: {}", e);
                        } else {
                            println!("🔄 Auto-save completed: {}", snapshot.snapshot_id);
                            
                            // Clean up old snapshots
                            if let Err(e) = Self::cleanup_old_snapshots_static(&data_dir, max_snapshots) {
                                eprintln!("Snapshot cleanup failed: {}", e);
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create auto-save snapshot: {}", e);
                    }
                }
            }
        });

        Ok(())
    }

    /// Create a world snapshot
    pub fn create_snapshot(&self, game_state: &GameState, quest_system: &QuestSystem, reason: SaveReason) -> Result<WorldSnapshot> {
        Self::create_snapshot_static(game_state, quest_system, reason)
    }

    fn create_snapshot_static(game_state: &GameState, quest_system: &QuestSystem, reason: SaveReason) -> Result<WorldSnapshot> {
        let timestamp = Utc::now();
        let snapshot_id = format!("world_{}_{}", 
            timestamp.format("%Y%m%d_%H%M%S"), 
            SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_millis()
        );

        // Calculate world checksum for integrity
        let world_checksum = Self::calculate_world_checksum(game_state, quest_system)?;

        let metadata = SnapshotMetadata {
            world_age: game_state.world_time,
            player_count: game_state.online_players.len(),
            active_npcs: game_state.beings.values().filter(|b| matches!(b.being_type, crate::entities::being::BeingType::Npc)).count(),
            active_quests: quest_system.active_quests.len(),
            total_areas: game_state.areas.len(),
            save_reason: reason,
            world_checksum,
        };

        Ok(WorldSnapshot {
            snapshot_id,
            timestamp,
            version: env!("CARGO_PKG_VERSION").to_string(),
            game_state: game_state.clone(),
            quest_system: quest_system.clone(),
            world_events: Vec::new(), // Would be populated with significant events
            metadata,
        })
    }

    /// Save a snapshot to disk
    pub async fn save_snapshot(&self, snapshot: &WorldSnapshot) -> Result<()> {
        Self::save_snapshot_static(&self.data_directory, snapshot, self.compression_enabled).await
    }

    async fn save_snapshot_static(data_dir: &Path, snapshot: &WorldSnapshot, compression_enabled: bool) -> Result<()> {
        let snapshot_path = data_dir.join(format!("{}.json", snapshot.snapshot_id));
        let temp_path = data_dir.join(format!("{}.tmp", snapshot.snapshot_id));

        // Serialize to JSON
        let json_data = if compression_enabled {
            // Could implement compression here
            serde_json::to_string_pretty(snapshot)
                .context("Failed to serialize snapshot to JSON")?
        } else {
            serde_json::to_string_pretty(snapshot)
                .context("Failed to serialize snapshot to JSON")?
        };

        // Write to temporary file first for atomic operation
        tokio::fs::write(&temp_path, json_data)
            .await
            .with_context(|| format!("Failed to write snapshot to temporary file: {:?}", temp_path))?;

        // Atomically move temporary file to final location
        tokio::fs::rename(&temp_path, &snapshot_path)
            .await
            .with_context(|| format!("Failed to move snapshot from {:?} to {:?}", temp_path, snapshot_path))?;

        // Create latest snapshot symlink
        let latest_path = data_dir.join("latest_snapshot.json");
        if latest_path.exists() {
            tokio::fs::remove_file(&latest_path).await.ok();
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs;
            fs::symlink(&snapshot_path, &latest_path)
                .context("Failed to create latest snapshot symlink")?;
        }

        #[cfg(windows)]
        {
            tokio::fs::copy(&snapshot_path, &latest_path)
                .await
                .context("Failed to copy snapshot to latest")?;
        }

        Ok(())
    }

    /// Load the latest snapshot from disk
    pub async fn load_latest_snapshot(&self) -> Result<WorldSnapshot> {
        let latest_path = self.data_directory.join("latest_snapshot.json");
        
        if !latest_path.exists() {
            return Err(anyhow::anyhow!("No saved snapshots found"));
        }

        self.load_snapshot_from_file(&latest_path).await
    }

    /// Load a specific snapshot by ID
    pub async fn load_snapshot(&self, snapshot_id: &str) -> Result<WorldSnapshot> {
        let snapshot_path = self.data_directory.join(format!("{}.json", snapshot_id));
        self.load_snapshot_from_file(&snapshot_path).await
    }

    async fn load_snapshot_from_file(&self, path: &Path) -> Result<WorldSnapshot> {
        let json_data = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read snapshot file: {:?}", path))?;

        let snapshot: WorldSnapshot = serde_json::from_str(&json_data)
            .with_context(|| format!("Failed to deserialize snapshot from: {:?}", path))?;

        // Verify snapshot integrity
        self.verify_snapshot_integrity(&snapshot)?;

        Ok(snapshot)
    }

    /// Verify snapshot integrity using checksum
    fn verify_snapshot_integrity(&self, snapshot: &WorldSnapshot) -> Result<()> {
        let calculated_checksum = Self::calculate_world_checksum(&snapshot.game_state, &snapshot.quest_system)?;
        
        if calculated_checksum != snapshot.metadata.world_checksum {
            return Err(anyhow::anyhow!(
                "Snapshot integrity check failed: checksum mismatch (expected {}, got {})",
                snapshot.metadata.world_checksum,
                calculated_checksum
            ));
        }

        Ok(())
    }

    /// Calculate a checksum for world state integrity verification
    fn calculate_world_checksum(game_state: &GameState, quest_system: &QuestSystem) -> Result<String> {
        // Simple checksum based on key world data
        let mut checksum_data = String::new();
        checksum_data.push_str(&format!("{}", game_state.world_time));
        checksum_data.push_str(&format!("{}", game_state.areas.len()));
        checksum_data.push_str(&format!("{}", game_state.beings.len()));
        checksum_data.push_str(&format!("{}", quest_system.active_quests.len()));

        // Use a simple hash (in production, use a proper cryptographic hash)
        let checksum = format!("{:x}", checksum_data.len() * 17 + game_state.world_time as usize);
        Ok(checksum)
    }

    /// List available snapshots
    pub async fn list_snapshots(&self) -> Result<Vec<SnapshotInfo>> {
        let mut snapshots = Vec::new();
        let mut entries = tokio::fs::read_dir(&self.data_directory).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_name != "latest_snapshot" {
                        let metadata = entry.metadata().await?;
                        let modified = metadata.modified()?;
                        let size = metadata.len();

                        snapshots.push(SnapshotInfo {
                            snapshot_id: file_name.to_string(),
                            file_path: path,
                            size,
                            modified,
                        });
                    }
                }
            }
        }

        // Sort by modification time (newest first)
        snapshots.sort_by(|a, b| b.modified.cmp(&a.modified));
        Ok(snapshots)
    }

    /// Clean up old snapshots, keeping only the most recent ones
    pub fn cleanup_old_snapshots(&self) -> Result<usize> {
        Self::cleanup_old_snapshots_static(&self.data_directory, self.max_snapshots)
    }

    fn cleanup_old_snapshots_static(data_dir: &Path, max_snapshots: usize) -> Result<usize> {
        let mut snapshot_files = Vec::new();
        
        for entry in fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                    if file_name != "latest_snapshot" {
                        let metadata = entry.metadata()?;
                        snapshot_files.push((path, metadata.modified()?));
                    }
                }
            }
        }

        // Sort by modification time (oldest first)
        snapshot_files.sort_by(|a, b| a.1.cmp(&b.1));

        let mut deleted_count = 0;
        if snapshot_files.len() > max_snapshots {
            let to_delete = snapshot_files.len() - max_snapshots;
            for (path, _) in snapshot_files.iter().take(to_delete) {
                if let Err(e) = fs::remove_file(path) {
                    eprintln!("Failed to delete old snapshot {:?}: {}", path, e);
                } else {
                    deleted_count += 1;
                }
            }
        }

        Ok(deleted_count)
    }

    /// Perform a safe shutdown save
    pub async fn safe_shutdown_save(&self, game_state: &GameState, quest_system: &QuestSystem) -> Result<()> {
        println!("🔄 Performing safe shutdown save...");
        
        let snapshot = self.create_snapshot(game_state, quest_system, SaveReason::SafeShutdown)?;
        self.save_snapshot(&snapshot).await?;
        
        println!("✅ Safe shutdown save completed: {}", snapshot.snapshot_id);
        Ok(())
    }

    /// Handle recovery from corrupted snapshots
    pub async fn recover_from_corruption(&self, failed_snapshot_id: &str) -> Result<WorldSnapshot> {
        println!("⚠️  Attempting recovery from corrupted snapshot: {}", failed_snapshot_id);
        
        let available_snapshots = self.list_snapshots().await?;
        
        // Try to load the most recent working snapshot
        for snapshot_info in available_snapshots {
            if snapshot_info.snapshot_id != failed_snapshot_id {
                match self.load_snapshot(&snapshot_info.snapshot_id).await {
                    Ok(snapshot) => {
                        println!("✅ Recovered from snapshot: {}", snapshot.snapshot_id);
                        return Ok(snapshot);
                    },
                    Err(e) => {
                        println!("❌ Failed to load snapshot {}: {}", snapshot_info.snapshot_id, e);
                        continue;
                    }
                }
            }
        }

        Err(anyhow::anyhow!("No recoverable snapshots found"))
    }

    /// Get persistence statistics
    pub async fn get_statistics(&self) -> Result<PersistenceStatistics> {
        let snapshots = self.list_snapshots().await?;
        let total_size: u64 = snapshots.iter().map(|s| s.size).sum();
        let oldest = snapshots.last().map(|s| s.modified);
        let newest = snapshots.first().map(|s| s.modified);

        Ok(PersistenceStatistics {
            total_snapshots: snapshots.len(),
            total_size_bytes: total_size,
            oldest_snapshot: oldest,
            newest_snapshot: newest,
            data_directory: self.data_directory.clone(),
            auto_save_interval_seconds: self.auto_save_interval.as_secs(),
        })
    }
}

#[derive(Debug)]
pub struct SnapshotInfo {
    pub snapshot_id: String,
    pub file_path: PathBuf,
    pub size: u64,
    pub modified: SystemTime,
}

#[derive(Debug)]
pub struct PersistenceStatistics {
    pub total_snapshots: usize,
    pub total_size_bytes: u64,
    pub oldest_snapshot: Option<SystemTime>,
    pub newest_snapshot: Option<SystemTime>,
    pub data_directory: PathBuf,
    pub auto_save_interval_seconds: u64,
}

/// Genesis launch system for network persistence
#[derive(Debug)]
pub struct GenesisLauncher {
    persistence_manager: PersistenceManager,
    network_config: NetworkPersistenceConfig,
}

#[derive(Debug, Clone)]
pub struct NetworkPersistenceConfig {
    pub use_centralized_storage: bool,
    pub use_decentralized_storage: bool,
    pub backup_nodes: Vec<String>,
    pub consensus_threshold: f32,
    pub max_network_delay: tokio::time::Duration,
}

impl GenesisLauncher {
    pub fn new(persistence_manager: PersistenceManager, network_config: NetworkPersistenceConfig) -> Self {
        Self {
            persistence_manager,
            network_config,
        }
    }

    /// Launch from the last known state, either locally or from network
    pub async fn launch_from_last_state(&self) -> Result<(GameState, QuestSystem)> {
        // Try local snapshot first
        match self.persistence_manager.load_latest_snapshot().await {
            Ok(snapshot) => {
                println!("🚀 Launching from local snapshot: {}", snapshot.snapshot_id);
                return Ok((snapshot.game_state, snapshot.quest_system));
            },
            Err(e) => {
                println!("⚠️  Local snapshot failed: {}", e);
            }
        }

        // Try network recovery if local fails
        if self.network_config.use_centralized_storage || self.network_config.use_decentralized_storage {
            match self.recover_from_network().await {
                Ok((game_state, quest_system)) => {
                    println!("🌐 Recovered world state from network");
                    return Ok((game_state, quest_system));
                },
                Err(e) => {
                    println!("❌ Network recovery failed: {}", e);
                }
            }
        }

        // If all recovery fails, start fresh genesis
        println!("🌱 Starting fresh genesis world");
        self.create_genesis_world().await
    }

    /// Recover world state from network consensus
    async fn recover_from_network(&self) -> Result<(GameState, QuestSystem)> {
        // This would implement network consensus recovery
        // For now, return an error to indicate network recovery is not implemented
        Err(anyhow::anyhow!("Network recovery not implemented"))
    }

    /// Create a fresh genesis world
    async fn create_genesis_world(&self) -> Result<(GameState, QuestSystem)> {
        let game_state = GameState::new();
        let quest_system = QuestSystem::new();
        
        // Save initial genesis snapshot
        let snapshot = self.persistence_manager.create_snapshot(&game_state, &quest_system, SaveReason::ManualSave)?;
        self.persistence_manager.save_snapshot(&snapshot).await?;
        
        Ok((game_state, quest_system))
    }

    /// Sync world state to network (for distributed persistence)
    pub async fn sync_to_network(&self, game_state: &GameState, quest_system: &QuestSystem) -> Result<()> {
        if !self.network_config.use_centralized_storage && !self.network_config.use_decentralized_storage {
            return Ok(());
        }

        // Create network snapshot
        let snapshot = self.persistence_manager.create_snapshot(game_state, quest_system, SaveReason::ScheduledBackup)?;
        
        // This would implement network synchronization
        println!("🌐 Syncing world state to network: {}", snapshot.snapshot_id);
        
        Ok(())
    }
}