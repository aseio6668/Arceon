use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, Duration};
use tokio::sync::RwLock;
use tracing::{info, warn, debug};
use uuid::Uuid;
use std::sync::Arc;

use crate::entities::{Item}; // Removed unused Being, Skill, Area imports
use crate::systems::VitalManager;

/// Comprehensive AFK/Idle mode system that allows characters to perform automated tasks
/// including combat, gathering, and crafting while online or offline
pub struct AFKSystem {
    /// Active AFK sessions by player ID
    active_sessions: Arc<RwLock<HashMap<Uuid, AFKSession>>>,
    /// Offline sessions that continue running when player disconnects
    offline_sessions: Arc<RwLock<HashMap<Uuid, OfflineAFKSession>>>,
    /// Global AFK configuration
    config: AFKConfig,
    /// AI behavior engine for AFK activities
    ai_engine: Arc<RwLock<AFKAIEngine>>,
    /// Performance metrics and statistics
    metrics: Arc<RwLock<AFKMetrics>>,
    /// Command queue for AFK operations
    command_queue: Arc<RwLock<VecDeque<AFKCommand>>>,
    /// Integration with other systems (systems not implemented - commented out)
    // combat_system: Option<Arc<RwLock<CombatSystem>>>,
    // crafting_system: Option<Arc<RwLock<CraftingSystem>>>,
    // gathering_system: Option<Arc<RwLock<GatheringSystem>>>,
    vital_manager: Option<Arc<RwLock<VitalManager>>>,
}

/// Active AFK session for online players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFKSession {
    pub session_id: Uuid,
    pub player_id: Uuid,
    pub character_id: Uuid,
    pub afk_mode: AFKMode,
    pub start_time: SystemTime,
    pub current_activity: Option<AFKActivity>,
    pub session_stats: AFKSessionStats,
    pub safety_settings: AFKSafetySettings,
    pub is_paused: bool,
    pub pause_reason: Option<String>,
    pub last_activity: SystemTime,
    pub total_runtime: Duration,
    pub offline_eligible: bool, // Can continue offline
}

/// Offline AFK session that persists when player disconnects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineAFKSession {
    pub session_id: Uuid,
    pub player_id: Uuid,
    pub character_id: Uuid,
    pub original_session: AFKSession,
    pub offline_start_time: SystemTime,
    pub offline_progress: OfflineProgress,
    pub will_expire_at: SystemTime,
    pub max_offline_duration: Duration,
    pub periodic_saves: Vec<OfflineProgressSnapshot>,
}

/// Different AFK modes available to players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AFKMode {
    /// Auto-combat mode for fighting NPCs and collecting loot
    Combat {
        target_types: Vec<String>, // NPC types to target
        auto_loot: bool,
        combat_radius: f64,
        retreat_on_low_health: bool,
        use_consumables: bool,
        skill_rotation: Vec<String>, // Skill names to rotate through
        priority_targets: Vec<String>, // High-priority target types
    },
    /// Auto-gathering mode for collecting resources
    Gathering {
        resource_types: Vec<String>, // Resource types to gather
        gathering_radius: f64,
        auto_move_to_resources: bool,
        ignore_dangerous_areas: bool,
        preferred_tools: Vec<String>,
        min_resource_quality: u8,
    },
    /// Auto-crafting mode for creating items
    Crafting {
        craft_queue: VecDeque<CraftingTask>,
        auto_restock_materials: bool,
        crafting_station_id: Option<Uuid>,
        skill_focus: Option<String>, // Skill to focus on for XP
        quality_threshold: u8, // Minimum quality to accept
        auto_salvage_failures: bool,
    },
    /// Hybrid mode combining multiple activities
    Hybrid {
        primary_mode: Box<AFKMode>,
        secondary_modes: Vec<AFKMode>,
        mode_switching_conditions: ModeSwitchingRules,
    },
}

/// Current activity being performed in AFK mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFKActivity {
    pub activity_type: AFKActivityType,
    pub target_id: Option<Uuid>, // Target being, resource, or crafting station
    pub started_at: SystemTime,
    pub estimated_completion: Option<SystemTime>,
    pub progress: f32, // 0.0 to 1.0
    pub interruption_count: u32,
    pub last_action_time: SystemTime,
}

/// Types of activities that can be performed AFK
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AFKActivityType {
    Fighting { target_being_id: Uuid },
    MovingToTarget { destination: (f64, f64) },
    Gathering { resource_id: Uuid },
    Crafting { item_name: String, quantity: u32 },
    Healing,
    RestockingSupplies,
    Banking,
    Traveling { destination_area: String },
    Waiting { reason: String },
}

/// Statistics for an AFK session
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AFKSessionStats {
    pub enemies_defeated: u32,
    pub resources_gathered: u32,
    pub items_crafted: u32,
    pub experience_gained: HashMap<String, u64>, // skill -> xp
    pub items_looted: Vec<Item>,
    pub gold_earned: u64,
    pub deaths: u32,
    pub resurrections: u32,
    pub total_damage_dealt: u64,
    pub total_damage_taken: u64,
    pub consumables_used: HashMap<String, u32>, // item_name -> quantity
    pub distance_traveled: f64,
    pub activities_completed: u32,
    pub interruptions: u32,
}

/// Safety settings to prevent unwanted behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AFKSafetySettings {
    pub max_death_count: u32, // Stop after X deaths
    pub min_health_threshold: f32, // Stop if health drops below X%
    pub max_session_duration: Duration,
    pub auto_pause_on_player_proximity: bool,
    pub auto_pause_on_guild_member_proximity: bool,
    pub emergency_logout_conditions: Vec<EmergencyCondition>,
    pub resource_protection: ResourceProtectionSettings,
    pub anti_griefing_measures: bool,
}

/// Conditions that trigger emergency logout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmergencyCondition {
    HealthBelowPercent(f32),
    TargetedByPlayer,
    ItemDurabilityTooLow,
    InventoryFull,
    OutOfConsumables,
    UnexpectedPlayerInteraction,
    SystemOverload,
}

/// Settings for protecting resources and inventory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceProtectionSettings {
    pub protect_valuable_items: bool,
    pub valuable_item_threshold: u64, // Gold value threshold
    pub bank_items_when_full: bool,
    pub auto_repair_equipment: bool,
    pub max_repair_cost: u64,
}

/// Rules for switching between different AFK modes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSwitchingRules {
    pub switch_conditions: Vec<ModeSwitchCondition>,
    pub cooldown_between_switches: Duration,
    pub max_switches_per_hour: u32,
    pub priority_order: Vec<AFKMode>,
}

/// Condition that triggers a mode switch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeSwitchCondition {
    pub condition_type: SwitchConditionType,
    pub target_mode: AFKMode,
    pub cooldown: Duration,
    pub priority: u8,
}

/// Types of conditions that can trigger mode switches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwitchConditionType {
    InventoryFull,
    NoTargetsInRange,
    ResourcesDepleted,
    LowOnSupplies,
    CraftingQueueEmpty,
    HealthLow,
    TimeOfDay(u8), // Hour of day
    PlayerCount(u32), // Switch when X players nearby
}

/// Individual crafting task in the queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingTask {
    pub task_id: Uuid,
    pub recipe_name: String,
    pub quantity: u32,
    pub priority: u8,
    pub required_materials: HashMap<String, u32>, // item_name -> quantity
    pub estimated_time: Duration,
    pub skill_requirements: HashMap<String, u32>, // skill -> level
    pub auto_gather_materials: bool,
}

/// Progress tracking for offline sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineProgress {
    pub activities_completed: Vec<CompletedOfflineActivity>,
    pub current_activity: Option<AFKActivity>,
    pub resources_consumed: HashMap<String, u32>,
    pub rewards_earned: OfflineRewards,
    pub time_remaining: Duration,
    pub efficiency_rating: f32, // 0.0 to 1.0, affects offline gains
}

/// Completed activity during offline session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedOfflineActivity {
    pub activity_type: AFKActivityType,
    pub completed_at: SystemTime,
    pub rewards: ActivityRewards,
    pub duration: Duration,
    pub success_rate: f32,
}

/// Rewards earned from offline AFK activities
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OfflineRewards {
    pub experience_gained: HashMap<String, u64>,
    pub items_obtained: Vec<Item>,
    pub gold_earned: u64,
    pub skill_improvements: HashMap<String, u32>, // skill -> levels gained
    pub achievements_unlocked: Vec<String>,
}

/// Rewards from a specific activity
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActivityRewards {
    pub experience: u64,
    pub items: Vec<Item>,
    pub gold: u64,
    pub skill_xp: HashMap<String, u64>,
}

/// Periodic snapshot of offline progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineProgressSnapshot {
    pub timestamp: SystemTime,
    pub progress: OfflineProgress,
    pub character_state: OfflineCharacterState,
}

/// Character state during offline session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineCharacterState {
    pub health: f32,
    pub mana: f32,
    pub stamina: f32,
    pub location: (f64, f64),
    pub area_id: String,
    pub inventory_state: InventorySnapshot,
    pub equipment_durability: HashMap<String, f32>,
}

/// Snapshot of inventory state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySnapshot {
    pub total_items: u32,
    pub capacity_used: f32,
    pub valuable_items_count: u32,
    pub consumables_remaining: HashMap<String, u32>,
}

/// AI engine for controlling AFK behavior
#[derive(Debug, Clone)]
pub struct AFKAIEngine {
    /// Behavior trees for different AFK modes
    behavior_trees: HashMap<String, AFKBehaviorTree>,
    /// Decision making parameters
    decision_weights: AFKDecisionWeights,
    /// Learning system for improving AFK efficiency
    learning_system: AFKLearningSystem,
    /// Threat assessment for safety
    threat_assessor: ThreatAssessmentSystem,
}

/// Behavior tree for AFK AI decision making
#[derive(Debug, Clone)]
pub struct AFKBehaviorTree {
    pub name: String,
    pub root_node: AFKBehaviorNode,
    pub variables: HashMap<String, f32>,
    pub success_rate: f32,
    pub last_updated: SystemTime,
}

/// Node in AFK behavior tree
#[derive(Debug, Clone)]
pub enum AFKBehaviorNode {
    Sequence(Vec<AFKBehaviorNode>),
    Selector(Vec<AFKBehaviorNode>),
    Condition(AFKCondition),
    Action(AFKAction),
    Decorator {
        decorator_type: DecoratorType,
        child: Box<AFKBehaviorNode>,
    },
}

/// Condition check in behavior tree
#[derive(Debug, Clone)]
pub enum AFKCondition {
    HealthAbove(f32),
    ManaAbove(f32),
    EnemiesInRange(f64),
    ResourcesAvailable,
    InventoryNotFull,
    MaterialsAvailable(HashMap<String, u32>),
    SafeArea,
    TimeElapsed(Duration),
}

/// Action to perform in behavior tree
#[derive(Debug, Clone)]
pub enum AFKAction {
    AttackNearestEnemy,
    UseSkill(String),
    MoveToResource,
    GatherResource,
    CraftItem(String),
    Heal,
    Rest,
    Bank,
    Repair,
    Restock,
    Flee,
    Wait(Duration),
}

/// Decorator types for behavior nodes
#[derive(Debug, Clone)]
pub enum DecoratorType {
    Repeat(u32),
    RepeatUntilSuccess,
    RepeatUntilFailure,
    Inverter,
    Timer(Duration),
    Cooldown(Duration),
}

/// Decision making weights for AI
#[derive(Debug, Clone)]
pub struct AFKDecisionWeights {
    pub safety_priority: f32,
    pub efficiency_priority: f32,
    pub experience_priority: f32,
    pub gold_priority: f32,
    pub exploration_tendency: f32,
    pub risk_tolerance: f32,
}

/// Learning system for improving AFK performance
#[derive(Debug, Clone)]
pub struct AFKLearningSystem {
    pub session_analytics: HashMap<Uuid, SessionAnalytics>,
    pub pattern_recognition: PatternRecognition,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
    pub performance_history: VecDeque<PerformanceSnapshot>,
}

/// Analytics for a specific AFK session
#[derive(Debug, Clone)]
pub struct SessionAnalytics {
    pub efficiency_metrics: EfficiencyMetrics,
    pub bottlenecks_identified: Vec<Bottleneck>,
    pub optimal_timings: HashMap<String, Duration>,
    pub success_patterns: Vec<SuccessPattern>,
}

/// Efficiency metrics for performance analysis
#[derive(Debug, Clone)]
pub struct EfficiencyMetrics {
    pub xp_per_hour: HashMap<String, f64>,
    pub gold_per_hour: f64,
    pub items_per_hour: f64,
    pub death_rate: f32,
    pub downtime_percentage: f32,
    pub resource_utilization: f32,
}

/// Identified bottleneck in AFK performance
#[derive(Debug, Clone)]
pub struct Bottleneck {
    pub bottleneck_type: BottleneckType,
    pub impact_severity: f32,
    pub suggested_solutions: Vec<String>,
    pub frequency: u32,
}

#[derive(Debug, Clone)]
pub enum BottleneckType {
    InventoryManagement,
    ResourceDepletion,
    CombatInefficiency,
    MovementOptimization,
    SupplyManagement,
    SkillRotation,
    TimingIssues,
}

/// Recognized success pattern
#[derive(Debug, Clone)]
pub struct SuccessPattern {
    pub pattern_name: String,
    pub conditions: Vec<String>,
    pub success_rate: f32,
    pub average_reward: f64,
    pub recommended_usage: String,
}

/// Optimization suggestion for improving AFK performance
#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub suggestion_id: Uuid,
    pub category: OptimizationCategory,
    pub description: String,
    pub expected_improvement: f32,
    pub implementation_difficulty: u8,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum OptimizationCategory {
    SkillRotation,
    MovementPath,
    ResourceManagement,
    TimingOptimization,
    SafetyImprovement,
    EfficiencyBoost,
}

/// Performance snapshot for trend analysis
#[derive(Debug, Clone)]
pub struct PerformanceSnapshot {
    pub timestamp: SystemTime,
    pub session_count: u32,
    pub average_efficiency: f32,
    pub common_issues: Vec<String>,
    pub top_performing_setups: Vec<AFKSetup>,
}

/// High-performing AFK setup configuration
#[derive(Debug, Clone)]
pub struct AFKSetup {
    pub setup_name: String,
    pub mode_configuration: AFKMode,
    pub safety_settings: AFKSafetySettings,
    pub performance_rating: f32,
    pub usage_frequency: u32,
    pub average_session_duration: Duration,
}

/// Threat assessment system for safety
#[derive(Debug, Clone)]
pub struct ThreatAssessmentSystem {
    pub threat_levels: HashMap<String, ThreatLevel>, // area_id -> threat_level
    pub player_threat_history: HashMap<Uuid, PlayerThreatProfile>,
    pub environmental_hazards: HashMap<String, Vec<EnvironmentalHazard>>,
    pub safe_zones: HashMap<String, SafeZoneInfo>,
}

/// Threat level assessment for areas
#[derive(Debug, Clone)]
pub enum ThreatLevel {
    Safe,        // No significant threats
    Low,         // Minor threats, manageable
    Moderate,    // Some risk, requires caution
    High,        // Dangerous, high chance of death
    Extreme,     // Almost certain death
    Unknown,     // Not enough data
}

/// Player threat profile for PvP assessment
#[derive(Debug, Clone)]
pub struct PlayerThreatProfile {
    pub player_id: Uuid,
    pub aggression_level: f32,
    pub skill_level_estimate: f32,
    pub last_hostile_action: Option<SystemTime>,
    pub kill_count: u32,
    pub reputation: i32,
}

/// Environmental hazard in an area
#[derive(Debug, Clone)]
pub struct EnvironmentalHazard {
    pub hazard_type: HazardType,
    pub location: (f64, f64),
    pub severity: f32,
    pub duration: Option<Duration>,
    pub affects_afk: bool,
}

#[derive(Debug, Clone)]
pub enum HazardType {
    PoisonGas,
    Lava,
    Trap,
    StrongEnemy,
    PvPZone,
    UnstableGround,
    MagicStorm,
    TimeDistortion,
}

/// Safe zone information
#[derive(Debug, Clone)]
pub struct SafeZoneInfo {
    pub zone_id: Uuid,
    pub location: (f64, f64),
    pub radius: f64,
    pub protection_level: ProtectionLevel,
    pub amenities: Vec<SafeZoneAmenity>,
}

#[derive(Debug, Clone)]
pub enum ProtectionLevel {
    Basic,      // Safe from NPCs only
    Enhanced,   // Safe from NPCs and most players
    Complete,   // Complete safety guarantee
    Temporary,  // Time-limited safety
}

#[derive(Debug, Clone)]
pub enum SafeZoneAmenity {
    Healing,
    Banking,
    Crafting,
    Trading,
    Resurrection,
    EquipmentRepair,
    SupplyVendor,
}

/// AFK system configuration
#[derive(Debug, Clone)]
pub struct AFKConfig {
    pub max_concurrent_sessions: u32,
    pub max_offline_duration: Duration,
    pub offline_efficiency_multiplier: f32, // Usually < 1.0
    pub auto_save_interval: Duration,
    pub max_session_duration: Duration,
    pub death_penalty_enabled: bool,
    pub pvp_protection_in_afk: bool,
    pub resource_competition_enabled: bool,
    pub anti_botting_measures: AntiBottingConfig,
    pub performance_limits: PerformanceLimits,
}

/// Anti-botting configuration
#[derive(Debug, Clone)]
pub struct AntiBottingConfig {
    pub require_periodic_interaction: bool,
    pub interaction_interval: Duration,
    pub randomize_behavior: bool,
    pub captcha_challenges: bool,
    pub behavioral_analysis: bool,
    pub max_continuous_afk_time: Duration,
}

/// Performance limits to prevent server overload
#[derive(Debug, Clone)]
pub struct PerformanceLimits {
    pub max_ai_calculations_per_second: u32,
    pub max_actions_per_minute: u32,
    pub max_movement_distance_per_second: f64,
    pub max_combat_engagements_per_minute: u32,
    pub max_crafting_operations_per_hour: u32,
}

/// Command for AFK system operations
#[derive(Debug, Clone)]
pub enum AFKCommand {
    StartSession {
        player_id: Uuid,
        character_id: Uuid,
        mode: AFKMode,
        safety_settings: AFKSafetySettings,
    },
    StopSession {
        session_id: Uuid,
        reason: String,
    },
    PauseSession {
        session_id: Uuid,
        reason: String,
    },
    ResumeSession {
        session_id: Uuid,
    },
    UpdateMode {
        session_id: Uuid,
        new_mode: AFKMode,
    },
    UpdateSafetySettings {
        session_id: Uuid,
        new_settings: AFKSafetySettings,
    },
    GoOffline {
        session_id: Uuid,
        max_offline_duration: Duration,
    },
    ReturnOnline {
        player_id: Uuid,
    },
    EmergencyStop {
        player_id: Uuid,
        reason: EmergencyCondition,
    },
}

/// Performance metrics for the AFK system
#[derive(Debug, Clone, Default)]
pub struct AFKMetrics {
    pub total_sessions: u64,
    pub active_sessions: u32,
    pub offline_sessions: u32,
    pub total_afk_time: Duration,
    pub average_session_duration: Duration,
    pub total_experience_gained: u64,
    pub total_gold_earned: u64,
    pub total_items_obtained: u64,
    pub death_rate: f32,
    pub efficiency_rating: f32,
    pub popular_modes: HashMap<String, u32>, // mode_name -> usage_count
    pub system_performance: SystemPerformanceMetrics,
}

/// System performance metrics
#[derive(Debug, Clone, Default)]
pub struct SystemPerformanceMetrics {
    pub average_cpu_usage: f32,
    pub peak_memory_usage: u64,
    pub average_response_time: Duration,
    pub error_rate: f32,
    pub uptime_percentage: f32,
}

impl AFKSystem {
    /// Create a new AFK system with default configuration
    pub async fn new(config: AFKConfig) -> Result<Self> {
        info!("🤖 Initializing AFK/Idle Mode System");

        let ai_engine = AFKAIEngine::new().await?;

        Ok(Self {
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            offline_sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
            ai_engine: Arc::new(RwLock::new(ai_engine)),
            metrics: Arc::new(RwLock::new(AFKMetrics::default())),
            command_queue: Arc::new(RwLock::new(VecDeque::new())),
            // combat_system: None,
            // crafting_system: None,
            // gathering_system: None,
            vital_manager: None,
        })
    }

    /// Start an AFK session for a player
    pub async fn start_afk_session(
        &self,
        player_id: Uuid,
        character_id: Uuid,
        mode: AFKMode,
        safety_settings: AFKSafetySettings,
    ) -> Result<Uuid> {
        info!("🎮 Starting AFK session for player {} in mode {:?}", player_id, std::mem::discriminant(&mode));

        // Validate session prerequisites
        self.validate_afk_prerequisites(player_id, &mode).await?;

        let session_id = Uuid::new_v4();
        let session = AFKSession {
            session_id,
            player_id,
            character_id,
            afk_mode: mode.clone(),
            start_time: SystemTime::now(),
            current_activity: None,
            session_stats: AFKSessionStats::default(),
            safety_settings,
            is_paused: false,
            pause_reason: None,
            last_activity: SystemTime::now(),
            total_runtime: Duration::default(),
            offline_eligible: true,
        };

        // Add to active sessions
        let mut active_sessions = self.active_sessions.write().await;
        active_sessions.insert(session_id, session);
        drop(active_sessions);

        // Initialize AI behavior for this mode
        let mut ai_engine = self.ai_engine.write().await;
        ai_engine.initialize_session_behavior(session_id, &mode).await?;
        drop(ai_engine);

        // Start session processing
        self.process_afk_session(session_id).await?;

        info!("✅ AFK session started successfully: {}", session_id);
        Ok(session_id)
    }

    /// Stop an AFK session
    pub async fn stop_afk_session(&self, session_id: Uuid, reason: String) -> Result<AFKSessionStats> {
        info!("🛑 Stopping AFK session: {} (Reason: {})", session_id, reason);

        let session_stats = {
            let mut active_sessions = self.active_sessions.write().await;
            if let Some(session) = active_sessions.remove(&session_id) {
                // Update metrics
                let mut metrics = self.metrics.write().await;
                metrics.total_sessions += 1;
                metrics.active_sessions = active_sessions.len() as u32;
                drop(metrics);

                session.session_stats
            } else {
                return Err(anyhow::anyhow!("AFK session not found: {}", session_id));
            }
        };

        // Clean up AI resources
        let mut ai_engine = self.ai_engine.write().await;
        ai_engine.cleanup_session(session_id).await?;

        info!("✅ AFK session stopped: {}", session_id);
        Ok(session_stats)
    }

    /// Transition session to offline mode
    pub async fn go_offline(&self, session_id: Uuid, max_offline_duration: Duration) -> Result<()> {
        info!("🌙 Transitioning AFK session to offline mode: {}", session_id);

        let session = {
            let mut active_sessions = self.active_sessions.write().await;
            match active_sessions.remove(&session_id) {
                Some(session) => session,
                None => return Err(anyhow::anyhow!("AFK session not found: {}", session_id)),
            }
        };

        if !session.offline_eligible {
            return Err(anyhow::anyhow!("Session is not eligible for offline mode"));
        }

        // Create offline session
        let offline_session = OfflineAFKSession {
            session_id,
            player_id: session.player_id,
            character_id: session.character_id,
            original_session: session,
            offline_start_time: SystemTime::now(),
            offline_progress: OfflineProgress {
                activities_completed: Vec::new(),
                current_activity: None,
                resources_consumed: HashMap::new(),
                rewards_earned: OfflineRewards::default(),
                time_remaining: max_offline_duration,
                efficiency_rating: 0.8, // Offline is slightly less efficient
            },
            will_expire_at: SystemTime::now() + max_offline_duration,
            max_offline_duration,
            periodic_saves: Vec::new(),
        };

        // Add to offline sessions
        let mut offline_sessions = self.offline_sessions.write().await;
        offline_sessions.insert(session_id, offline_session);
        drop(offline_sessions);

        // Update metrics
        let mut metrics = self.metrics.write().await;
        metrics.offline_sessions += 1;
        metrics.active_sessions -= 1;

        info!("✅ Session transitioned to offline mode: {}", session_id);
        Ok(())
    }

    /// Return from offline mode to online
    pub async fn return_online(&self, player_id: Uuid) -> Result<Option<AFKSessionStats>> {
        info!("☀️ Player returning online: {}", player_id);

        let offline_session = {
            let mut offline_sessions = self.offline_sessions.write().await;
            offline_sessions
                .iter()
                .find(|(_, session)| session.player_id == player_id)
                .map(|(session_id, _)| *session_id)
                .and_then(|session_id| offline_sessions.remove(&session_id))
        };

        if let Some(offline_session) = offline_session {
            // Calculate offline progress
            let offline_duration = SystemTime::now()
                .duration_since(offline_session.offline_start_time)
                .unwrap_or_default();

            // Process offline activities
            let offline_rewards = self.calculate_offline_rewards(&offline_session, offline_duration).await?;

            // Create session stats from offline progress
            let mut session_stats = offline_session.original_session.session_stats.clone();
            
            // Calculate totals before moving values
            let total_xp: u64 = offline_rewards.experience_gained.values().sum();
            let total_items = offline_rewards.items_obtained.len();
            let total_gold = offline_rewards.gold_earned;
            
            session_stats.experience_gained.extend(offline_rewards.experience_gained);
            session_stats.items_looted.extend(offline_rewards.items_obtained);
            session_stats.gold_earned += offline_rewards.gold_earned;

            info!("✅ Player returned online with offline rewards: {} XP, {} gold, {} items",
                total_xp, total_gold, total_items
            );

            Ok(Some(session_stats))
        } else {
            info!("No offline session found for player: {}", player_id);
            Ok(None)
        }
    }

    /// Validate prerequisites for starting AFK mode
    async fn validate_afk_prerequisites(&self, player_id: Uuid, mode: &AFKMode) -> Result<()> {
        debug!("Validating AFK prerequisites for player: {}", player_id);

        // Check if player is already in AFK mode
        let active_sessions = self.active_sessions.read().await;
        if active_sessions.values().any(|s| s.player_id == player_id) {
            return Err(anyhow::anyhow!("Player already has an active AFK session"));
        }
        drop(active_sessions);

        // Validate mode-specific requirements
        match mode {
            AFKMode::Combat { target_types, skill_rotation, .. } => {
                // Validate combat prerequisites
                if target_types.is_empty() {
                    return Err(anyhow::anyhow!("Combat mode requires at least one target type"));
                }
                if skill_rotation.is_empty() {
                    warn!("No skill rotation specified for combat mode, using basic attacks only");
                }
            }
            AFKMode::Gathering { resource_types, .. } => {
                if resource_types.is_empty() {
                    return Err(anyhow::anyhow!("Gathering mode requires at least one resource type"));
                }
            }
            AFKMode::Crafting { craft_queue, .. } => {
                if craft_queue.is_empty() {
                    return Err(anyhow::anyhow!("Crafting mode requires at least one item in craft queue"));
                }
            }
            AFKMode::Hybrid { primary_mode, secondary_modes, .. } => {
                Box::pin(self.validate_afk_prerequisites(player_id, primary_mode)).await?;
                for secondary_mode in secondary_modes {
                    Box::pin(self.validate_afk_prerequisites(player_id, secondary_mode)).await?;
                }
            }
        }

        // Check system limits
        let active_count = self.active_sessions.read().await.len() as u32;
        if active_count >= self.config.max_concurrent_sessions {
            return Err(anyhow::anyhow!("Maximum concurrent AFK sessions reached"));
        }

        Ok(())
    }

    /// Process an active AFK session
    async fn process_afk_session(&self, session_id: Uuid) -> Result<()> {
        let session_clone = {
            let active_sessions = self.active_sessions.read().await;
            match active_sessions.get(&session_id) {
                Some(session) => session.clone(),
                None => return Ok(()), // Session was stopped
            }
        };

        if session_clone.is_paused {
            debug!("AFK session is paused: {}", session_id);
            return Ok(());
        }

        // Get next action from AI engine
        let ai_engine = self.ai_engine.read().await;
        let next_action = ai_engine.get_next_action(session_id, &session_clone).await?;
        drop(ai_engine);

        // Execute the action
        match next_action {
            AFKAction::AttackNearestEnemy => {
                self.execute_combat_action(session_id).await?;
            }
            AFKAction::GatherResource => {
                self.execute_gathering_action(session_id).await?;
            }
            AFKAction::CraftItem(item_name) => {
                self.execute_crafting_action(session_id, item_name).await?;
            }
            AFKAction::Heal => {
                self.execute_healing_action(session_id).await?;
            }
            AFKAction::Bank => {
                self.execute_banking_action(session_id).await?;
            }
            AFKAction::Wait(duration) => {
                debug!("AFK session waiting: {} for {:?}", session_id, duration);
                tokio::time::sleep(duration).await;
            }
            _ => {
                debug!("Unimplemented AFK action: {:?}", next_action);
            }
        }

        // Update session activity
        self.update_session_activity(session_id).await?;

        Ok(())
    }

    /// Execute combat action for AFK session
    async fn execute_combat_action(&self, session_id: Uuid) -> Result<()> {
        debug!("Executing combat action for AFK session: {}", session_id);

        // Combat system integration disabled (system not implemented)
        // if let Some(combat_system) = &self.combat_system {
        //     let combat_system = combat_system.read().await;
        //     // Execute combat logic
        //     // combat_system.auto_attack(character_id, target_id).await?;
        // }

        // Update session stats
        let mut active_sessions = self.active_sessions.write().await;
        if let Some(session) = active_sessions.get_mut(&session_id) {
            session.session_stats.enemies_defeated += 1;
            session.last_activity = SystemTime::now();
        }

        Ok(())
    }

    /// Execute gathering action for AFK session
    async fn execute_gathering_action(&self, session_id: Uuid) -> Result<()> {
        debug!("Executing gathering action for AFK session: {}", session_id);

        // Gathering system integration disabled (system not implemented)
        // if let Some(gathering_system) = &self.gathering_system {
        //     let gathering_system = gathering_system.read().await;
        //     // Execute gathering logic
        //     // gathering_system.auto_gather(character_id, resource_id).await?;
        // }

        // Update session stats
        let mut active_sessions = self.active_sessions.write().await;
        if let Some(session) = active_sessions.get_mut(&session_id) {
            session.session_stats.resources_gathered += 1;
            session.last_activity = SystemTime::now();
        }

        Ok(())
    }

    /// Execute crafting action for AFK session
    async fn execute_crafting_action(&self, session_id: Uuid, item_name: String) -> Result<()> {
        debug!("Executing crafting action for AFK session: {} (Item: {})", session_id, item_name);

        // Crafting system integration disabled (system not implemented)
        // if let Some(crafting_system) = &self.crafting_system {
        //     let crafting_system = crafting_system.read().await;
        //     // Execute crafting logic
        //     // crafting_system.auto_craft(character_id, item_name).await?;
        // }

        // Update session stats
        let mut active_sessions = self.active_sessions.write().await;
        if let Some(session) = active_sessions.get_mut(&session_id) {
            session.session_stats.items_crafted += 1;
            session.last_activity = SystemTime::now();
        }

        Ok(())
    }

    /// Execute healing action for AFK session
    async fn execute_healing_action(&self, session_id: Uuid) -> Result<()> {
        debug!("Executing healing action for AFK session: {}", session_id);

        // This would integrate with the vital manager
        if let Some(vital_manager) = &self.vital_manager {
            let _vital_manager = vital_manager.read().await;
            // Execute healing logic
            // vital_manager.auto_heal(character_id).await?;
        }

        Ok(())
    }

    /// Execute banking action for AFK session
    async fn execute_banking_action(&self, session_id: Uuid) -> Result<()> {
        debug!("Executing banking action for AFK session: {}", session_id);
        
        // Banking logic would go here
        // This would involve depositing items, withdrawing supplies, etc.

        Ok(())
    }

    /// Update session activity tracking
    async fn update_session_activity(&self, session_id: Uuid) -> Result<()> {
        let mut active_sessions = self.active_sessions.write().await;
        if let Some(session) = active_sessions.get_mut(&session_id) {
            session.last_activity = SystemTime::now();
            session.total_runtime = SystemTime::now()
                .duration_since(session.start_time)
                .unwrap_or_default();

            // Check safety conditions
            if let Err(safety_error) = self.check_safety_conditions(&session).await {
                warn!("Safety check failed for session {}: {}", session_id, safety_error);
                session.is_paused = true;
                session.pause_reason = Some(safety_error.to_string());
            }
        }

        Ok(())
    }

    /// Check safety conditions for an AFK session
    async fn check_safety_conditions(&self, session: &AFKSession) -> Result<()> {
        // Check death count
        if session.session_stats.deaths >= session.safety_settings.max_death_count {
            return Err(anyhow::anyhow!("Maximum death count exceeded"));
        }

        // Check session duration
        if session.total_runtime >= session.safety_settings.max_session_duration {
            return Err(anyhow::anyhow!("Maximum session duration exceeded"));
        }

        // Additional safety checks would go here
        
        Ok(())
    }

    /// Calculate rewards for offline session
    async fn calculate_offline_rewards(
        &self,
        offline_session: &OfflineAFKSession,
        offline_duration: Duration,
    ) -> Result<OfflineRewards> {
        debug!("Calculating offline rewards for session: {} (Duration: {:?})", 
            offline_session.session_id, offline_duration);

        let mut rewards = OfflineRewards::default();

        // Apply offline efficiency multiplier
        let efficiency = offline_session.offline_progress.efficiency_rating * self.config.offline_efficiency_multiplier;

        // Calculate experience based on AFK mode and duration
        match &offline_session.original_session.afk_mode {
            AFKMode::Combat { .. } => {
                let base_xp_per_hour = 1000_u64;
                let combat_xp = (base_xp_per_hour as f32 * offline_duration.as_secs_f32() / 3600.0 * efficiency) as u64;
                rewards.experience_gained.insert("Combat".to_string(), combat_xp);
                rewards.gold_earned = (combat_xp / 10) as u64; // 1 gold per 10 XP
            }
            AFKMode::Gathering { .. } => {
                let base_xp_per_hour = 800_u64;
                let gathering_xp = (base_xp_per_hour as f32 * offline_duration.as_secs_f32() / 3600.0 * efficiency) as u64;
                rewards.experience_gained.insert("Gathering".to_string(), gathering_xp);
            }
            AFKMode::Crafting { .. } => {
                let base_xp_per_hour = 600_u64;
                let crafting_xp = (base_xp_per_hour as f32 * offline_duration.as_secs_f32() / 3600.0 * efficiency) as u64;
                rewards.experience_gained.insert("Crafting".to_string(), crafting_xp);
            }
            AFKMode::Hybrid { .. } => {
                // Mixed rewards for hybrid mode
                let base_xp_per_hour = 700_u64;
                let total_xp = (base_xp_per_hour as f32 * offline_duration.as_secs_f32() / 3600.0 * efficiency) as u64;
                rewards.experience_gained.insert("Mixed".to_string(), total_xp);
                rewards.gold_earned = (total_xp / 15) as u64;
            }
        }

        info!("Calculated offline rewards: {} total XP, {} gold", 
            rewards.experience_gained.values().sum::<u64>(), rewards.gold_earned);

        Ok(rewards)
    }

    /// Get comprehensive AFK system statistics
    pub async fn get_system_stats(&self) -> AFKMetrics {
        self.metrics.read().await.clone()
    }

    /// Get active session information
    pub async fn get_active_session(&self, session_id: Uuid) -> Option<AFKSession> {
        let active_sessions = self.active_sessions.read().await;
        active_sessions.get(&session_id).cloned()
    }

    /// Get all active sessions for a player
    pub async fn get_player_sessions(&self, player_id: Uuid) -> Vec<AFKSession> {
        let active_sessions = self.active_sessions.read().await;
        active_sessions
            .values()
            .filter(|session| session.player_id == player_id)
            .cloned()
            .collect()
    }
}

impl AFKAIEngine {
    /// Create a new AFK AI engine
    pub async fn new() -> Result<Self> {
        let mut behavior_trees = HashMap::new();
        
        // Create default behavior trees for each AFK mode
        behavior_trees.insert("combat".to_string(), Self::create_combat_behavior_tree());
        behavior_trees.insert("gathering".to_string(), Self::create_gathering_behavior_tree());
        behavior_trees.insert("crafting".to_string(), Self::create_crafting_behavior_tree());

        Ok(Self {
            behavior_trees,
            decision_weights: AFKDecisionWeights {
                safety_priority: 0.8,
                efficiency_priority: 0.6,
                experience_priority: 0.7,
                gold_priority: 0.5,
                exploration_tendency: 0.3,
                risk_tolerance: 0.4,
            },
            learning_system: AFKLearningSystem {
                session_analytics: HashMap::new(),
                pattern_recognition: PatternRecognition::new(),
                optimization_suggestions: Vec::new(),
                performance_history: VecDeque::new(),
            },
            threat_assessor: ThreatAssessmentSystem {
                threat_levels: HashMap::new(),
                player_threat_history: HashMap::new(),
                environmental_hazards: HashMap::new(),
                safe_zones: HashMap::new(),
            },
        })
    }

    /// Initialize behavior for a new AFK session
    pub async fn initialize_session_behavior(&mut self, session_id: Uuid, mode: &AFKMode) -> Result<()> {
        debug!("Initializing AI behavior for AFK session: {}", session_id);

        let behavior_tree_name = match mode {
            AFKMode::Combat { .. } => "combat",
            AFKMode::Gathering { .. } => "gathering", 
            AFKMode::Crafting { .. } => "crafting",
            AFKMode::Hybrid { primary_mode, .. } => {
                // Use primary mode's behavior tree for hybrid
                match primary_mode.as_ref() {
                    AFKMode::Combat { .. } => "combat",
                    AFKMode::Gathering { .. } => "gathering",
                    AFKMode::Crafting { .. } => "crafting",
                    _ => "combat", // Default fallback
                }
            }
        };

        // Initialize session-specific variables in behavior tree
        if let Some(behavior_tree) = self.behavior_trees.get_mut(behavior_tree_name) {
            behavior_tree.variables.insert("session_id".to_string(), session_id.as_u128() as f32);
            behavior_tree.last_updated = SystemTime::now();
        }

        Ok(())
    }

    /// Get the next action for an AFK session
    pub async fn get_next_action(&self, _session_id: Uuid, session: &AFKSession) -> Result<AFKAction> {
        let behavior_tree_name = match &session.afk_mode {
            AFKMode::Combat { .. } => "combat",
            AFKMode::Gathering { .. } => "gathering",
            AFKMode::Crafting { .. } => "crafting", 
            AFKMode::Hybrid { primary_mode, .. } => {
                match primary_mode.as_ref() {
                    AFKMode::Combat { .. } => "combat",
                    AFKMode::Gathering { .. } => "gathering",
                    AFKMode::Crafting { .. } => "crafting",
                    _ => "combat",
                }
            }
        };

        if let Some(behavior_tree) = self.behavior_trees.get(behavior_tree_name) {
            // Execute behavior tree and return the resulting action
            self.execute_behavior_tree(&behavior_tree.root_node, session).await
        } else {
            // Default action if no behavior tree found
            Ok(AFKAction::Wait(Duration::from_secs(1)))
        }
    }

    /// Execute a behavior tree node
    async fn execute_behavior_tree(&self, node: &AFKBehaviorNode, session: &AFKSession) -> Result<AFKAction> {
        match node {
            AFKBehaviorNode::Action(action) => Ok(action.clone()),
            AFKBehaviorNode::Condition(condition) => {
                // Conditions don't return actions directly
                if self.evaluate_condition(condition, session).await {
                    Ok(AFKAction::Wait(Duration::from_millis(100)))
                } else {
                    Ok(AFKAction::Wait(Duration::from_secs(1)))
                }
            }
            AFKBehaviorNode::Sequence(nodes) => {
                // Execute first successful node in sequence
                for node in nodes {
                    let action = Box::pin(self.execute_behavior_tree(node, session)).await?;
                    if !matches!(action, AFKAction::Wait(_)) {
                        return Ok(action);
                    }
                }
                Ok(AFKAction::Wait(Duration::from_secs(1)))
            }
            AFKBehaviorNode::Selector(nodes) => {
                // Execute first applicable node
                for node in nodes {
                    let action = Box::pin(self.execute_behavior_tree(node, session)).await?;
                    return Ok(action);
                }
                Ok(AFKAction::Wait(Duration::from_secs(1)))
            }
            AFKBehaviorNode::Decorator { child, .. } => {
                // For now, just execute the child node
                Box::pin(self.execute_behavior_tree(child, session)).await
            }
        }
    }

    /// Evaluate a condition in the behavior tree
    async fn evaluate_condition(&self, condition: &AFKCondition, _session: &AFKSession) -> bool {
        match condition {
            AFKCondition::HealthAbove(_threshold) => {
                // Would check actual character health
                true // Placeholder
            }
            AFKCondition::ManaAbove(_threshold) => {
                // Would check actual character mana
                true // Placeholder  
            }
            AFKCondition::EnemiesInRange(_range) => {
                // Would check for enemies in range
                false // Placeholder
            }
            AFKCondition::ResourcesAvailable => {
                // Would check for available resources
                false // Placeholder
            }
            AFKCondition::InventoryNotFull => {
                // Would check inventory space
                true // Placeholder
            }
            AFKCondition::MaterialsAvailable(_materials) => {
                // Would check for crafting materials
                true // Placeholder
            }
            AFKCondition::SafeArea => {
                // Would check threat assessment
                true // Placeholder
            }
            AFKCondition::TimeElapsed(_duration) => {
                // Would check elapsed time
                false // Placeholder
            }
        }
    }

    /// Create combat behavior tree
    fn create_combat_behavior_tree() -> AFKBehaviorTree {
        let root_node = AFKBehaviorNode::Selector(vec![
            // Safety first - heal if health low
            AFKBehaviorNode::Sequence(vec![
                AFKBehaviorNode::Condition(AFKCondition::HealthAbove(0.3)),
                AFKBehaviorNode::Action(AFKAction::Heal),
            ]),
            // Attack enemies if available
            AFKBehaviorNode::Sequence(vec![
                AFKBehaviorNode::Condition(AFKCondition::EnemiesInRange(10.0)),
                AFKBehaviorNode::Condition(AFKCondition::SafeArea),
                AFKBehaviorNode::Action(AFKAction::AttackNearestEnemy),
            ]),
            // Default wait action
            AFKBehaviorNode::Action(AFKAction::Wait(Duration::from_secs(2))),
        ]);

        AFKBehaviorTree {
            name: "Combat".to_string(),
            root_node,
            variables: HashMap::new(),
            success_rate: 0.85,
            last_updated: SystemTime::now(),
        }
    }

    /// Create gathering behavior tree
    fn create_gathering_behavior_tree() -> AFKBehaviorTree {
        let root_node = AFKBehaviorNode::Selector(vec![
            // Check inventory space
            AFKBehaviorNode::Sequence(vec![
                AFKBehaviorNode::Condition(AFKCondition::InventoryNotFull),
                AFKBehaviorNode::Condition(AFKCondition::ResourcesAvailable),
                AFKBehaviorNode::Action(AFKAction::GatherResource),
            ]),
            // Bank items if inventory full
            AFKBehaviorNode::Sequence(vec![
                AFKBehaviorNode::Action(AFKAction::Bank),
            ]),
            // Default wait
            AFKBehaviorNode::Action(AFKAction::Wait(Duration::from_secs(3))),
        ]);

        AFKBehaviorTree {
            name: "Gathering".to_string(),
            root_node,
            variables: HashMap::new(),
            success_rate: 0.90,
            last_updated: SystemTime::now(),
        }
    }

    /// Create crafting behavior tree
    fn create_crafting_behavior_tree() -> AFKBehaviorTree {
        let root_node = AFKBehaviorNode::Selector(vec![
            // Craft items if materials available
            AFKBehaviorNode::Sequence(vec![
                AFKBehaviorNode::Condition(AFKCondition::MaterialsAvailable(HashMap::new())),
                AFKBehaviorNode::Action(AFKAction::CraftItem("DefaultItem".to_string())),
            ]),
            // Restock materials if needed
            AFKBehaviorNode::Action(AFKAction::Restock),
            // Default wait
            AFKBehaviorNode::Action(AFKAction::Wait(Duration::from_secs(5))),
        ]);

        AFKBehaviorTree {
            name: "Crafting".to_string(),
            root_node,
            variables: HashMap::new(),
            success_rate: 0.80,
            last_updated: SystemTime::now(),
        }
    }

    /// Clean up resources for a finished session
    pub async fn cleanup_session(&mut self, session_id: Uuid) -> Result<()> {
        debug!("Cleaning up AI resources for session: {}", session_id);

        // Remove session-specific data from behavior trees
        for behavior_tree in self.behavior_trees.values_mut() {
            behavior_tree.variables.remove(&format!("session_{}", session_id));
        }

        // Clean up analytics data
        self.learning_system.session_analytics.remove(&session_id);

        Ok(())
    }
}

// Placeholder implementations for referenced types
#[derive(Debug, Clone)]
pub struct PatternRecognition;

impl PatternRecognition {
    pub fn new() -> Self {
        Self
    }
}

impl Default for AFKConfig {
    fn default() -> Self {
        Self {
            max_concurrent_sessions: 1000,
            max_offline_duration: <Duration as DurationExt>::from_hours(8),
            offline_efficiency_multiplier: 0.75,
            auto_save_interval: <Duration as DurationExt>::from_minutes(5),
            max_session_duration: <Duration as DurationExt>::from_hours(12),
            death_penalty_enabled: false,
            pvp_protection_in_afk: true,
            resource_competition_enabled: true,
            anti_botting_measures: AntiBottingConfig {
                require_periodic_interaction: false,
                interaction_interval: <Duration as DurationExt>::from_hours(2),
                randomize_behavior: true,
                captcha_challenges: false,
                behavioral_analysis: true,
                max_continuous_afk_time: <Duration as DurationExt>::from_hours(12),
            },
            performance_limits: PerformanceLimits {
                max_ai_calculations_per_second: 100,
                max_actions_per_minute: 60,
                max_movement_distance_per_second: 10.0,
                max_combat_engagements_per_minute: 10,
                max_crafting_operations_per_hour: 1000,
            },
        }
    }
}

trait DurationExt {
    fn from_hours(hours: u64) -> Duration;
    fn from_minutes(minutes: u64) -> Duration;
}

impl DurationExt for Duration {
    fn from_hours(hours: u64) -> Duration {
        Duration::from_secs(hours * 3600)
    }

    fn from_minutes(minutes: u64) -> Duration {
        Duration::from_secs(minutes * 60)
    }
}