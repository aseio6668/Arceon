/*!
# Arena System

Arena and battleground management for PvP matches including
map selection, environmental features, and match coordination.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{ArenaId, MatchId, PlayerId, TeamId, PvPMatch, CombatEvent, MatchResult, CombatMode};

/// Arena management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaManager {
    pub arenas: HashMap<ArenaId, Arena>,
    pub active_matches: HashMap<MatchId, PvPMatch>,
    pub arena_templates: Vec<ArenaTemplate>,
    pub match_instances: HashMap<ArenaId, Vec<MatchInstance>>,
    pub server_instances: HashMap<String, ServerInstance>,
}

/// Individual arena/battleground
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arena {
    pub arena_id: ArenaId,
    pub name: String,
    pub description: String,
    pub arena_type: ArenaType,
    pub map_data: MapData,
    pub supported_modes: Vec<CombatMode>,
    pub capacity: ArenaCapacity,
    pub environmental_features: Vec<EnvironmentalFeature>,
    pub spawn_points: Vec<SpawnPoint>,
    pub objectives: Vec<ArenaObjective>,
    pub power_up_spawns: Vec<PowerUpSpawn>,
    pub settings: ArenaSettings,
    pub status: ArenaStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArenaType {
    Duel,           // Small 1v1 arenas
    Skirmish,       // Medium team battles
    Battleground,   // Large objective-based maps
    Siege,          // Castle/fortress assault
    Open,           // Open world PvP areas
    Custom,         // User-created arenas
}

/// Map geometry and layout data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub dimensions: (f64, f64, f64), // Width, Height, Depth
    pub terrain_type: TerrainType,
    pub elevation_map: Vec<Vec<f64>>,
    pub collision_boundaries: Vec<Boundary>,
    pub visual_theme: String,
    pub weather_conditions: Vec<WeatherCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TerrainType {
    Flat,
    Hilly,
    Mountainous,
    Urban,
    Forest,
    Desert,
    Swamp,
    Volcanic,
    Arctic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Boundary {
    pub boundary_id: Uuid,
    pub shape: BoundaryShape,
    pub boundary_type: BoundaryType,
    pub position: (f64, f64, f64),
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryShape {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Polygon { points: Vec<(f64, f64)> },
    Complex(String), // JSON geometry data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoundaryType {
    Wall,
    Water,
    Lava,
    Pit,
    Barrier,
    SafeZone,
    DeathZone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub condition_type: WeatherType,
    pub intensity: f64,
    pub duration_minutes: Option<u32>,
    pub effects: Vec<WeatherEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    Clear,
    Rain,
    Snow,
    Fog,
    Storm,
    Sandstorm,
    Blizzard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherEffect {
    pub effect_name: String,
    pub magnitude: f64,
    pub affected_stats: Vec<String>,
}

/// Arena capacity and limitations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaCapacity {
    pub min_players: u32,
    pub max_players: u32,
    pub optimal_players: u32,
    pub spectator_limit: Option<u32>,
    pub concurrent_matches: u32,
}

/// Environmental features and hazards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalFeature {
    pub feature_id: Uuid,
    pub name: String,
    pub feature_type: FeatureType,
    pub position: (f64, f64, f64),
    pub area_of_effect: Option<f64>,
    pub activation_conditions: Vec<ActivationCondition>,
    pub effects: Vec<EnvironmentalEffect>,
    pub duration: Option<u32>, // Duration in seconds
    pub cooldown: Option<u32>, // Cooldown in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureType {
    Trap,
    Hazard,
    Buff,
    Teleporter,
    MovingPlatform,
    DestructibleWall,
    Bridge,
    ElevationChange,
    Cover,
    Interactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationCondition {
    PlayerProximity(f64),
    TimeInterval(u32),
    PlayerCount(u32),
    HealthThreshold(f64),
    RandomChance(f64),
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalEffect {
    pub effect_name: String,
    pub effect_type: EffectType,
    pub magnitude: f64,
    pub target_type: EffectTarget,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Damage,
    Healing,
    Buff,
    Debuff,
    Teleport,
    Knockback,
    Stun,
    Slow,
    Speed,
    Invisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectTarget {
    All,
    Team(TeamId),
    Player(PlayerId),
    Proximity(f64),
    Random(u32),
}

/// Player spawn locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPoint {
    pub spawn_id: Uuid,
    pub position: (f64, f64, f64),
    pub orientation: f64, // Facing direction in radians
    pub team_assignment: Option<TeamId>,
    pub spawn_type: SpawnType,
    pub conditions: Vec<SpawnCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnType {
    Initial,
    Respawn,
    Reinforcement,
    Objective,
    Random,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnCondition {
    TeamControl(TeamId),
    ObjectiveStatus(Uuid),
    PlayerCount(u32),
    TimeElapsed(u32),
    Always,
}

/// Arena objectives for gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaObjective {
    pub objective_id: Uuid,
    pub name: String,
    pub objective_type: ObjectiveType,
    pub position: (f64, f64, f64),
    pub capture_radius: f64,
    pub capture_time: u32, // Time in seconds to capture
    pub control_requirements: ControlRequirements,
    pub rewards: Vec<ObjectiveReward>,
    pub status: ObjectiveStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    CapturePoint,
    FlagCarry,
    PayloadEscort,
    Artifact,
    Territory,
    Destruction,
    Defense,
    Collection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlRequirements {
    pub minimum_players: u32,
    pub contested_delay: u32, // Delay when contested
    pub decay_rate: f64, // How fast control decays without presence
    pub team_exclusive: bool, // Only one team can control
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveReward {
    pub reward_type: ObjectiveRewardType,
    pub amount: u64,
    pub frequency: RewardFrequency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveRewardType {
    Points,
    Resources,
    Buffs,
    MapControl,
    VictoryProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardFrequency {
    OnCapture,
    PerSecond,
    PerMinute,
    OnHold(u32), // Every X seconds while held
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveStatus {
    pub controlling_team: Option<TeamId>,
    pub capture_progress: f64, // 0.0 to 1.0
    pub contested: bool,
    pub last_interaction: DateTime<Utc>,
    pub total_control_time: HashMap<TeamId, u32>,
}

/// Power-up spawn locations and types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerUpSpawn {
    pub spawn_id: Uuid,
    pub position: (f64, f64, f64),
    pub power_up_types: Vec<PowerUpType>,
    pub spawn_interval: u32, // Seconds between spawns
    pub spawn_conditions: Vec<SpawnCondition>,
    pub current_power_up: Option<PowerUpInstance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PowerUpType {
    HealthBoost,
    DamageBoost,
    SpeedBoost,
    Shield,
    Invisibility,
    DoubleJump,
    Weapon,
    Ammo,
    Special(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowerUpInstance {
    pub instance_id: Uuid,
    pub power_up_type: PowerUpType,
    pub spawn_time: DateTime<Utc>,
    pub duration: u32, // How long it lasts when picked up
    pub picked_up: bool,
    pub picked_up_by: Option<PlayerId>,
}

/// Arena configuration settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaSettings {
    pub friendly_fire: bool,
    pub respawn_enabled: bool,
    pub respawn_delay: u32,
    pub match_time_limit: u32,
    pub sudden_death: bool,
    pub environmental_hazards_enabled: bool,
    pub power_ups_enabled: bool,
    pub spectating_enabled: bool,
    pub recording_enabled: bool,
    pub custom_rules: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArenaStatus {
    Available,
    InUse,
    Maintenance,
    Disabled,
    Private,
}

/// Arena template for creating new arenas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArenaTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub arena_type: ArenaType,
    pub base_map_data: MapData,
    pub default_settings: ArenaSettings,
    pub recommended_modes: Vec<CombatMode>,
    pub difficulty_rating: u32,
    pub popularity_score: f64,
}

/// Match instance running in an arena
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchInstance {
    pub instance_id: Uuid,
    pub match_id: MatchId,
    pub arena_id: ArenaId,
    pub server_id: String,
    pub participants: Vec<PlayerId>,
    pub spectators: Vec<PlayerId>,
    pub start_time: DateTime<Utc>,
    pub estimated_end_time: DateTime<Utc>,
    pub current_state: InstanceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceState {
    Initializing,
    WaitingForPlayers,
    Starting,
    Active,
    Paused,
    Ending,
    Cleanup,
}

/// Server instance for hosting matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInstance {
    pub server_id: String,
    pub region: String,
    pub capacity: u32,
    pub current_load: u32,
    pub active_matches: Vec<MatchId>,
    pub performance_metrics: ServerMetrics,
    pub status: ServerStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub network_latency: f64,
    pub player_count: u32,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerStatus {
    Online,
    Offline,
    Maintenance,
    Overloaded,
    Restarting,
}

impl ArenaManager {
    /// Create new arena manager
    pub fn new() -> Self {
        let mut manager = Self {
            arenas: HashMap::new(),
            active_matches: HashMap::new(),
            arena_templates: vec![],
            match_instances: HashMap::new(),
            server_instances: HashMap::new(),
        };

        manager.initialize_default_arenas();
        manager
    }

    /// Initialize default arenas
    fn initialize_default_arenas(&mut self) {
        // Classic Duel Arena
        let duel_arena = Arena {
            arena_id: Uuid::new_v4(),
            name: "Classic Duel Grounds".to_string(),
            description: "Traditional 1v1 combat arena with balanced terrain".to_string(),
            arena_type: ArenaType::Duel,
            map_data: MapData {
                dimensions: (50.0, 50.0, 10.0),
                terrain_type: TerrainType::Flat,
                elevation_map: vec![],
                collision_boundaries: vec![],
                visual_theme: "medieval".to_string(),
                weather_conditions: vec![WeatherCondition {
                    condition_type: WeatherType::Clear,
                    intensity: 1.0,
                    duration_minutes: None,
                    effects: vec![],
                }],
            },
            supported_modes: vec![CombatMode::Duel],
            capacity: ArenaCapacity {
                min_players: 2,
                max_players: 2,
                optimal_players: 2,
                spectator_limit: Some(10),
                concurrent_matches: 4,
            },
            environmental_features: vec![],
            spawn_points: vec![
                SpawnPoint {
                    spawn_id: Uuid::new_v4(),
                    position: (-20.0, 0.0, 0.0),
                    orientation: 0.0,
                    team_assignment: None,
                    spawn_type: SpawnType::Initial,
                    conditions: vec![SpawnCondition::Always],
                },
                SpawnPoint {
                    spawn_id: Uuid::new_v4(),
                    position: (20.0, 0.0, 0.0),
                    orientation: std::f64::consts::PI,
                    team_assignment: None,
                    spawn_type: SpawnType::Initial,
                    conditions: vec![SpawnCondition::Always],
                },
            ],
            objectives: vec![],
            power_up_spawns: vec![],
            settings: ArenaSettings::default_duel(),
            status: ArenaStatus::Available,
        };

        // Team Battle Arena
        let team_arena = Arena {
            arena_id: Uuid::new_v4(),
            name: "Contested Highlands".to_string(),
            description: "Multi-team arena with capture points and elevation advantage".to_string(),
            arena_type: ArenaType::Skirmish,
            map_data: MapData {
                dimensions: (100.0, 100.0, 20.0),
                terrain_type: TerrainType::Hilly,
                elevation_map: vec![],
                collision_boundaries: vec![],
                visual_theme: "highland".to_string(),
                weather_conditions: vec![],
            },
            supported_modes: vec![CombatMode::SmallGroup, CombatMode::TeamBattle],
            capacity: ArenaCapacity {
                min_players: 6,
                max_players: 10,
                optimal_players: 8,
                spectator_limit: Some(20),
                concurrent_matches: 2,
            },
            environmental_features: vec![
                EnvironmentalFeature {
                    feature_id: Uuid::new_v4(),
                    name: "High Ground Buff".to_string(),
                    feature_type: FeatureType::Buff,
                    position: (0.0, 0.0, 10.0),
                    area_of_effect: Some(15.0),
                    activation_conditions: vec![ActivationCondition::PlayerProximity(15.0)],
                    effects: vec![EnvironmentalEffect {
                        effect_name: "Elevation Advantage".to_string(),
                        effect_type: EffectType::Buff,
                        magnitude: 1.2,
                        target_type: EffectTarget::Proximity(15.0),
                    }],
                    duration: None,
                    cooldown: None,
                },
            ],
            spawn_points: self.create_team_spawn_points(),
            objectives: vec![
                ArenaObjective {
                    objective_id: Uuid::new_v4(),
                    name: "Central Control Point".to_string(),
                    objective_type: ObjectiveType::CapturePoint,
                    position: (0.0, 0.0, 5.0),
                    capture_radius: 10.0,
                    capture_time: 15,
                    control_requirements: ControlRequirements {
                        minimum_players: 1,
                        contested_delay: 5,
                        decay_rate: 0.1,
                        team_exclusive: true,
                    },
                    rewards: vec![ObjectiveReward {
                        reward_type: ObjectiveRewardType::Points,
                        amount: 10,
                        frequency: RewardFrequency::PerSecond,
                    }],
                    status: ObjectiveStatus {
                        controlling_team: None,
                        capture_progress: 0.0,
                        contested: false,
                        last_interaction: Utc::now(),
                        total_control_time: HashMap::new(),
                    },
                },
            ],
            power_up_spawns: vec![
                PowerUpSpawn {
                    spawn_id: Uuid::new_v4(),
                    position: (-30.0, 30.0, 0.0),
                    power_up_types: vec![PowerUpType::DamageBoost, PowerUpType::HealthBoost],
                    spawn_interval: 60,
                    spawn_conditions: vec![SpawnCondition::TimeElapsed(60)],
                    current_power_up: None,
                },
                PowerUpSpawn {
                    spawn_id: Uuid::new_v4(),
                    position: (30.0, -30.0, 0.0),
                    power_up_types: vec![PowerUpType::SpeedBoost, PowerUpType::Shield],
                    spawn_interval: 90,
                    spawn_conditions: vec![SpawnCondition::TimeElapsed(90)],
                    current_power_up: None,
                },
            ],
            settings: ArenaSettings::default_team(),
            status: ArenaStatus::Available,
        };

        self.arenas.insert(duel_arena.arena_id, duel_arena);
        self.arenas.insert(team_arena.arena_id, team_arena);
    }

    /// Create spawn points for team-based matches
    fn create_team_spawn_points(&self) -> Vec<SpawnPoint> {
        vec![
            // Team 1 spawns
            SpawnPoint {
                spawn_id: Uuid::new_v4(),
                position: (-40.0, -40.0, 0.0),
                orientation: std::f64::consts::PI / 4.0,
                team_assignment: None, // Will be assigned dynamically
                spawn_type: SpawnType::Initial,
                conditions: vec![SpawnCondition::Always],
            },
            SpawnPoint {
                spawn_id: Uuid::new_v4(),
                position: (-35.0, -45.0, 0.0),
                orientation: std::f64::consts::PI / 4.0,
                team_assignment: None,
                spawn_type: SpawnType::Initial,
                conditions: vec![SpawnCondition::Always],
            },
            // Team 2 spawns
            SpawnPoint {
                spawn_id: Uuid::new_v4(),
                position: (40.0, 40.0, 0.0),
                orientation: 5.0 * std::f64::consts::PI / 4.0,
                team_assignment: None,
                spawn_type: SpawnType::Initial,
                conditions: vec![SpawnCondition::Always],
            },
            SpawnPoint {
                spawn_id: Uuid::new_v4(),
                position: (35.0, 45.0, 0.0),
                orientation: 5.0 * std::f64::consts::PI / 4.0,
                team_assignment: None,
                spawn_type: SpawnType::Initial,
                conditions: vec![SpawnCondition::Always],
            },
        ]
    }

    /// Select appropriate arena for match
    pub fn select_arena(&self, combat_mode: &CombatMode, player_count: u32) -> Option<ArenaId> {
        self.arenas.values()
            .filter(|arena| {
                arena.supported_modes.contains(combat_mode) &&
                arena.capacity.min_players <= player_count &&
                arena.capacity.max_players >= player_count &&
                matches!(arena.status, ArenaStatus::Available)
            })
            .max_by(|a, b| {
                // Prefer arenas with optimal player count close to actual count
                let a_diff = (a.capacity.optimal_players as i32 - player_count as i32).abs();
                let b_diff = (b.capacity.optimal_players as i32 - player_count as i32).abs();
                b_diff.cmp(&a_diff)
            })
            .map(|arena| arena.arena_id)
    }

    /// Add match to arena system
    pub fn add_match(&mut self, mut pvp_match: PvPMatch) -> Result<()> {
        // Select arena if not specified
        if pvp_match.arena_id.is_none() {
            let arena_id = self.select_arena(&pvp_match.combat_mode, pvp_match.participants.len() as u32)
                .ok_or_else(|| anyhow::anyhow!("No suitable arena available"))?;
            pvp_match.arena_id = Some(arena_id);
        }

        let match_id = pvp_match.match_id;
        self.active_matches.insert(match_id, pvp_match);
        Ok(())
    }

    /// Add player to existing match
    pub fn add_player_to_match(&mut self, match_id: MatchId, player_id: PlayerId) -> Result<()> {
        let pvp_match = self.active_matches.get_mut(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        // Check if player is already in match
        if pvp_match.participants.iter().any(|p| p.player_id == player_id) {
            return Err(anyhow::anyhow!("Player already in match"));
        }

        // Check capacity
        let arena_id = pvp_match.arena_id
            .ok_or_else(|| anyhow::anyhow!("Match has no assigned arena"))?;
        let arena = self.arenas.get(&arena_id)
            .ok_or_else(|| anyhow::anyhow!("Arena not found"))?;

        if pvp_match.participants.len() >= arena.capacity.max_players as usize {
            return Err(anyhow::anyhow!("Match at capacity"));
        }

        // Add participant
        let participant = crate::MatchParticipant {
            player_id,
            team_id: self.assign_team(match_id, player_id)?,
            ready_status: false,
            connection_status: crate::ConnectionStatus::Connected,
            performance: crate::ParticipantPerformance::default(),
        };

        pvp_match.participants.push(participant);
        Ok(())
    }

    /// Assign player to a team
    fn assign_team(&mut self, match_id: MatchId, player_id: PlayerId) -> Result<TeamId> {
        let pvp_match = self.active_matches.get_mut(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        // Find team with fewest players
        let team_id = if let Some(team) = pvp_match.teams.iter()
            .min_by_key(|team| team.members.len()) {
            let team_id = team.team_id;
            
            // Add player to team
            if let Some(team) = pvp_match.teams.iter_mut().find(|t| t.team_id == team_id) {
                team.members.push(player_id);
            }
            
            team_id
        } else {
            // Create new team if none exist
            let team_id = Uuid::new_v4();
            let team = crate::Team {
                team_id,
                name: format!("Team {}", pvp_match.teams.len() + 1),
                color: self.get_team_color(pvp_match.teams.len()),
                members: vec![player_id],
                captain: Some(player_id),
                team_rating: 1000, // Default rating
            };
            
            pvp_match.teams.push(team);
            team_id
        };

        Ok(team_id)
    }

    /// Get team color based on team index
    fn get_team_color(&self, team_index: usize) -> String {
        let colors = ["#FF0000", "#0000FF", "#00FF00", "#FFFF00", "#FF00FF", "#00FFFF"];
        colors.get(team_index % colors.len())
            .unwrap_or(&"#808080")
            .to_string()
    }

    /// Check if match can start
    pub fn can_match_start(&self, match_id: MatchId) -> Result<bool> {
        let pvp_match = self.active_matches.get(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        let arena_id = pvp_match.arena_id
            .ok_or_else(|| anyhow::anyhow!("Match has no assigned arena"))?;
        let arena = self.arenas.get(&arena_id)
            .ok_or_else(|| anyhow::anyhow!("Arena not found"))?;

        // Check minimum players
        if pvp_match.participants.len() < arena.capacity.min_players as usize {
            return Ok(false);
        }

        // Check all players are ready
        let all_ready = pvp_match.participants.iter().all(|p| p.ready_status);
        if !all_ready {
            return Ok(false);
        }

        // Check teams are balanced (for team modes)
        if pvp_match.teams.len() > 1 {
            let team_sizes: Vec<usize> = pvp_match.teams.iter()
                .map(|team| team.members.len())
                .collect();
            
            let min_size = *team_sizes.iter().min().unwrap_or(&0);
            let max_size = *team_sizes.iter().max().unwrap_or(&0);
            
            // Teams should be within 1 player of each other
            if max_size - min_size > 1 {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Start match
    pub fn start_match(&mut self, match_id: MatchId) -> Result<()> {
        let pvp_match = self.active_matches.get_mut(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        pvp_match.status = crate::MatchStatus::InProgress;
        pvp_match.start_time = Some(Utc::now());

        // Initialize power-up spawns if arena supports them
        if let Some(arena_id) = pvp_match.arena_id {
            if let Some(arena) = self.arenas.get_mut(&arena_id) {
                if arena.settings.power_ups_enabled {
                    self.initialize_power_ups(&arena_id)?;
                }
            }
        }

        tracing::info!("Started match {} with {} participants", match_id, pvp_match.participants.len());
        Ok(())
    }

    /// Initialize power-ups for arena
    fn initialize_power_ups(&mut self, arena_id: &ArenaId) -> Result<()> {
        if let Some(arena) = self.arenas.get_mut(arena_id) {
            for power_up_spawn in &mut arena.power_up_spawns {
                if power_up_spawn.current_power_up.is_none() {
                    let power_up_type = power_up_spawn.power_up_types.first()
                        .cloned()
                        .unwrap_or(PowerUpType::HealthBoost);
                    
                    power_up_spawn.current_power_up = Some(PowerUpInstance {
                        instance_id: Uuid::new_v4(),
                        power_up_type,
                        spawn_time: Utc::now(),
                        duration: 30, // 30 seconds duration
                        picked_up: false,
                        picked_up_by: None,
                    });
                }
            }
        }
        Ok(())
    }

    /// Record combat event
    pub fn record_event(&mut self, match_id: MatchId, event: CombatEvent) -> Result<()> {
        let pvp_match = self.active_matches.get_mut(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        pvp_match.match_data.combat_log.push(event);

        // Update participant performance based on event
        if let Some(attacker_id) = event.attacker_id {
            if let Some(participant) = pvp_match.participants.iter_mut()
                .find(|p| p.player_id == attacker_id) {
                
                match event.event_type {
                    crate::CombatEventType::Kill => participant.performance.kills += 1,
                    crate::CombatEventType::Attack => {
                        if let Some(damage) = event.damage_amount {
                            participant.performance.damage_dealt += damage;
                        }
                    },
                    _ => {},
                }
            }
        }

        if let Some(target_id) = event.target_id {
            if let Some(participant) = pvp_match.participants.iter_mut()
                .find(|p| p.player_id == target_id) {
                
                match event.event_type {
                    crate::CombatEventType::Death => participant.performance.deaths += 1,
                    crate::CombatEventType::Attack => {
                        if let Some(damage) = event.damage_amount {
                            participant.performance.damage_taken += damage;
                        }
                    },
                    _ => {},
                }
            }
        }

        Ok(())
    }

    /// End match
    pub fn end_match(&mut self, match_id: MatchId, winner: Option<TeamId>) -> Result<MatchResult> {
        let pvp_match = self.active_matches.get_mut(&match_id)
            .ok_or_else(|| anyhow::anyhow!("Match not found"))?;

        pvp_match.status = crate::MatchStatus::Finished;
        pvp_match.end_time = Some(Utc::now());
        pvp_match.winner = winner;

        // Calculate match duration
        let duration = if let (Some(start), Some(end)) = (pvp_match.start_time, pvp_match.end_time) {
            (end - start).num_seconds() as u32
        } else {
            0
        };

        // Create match result
        let match_result = MatchResult {
            match_id,
            match_type: pvp_match.match_type.clone(),
            combat_mode: pvp_match.combat_mode.clone(),
            opponents: pvp_match.participants.iter()
                .map(|p| p.player_id)
                .collect(),
            teammates: vec![], // TODO: Extract actual teammates
            result: if winner.is_some() { 
                crate::MatchOutcome::Victory 
            } else { 
                crate::MatchOutcome::Draw 
            },
            duration_seconds: duration,
            damage_dealt: pvp_match.participants.iter()
                .map(|p| p.performance.damage_dealt)
                .sum(),
            damage_taken: pvp_match.participants.iter()
                .map(|p| p.performance.damage_taken)
                .sum(),
            kills: pvp_match.participants.iter()
                .map(|p| p.performance.kills)
                .sum(),
            deaths: pvp_match.participants.iter()
                .map(|p| p.performance.deaths)
                .sum(),
            assists: pvp_match.participants.iter()
                .map(|p| p.performance.assists)
                .sum(),
            rating_change: 0, // Will be calculated by rating system
            timestamp: Utc::now(),
        };

        // Clean up match
        self.active_matches.remove(&match_id);

        tracing::info!("Match {} ended with winner: {:?}", match_id, winner);
        Ok(match_result)
    }

    /// Get arena by ID
    pub fn get_arena(&self, arena_id: ArenaId) -> Option<&Arena> {
        self.arenas.get(&arena_id)
    }

    /// Get active match
    pub fn get_match(&self, match_id: MatchId) -> Option<&PvPMatch> {
        self.active_matches.get(&match_id)
    }

    /// Update arena settings
    pub fn update_arena_settings(&mut self, arena_id: ArenaId, settings: ArenaSettings) -> Result<()> {
        let arena = self.arenas.get_mut(&arena_id)
            .ok_or_else(|| anyhow::anyhow!("Arena not found"))?;
        
        arena.settings = settings;
        Ok(())
    }

    /// Get available arenas for combat mode
    pub fn get_available_arenas(&self, combat_mode: &CombatMode) -> Vec<&Arena> {
        self.arenas.values()
            .filter(|arena| {
                arena.supported_modes.contains(combat_mode) &&
                matches!(arena.status, ArenaStatus::Available)
            })
            .collect()
    }
}

impl ArenaSettings {
    /// Default settings for duel arenas
    fn default_duel() -> Self {
        Self {
            friendly_fire: false,
            respawn_enabled: false,
            respawn_delay: 0,
            match_time_limit: 300, // 5 minutes
            sudden_death: true,
            environmental_hazards_enabled: false,
            power_ups_enabled: false,
            spectating_enabled: true,
            recording_enabled: true,
            custom_rules: HashMap::new(),
        }
    }

    /// Default settings for team arenas
    fn default_team() -> Self {
        Self {
            friendly_fire: false,
            respawn_enabled: true,
            respawn_delay: 5,
            match_time_limit: 900, // 15 minutes
            sudden_death: false,
            environmental_hazards_enabled: true,
            power_ups_enabled: true,
            spectating_enabled: true,
            recording_enabled: true,
            custom_rules: HashMap::new(),
        }
    }
}

impl Default for crate::ParticipantPerformance {
    fn default() -> Self {
        Self {
            kills: 0,
            deaths: 0,
            assists: 0,
            damage_dealt: 0,
            damage_taken: 0,
            healing_done: 0,
            objectives_completed: 0,
            accuracy_percentage: 0.0,
            performance_score: 0.0,
        }
    }
}

impl Default for ArenaManager {
    fn default() -> Self {
        Self::new()
    }
}