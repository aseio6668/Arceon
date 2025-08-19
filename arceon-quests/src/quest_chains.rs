/*!
# Quest Chain Management

Manages interconnected quest sequences, storylines, and complex
multi-part adventures with branching narratives.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{Quest, QuestId, PlayerId, ChainId, QuestType};

/// Quest chain management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestChainManager {
    pub active_chains: HashMap<ChainId, QuestChain>,
    pub player_chains: HashMap<PlayerId, Vec<ChainId>>,
    pub chain_templates: Vec<ChainTemplate>,
    pub completion_rewards: HashMap<ChainId, ChainRewards>,
    pub branching_logic: BranchingLogic,
}

/// A sequence of interconnected quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestChain {
    pub chain_id: ChainId,
    pub name: String,
    pub description: String,
    pub chain_type: ChainType,
    pub quests: Vec<ChainQuest>,
    pub current_position: HashMap<PlayerId, ChainPosition>,
    pub chain_state: ChainState,
    pub requirements: ChainRequirements,
    pub rewards: ChainRewards,
    pub branching_points: Vec<BranchingPoint>,
    pub metadata: ChainMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainType {
    Linear,         // Sequential progression
    Branching,      // Multiple paths available
    Parallel,       // Multiple quests can be done simultaneously
    Conditional,    // Progression depends on choices/conditions
    Cyclic,         // Can repeat certain sections
    Adaptive,       // Changes based on player behavior
    Collaborative,  // Requires multiple players
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainQuest {
    pub quest_id: QuestId,
    pub position_in_chain: u32,
    pub required_for_progression: bool,
    pub prerequisites: Vec<ChainPrerequisite>,
    pub unlocks: Vec<ChainUnlock>,
    pub variations: Vec<QuestVariation>,
    pub completion_effects: Vec<ChainEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainPrerequisite {
    QuestCompleted(QuestId),
    QuestFailed(QuestId),
    ChoiceMade(String, String), // choice_id, option
    PlayerLevel(u32),
    TimeElapsed(u32), // seconds since chain start
    BranchTaken(String),
    CollaboratorAction(PlayerId, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainUnlock {
    Quest(QuestId),
    Branch(String),
    Reward(String),
    NarrativeContent(String),
    WorldState(String),
    Character(crate::NPCId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestVariation {
    pub variation_id: String,
    pub conditions: Vec<VariationCondition>,
    pub quest_modifications: QuestModifications,
    pub narrative_changes: NarrativeChanges,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariationCondition {
    PlayerChoice(String),
    PreviousQuestOutcome(QuestId, String),
    PlayerClass(String),
    Reputation(String, i32),
    TimeOfDay(u32, u32),
    Seasonal(String),
    RandomChance(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModifications {
    pub objective_changes: Vec<ObjectiveChange>,
    pub reward_modifiers: RewardModifiers,
    pub difficulty_adjustment: f64,
    pub time_limit_changes: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveChange {
    Add(String), // Add new objective
    Remove(String), // Remove existing objective
    Modify(String, String), // Modify objective description
    ChangeTarget(String, String), // Change objective target
    AdjustQuantity(String, i32), // Adjust required quantity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardModifiers {
    pub experience_multiplier: f64,
    pub gold_multiplier: f64,
    pub additional_items: Vec<String>,
    pub reputation_bonuses: HashMap<String, i32>,
    pub title_unlocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeChanges {
    pub dialogue_variations: HashMap<crate::NPCId, String>,
    pub story_flag_changes: HashMap<String, bool>,
    pub mood_changes: Vec<(String, crate::NarrativeMood)>,
    pub character_relationship_effects: Vec<RelationshipEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEffect {
    pub character_id: crate::NPCId,
    pub relationship_change: f64,
    pub relationship_aspect: RelationshipAspect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipAspect {
    Trust,
    Affection,
    Respect,
    Fear,
    Loyalty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainEffect {
    UnlockChain(ChainId),
    ModifyWorldState(String, String),
    TriggerEvent(String),
    CharacterStateChange(crate::NPCId, String),
    PlayerFlagSet(String, bool),
    ReputationChange(String, i32),
    ItemGrant(crate::ItemId, u32),
    SkillUnlock(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainPosition {
    pub current_quest_index: u32,
    pub completed_quests: Vec<QuestId>,
    pub available_quests: Vec<QuestId>,
    pub branch_history: Vec<String>,
    pub choice_history: Vec<PlayerChainChoice>,
    pub start_time: DateTime<Utc>,
    pub completion_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChainChoice {
    pub choice_id: String,
    pub chosen_option: String,
    pub timestamp: DateTime<Utc>,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainState {
    Active,
    Paused,
    Completed,
    Failed,
    Abandoned,
    Locked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainRequirements {
    pub minimum_level: Option<u32>,
    pub prerequisite_chains: Vec<ChainId>,
    pub prerequisite_quests: Vec<QuestId>,
    pub faction_requirements: HashMap<String, i32>,
    pub item_requirements: Vec<crate::ItemRequirement>,
    pub skill_requirements: HashMap<String, u32>,
    pub time_restrictions: Option<TimeRestrictions>,
    pub player_count: Option<PlayerCountRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestrictions {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_limit: Option<u32>, // Maximum duration in seconds
    pub cooldown_period: Option<u32>, // Cooldown before retry in seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCountRequirement {
    pub minimum: u32,
    pub maximum: u32,
    pub roles_required: Vec<String>, // Specific player roles/classes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainRewards {
    pub completion_rewards: CompletionRewards,
    pub milestone_rewards: HashMap<u32, MilestoneReward>,
    pub branch_specific_rewards: HashMap<String, BranchRewards>,
    pub collaborative_bonuses: Vec<CollaborativeBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionRewards {
    pub experience: u64,
    pub gold: u64,
    pub items: Vec<crate::ItemReward>,
    pub titles: Vec<String>,
    pub achievements: Vec<String>,
    pub world_unlocks: Vec<String>,
    pub narrative_conclusions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneReward {
    pub milestone_name: String,
    pub quest_index: u32,
    pub rewards: CompletionRewards,
    pub celebration_event: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchRewards {
    pub branch_name: String,
    pub unique_rewards: CompletionRewards,
    pub exclusive_content: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborativeBonus {
    pub bonus_name: String,
    pub required_participants: u32,
    pub bonus_multiplier: f64,
    pub special_rewards: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingPoint {
    pub branch_id: String,
    pub quest_index: u32,
    pub decision_prompt: String,
    pub options: Vec<BranchOption>,
    pub consequences: Vec<BranchConsequence>,
    pub merge_points: Vec<u32>, // Quest indices where branches can merge
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchOption {
    pub option_id: String,
    pub description: String,
    pub requirements: Vec<BranchRequirement>,
    pub immediate_effects: Vec<ChainEffect>,
    pub long_term_consequences: Vec<LongTermConsequence>,
    pub narrative_impact: NarrativeImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BranchRequirement {
    PlayerLevel(u32),
    Skill(String, u32),
    Item(crate::ItemId),
    Reputation(String, i32),
    PreviousChoice(String, String),
    CharacterRelationship(crate::NPCId, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermConsequence {
    pub consequence_type: ConsequenceType,
    pub activation_delay: u32, // Seconds before consequence takes effect
    pub duration: Option<u32>, // How long the consequence lasts
    pub magnitude: f64,
    pub affected_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    QuestAvailabilityChange,
    WorldStateModification,
    CharacterBehaviorChange,
    PlayerAbilityModification,
    EconomicImpact,
    ReputationShift,
    NarrativePathChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeImpact {
    pub mood_change: Option<crate::NarrativeMood>,
    pub theme_emphasis: Vec<String>,
    pub character_development: Vec<CharacterDevelopment>,
    pub story_revelation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDevelopment {
    pub character_id: crate::NPCId,
    pub development_type: DevelopmentType,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DevelopmentType {
    PersonalityShift,
    MotivationChange,
    RelationshipEvolution,
    SkillDevelopment,
    MoralAlignment,
    EmotionalGrowth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchConsequence {
    pub trigger_condition: ConsequenceTrigger,
    pub effect: ChainEffect,
    pub reversible: bool,
    pub narrative_significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceTrigger {
    Immediate,
    QuestCompletion(QuestId),
    TimeElapsed(u32),
    PlayerAction(String),
    WorldEvent(String),
    CollaborativeGoal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainMetadata {
    pub created_at: DateTime<Utc>,
    pub creator: ChainCreator,
    pub estimated_duration: u32, // Total estimated time in minutes
    pub difficulty_rating: ChainDifficulty,
    pub tags: Vec<String>,
    pub storyline_type: StorylineType,
    pub player_rating: f64,
    pub completion_statistics: CompletionStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainCreator {
    System,
    Designer(String),
    Player(PlayerId),
    Community,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainDifficulty {
    Casual,
    Normal,
    Challenging,
    Hardcore,
    Legendary,
    Mythic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorylineType {
    PersonalJourney,
    EpicQuest,
    PoliticalIntrigue,
    Mystery,
    Romance,
    Tragedy,
    Comedy,
    Horror,
    Adventure,
    Drama,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionStatistics {
    pub total_attempts: u32,
    pub successful_completions: u32,
    pub average_completion_time: u32,
    pub most_popular_branch: Option<String>,
    pub abandonment_rate: f64,
    pub player_ratings: Vec<PlayerRating>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRating {
    pub player_id: PlayerId,
    pub rating: f64, // 1.0 to 5.0
    pub comment: Option<String>,
    pub timestamp: DateTime<Utc>,
}

/// Template for creating quest chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub chain_type: ChainType,
    pub template_structure: TemplateStructure,
    pub quest_patterns: Vec<QuestPattern>,
    pub branching_patterns: Vec<BranchingPattern>,
    pub narrative_flow: NarrativeFlow,
    pub customization_options: CustomizationOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStructure {
    pub acts: Vec<ChainAct>,
    pub total_estimated_quests: u32,
    pub minimum_quests: u32,
    pub maximum_quests: u32,
    pub expected_branches: u32,
    pub convergence_points: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainAct {
    pub act_number: u32,
    pub name: String,
    pub purpose: String,
    pub quest_count_range: (u32, u32),
    pub tension_level: f64,
    pub major_decision_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestPattern {
    pub pattern_name: String,
    pub quest_types: Vec<QuestType>,
    pub difficulty_progression: DifficultyProgression,
    pub narrative_role: NarrativeRole,
    pub frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyProgression {
    Linear,          // Steady increase
    Exponential,     // Rapid increase
    Stepped,         // Plateaus with jumps
    Wave,            // Up and down pattern
    Custom(Vec<f64>), // Custom curve
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeRole {
    Introduction,
    Development,
    Climax,
    Resolution,
    Transition,
    Exploration,
    CharacterBuilding,
    WorldBuilding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingPattern {
    pub pattern_name: String,
    pub branch_count: u32,
    pub merge_strategy: MergeStrategy,
    pub decision_weight: f64,
    pub impact_scope: ImpactScope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeStrategy {
    NoMerge,         // Branches remain separate
    EarlyMerge,      // Merge after a few quests
    LateMerge,       // Merge near end
    ConditionalMerge, // Merge based on conditions
    MultipleMergePoints, // Several merge opportunities
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactScope {
    Local,           // Affects only immediate chain
    Regional,        // Affects region/zone
    Global,          // Affects entire world
    Personal,        // Affects only player
    Social,          // Affects player relationships
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFlow {
    pub pacing: PacingStructure,
    pub emotional_arc: EmotionalArc,
    pub theme_development: ThemeDevelopment,
    pub character_involvement: CharacterInvolvement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingStructure {
    pub introduction_length: f64,  // Percentage of total chain
    pub rising_action_length: f64,
    pub climax_position: f64,
    pub falling_action_length: f64,
    pub resolution_length: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalArc {
    pub starting_emotion: crate::NarrativeMood,
    pub emotional_peaks: Vec<EmotionalPeak>,
    pub ending_emotion: crate::NarrativeMood,
    pub emotional_range: f64, // How much emotion varies
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPeak {
    pub position: f64, // Position in chain (0.0 to 1.0)
    pub emotion: crate::NarrativeMood,
    pub intensity: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDevelopment {
    pub primary_themes: Vec<String>,
    pub theme_introduction_points: HashMap<String, f64>,
    pub theme_resolution_points: HashMap<String, f64>,
    pub thematic_consistency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterInvolvement {
    pub protagonist_development: CharacterArcStructure,
    pub supporting_characters: Vec<SupportingCharacterRole>,
    pub antagonist_presence: AntagonistStructure,
    pub character_interaction_frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArcStructure {
    pub arc_type: crate::narrative_engine::CharacterArcType,
    pub development_stages: Vec<String>,
    pub key_growth_moments: Vec<f64>, // Positions in chain
    pub relationship_changes: Vec<RelationshipChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipChange {
    pub target_character: crate::NPCId,
    pub change_position: f64,
    pub change_magnitude: f64,
    pub change_type: RelationshipAspect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportingCharacterRole {
    pub character_id: Option<crate::NPCId>,
    pub role_type: SupportingRole,
    pub involvement_level: f64,
    pub key_moments: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupportingRole {
    Mentor,
    Ally,
    Rival,
    LoveInterest,
    ComicRelief,
    WiseOldOne,
    Trickster,
    Herald,
    ThresholdGuardian,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntagonistStructure {
    pub antagonist_type: AntagonistType,
    pub reveal_position: f64,
    pub confrontation_points: Vec<f64>,
    pub power_progression: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AntagonistType {
    PersonalEnemy,
    SystemicThreat,
    NaturalForce,
    InternalConflict,
    SocietalPressure,
    CosmicHorror,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationOptions {
    pub variable_quest_order: bool,
    pub optional_side_branches: Vec<String>,
    pub difficulty_scaling: bool,
    pub player_choice_weight: f64,
    pub narrative_customization: NarrativeCustomization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeCustomization {
    pub allow_character_substitution: bool,
    pub allow_location_variation: bool,
    pub allow_theme_modification: bool,
    pub player_background_integration: f64,
}

/// Logic for handling branching decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingLogic {
    pub decision_algorithms: HashMap<String, DecisionAlgorithm>,
    pub merge_strategies: HashMap<String, MergeAlgorithm>,
    pub consequence_calculators: HashMap<String, ConsequenceCalculator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionAlgorithm {
    pub algorithm_name: String,
    pub factors: Vec<DecisionFactor>,
    pub weights: HashMap<String, f64>,
    pub fallback_strategy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionFactor {
    PlayerPreference,
    CharacterRelationships,
    PreviousChoices,
    WorldState,
    RandomElement,
    NarrativeFlow,
    PlayerSkills,
    TimeConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeAlgorithm {
    pub algorithm_name: String,
    pub merge_conditions: Vec<MergeCondition>,
    pub state_reconciliation: StateReconciliation,
    pub narrative_smoothing: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MergeCondition {
    QuestCompletion(QuestId),
    PlayerConsensus(f64),
    TimeElapsed(u32),
    WorldStateConvergence,
    NarrativeNecessity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateReconciliation {
    pub conflicting_flags: HashMap<String, ReconciliationStrategy>,
    pub relationship_averaging: bool,
    pub reputation_blending: f64,
    pub item_consolidation: ItemConsolidationStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReconciliationStrategy {
    TakeHighest,
    TakeLowest,
    Average,
    PlayerChoice,
    NarrativePriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemConsolidationStrategy {
    KeepAll,
    RemoveDuplicates,
    PlayerChoice,
    HighestQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequenceCalculator {
    pub calculator_name: String,
    pub impact_factors: Vec<ImpactFactor>,
    pub time_decay: f64,
    pub cascade_effects: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImpactFactor {
    PlayerLevel,
    QuestImportance,
    WorldSignificance,
    CharacterInvolvement,
    PlayerInvestment,
    NarrativeWeight,
}

impl QuestChainManager {
    /// Create new quest chain manager
    pub fn new() -> Self {
        let mut manager = Self {
            active_chains: HashMap::new(),
            player_chains: HashMap::new(),
            chain_templates: vec![],
            completion_rewards: HashMap::new(),
            branching_logic: BranchingLogic::default(),
        };
        
        manager.initialize_default_templates();
        manager
    }

    /// Initialize default chain templates
    fn initialize_default_templates(&mut self) {
        // Hero's Journey Template
        let heroes_journey = ChainTemplate {
            template_id: Uuid::new_v4(),
            name: "Hero's Journey".to_string(),
            chain_type: ChainType::Linear,
            template_structure: TemplateStructure {
                acts: vec![
                    ChainAct {
                        act_number: 1,
                        name: "The Call".to_string(),
                        purpose: "Introduce the adventure and initial challenges".to_string(),
                        quest_count_range: (2, 4),
                        tension_level: 0.3,
                        major_decision_points: vec!["Accept the call".to_string()],
                    },
                    ChainAct {
                        act_number: 2,
                        name: "The Journey".to_string(),
                        purpose: "Character growth through trials and allies".to_string(),
                        quest_count_range: (4, 8),
                        tension_level: 0.7,
                        major_decision_points: vec!["Choose your path".to_string(), "Face the abyss".to_string()],
                    },
                    ChainAct {
                        act_number: 3,
                        name: "The Return".to_string(),
                        purpose: "Apply wisdom and complete transformation".to_string(),
                        quest_count_range: (2, 3),
                        tension_level: 0.9,
                        major_decision_points: vec!["Final confrontation".to_string()],
                    },
                ],
                total_estimated_quests: 10,
                minimum_quests: 6,
                maximum_quests: 15,
                expected_branches: 2,
                convergence_points: vec![5, 8],
            },
            quest_patterns: vec![
                QuestPattern {
                    pattern_name: "Call to Adventure".to_string(),
                    quest_types: vec![QuestType::Discovery, QuestType::Social],
                    difficulty_progression: DifficultyProgression::Linear,
                    narrative_role: NarrativeRole::Introduction,
                    frequency: 1.0,
                },
                QuestPattern {
                    pattern_name: "Trial by Fire".to_string(),
                    quest_types: vec![QuestType::Combat, QuestType::Personal],
                    difficulty_progression: DifficultyProgression::Exponential,
                    narrative_role: NarrativeRole::Climax,
                    frequency: 0.7,
                },
            ],
            branching_patterns: vec![
                BranchingPattern {
                    pattern_name: "Mentor Choice".to_string(),
                    branch_count: 2,
                    merge_strategy: MergeStrategy::LateMerge,
                    decision_weight: 0.8,
                    impact_scope: ImpactScope::Personal,
                },
            ],
            narrative_flow: NarrativeFlow {
                pacing: PacingStructure {
                    introduction_length: 0.2,
                    rising_action_length: 0.4,
                    climax_position: 0.8,
                    falling_action_length: 0.15,
                    resolution_length: 0.05,
                },
                emotional_arc: EmotionalArc {
                    starting_emotion: crate::NarrativeMood::Peaceful,
                    emotional_peaks: vec![
                        EmotionalPeak {
                            position: 0.6,
                            emotion: crate::NarrativeMood::Tense,
                            intensity: 0.8,
                            duration: 0.1,
                        },
                        EmotionalPeak {
                            position: 0.85,
                            emotion: crate::NarrativeMood::Heroic,
                            intensity: 1.0,
                            duration: 0.05,
                        },
                    ],
                    ending_emotion: crate::NarrativeMood::Uplifting,
                    emotional_range: 0.8,
                },
                theme_development: ThemeDevelopment {
                    primary_themes: vec!["growth".to_string(), "courage".to_string(), "wisdom".to_string()],
                    theme_introduction_points: [
                        ("growth".to_string(), 0.1),
                        ("courage".to_string(), 0.3),
                        ("wisdom".to_string(), 0.6),
                    ].iter().cloned().collect(),
                    theme_resolution_points: [
                        ("growth".to_string(), 0.9),
                        ("courage".to_string(), 0.85),
                        ("wisdom".to_string(), 0.95),
                    ].iter().cloned().collect(),
                    thematic_consistency: 0.9,
                },
                character_involvement: CharacterInvolvement {
                    protagonist_development: CharacterArcStructure {
                        arc_type: crate::narrative_engine::CharacterArcType::Growth,
                        development_stages: vec![
                            "Reluctant hero".to_string(),
                            "Willing participant".to_string(),
                            "Tested warrior".to_string(),
                            "Wise hero".to_string(),
                        ],
                        key_growth_moments: vec![0.2, 0.5, 0.8],
                        relationship_changes: vec![],
                    },
                    supporting_characters: vec![
                        SupportingCharacterRole {
                            character_id: None,
                            role_type: SupportingRole::Mentor,
                            involvement_level: 0.7,
                            key_moments: vec![0.1, 0.4, 0.9],
                        },
                        SupportingCharacterRole {
                            character_id: None,
                            role_type: SupportingRole::Ally,
                            involvement_level: 0.6,
                            key_moments: vec![0.3, 0.6, 0.8],
                        },
                    ],
                    antagonist_presence: AntagonistStructure {
                        antagonist_type: AntagonistType::PersonalEnemy,
                        reveal_position: 0.3,
                        confrontation_points: vec![0.6, 0.85],
                        power_progression: 0.8,
                    },
                    character_interaction_frequency: 0.7,
                },
            },
            customization_options: CustomizationOptions {
                variable_quest_order: false,
                optional_side_branches: vec!["romance".to_string(), "revenge".to_string()],
                difficulty_scaling: true,
                player_choice_weight: 0.6,
                narrative_customization: NarrativeCustomization {
                    allow_character_substitution: true,
                    allow_location_variation: true,
                    allow_theme_modification: false,
                    player_background_integration: 0.8,
                },
            },
        };

        self.chain_templates.push(heroes_journey);
    }

    /// Create a new quest chain from template
    pub fn create_chain_from_template(
        &mut self,
        template_id: Uuid,
        player_id: PlayerId,
        customization: Option<ChainCustomization>,
    ) -> Result<ChainId> {
        let template = self.chain_templates.iter()
            .find(|t| t.template_id == template_id)
            .ok_or_else(|| anyhow::anyhow!("Chain template not found"))?;

        let chain_id = Uuid::new_v4();
        let chain = self.generate_chain_from_template(chain_id, template, customization)?;

        self.active_chains.insert(chain_id, chain);
        self.player_chains.entry(player_id).or_default().push(chain_id);

        tracing::info!("Created quest chain {} from template {} for player {}", 
                      chain_id, template_id, player_id);

        Ok(chain_id)
    }

    /// Generate chain from template
    fn generate_chain_from_template(
        &self,
        chain_id: ChainId,
        template: &ChainTemplate,
        _customization: Option<ChainCustomization>,
    ) -> Result<QuestChain> {
        let chain = QuestChain {
            chain_id,
            name: template.name.clone(),
            description: format!("An epic {} adventure awaits", template.name.to_lowercase()),
            chain_type: template.chain_type.clone(),
            quests: vec![], // Would be populated with actual quests
            current_position: HashMap::new(),
            chain_state: ChainState::Active,
            requirements: ChainRequirements {
                minimum_level: Some(1),
                prerequisite_chains: vec![],
                prerequisite_quests: vec![],
                faction_requirements: HashMap::new(),
                item_requirements: vec![],
                skill_requirements: HashMap::new(),
                time_restrictions: None,
                player_count: None,
            },
            rewards: ChainRewards {
                completion_rewards: CompletionRewards {
                    experience: 10000,
                    gold: 5000,
                    items: vec![],
                    titles: vec!["Hero".to_string()],
                    achievements: vec!["Completed Hero's Journey".to_string()],
                    world_unlocks: vec!["Advanced Quests".to_string()],
                    narrative_conclusions: vec!["Your legend begins".to_string()],
                },
                milestone_rewards: HashMap::new(),
                branch_specific_rewards: HashMap::new(),
                collaborative_bonuses: vec![],
            },
            branching_points: vec![],
            metadata: ChainMetadata {
                created_at: Utc::now(),
                creator: ChainCreator::System,
                estimated_duration: template.template_structure.total_estimated_quests * 30, // 30 min per quest
                difficulty_rating: ChainDifficulty::Normal,
                tags: template.narrative_flow.theme_development.primary_themes.clone(),
                storyline_type: StorylineType::PersonalJourney,
                player_rating: 0.0,
                completion_statistics: CompletionStatistics {
                    total_attempts: 0,
                    successful_completions: 0,
                    average_completion_time: 0,
                    most_popular_branch: None,
                    abandonment_rate: 0.0,
                    player_ratings: vec![],
                },
            },
        };

        Ok(chain)
    }

    /// Find suitable chain for a quest
    pub fn find_suitable_chain(&self, quest: &Quest) -> Option<ChainId> {
        // Look for chains that might fit this quest based on type and theme
        self.active_chains.iter()
            .find(|(_, chain)| {
                matches!(chain.chain_state, ChainState::Active) &&
                self.quest_fits_chain_theme(quest, chain)
            })
            .map(|(chain_id, _)| *chain_id)
    }

    /// Check if quest fits chain theme
    fn quest_fits_chain_theme(&self, quest: &Quest, chain: &QuestChain) -> bool {
        // Check if quest themes overlap with chain themes
        let quest_themes = &quest.narrative_context.themes;
        let chain_themes = &chain.metadata.tags;

        quest_themes.iter().any(|theme| chain_themes.contains(theme)) ||
        matches!(quest.quest_type, QuestType::Main | QuestType::Chain)
    }

    /// Add quest to existing chain
    pub fn add_quest_to_chain(&mut self, chain_id: ChainId, quest_id: QuestId) -> Result<()> {
        let chain = self.active_chains.get_mut(&chain_id)
            .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;

        if chain.quests.iter().any(|q| q.quest_id == quest_id) {
            return Err(anyhow::anyhow!("Quest already in chain"));
        }

        let chain_quest = ChainQuest {
            quest_id,
            position_in_chain: chain.quests.len() as u32,
            required_for_progression: true,
            prerequisites: vec![],
            unlocks: vec![],
            variations: vec![],
            completion_effects: vec![],
        };

        chain.quests.push(chain_quest);
        tracing::info!("Added quest {} to chain {}", quest_id, chain_id);

        Ok(())
    }

    /// Handle quest completion in chain context
    pub fn on_quest_completed(&mut self, quest_id: QuestId) -> Result<()> {
        let chain_ids: Vec<ChainId> = self.active_chains.iter()
            .filter(|(_, chain)| chain.quests.iter().any(|q| q.quest_id == quest_id))
            .map(|(chain_id, _)| *chain_id)
            .collect();

        for chain_id in chain_ids {
            self.advance_chain_progress(chain_id, quest_id)?;
        }

        Ok(())
    }

    /// Advance chain progress after quest completion
    fn advance_chain_progress(&mut self, chain_id: ChainId, completed_quest_id: QuestId) -> Result<()> {
        // First, collect the data we need without holding the mutable borrow
        let (player_ids, chain_quest_count) = {
            let chain = self.active_chains.get(&chain_id)
                .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;
            (chain.current_position.keys().cloned().collect::<Vec<_>>(), chain.quests.len())
        };
        
        // Collect players that completed or need updates
        let mut completed_players = Vec::new();
        let mut update_players = Vec::new();
        
        // Update player positions
        if let Some(chain) = self.active_chains.get_mut(&chain_id) {
            for player_id in player_ids {
                if let Some(position) = chain.current_position.get_mut(&player_id) {
                    // Add to completed quests
                    position.completed_quests.push(completed_quest_id);
                    
                    // Update completion percentage
                    position.completion_percentage = position.completed_quests.len() as f64 / chain_quest_count as f64;
                    
                    // Check for chain completion
                    if position.completion_percentage >= 1.0 {
                        completed_players.push(player_id);
                    } else {
                        update_players.push(player_id);
                    }
                }
            }
        }
        
        // Now handle completions and updates without holding the chain borrow
        for player_id in completed_players {
            self.complete_chain(chain_id, player_id)?;
        }
        
        for player_id in update_players {
            self.update_available_quests(chain_id, player_id)?;
        }

        Ok(())
    }

    /// Complete chain for player
    fn complete_chain(&mut self, chain_id: ChainId, player_id: PlayerId) -> Result<()> {
        let chain = self.active_chains.get_mut(&chain_id)
            .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;

        chain.chain_state = ChainState::Completed;
        
        // Update completion statistics
        chain.metadata.completion_statistics.successful_completions += 1;
        
        tracing::info!("Player {} completed chain {}", player_id, chain_id);
        Ok(())
    }

    /// Update available quests for player in chain
    fn update_available_quests(&mut self, chain_id: ChainId, player_id: PlayerId) -> Result<()> {
        let chain = self.active_chains.get(&chain_id)
            .ok_or_else(|| anyhow::anyhow!("Chain not found"))?;

        let position = chain.current_position.get(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player not in chain"))?;

        // Find next quests that can be unlocked
        let next_available: Vec<QuestId> = chain.quests.iter()
            .filter(|quest| {
                // Quest not yet completed and prerequisites met
                !position.completed_quests.contains(&quest.quest_id) &&
                self.prerequisites_met(quest, position)
            })
            .map(|quest| quest.quest_id)
            .collect();

        // Update available quests (would need mutable access to modify position)
        tracing::info!("Updated available quests for player {} in chain {}: {:?}", 
                      player_id, chain_id, next_available);

        Ok(())
    }

    /// Check if quest prerequisites are met
    fn prerequisites_met(&self, quest: &ChainQuest, position: &ChainPosition) -> bool {
        quest.prerequisites.iter().all(|prereq| {
            match prereq {
                ChainPrerequisite::QuestCompleted(quest_id) => {
                    position.completed_quests.contains(quest_id)
                },
                ChainPrerequisite::ChoiceMade(choice_id, option) => {
                    position.choice_history.iter().any(|choice| 
                        choice.choice_id == *choice_id && choice.chosen_option == *option
                    )
                },
                ChainPrerequisite::BranchTaken(branch) => {
                    position.branch_history.contains(branch)
                },
                _ => true, // Other prerequisites not implemented yet
            }
        })
    }

    /// Get player's progress in chain
    pub fn get_chain_progress(&self, chain_id: ChainId, player_id: PlayerId) -> Option<&ChainPosition> {
        self.active_chains.get(&chain_id)
            .and_then(|chain| chain.current_position.get(&player_id))
    }

    /// Get active chains for player
    pub fn get_player_chains(&self, player_id: PlayerId) -> Vec<&QuestChain> {
        if let Some(chain_ids) = self.player_chains.get(&player_id) {
            chain_ids.iter()
                .filter_map(|chain_id| self.active_chains.get(chain_id))
                .filter(|chain| matches!(chain.chain_state, ChainState::Active))
                .collect()
        } else {
            vec![]
        }
    }
}

/// Customization options for chain creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainCustomization {
    pub preferred_themes: Vec<String>,
    pub difficulty_preference: ChainDifficulty,
    pub estimated_time: Option<u32>,
    pub collaboration_preference: bool,
    pub narrative_style: Option<StorylineType>,
}

impl Default for BranchingLogic {
    fn default() -> Self {
        Self {
            decision_algorithms: HashMap::new(),
            merge_strategies: HashMap::new(),
            consequence_calculators: HashMap::new(),
        }
    }
}

impl Default for QuestChainManager {
    fn default() -> Self {
        Self::new()
    }
}