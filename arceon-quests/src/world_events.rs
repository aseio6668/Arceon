/*!
# World Events System

Manages dynamic world events, seasonal changes, and large-scale
happenings that affect quests and narrative progression.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, LocationId, EventId, QuestId, NPCId};

/// World event coordination system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEventCoordinator {
    pub active_events: HashMap<EventId, WorldEvent>,
    pub scheduled_events: Vec<ScheduledEvent>,
    pub event_templates: Vec<EventTemplate>,
    pub world_state: WorldState,
    pub event_history: Vec<CompletedEvent>,
    pub player_participation: HashMap<PlayerId, ParticipationRecord>,
    pub regional_effects: HashMap<String, RegionalEffect>,
}

/// Individual world event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub event_id: EventId,
    pub name: String,
    pub description: String,
    pub event_type: EventType,
    pub scope: EventScope,
    pub status: EventStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<u32>, // Duration in seconds if not permanent
    pub affected_locations: Vec<LocationId>,
    pub participants: Vec<EventParticipant>,
    pub objectives: Vec<EventObjective>,
    pub rewards: EventRewards,
    pub consequences: Vec<EventConsequence>,
    pub narrative_impact: NarrativeImpact,
    pub triggers: Vec<EventTrigger>,
    pub conditions: Vec<EventCondition>,
    pub phases: Vec<EventPhase>,
    pub current_phase: u32,
    pub metadata: EventMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventType {
    Natural,        // Natural disasters, weather
    Political,      // Wars, treaties, political changes
    Social,         // Festivals, celebrations, gatherings
    Economic,       // Market crashes, trade routes
    Mystical,       // Magical phenomena, supernatural events
    Invasion,       // Monster invasions, enemy attacks
    Discovery,      // New lands, artifacts, technologies
    Catastrophic,   // World-ending threats
    Seasonal,       // Regular seasonal events
    Player,         // Player-initiated events
    Quest,          // Quest-related events
    Faction,        // Faction-specific events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventScope {
    Global,         // Affects entire world
    Continental,    // Affects continent
    Regional,       // Affects region/zone
    Local,          // Affects specific locations
    Personal,       // Affects individual players
    Faction,        // Affects faction members
    Guild,          // Affects guild members
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Scheduled,      // Not yet started
    Starting,       // Beginning phase
    Active,         // Currently happening
    Escalating,     // Intensifying
    Climax,         // Peak phase
    Resolving,      // Winding down
    Completed,      // Finished
    Failed,         // Event failed to complete
    Cancelled,      // Manually cancelled
    Paused,         // Temporarily stopped
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParticipant {
    pub participant_id: Uuid,
    pub participant_type: ParticipantType,
    pub role: ParticipantRole,
    pub contribution_score: f64,
    pub joined_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantType {
    Player(PlayerId),
    Guild(Uuid),
    Faction(String),
    NPC(NPCId),
    AIGroup(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParticipantRole {
    Leader,
    Organizer,
    Participant,
    Helper,
    Observer,
    Opponent,
    Neutral,
    Victim,
    Hero,
    Villain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventObjective {
    pub objective_id: Uuid,
    pub title: String,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub target_value: f64,
    pub current_value: f64,
    pub completion_criteria: CompletionCriteria,
    pub participants_required: Option<u32>,
    pub time_limit: Option<DateTime<Utc>>,
    pub rewards: Vec<ObjectiveReward>,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    Survival,       // Survive for duration
    Collection,     // Collect items/resources
    Elimination,    // Defeat enemies
    Protection,     // Protect something/someone
    Construction,   // Build something
    Exploration,    // Discover locations
    Cooperation,    // Work together
    Competition,    // Compete against others
    Sacrifice,      // Give up something valuable
    Innovation,     // Create/invent something
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionCriteria {
    Threshold(f64),                    // Reach target value
    Percentage(f64),                   // Percentage of participants succeed
    Collaborative(f64),                // Combined effort reaches threshold
    FirstToComplete,                   // First participant to finish
    TimeLimit(DateTime<Utc>),         // Complete before deadline
    Consensus(f64),                   // Agreement percentage
    Competition(CompetitionType),      // Competition-based completion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompetitionType {
    HighestScore,
    LowestTime,
    LastStanding,
    MostContributions,
    BestQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveReward {
    pub reward_type: RewardType,
    pub amount: u64,
    pub recipient: RewardRecipient,
    pub condition: Option<RewardCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    Experience,
    Gold,
    Items,
    Reputation,
    Title,
    Achievement,
    WorldInfluence,
    SpecialAccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardRecipient {
    Individual,     // Each participant
    TopContributors(u32), // Top N contributors
    Winners,        // Competition winners
    Everyone,       // All participants
    Leaders,        // Only leaders
    RandomSelection(u32), // Random N participants
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardCondition {
    MinimumContribution(f64),
    ParticipationDuration(u32),
    ObjectiveCompletion(Uuid),
    LeaderboardPosition(u32),
    TeamSuccess,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRewards {
    pub completion_rewards: Vec<ObjectiveReward>,
    pub participation_rewards: Vec<ObjectiveReward>,
    pub milestone_rewards: HashMap<String, Vec<ObjectiveReward>>,
    pub leaderboard_rewards: Vec<LeaderboardReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardReward {
    pub position: u32,
    pub rewards: Vec<ObjectiveReward>,
    pub title: Option<String>,
    pub special_recognition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConsequence {
    pub consequence_id: Uuid,
    pub consequence_type: ConsequenceType,
    pub target: ConsequenceTarget,
    pub magnitude: f64,
    pub duration: Option<u32>,
    pub delay: Option<u32>, // Seconds before consequence takes effect
    pub conditions: Vec<ConsequenceCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    WorldStateChange,
    LocationModification,
    NPCBehaviorChange,
    QuestAvailability,
    ItemAvailability,
    EconomicChange,
    PoliticalShift,
    EnvironmentalChange,
    MagicalEffect,
    TechnologicalAdvancement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceTarget {
    Global,
    Region(String),
    Location(LocationId),
    Faction(String),
    PlayerGroup(Vec<PlayerId>),
    NPCGroup(Vec<NPCId>),
    Specific(String), // Specific entity ID
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceCondition {
    EventSuccess,
    EventFailure,
    ParticipantThreshold(u32),
    ObjectiveCompletion(Uuid),
    TimeElapsed(u32),
    PlayerAction(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeImpact {
    pub significance_level: f64, // 0.0 to 1.0
    pub affected_storylines: Vec<Uuid>,
    pub character_development: Vec<CharacterDevelopment>,
    pub world_lore_additions: Vec<String>,
    pub historical_importance: f64,
    pub player_legend_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDevelopment {
    pub character_id: NPCId,
    pub development_type: DevelopmentType,
    pub magnitude: f64,
    pub permanent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentType {
    PersonalityChange,
    StatusChange,
    RelationshipChange,
    SkillDevelopment,
    MoralAlignment,
    PowerLevel,
    Reputation,
    Motivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventTrigger {
    TimeScheduled(DateTime<Utc>),
    PlayerAction(PlayerId, String),
    QuestCompletion(QuestId),
    WorldStateReached(String, String),
    PlayerCount(u32),
    LocationDiscovered(LocationId),
    ItemUsed(crate::ItemId),
    NPCAction(NPCId, String),
    SeasonalCycle(String),
    RandomOccurrence(f64),
    ChainReaction(EventId),
    PlayerRequest(PlayerId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCondition {
    MinimumPlayers(u32),
    MaximumPlayers(u32),
    PlayerLevel(u32, u32), // min, max
    QuestStatus(QuestId, String),
    WorldState(String, String),
    TimeOfDay(u32, u32),
    Season(String),
    WeatherCondition(String),
    LocationAccessible(LocationId),
    FactionStanding(String, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPhase {
    pub phase_id: String,
    pub name: String,
    pub description: String,
    pub duration: Option<u32>, // Duration in seconds
    pub objectives: Vec<Uuid>, // Objective IDs for this phase
    pub start_conditions: Vec<PhaseCondition>,
    pub end_conditions: Vec<PhaseCondition>,
    pub phase_effects: Vec<PhaseEffect>,
    pub narrative_content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseCondition {
    TimeElapsed(u32),
    ObjectiveCompleted(Uuid),
    ParticipantCount(u32),
    PlayerAction(String),
    WorldStateChange(String),
    PreviousPhaseComplete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseEffect {
    pub effect_type: PhaseEffectType,
    pub target: String,
    pub magnitude: f64,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhaseEffectType {
    SpawnRate,
    Difficulty,
    Rewards,
    Environment,
    NPCBehavior,
    PlayerAbilities,
    WorldState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub created_at: DateTime<Utc>,
    pub creator: EventCreator,
    pub estimated_duration: u32, // Total estimated duration in seconds
    pub max_participants: Option<u32>,
    pub difficulty_rating: EventDifficulty,
    pub tags: Vec<String>,
    pub repeatable: bool,
    pub seasonal: bool,
    pub player_initiated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventCreator {
    System,
    GameMaster,
    Player(PlayerId),
    AI,
    Community,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventDifficulty {
    Trivial,
    Easy,
    Normal,
    Hard,
    Extreme,
    Legendary,
    WorldChanging,
}

/// Scheduled future event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledEvent {
    pub schedule_id: Uuid,
    pub event_template_id: Uuid,
    pub scheduled_time: DateTime<Utc>,
    pub conditions: Vec<EventCondition>,
    pub customization: EventCustomization,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCustomization {
    pub name_override: Option<String>,
    pub location_override: Option<Vec<LocationId>>,
    pub participant_limits: Option<(u32, u32)>, // min, max
    pub duration_modifier: f64,
    pub reward_modifier: f64,
    pub difficulty_modifier: f64,
}

/// Template for creating events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub template_type: EventType,
    pub base_configuration: EventConfiguration,
    pub variable_parameters: Vec<VariableParameter>,
    pub generation_rules: GenerationRules,
    pub narrative_framework: NarrativeFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventConfiguration {
    pub default_duration: u32,
    pub participant_range: (u32, u32),
    pub objective_templates: Vec<ObjectiveTemplate>,
    pub reward_formulas: RewardFormulas,
    pub consequence_patterns: Vec<ConsequencePattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveTemplate {
    pub objective_type: ObjectiveType,
    pub scaling_formula: String,
    pub difficulty_curve: Vec<f64>,
    pub reward_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardFormulas {
    pub base_experience: u64,
    pub experience_scaling: f64,
    pub base_gold: u64,
    pub gold_scaling: f64,
    pub item_drop_chances: HashMap<String, f64>,
    pub reputation_gains: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequencePattern {
    pub pattern_type: ConsequenceType,
    pub probability: f64,
    pub magnitude_range: (f64, f64),
    pub duration_range: (u32, u32),
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableParameter {
    pub parameter_name: String,
    pub parameter_type: ParameterType,
    pub default_value: String,
    pub variation_rules: Vec<VariationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    Integer,
    Float,
    String,
    Location,
    Duration,
    PlayerCount,
    RewardMultiplier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationRule {
    pub condition: String,
    pub modification: ParameterModification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterModification {
    Set(String),
    Add(f64),
    Multiply(f64),
    RandomRange(f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRules {
    pub frequency: EventFrequency,
    pub timing_constraints: TimingConstraints,
    pub prerequisite_events: Vec<EventId>,
    pub exclusion_events: Vec<EventId>, // Events that prevent this one
    pub population_requirements: PopulationRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventFrequency {
    Once,
    Daily,
    Weekly,
    Monthly,
    Seasonal,
    Yearly,
    Random(f64), // Average days between occurrences
    Triggered,   // Only when triggered
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingConstraints {
    pub time_of_day: Option<(u32, u32)>,
    pub days_of_week: Option<Vec<u32>>,
    pub seasons: Option<Vec<String>>,
    pub avoid_conflicts: bool,
    pub minimum_gap: Option<u32>, // Minimum seconds between similar events
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationRequirements {
    pub minimum_online: u32,
    pub target_regions: Option<Vec<String>>,
    pub player_level_range: Option<(u32, u32)>,
    pub faction_balance: Option<HashMap<String, f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFramework {
    pub story_arc: StoryArc,
    pub character_roles: Vec<CharacterRole>,
    pub plot_devices: Vec<PlotDevice>,
    pub emotional_journey: EmotionalJourney,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryArc {
    pub setup_phase: PhaseNarrative,
    pub escalation_phase: PhaseNarrative,
    pub climax_phase: PhaseNarrative,
    pub resolution_phase: PhaseNarrative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseNarrative {
    pub description: String,
    pub key_moments: Vec<String>,
    pub character_interactions: Vec<String>,
    pub world_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRole {
    pub role_type: RoleType,
    pub character_id: Option<NPCId>,
    pub involvement_level: f64,
    pub narrative_function: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoleType {
    Catalyst,    // Triggers the event
    Guide,       // Helps players understand
    Antagonist,  // Opposes players
    Victim,      // Needs help/rescue
    Herald,      // Announces/explains
    Mentor,      // Teaches/advises
    Trickster,   // Complicates situation
    Ally,        // Supports players
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotDevice {
    MacGuffin,        // Important object
    RedHerring,       // False clue
    ChekhovsGun,      // Something important later
    Doppelganger,     // Imposter/duplicate
    Flashback,        // Past events revealed
    Foreshadowing,    // Hints at future
    Twist,            // Unexpected revelation
    Sacrifice,        // Something must be given up
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalJourney {
    pub starting_emotion: crate::narrative_engine::Emotion,
    pub emotional_peaks: Vec<EmotionalPeak>,
    pub ending_emotion: crate::narrative_engine::Emotion,
    pub tension_curve: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPeak {
    pub phase: String,
    pub emotion: crate::narrative_engine::Emotion,
    pub intensity: f64,
    pub trigger: String,
}

/// Current world state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub current_season: String,
    pub time_of_day: u32,
    pub weather_patterns: HashMap<String, WeatherCondition>,
    pub political_tensions: HashMap<String, f64>,
    pub economic_indicators: HashMap<String, f64>,
    pub magical_levels: HashMap<String, f64>,
    pub population_distribution: HashMap<String, u32>,
    pub resource_availability: HashMap<String, f64>,
    pub threat_levels: HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub weather_type: String,
    pub intensity: f64,
    pub duration: Option<u32>,
    pub affected_areas: Vec<String>,
}

/// Completed event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedEvent {
    pub event_id: EventId,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub participants: Vec<EventParticipant>,
    pub outcome: EventOutcome,
    pub consequences_applied: Vec<EventConsequence>,
    pub narrative_impact: NarrativeImpact,
    pub statistics: EventStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventOutcome {
    Success,
    Failure,
    PartialSuccess,
    Cancelled,
    Interrupted,
    Escalated,
    Transformed, // Event changed into something else
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventStatistics {
    pub total_participants: u32,
    pub objectives_completed: u32,
    pub total_objectives: u32,
    pub average_contribution: f64,
    pub top_contributors: Vec<(Uuid, f64)>,
    pub duration_seconds: u32,
    pub resources_consumed: HashMap<String, u64>,
    pub rewards_distributed: HashMap<String, u64>,
}

/// Player participation tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipationRecord {
    pub player_id: PlayerId,
    pub events_participated: Vec<EventParticipation>,
    pub total_contribution_score: f64,
    pub preferred_event_types: HashMap<EventType, f64>,
    pub leadership_experience: f64,
    pub reputation_from_events: HashMap<String, i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventParticipation {
    pub event_id: EventId,
    pub role: ParticipantRole,
    pub contribution_score: f64,
    pub objectives_completed: Vec<Uuid>,
    pub duration_participated: u32,
    pub rewards_received: Vec<ObjectiveReward>,
    pub satisfaction_rating: Option<f64>,
}

/// Regional effects from events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalEffect {
    pub region: String,
    pub effect_type: EffectType,
    pub intensity: f64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub source_event: EventId,
    pub affected_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Economic,
    Environmental,
    Political,
    Magical,
    Social,
    Military,
    Technological,
    Cultural,
}

impl WorldEventCoordinator {
    /// Create new world event coordinator
    pub fn new() -> Self {
        let mut coordinator = Self {
            active_events: HashMap::new(),
            scheduled_events: vec![],
            event_templates: vec![],
            world_state: WorldState::default(),
            event_history: vec![],
            player_participation: HashMap::new(),
            regional_effects: HashMap::new(),
        };
        
        coordinator.initialize_default_templates();
        coordinator
    }

    /// Initialize default event templates
    fn initialize_default_templates(&mut self) {
        // Seasonal Festival Template
        let festival_template = EventTemplate {
            template_id: Uuid::new_v4(),
            name: "Harvest Festival".to_string(),
            template_type: EventType::Social,
            base_configuration: EventConfiguration {
                default_duration: 3600 * 24 * 3, // 3 days
                participant_range: (10, 1000),
                objective_templates: vec![
                    ObjectiveTemplate {
                        objective_type: ObjectiveType::Collection,
                        scaling_formula: "base * participant_count * 0.1".to_string(),
                        difficulty_curve: vec![0.5, 0.7, 0.9],
                        reward_weight: 1.0,
                    },
                    ObjectiveTemplate {
                        objective_type: ObjectiveType::Cooperation,
                        scaling_formula: "base * sqrt(participant_count)".to_string(),
                        difficulty_curve: vec![0.3, 0.6, 0.8],
                        reward_weight: 1.5,
                    },
                ],
                reward_formulas: RewardFormulas {
                    base_experience: 1000,
                    experience_scaling: 1.2,
                    base_gold: 500,
                    gold_scaling: 1.1,
                    item_drop_chances: [
                        ("festival_token".to_string(), 0.8),
                        ("rare_food".to_string(), 0.3),
                        ("decoration".to_string(), 0.5),
                    ].iter().cloned().collect(),
                    reputation_gains: [
                        ("village".to_string(), 50),
                        ("merchants".to_string(), 25),
                    ].iter().cloned().collect(),
                },
                consequence_patterns: vec![
                    ConsequencePattern {
                        pattern_type: ConsequenceType::EconomicChange,
                        probability: 0.7,
                        magnitude_range: (0.1, 0.3),
                        duration_range: (3600 * 24, 3600 * 24 * 7),
                        conditions: vec!["successful_completion".to_string()],
                    },
                ],
            },
            variable_parameters: vec![
                VariableParameter {
                    parameter_name: "harvest_amount".to_string(),
                    parameter_type: ParameterType::Integer,
                    default_value: "1000".to_string(),
                    variation_rules: vec![
                        VariationRule {
                            condition: "good_weather".to_string(),
                            modification: ParameterModification::Multiply(1.5),
                        },
                    ],
                },
            ],
            generation_rules: GenerationRules {
                frequency: EventFrequency::Seasonal,
                timing_constraints: TimingConstraints {
                    time_of_day: None,
                    days_of_week: None,
                    seasons: Some(vec!["autumn".to_string()]),
                    avoid_conflicts: true,
                    minimum_gap: Some(3600 * 24 * 30), // 30 days
                },
                prerequisite_events: vec![],
                exclusion_events: vec![],
                population_requirements: PopulationRequirements {
                    minimum_online: 5,
                    target_regions: Some(vec!["farmlands".to_string(), "villages".to_string()]),
                    player_level_range: None,
                    faction_balance: None,
                },
            },
            narrative_framework: NarrativeFramework {
                story_arc: StoryArc {
                    setup_phase: PhaseNarrative {
                        description: "Preparations begin for the annual harvest celebration".to_string(),
                        key_moments: vec!["Festival announcement".to_string(), "Decorations setup".to_string()],
                        character_interactions: vec!["Meet the festival organizer".to_string()],
                        world_changes: vec!["Festival decorations appear".to_string()],
                    },
                    escalation_phase: PhaseNarrative {
                        description: "Festival activities intensify as more people join".to_string(),
                        key_moments: vec!["Harvest competition".to_string(), "Community feast".to_string()],
                        character_interactions: vec!["Compete with other players".to_string()],
                        world_changes: vec!["Market prices fluctuate".to_string()],
                    },
                    climax_phase: PhaseNarrative {
                        description: "Grand finale with major celebrations".to_string(),
                        key_moments: vec!["Harvest ceremony".to_string(), "Fireworks display".to_string()],
                        character_interactions: vec!["Address the crowd".to_string()],
                        world_changes: vec!["New trade routes established".to_string()],
                    },
                    resolution_phase: PhaseNarrative {
                        description: "Festival winds down with lasting impacts".to_string(),
                        key_moments: vec!["Cleanup activities".to_string(), "Final rewards".to_string()],
                        character_interactions: vec!["Thank participants".to_string()],
                        world_changes: vec!["Economic boost takes effect".to_string()],
                    },
                },
                character_roles: vec![
                    CharacterRole {
                        role_type: RoleType::Guide,
                        character_id: None,
                        involvement_level: 0.8,
                        narrative_function: "Festival organizer and guide".to_string(),
                    },
                    CharacterRole {
                        role_type: RoleType::Mentor,
                        character_id: None,
                        involvement_level: 0.5,
                        narrative_function: "Experienced farmer teaching techniques".to_string(),
                    },
                ],
                plot_devices: vec![PlotDevice::MacGuffin], // Special harvest artifact
                emotional_journey: EmotionalJourney {
                    starting_emotion: crate::narrative_engine::Emotion::Excited,
                    emotional_peaks: vec![
                        EmotionalPeak {
                            phase: "climax".to_string(),
                            emotion: crate::narrative_engine::Emotion::Happy,
                            intensity: 1.0,
                            trigger: "Successful completion".to_string(),
                        },
                    ],
                    ending_emotion: crate::narrative_engine::Emotion::Grateful,
                    tension_curve: vec![0.3, 0.6, 0.9, 0.4],
                },
            },
        };

        // Dragon Invasion Template
        let invasion_template = EventTemplate {
            template_id: Uuid::new_v4(),
            name: "Dragon Invasion".to_string(),
            template_type: EventType::Invasion,
            base_configuration: EventConfiguration {
                default_duration: 3600 * 6, // 6 hours
                participant_range: (5, 100),
                objective_templates: vec![
                    ObjectiveTemplate {
                        objective_type: ObjectiveType::Elimination,
                        scaling_formula: "base + participant_count * 2".to_string(),
                        difficulty_curve: vec![0.7, 0.8, 0.95],
                        reward_weight: 2.0,
                    },
                    ObjectiveTemplate {
                        objective_type: ObjectiveType::Protection,
                        scaling_formula: "100 - damage_taken".to_string(),
                        difficulty_curve: vec![0.6, 0.8, 0.9],
                        reward_weight: 1.5,
                    },
                ],
                reward_formulas: RewardFormulas {
                    base_experience: 5000,
                    experience_scaling: 2.0,
                    base_gold: 2500,
                    gold_scaling: 1.8,
                    item_drop_chances: [
                        ("dragon_scale".to_string(), 0.6),
                        ("rare_weapon".to_string(), 0.15),
                        ("legendary_armor".to_string(), 0.05),
                    ].iter().cloned().collect(),
                    reputation_gains: [
                        ("kingdom".to_string(), 100),
                        ("dragon_slayers".to_string(), 200),
                    ].iter().cloned().collect(),
                },
                consequence_patterns: vec![
                    ConsequencePattern {
                        pattern_type: ConsequenceType::WorldStateChange,
                        probability: 1.0,
                        magnitude_range: (0.8, 1.0),
                        duration_range: (3600 * 24 * 7, 3600 * 24 * 30),
                        conditions: vec!["dragon_defeated".to_string()],
                    },
                ],
            },
            variable_parameters: vec![
                VariableParameter {
                    parameter_name: "dragon_count".to_string(),
                    parameter_type: ParameterType::Integer,
                    default_value: "1".to_string(),
                    variation_rules: vec![
                        VariationRule {
                            condition: "high_player_count".to_string(),
                            modification: ParameterModification::Add(1.0),
                        },
                    ],
                },
            ],
            generation_rules: GenerationRules {
                frequency: EventFrequency::Random(90.0), // Every ~3 months
                timing_constraints: TimingConstraints {
                    time_of_day: None,
                    days_of_week: None,
                    seasons: None,
                    avoid_conflicts: true,
                    minimum_gap: Some(3600 * 24 * 30), // 30 days minimum
                },
                prerequisite_events: vec![],
                exclusion_events: vec![],
                population_requirements: PopulationRequirements {
                    minimum_online: 3,
                    target_regions: Some(vec!["kingdom".to_string(), "mountains".to_string()]),
                    player_level_range: Some((15, 100)),
                    faction_balance: None,
                },
            },
            narrative_framework: NarrativeFramework {
                story_arc: StoryArc {
                    setup_phase: PhaseNarrative {
                        description: "Dark clouds gather as dragons are spotted on the horizon".to_string(),
                        key_moments: vec!["Dragon sighting report".to_string(), "Emergency召集".to_string()],
                        character_interactions: vec!["Receive urgent quest from guard captain".to_string()],
                        world_changes: vec!["Panic spreads through settlements".to_string()],
                    },
                    escalation_phase: PhaseNarrative {
                        description: "Dragons attack settlements and heroes mobilize".to_string(),
                        key_moments: vec!["First dragon attack".to_string(), "Heroes assemble".to_string()],
                        character_interactions: vec!["Form raid groups".to_string()],
                        world_changes: vec!["Buildings damaged, NPCs flee".to_string()],
                    },
                    climax_phase: PhaseNarrative {
                        description: "Epic battle against the dragon threat".to_string(),
                        key_moments: vec!["Dragon boss fight".to_string(), "Heroic sacrifices".to_string()],
                        character_interactions: vec!["Coordinate with other players".to_string()],
                        world_changes: vec!["Massive destruction or salvation".to_string()],
                    },
                    resolution_phase: PhaseNarrative {
                        description: "Aftermath and recovery from the dragon invasion".to_string(),
                        key_moments: vec!["Victory celebration or mourning".to_string(), "Reconstruction begins".to_string()],
                        character_interactions: vec!["Receive recognition from leaders".to_string()],
                        world_changes: vec!["New monuments or memorials erected".to_string()],
                    },
                },
                character_roles: vec![
                    CharacterRole {
                        role_type: RoleType::Herald,
                        character_id: None,
                        involvement_level: 0.6,
                        narrative_function: "Warning messenger and quest giver".to_string(),
                    },
                    CharacterRole {
                        role_type: RoleType::Antagonist,
                        character_id: None,
                        involvement_level: 1.0,
                        narrative_function: "Dragon lord leading the invasion".to_string(),
                    },
                ],
                plot_devices: vec![PlotDevice::Sacrifice, PlotDevice::ChekhovsGun],
                emotional_journey: EmotionalJourney {
                    starting_emotion: crate::narrative_engine::Emotion::Fearful,
                    emotional_peaks: vec![
                        EmotionalPeak {
                            phase: "climax".to_string(),
                            emotion: crate::narrative_engine::Emotion::Determined,
                            intensity: 1.0,
                            trigger: "Dragon battle begins".to_string(),
                        },
                    ],
                    ending_emotion: crate::narrative_engine::Emotion::Proud,
                    tension_curve: vec![0.4, 0.7, 1.0, 0.3],
                },
            },
        };

        self.event_templates.extend(vec![festival_template, invasion_template]);
    }

    /// Create event from template
    pub fn create_event_from_template(&mut self, template_id: Uuid, customization: Option<EventCustomization>) -> Result<EventId> {
        let template = self.event_templates.iter()
            .find(|t| t.template_id == template_id)
            .ok_or_else(|| anyhow::anyhow!("Event template not found"))?;

        let event_id = Uuid::new_v4();
        let event = self.generate_event_from_template(event_id, template, customization)?;

        self.active_events.insert(event_id, event);
        tracing::info!("Created event {} from template {}", event_id, template_id);

        Ok(event_id)
    }

    /// Generate event from template
    fn generate_event_from_template(
        &self,
        event_id: EventId,
        template: &EventTemplate,
        customization: Option<EventCustomization>,
    ) -> Result<WorldEvent> {
        let name = customization.as_ref()
            .and_then(|c| c.name_override.clone())
            .unwrap_or_else(|| template.name.clone());

        let duration = (template.base_configuration.default_duration as f64 *
                       customization.as_ref().map_or(1.0, |c| c.duration_modifier)) as u32;

        let event = WorldEvent {
            event_id,
            name,
            description: format!("A {} event that will shape the world", template.template_type.type_name()),
            event_type: template.template_type.clone(),
            scope: EventScope::Regional, // Default scope
            status: EventStatus::Scheduled,
            start_time: Utc::now(),
            end_time: Some(Utc::now() + chrono::Duration::seconds(duration as i64)),
            duration: Some(duration),
            affected_locations: customization.as_ref()
                .and_then(|c| c.location_override.clone())
                .unwrap_or_default(),
            participants: vec![],
            objectives: self.generate_objectives_from_template(template)?,
            rewards: EventRewards {
                completion_rewards: vec![],
                participation_rewards: vec![],
                milestone_rewards: HashMap::new(),
                leaderboard_rewards: vec![],
            },
            consequences: vec![],
            narrative_impact: NarrativeImpact {
                significance_level: 0.5,
                affected_storylines: vec![],
                character_development: vec![],
                world_lore_additions: vec![],
                historical_importance: 0.6,
                player_legend_impact: 0.4,
            },
            triggers: vec![EventTrigger::TimeScheduled(Utc::now())],
            conditions: vec![],
            phases: self.generate_phases_from_narrative(&template.narrative_framework)?,
            current_phase: 0,
            metadata: EventMetadata {
                created_at: Utc::now(),
                creator: EventCreator::System,
                estimated_duration: duration,
                max_participants: customization.as_ref()
                    .and_then(|c| c.participant_limits.map(|(_, max)| max)),
                difficulty_rating: EventDifficulty::Normal,
                tags: vec![template.template_type.type_name()],
                repeatable: matches!(template.generation_rules.frequency, EventFrequency::Daily | EventFrequency::Weekly | EventFrequency::Monthly),
                seasonal: matches!(template.generation_rules.frequency, EventFrequency::Seasonal),
                player_initiated: false,
            },
        };

        Ok(event)
    }

    /// Generate objectives from template
    fn generate_objectives_from_template(&self, template: &EventTemplate) -> Result<Vec<EventObjective>> {
        let mut objectives = vec![];

        for obj_template in &template.base_configuration.objective_templates {
            let objective = EventObjective {
                objective_id: Uuid::new_v4(),
                title: format!("{:?} Objective", obj_template.objective_type),
                description: format!("Complete the {:?} task to help with the event", obj_template.objective_type),
                objective_type: obj_template.objective_type.clone(),
                target_value: 100.0, // Would be calculated from scaling formula
                current_value: 0.0,
                completion_criteria: CompletionCriteria::Threshold(100.0),
                participants_required: Some(1),
                time_limit: None,
                rewards: vec![],
                optional: false,
            };

            objectives.push(objective);
        }

        Ok(objectives)
    }

    /// Generate phases from narrative framework
    fn generate_phases_from_narrative(&self, framework: &NarrativeFramework) -> Result<Vec<EventPhase>> {
        let mut phases = vec![];

        phases.push(EventPhase {
            phase_id: "setup".to_string(),
            name: "Setup".to_string(),
            description: framework.story_arc.setup_phase.description.clone(),
            duration: None,
            objectives: vec![],
            start_conditions: vec![PhaseCondition::PreviousPhaseComplete],
            end_conditions: vec![PhaseCondition::TimeElapsed(1800)], // 30 minutes
            phase_effects: vec![],
            narrative_content: Some(framework.story_arc.setup_phase.description.clone()),
        });

        phases.push(EventPhase {
            phase_id: "escalation".to_string(),
            name: "Escalation".to_string(),
            description: framework.story_arc.escalation_phase.description.clone(),
            duration: None,
            objectives: vec![],
            start_conditions: vec![PhaseCondition::PreviousPhaseComplete],
            end_conditions: vec![PhaseCondition::TimeElapsed(3600)], // 1 hour
            phase_effects: vec![],
            narrative_content: Some(framework.story_arc.escalation_phase.description.clone()),
        });

        phases.push(EventPhase {
            phase_id: "climax".to_string(),
            name: "Climax".to_string(),
            description: framework.story_arc.climax_phase.description.clone(),
            duration: None,
            objectives: vec![],
            start_conditions: vec![PhaseCondition::PreviousPhaseComplete],
            end_conditions: vec![PhaseCondition::TimeElapsed(1800)], // 30 minutes
            phase_effects: vec![],
            narrative_content: Some(framework.story_arc.climax_phase.description.clone()),
        });

        phases.push(EventPhase {
            phase_id: "resolution".to_string(),
            name: "Resolution".to_string(),
            description: framework.story_arc.resolution_phase.description.clone(),
            duration: None,
            objectives: vec![],
            start_conditions: vec![PhaseCondition::PreviousPhaseComplete],
            end_conditions: vec![PhaseCondition::TimeElapsed(900)], // 15 minutes
            phase_effects: vec![],
            narrative_content: Some(framework.story_arc.resolution_phase.description.clone()),
        });

        Ok(phases)
    }

    /// Start an event
    pub fn start_event(&mut self, event_id: EventId) -> Result<()> {
        let event = self.active_events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        event.status = EventStatus::Active;
        event.start_time = Utc::now();

        tracing::info!("Started event: {} ({})", event.name, event_id);
        Ok(())
    }

    /// Add participant to event
    pub fn add_participant(&mut self, event_id: EventId, participant_type: ParticipantType) -> Result<()> {
        let event = self.active_events.get_mut(&event_id)
            .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

        let participant = EventParticipant {
            participant_id: Uuid::new_v4(),
            participant_type,
            role: ParticipantRole::Participant,
            contribution_score: 0.0,
            joined_at: Utc::now(),
            active: true,
        };

        event.participants.push(participant);
        Ok(())
    }

    /// Update event progress
    pub fn update_event_progress(&mut self, event_id: EventId) -> Result<()> {
        let should_advance = {
            let event = self.active_events.get(&event_id)
                .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

            // Check if current phase should end
            if let Some(current_phase) = event.phases.get(event.current_phase as usize) {
                current_phase.end_conditions.iter().any(|condition| {
                    self.evaluate_phase_condition(condition, event)
                })
            } else {
                false
            }
        };

        if should_advance {
            let event = self.active_events.get_mut(&event_id)
                .ok_or_else(|| anyhow::anyhow!("Event not found"))?;
            
            event.current_phase += 1;
            
            if event.current_phase >= event.phases.len() as u32 {
                self.complete_event(event_id)?;
            } else {
                tracing::info!("Event {} advanced to phase {}", event_id, event.current_phase);
            }
        }

        Ok(())
    }

    /// Evaluate phase condition
    fn evaluate_phase_condition(&self, condition: &PhaseCondition, event: &WorldEvent) -> bool {
        match condition {
            PhaseCondition::TimeElapsed(duration) => {
                let elapsed = (Utc::now() - event.start_time).num_seconds();
                elapsed >= *duration as i64
            },
            PhaseCondition::ParticipantCount(required) => {
                event.participants.len() >= *required as usize
            },
            _ => true, // Other conditions not implemented yet
        }
    }

    /// Complete event
    pub fn complete_event(&mut self, event_id: EventId) -> Result<EventOutcome> {
        // First, collect data and apply consequences without holding mutable reference
        let (completed_event, outcome, consequences) = {
            let event = self.active_events.get_mut(&event_id)
                .ok_or_else(|| anyhow::anyhow!("Event not found"))?;

            event.status = EventStatus::Completed;
            event.end_time = Some(Utc::now());

            let outcome = EventOutcome::Success; // Determine based on objectives completed

            let completed_event = CompletedEvent {
                event_id,
                name: event.name.clone(),
                start_time: event.start_time,
                end_time: event.end_time.unwrap(),
                participants: event.participants.clone(),
                outcome: outcome.clone(),
                consequences_applied: event.consequences.clone(),
                narrative_impact: event.narrative_impact.clone(),
                statistics: EventStatistics {
                    total_participants: event.participants.len() as u32,
                    objectives_completed: event.objectives.iter()
                        .filter(|obj| obj.current_value >= obj.target_value)
                        .count() as u32,
                    total_objectives: event.objectives.len() as u32,
                    average_contribution: event.participants.iter()
                        .map(|p| p.contribution_score)
                        .sum::<f64>() / event.participants.len().max(1) as f64,
                    top_contributors: vec![],
                    duration_seconds: (event.end_time.unwrap() - event.start_time).num_seconds() as u32,
                    resources_consumed: HashMap::new(),
                    rewards_distributed: HashMap::new(),
                },
            };

            (completed_event, outcome, event.consequences.clone())
        };

        // Apply consequences after releasing the mutable borrow
        for consequence in &consequences {
            self.apply_event_consequence(consequence)?;
        }

        self.event_history.push(completed_event);
        self.active_events.remove(&event_id);

        tracing::info!("Completed event {} with outcome: {:?}", event_id, outcome);
        Ok(outcome)
    }

    /// Apply event consequence
    fn apply_event_consequence(&mut self, consequence: &EventConsequence) -> Result<()> {
        tracing::info!("Applying event consequence: {:?} to {:?} with magnitude {}", 
                      consequence.consequence_type, consequence.target, consequence.magnitude);

        match consequence.consequence_type {
            ConsequenceType::WorldStateChange => {
                // Modify world state
            },
            ConsequenceType::EconomicChange => {
                // Adjust economic indicators
            },
            _ => {
                // Handle other consequence types
            },
        }

        Ok(())
    }

    /// Get active events
    pub fn get_active_events(&self) -> Vec<&WorldEvent> {
        self.active_events.values()
            .filter(|event| matches!(event.status, EventStatus::Active))
            .collect()
    }

    /// Get events player can participate in
    pub fn get_available_events(&self, _player_id: PlayerId) -> Vec<&WorldEvent> {
        // Filter events based on player eligibility
        self.active_events.values()
            .filter(|event| matches!(event.status, EventStatus::Active | EventStatus::Starting))
            .collect()
    }
}

impl EventType {
    fn type_name(&self) -> String {
        match self {
            EventType::Natural => "natural".to_string(),
            EventType::Political => "political".to_string(),
            EventType::Social => "social".to_string(),
            EventType::Economic => "economic".to_string(),
            EventType::Mystical => "mystical".to_string(),
            EventType::Invasion => "invasion".to_string(),
            EventType::Discovery => "discovery".to_string(),
            EventType::Catastrophic => "catastrophic".to_string(),
            EventType::Seasonal => "seasonal".to_string(),
            EventType::Player => "player".to_string(),
            EventType::Quest => "quest".to_string(),
            EventType::Faction => "faction".to_string(),
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            current_season: "spring".to_string(),
            time_of_day: 12,
            weather_patterns: HashMap::new(),
            political_tensions: HashMap::new(),
            economic_indicators: HashMap::new(),
            magical_levels: HashMap::new(),
            population_distribution: HashMap::new(),
            resource_availability: HashMap::new(),
            threat_levels: HashMap::new(),
        }
    }
}

impl Default for WorldEventCoordinator {
    fn default() -> Self {
        Self::new()
    }
}