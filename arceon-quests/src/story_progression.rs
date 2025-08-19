/*!
# Story Progression Tracking

Tracks player narrative progression, story milestones,
and overall journey through the game's narrative arcs.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, QuestId, EventId, NPCId, LocationId};

/// Story progression tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryProgressionTracker {
    pub player_progressions: HashMap<PlayerId, PlayerStoryProgression>,
    pub story_arcs: Vec<StoryArc>,
    pub milestones: Vec<StoryMilestone>,
    pub global_story_state: GlobalStoryState,
    pub narrative_analytics: NarrativeAnalytics,
}

/// Individual player's story progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStoryProgression {
    pub player_id: PlayerId,
    pub overall_progress: f64, // 0.0 to 1.0
    pub active_arcs: Vec<ActiveArc>,
    pub completed_arcs: Vec<CompletedArc>,
    pub milestone_achievements: Vec<MilestoneAchievement>,
    pub narrative_choices: HashMap<String, String>,
    pub character_relationships: HashMap<NPCId, RelationshipProgression>,
    pub story_flags: HashMap<String, StoryFlag>,
    pub personal_legend: PersonalLegend,
    pub narrative_preferences: NarrativePreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveArc {
    pub arc_id: Uuid,
    pub progress: f64, // 0.0 to 1.0
    pub current_chapter: u32,
    pub completed_quests: Vec<QuestId>,
    pub active_quests: Vec<QuestId>,
    pub available_branches: Vec<String>,
    pub chosen_path: Option<String>,
    pub start_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedArc {
    pub arc_id: Uuid,
    pub completion_time: DateTime<Utc>,
    pub final_outcome: ArcOutcome,
    pub choices_made: Vec<SignificantChoice>,
    pub character_impacts: Vec<CharacterImpact>,
    pub world_changes: Vec<String>,
    pub satisfaction_rating: Option<f64>,
    pub replay_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArcOutcome {
    HeroicSuccess,
    TragiHeroicSuccess,
    BittersweetVictory,
    PyrrhicVictory,
    Failure,
    Tragedy,
    Redemption,
    Sacrifice,
    Transformation,
    Mystery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificantChoice {
    pub choice_id: String,
    pub description: String,
    pub alternatives: Vec<String>,
    pub chosen_option: String,
    pub consequences: Vec<String>,
    pub narrative_weight: f64,
    pub emotional_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterImpact {
    pub character_id: NPCId,
    pub relationship_change: f64,
    pub character_development: String,
    pub fate: Option<CharacterFate>,
    pub player_influence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterFate {
    Thriving,
    Content,
    Struggling,
    Deceased,
    Missing,
    Transformed,
    Redeemed,
    Corrupted,
    Ascended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneAchievement {
    pub milestone_id: Uuid,
    pub achieved_at: DateTime<Utc>,
    pub description: String,
    pub significance: MilestoneSignificance,
    pub associated_quest: Option<QuestId>,
    pub associated_event: Option<EventId>,
    pub narrative_context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneSignificance {
    Personal,      // Affects only player
    Local,         // Affects local area
    Regional,      // Affects region
    Global,        // Affects entire world
    Legendary,     // Creates lasting legend
    Mythical,      // Becomes myth/legend
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipProgression {
    pub character_id: NPCId,
    pub initial_relationship: f64,
    pub current_relationship: f64,
    pub relationship_milestones: Vec<RelationshipMilestone>,
    pub shared_experiences: Vec<SharedExperience>,
    pub future_potential: RelationshipPotential,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipMilestone {
    pub milestone_type: RelationshipMilestoneType,
    pub achieved_at: DateTime<Utc>,
    pub description: String,
    pub impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipMilestoneType {
    FirstMeeting,
    TrustGained,
    TrustLost,
    RomanticInterest,
    Friendship,
    Rivalry,
    Betrayal,
    Forgiveness,
    Sacrifice,
    SharedTrial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedExperience {
    pub experience_id: Uuid,
    pub description: String,
    pub emotional_intensity: f64,
    pub date: DateTime<Utc>,
    pub quest_context: Option<QuestId>,
    pub event_context: Option<EventId>,
    pub outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipPotential {
    CloseFriend,
    Mentor,
    Student,
    Romantic,
    Rival,
    Enemy,
    Family,
    Ally,
    Neutral,
    Unpredictable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryFlag {
    pub flag_name: String,
    pub value: FlagValue,
    pub set_at: DateTime<Utc>,
    pub set_by: FlagSource,
    pub expires: Option<DateTime<Utc>>,
    pub significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagValue {
    Boolean(bool),
    Integer(i32),
    Float(f64),
    Text(String),
    List(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlagSource {
    Quest(QuestId),
    Event(EventId),
    Dialogue(NPCId),
    PlayerChoice,
    SystemGenerated,
    WorldEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalLegend {
    pub legend_title: String,
    pub reputation_level: LegendLevel,
    pub known_deeds: Vec<LegendaryDeed>,
    pub titles_earned: Vec<String>,
    pub epithets: Vec<String>,
    pub legend_spread: LegendSpread,
    pub historical_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendLevel {
    Unknown,
    LocalHero,
    RegionalChampion,
    WorldRenowned,
    Legendary,
    Mythical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendaryDeed {
    pub deed_id: Uuid,
    pub title: String,
    pub description: String,
    pub witnesses: Vec<NPCId>,
    pub impact_scope: MilestoneSignificance,
    pub embellishment_factor: f64, // How much the story has grown in telling
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendSpread {
    pub known_in_locations: Vec<LocationId>,
    pub recognition_level: HashMap<String, f64>, // region -> recognition
    pub story_variations: Vec<StoryVariation>,
    pub cultural_penetration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryVariation {
    pub region: String,
    pub variation: String,
    pub accuracy: f64,
    pub popularity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativePreferences {
    pub preferred_genres: HashMap<String, f64>,
    pub pacing_preference: PacingPreference,
    pub choice_complexity_preference: f64,
    pub emotional_engagement_level: f64,
    pub exploration_vs_action: f64, // 0.0 = action, 1.0 = exploration
    pub individual_vs_group: f64,   // 0.0 = solo, 1.0 = group stories
    pub linear_vs_branching: f64,   // 0.0 = linear, 1.0 = branching
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PacingPreference {
    Fast,
    Medium,
    Slow,
    Variable,
    Adaptive,
}

/// Story arc definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryArc {
    pub arc_id: Uuid,
    pub name: String,
    pub description: String,
    pub arc_type: ArcType,
    pub chapters: Vec<StoryChapter>,
    pub prerequisites: Vec<ArcPrerequisite>,
    pub exclusions: Vec<ArcExclusion>,
    pub themes: Vec<String>,
    pub estimated_duration: u32, // Minutes
    pub difficulty_level: u32,
    pub emotional_journey: EmotionalJourney,
    pub character_roles: HashMap<NPCId, String>,
    pub world_impact: WorldImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArcType {
    MainStory,
    PersonalJourney,
    Romance,
    Mystery,
    Adventure,
    Political,
    Spiritual,
    Comedy,
    Tragedy,
    Epic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChapter {
    pub chapter_id: String,
    pub title: String,
    pub summary: String,
    pub required_quests: Vec<QuestId>,
    pub optional_quests: Vec<QuestId>,
    pub key_events: Vec<String>,
    pub decision_points: Vec<DecisionPoint>,
    pub narrative_beats: Vec<NarrativeBeat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionPoint {
    pub decision_id: String,
    pub prompt: String,
    pub options: Vec<DecisionOption>,
    pub consequences: HashMap<String, Vec<String>>,
    pub narrative_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub option_id: String,
    pub text: String,
    pub requirements: Vec<String>,
    pub immediate_consequences: Vec<String>,
    pub long_term_implications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeBeat {
    pub beat_type: BeatType,
    pub description: String,
    pub emotional_tone: crate::narrative_engine::Emotion,
    pub characters_involved: Vec<NPCId>,
    pub player_agency_level: f64, // How much control player has
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BeatType {
    Opening,
    IncitingIncident,
    PlotPoint,
    Midpoint,
    Crisis,
    Climax,
    Resolution,
    Denouement,
    Twist,
    Revelation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArcPrerequisite {
    QuestCompleted(QuestId),
    ArcCompleted(Uuid),
    PlayerLevel(u32),
    Reputation(String, i32),
    Relationship(NPCId, f64),
    Choice(String, String),
    Flag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArcExclusion {
    ArcActive(Uuid),
    ArcCompleted(Uuid),
    ChoiceMade(String, String),
    CharacterDead(NPCId),
    WorldState(String, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalJourney {
    pub starting_emotion: crate::narrative_engine::Emotion,
    pub emotional_arc: Vec<EmotionalPoint>,
    pub climax_emotion: crate::narrative_engine::Emotion,
    pub resolution_emotion: crate::narrative_engine::Emotion,
    pub overall_tone: NarrativeTone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub chapter: String,
    pub emotion: crate::narrative_engine::Emotion,
    pub intensity: f64,
    pub trigger: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeTone {
    Heroic,
    Dark,
    Humorous,
    Romantic,
    Mysterious,
    Tragic,
    Hopeful,
    Melancholic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldImpact {
    pub scope: WorldImpactScope,
    pub magnitude: f64,
    pub duration: WorldImpactDuration,
    pub affected_systems: Vec<String>,
    pub legacy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldImpactScope {
    Personal,
    Local,
    Regional,
    Continental,
    Global,
    Cosmic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldImpactDuration {
    Temporary,
    ShortTerm,
    MediumTerm,
    LongTerm,
    Permanent,
    Eternal,
}

/// Story milestone definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryMilestone {
    pub milestone_id: Uuid,
    pub name: String,
    pub description: String,
    pub trigger_conditions: Vec<MilestoneTrigger>,
    pub requirements: Vec<MilestoneRequirement>,
    pub rewards: Vec<MilestoneReward>,
    pub narrative_significance: f64,
    pub rarity: MilestoneRarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneTrigger {
    QuestCompletion(QuestId),
    ArcCompletion(Uuid),
    RelationshipLevel(NPCId, f64),
    ChoicesCombination(Vec<(String, String)>),
    WorldEventParticipation(EventId),
    TimeElapsed(u32),
    PlayerLevel(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneRequirement {
    ConsecutiveChoices(Vec<String>),
    SimultaneousConditions(Vec<String>),
    NegativeConditions(Vec<String>), // Things that must NOT have happened
    TimingRequirement(u32, u32),     // Must occur within time window
    SequenceRequirement(Vec<String>), // Must happen in specific order
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneReward {
    pub reward_type: MilestoneRewardType,
    pub value: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneRewardType {
    Title,
    Epithet,
    NarrativeContent,
    WorldRecognition,
    CharacterUnlock,
    AreaUnlock,
    SpecialDialogue,
    LegendEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MilestoneRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Unique,
}

/// Global story state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalStoryState {
    pub world_age: u32, // Game world time
    pub major_events_completed: Vec<EventId>,
    pub world_changing_choices: HashMap<String, String>,
    pub dominant_factions: Vec<String>,
    pub technological_level: u32,
    pub magical_saturation: f64,
    pub political_climate: String,
    pub economic_state: String,
    pub threat_level: u32,
    pub hope_level: f64,
    pub collective_player_impact: CollectiveImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectiveImpact {
    pub total_quests_completed: u64,
    pub world_events_participated: u64,
    pub average_morality_alignment: f64,
    pub popular_choices: HashMap<String, f64>,
    pub legendary_players: Vec<PlayerId>,
    pub collective_reputation: HashMap<String, f64>,
}

/// Analytics for narrative engagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeAnalytics {
    pub engagement_metrics: EngagementMetrics,
    pub choice_analytics: ChoiceAnalytics,
    pub completion_statistics: CompletionStatistics,
    pub emotional_response_data: EmotionalResponseData,
    pub narrative_flow_analysis: NarrativeFlowAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub average_session_story_time: f64,
    pub story_retention_rate: f64,
    pub narrative_skip_rate: f64,
    pub dialogue_engagement: f64,
    pub cutscene_completion_rate: f64,
    pub player_agency_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceAnalytics {
    pub choice_distribution: HashMap<String, HashMap<String, f64>>,
    pub regret_indicators: HashMap<String, f64>,
    pub choice_correlation_patterns: Vec<ChoiceCorrelation>,
    pub decision_time_analysis: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceCorrelation {
    pub choice_pair: (String, String),
    pub correlation_strength: f64,
    pub narrative_significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionStatistics {
    pub arc_completion_rates: HashMap<Uuid, f64>,
    pub average_completion_time: HashMap<Uuid, u32>,
    pub abandonment_points: HashMap<String, f64>,
    pub replay_frequency: HashMap<Uuid, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalResponseData {
    pub emotional_peaks: Vec<EmotionalDataPoint>,
    pub player_emotional_range: HashMap<PlayerId, f64>,
    pub narrative_emotional_effectiveness: HashMap<String, f64>,
    pub emotional_contagion_patterns: Vec<ContagionPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalDataPoint {
    pub moment: String,
    pub average_intensity: f64,
    pub emotion_type: crate::narrative_engine::Emotion,
    pub player_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContagionPattern {
    pub source_event: String,
    pub affected_players: u32,
    pub emotional_spread_rate: f64,
    pub duration: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeFlowAnalysis {
    pub pacing_effectiveness: HashMap<String, f64>,
    pub narrative_bottlenecks: Vec<String>,
    pub flow_optimization_suggestions: Vec<String>,
    pub player_path_diversity: f64,
}

impl StoryProgressionTracker {
    /// Create new story progression tracker
    pub fn new() -> Self {
        let mut tracker = Self {
            player_progressions: HashMap::new(),
            story_arcs: vec![],
            milestones: vec![],
            global_story_state: GlobalStoryState::default(),
            narrative_analytics: NarrativeAnalytics::default(),
        };
        
        tracker.initialize_default_arcs();
        tracker.initialize_default_milestones();
        tracker
    }

    /// Initialize default story arcs
    fn initialize_default_arcs(&mut self) {
        // Hero's Origin Arc
        let origin_arc = StoryArc {
            arc_id: Uuid::new_v4(),
            name: "Hero's Origin".to_string(),
            description: "The beginning of your legendary journey".to_string(),
            arc_type: ArcType::MainStory,
            chapters: vec![
                StoryChapter {
                    chapter_id: "awakening".to_string(),
                    title: "The Awakening".to_string(),
                    summary: "Discover your destiny and take the first steps".to_string(),
                    required_quests: vec![],
                    optional_quests: vec![],
                    key_events: vec!["Character creation".to_string(), "First quest".to_string()],
                    decision_points: vec![
                        DecisionPoint {
                            decision_id: "origin_choice".to_string(),
                            prompt: "What drives you to adventure?".to_string(),
                            options: vec![
                                DecisionOption {
                                    option_id: "glory".to_string(),
                                    text: "Seek glory and fame".to_string(),
                                    requirements: vec![],
                                    immediate_consequences: vec!["Reputation focus".to_string()],
                                    long_term_implications: vec!["Fame-based quests".to_string()],
                                },
                                DecisionOption {
                                    option_id: "knowledge".to_string(),
                                    text: "Pursue wisdom and knowledge".to_string(),
                                    requirements: vec![],
                                    immediate_consequences: vec!["Lore focus".to_string()],
                                    long_term_implications: vec!["Scholar quests".to_string()],
                                },
                            ],
                            consequences: HashMap::new(),
                            narrative_weight: 0.8,
                        },
                    ],
                    narrative_beats: vec![
                        NarrativeBeat {
                            beat_type: BeatType::Opening,
                            description: "Player enters the world".to_string(),
                            emotional_tone: crate::narrative_engine::Emotion::Excited,
                            characters_involved: vec![],
                            player_agency_level: 0.9,
                        },
                    ],
                },
            ],
            prerequisites: vec![],
            exclusions: vec![],
            themes: vec!["beginning".to_string(), "destiny".to_string(), "choice".to_string()],
            estimated_duration: 120, // 2 hours
            difficulty_level: 1,
            emotional_journey: EmotionalJourney {
                starting_emotion: crate::narrative_engine::Emotion::Neutral,
                emotional_arc: vec![
                    EmotionalPoint {
                        chapter: "awakening".to_string(),
                        emotion: crate::narrative_engine::Emotion::Excited,
                        intensity: 0.7,
                        trigger: "First quest acceptance".to_string(),
                    },
                ],
                climax_emotion: crate::narrative_engine::Emotion::Determined,
                resolution_emotion: crate::narrative_engine::Emotion::Hopeful,
                overall_tone: NarrativeTone::Heroic,
            },
            character_roles: HashMap::new(),
            world_impact: WorldImpact {
                scope: WorldImpactScope::Personal,
                magnitude: 0.3,
                duration: WorldImpactDuration::Permanent,
                affected_systems: vec!["player_identity".to_string()],
                legacy: "The hero's journey begins".to_string(),
            },
        };

        self.story_arcs.push(origin_arc);
    }

    /// Initialize default milestones
    fn initialize_default_milestones(&mut self) {
        let first_quest = StoryMilestone {
            milestone_id: Uuid::new_v4(),
            name: "First Steps".to_string(),
            description: "Complete your very first quest".to_string(),
            trigger_conditions: vec![MilestoneTrigger::QuestCompletion(Uuid::new_v4())],
            requirements: vec![],
            rewards: vec![
                MilestoneReward {
                    reward_type: MilestoneRewardType::Title,
                    value: "Novice Adventurer".to_string(),
                    description: "Recognition of your first steps into adventure".to_string(),
                },
            ],
            narrative_significance: 0.6,
            rarity: MilestoneRarity::Common,
        };

        let hero_level = StoryMilestone {
            milestone_id: Uuid::new_v4(),
            name: "Rising Hero".to_string(),
            description: "Reach level 20 and gain recognition".to_string(),
            trigger_conditions: vec![MilestoneTrigger::PlayerLevel(20)],
            requirements: vec![],
            rewards: vec![
                MilestoneReward {
                    reward_type: MilestoneRewardType::WorldRecognition,
                    value: "regional_hero".to_string(),
                    description: "NPCs recognize you as a regional hero".to_string(),
                },
            ],
            narrative_significance: 0.8,
            rarity: MilestoneRarity::Uncommon,
        };

        self.milestones.extend(vec![first_quest, hero_level]);
    }

    /// Record quest start
    pub fn record_quest_start(&mut self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        let progression = self.player_progressions.entry(player_id).or_insert_with(|| {
            PlayerStoryProgression {
                player_id,
                overall_progress: 0.0,
                active_arcs: vec![],
                completed_arcs: vec![],
                milestone_achievements: vec![],
                narrative_choices: HashMap::new(),
                character_relationships: HashMap::new(),
                story_flags: HashMap::new(),
                personal_legend: PersonalLegend {
                    legend_title: "Unknown Adventurer".to_string(),
                    reputation_level: LegendLevel::Unknown,
                    known_deeds: vec![],
                    titles_earned: vec![],
                    epithets: vec![],
                    legend_spread: LegendSpread {
                        known_in_locations: vec![],
                        recognition_level: HashMap::new(),
                        story_variations: vec![],
                        cultural_penetration: 0.0,
                    },
                    historical_impact: 0.0,
                },
                narrative_preferences: NarrativePreferences {
                    preferred_genres: HashMap::new(),
                    pacing_preference: PacingPreference::Medium,
                    choice_complexity_preference: 0.5,
                    emotional_engagement_level: 0.7,
                    exploration_vs_action: 0.5,
                    individual_vs_group: 0.6,
                    linear_vs_branching: 0.4,
                },
            }
        });

        // Check if this quest is part of any active arcs
        for arc in &mut progression.active_arcs {
            if !arc.active_quests.contains(&quest_id) {
                arc.active_quests.push(quest_id);
                arc.last_activity = Utc::now();
            }
        }

        tracing::info!("Recorded quest start for player {}: {}", player_id, quest_id);
        Ok(())
    }

    /// Record quest completion
    pub fn record_quest_completion(&mut self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        let progression = self.player_progressions.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

        // Update active arcs
        for arc in &mut progression.active_arcs {
            if let Some(pos) = arc.active_quests.iter().position(|&q| q == quest_id) {
                arc.active_quests.remove(pos);
                arc.completed_quests.push(quest_id);
                arc.last_activity = Utc::now();
                
                // Update arc progress
                let total_quests = arc.completed_quests.len() + arc.active_quests.len();
                if total_quests > 0 {
                    arc.progress = arc.completed_quests.len() as f64 / total_quests as f64;
                }
            }
        }

        // Check for milestone achievements
        self.check_milestones(player_id)?;

        // Update overall progress
        self.update_overall_progress(player_id)?;

        tracing::info!("Recorded quest completion for player {}: {}", player_id, quest_id);
        Ok(())
    }

    /// Check for milestone achievements
    fn check_milestones(&mut self, player_id: PlayerId) -> Result<()> {
        // Collect milestones to check without holding mutable reference
        let milestones_to_check: Vec<(Uuid, String, bool)> = {
            let progression = self.player_progressions.get(&player_id)
                .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

            self.milestones.iter()
                .filter_map(|milestone| {
                    // Check if milestone is already achieved
                    if progression.milestone_achievements.iter().any(|m| m.milestone_id == milestone.milestone_id) {
                        return None;
                    }

                    // Check trigger conditions
                    let achieved = milestone.trigger_conditions.iter().all(|trigger| {
                        self.evaluate_milestone_trigger(trigger, progression)
                    });

                    if achieved {
                        Some((milestone.milestone_id, milestone.name.clone(), achieved))
                    } else {
                        None
                    }
                })
                .collect()
        };

        // Apply achievements
        for (milestone_id, milestone_name, _) in milestones_to_check {
            // Clone the milestone to avoid borrowing issues
            let milestone = self.milestones.iter()
                .find(|m| m.milestone_id == milestone_id)
                .unwrap()
                .clone();
            
            let achievement = MilestoneAchievement {
                milestone_id,
                achieved_at: Utc::now(),
                description: milestone.description.clone(),
                significance: match milestone.rarity {
                    MilestoneRarity::Common => MilestoneSignificance::Personal,
                    MilestoneRarity::Uncommon => MilestoneSignificance::Local,
                    MilestoneRarity::Rare => MilestoneSignificance::Regional,
                    MilestoneRarity::Epic => MilestoneSignificance::Global,
                    MilestoneRarity::Legendary => MilestoneSignificance::Legendary,
                    MilestoneRarity::Unique => MilestoneSignificance::Mythical,
                },
                associated_quest: None,
                associated_event: None,
                narrative_context: "Story progression".to_string(),
            };

            if let Some(progression) = self.player_progressions.get_mut(&player_id) {
                progression.milestone_achievements.push(achievement);
            }

            // Apply milestone rewards
            self.apply_milestone_rewards(player_id, &milestone)?;

            tracing::info!("Player {} achieved milestone: {}", player_id, milestone_name);
        }

        Ok(())
    }

    /// Evaluate milestone trigger
    fn evaluate_milestone_trigger(&self, trigger: &MilestoneTrigger, progression: &PlayerStoryProgression) -> bool {
        match trigger {
            MilestoneTrigger::QuestCompletion(_quest_id) => {
                // Would check if specific quest was completed
                !progression.completed_arcs.is_empty() // Simplified check
            },
            MilestoneTrigger::PlayerLevel(level) => {
                // Would check actual player level
                progression.overall_progress * 100.0 >= *level as f64 // Simplified
            },
            MilestoneTrigger::RelationshipLevel(character_id, level) => {
                progression.character_relationships.get(character_id)
                    .map(|rel| rel.current_relationship >= *level)
                    .unwrap_or(false)
            },
            _ => false, // Other triggers not implemented yet
        }
    }

    /// Apply milestone rewards
    fn apply_milestone_rewards(&mut self, player_id: PlayerId, milestone: &StoryMilestone) -> Result<()> {
        let progression = self.player_progressions.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

        for reward in &milestone.rewards {
            match reward.reward_type {
                MilestoneRewardType::Title => {
                    progression.personal_legend.titles_earned.push(reward.value.clone());
                },
                MilestoneRewardType::Epithet => {
                    progression.personal_legend.epithets.push(reward.value.clone());
                },
                MilestoneRewardType::WorldRecognition => {
                    // Update legend spread
                    progression.personal_legend.legend_spread.cultural_penetration += 0.1;
                },
                _ => {
                    // Handle other reward types
                },
            }
        }

        Ok(())
    }

    /// Update overall progress
    fn update_overall_progress(&mut self, player_id: PlayerId) -> Result<()> {
        let progression = self.player_progressions.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

        // Calculate progress based on completed arcs and milestones
        let arc_progress: f64 = progression.active_arcs.iter()
            .map(|arc| arc.progress)
            .sum::<f64>() / progression.active_arcs.len().max(1) as f64;

        let completed_arcs_weight = progression.completed_arcs.len() as f64 * 0.1;
        let milestone_weight = progression.milestone_achievements.len() as f64 * 0.05;

        progression.overall_progress = (arc_progress + completed_arcs_weight + milestone_weight).min(1.0);

        // Update legend level based on progress
        progression.personal_legend.reputation_level = match progression.overall_progress {
            0.0..0.1 => LegendLevel::Unknown,
            0.1..0.3 => LegendLevel::LocalHero,
            0.3..0.6 => LegendLevel::RegionalChampion,
            0.6..0.8 => LegendLevel::WorldRenowned,
            0.8..0.95 => LegendLevel::Legendary,
            _ => LegendLevel::Mythical,
        };

        Ok(())
    }

    /// Record narrative choice
    pub fn record_narrative_choice(&mut self, player_id: PlayerId, choice_id: String, chosen_option: String) -> Result<()> {
        let progression = self.player_progressions.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

        progression.narrative_choices.insert(choice_id.clone(), chosen_option.clone());

        // Set story flag
        let flag = StoryFlag {
            flag_name: format!("choice_{}", choice_id),
            value: FlagValue::Text(chosen_option),
            set_at: Utc::now(),
            set_by: FlagSource::PlayerChoice,
            expires: None,
            significance: 0.5,
        };

        progression.story_flags.insert(flag.flag_name.clone(), flag);

        tracing::info!("Recorded narrative choice for player {}: {} = {}", 
                      player_id, choice_id, progression.narrative_choices[&choice_id]);

        Ok(())
    }

    /// Get player's story progress
    pub fn get_player_progress(&self, player_id: PlayerId) -> Option<&PlayerStoryProgression> {
        self.player_progressions.get(&player_id)
    }

    /// Get available story arcs for player
    pub fn get_available_arcs(&self, player_id: PlayerId) -> Vec<&StoryArc> {
        let progression = match self.player_progressions.get(&player_id) {
            Some(p) => p,
            None => return vec![],
        };

        self.story_arcs.iter()
            .filter(|arc| {
                // Check prerequisites
                arc.prerequisites.iter().all(|prereq| {
                    self.evaluate_arc_prerequisite(prereq, progression)
                }) &&
                // Check exclusions
                arc.exclusions.iter().all(|exclusion| {
                    !self.evaluate_arc_exclusion(exclusion, progression)
                }) &&
                // Not already active or completed
                !progression.active_arcs.iter().any(|a| a.arc_id == arc.arc_id) &&
                !progression.completed_arcs.iter().any(|a| a.arc_id == arc.arc_id)
            })
            .collect()
    }

    /// Evaluate arc prerequisite
    fn evaluate_arc_prerequisite(&self, prerequisite: &ArcPrerequisite, progression: &PlayerStoryProgression) -> bool {
        match prerequisite {
            ArcPrerequisite::QuestCompleted(_quest_id) => {
                // Would check if specific quest was completed
                true // Simplified
            },
            ArcPrerequisite::ArcCompleted(arc_id) => {
                progression.completed_arcs.iter().any(|a| a.arc_id == *arc_id)
            },
            ArcPrerequisite::Choice(choice_id, option) => {
                progression.narrative_choices.get(choice_id) == Some(option)
            },
            _ => true, // Other prerequisites not implemented yet
        }
    }

    /// Evaluate arc exclusion
    fn evaluate_arc_exclusion(&self, exclusion: &ArcExclusion, progression: &PlayerStoryProgression) -> bool {
        match exclusion {
            ArcExclusion::ArcActive(arc_id) => {
                progression.active_arcs.iter().any(|a| a.arc_id == *arc_id)
            },
            ArcExclusion::ArcCompleted(arc_id) => {
                progression.completed_arcs.iter().any(|a| a.arc_id == *arc_id)
            },
            ArcExclusion::ChoiceMade(choice_id, option) => {
                progression.narrative_choices.get(choice_id) == Some(option)
            },
            _ => false, // Other exclusions not implemented yet
        }
    }

    /// Start story arc for player
    pub fn start_arc(&mut self, player_id: PlayerId, arc_id: Uuid) -> Result<()> {
        let arc = self.story_arcs.iter().find(|a| a.arc_id == arc_id)
            .ok_or_else(|| anyhow::anyhow!("Story arc not found"))?;

        let progression = self.player_progressions.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player progression not found"))?;

        let active_arc = ActiveArc {
            arc_id,
            progress: 0.0,
            current_chapter: 0,
            completed_quests: vec![],
            active_quests: vec![],
            available_branches: vec![],
            chosen_path: None,
            start_time: Utc::now(),
            last_activity: Utc::now(),
        };

        progression.active_arcs.push(active_arc);

        tracing::info!("Started story arc '{}' for player {}", arc.name, player_id);
        Ok(())
    }
}

impl Default for GlobalStoryState {
    fn default() -> Self {
        Self {
            world_age: 0,
            major_events_completed: vec![],
            world_changing_choices: HashMap::new(),
            dominant_factions: vec!["neutral".to_string()],
            technological_level: 1,
            magical_saturation: 0.5,
            political_climate: "stable".to_string(),
            economic_state: "balanced".to_string(),
            threat_level: 1,
            hope_level: 0.7,
            collective_player_impact: CollectiveImpact {
                total_quests_completed: 0,
                world_events_participated: 0,
                average_morality_alignment: 0.0,
                popular_choices: HashMap::new(),
                legendary_players: vec![],
                collective_reputation: HashMap::new(),
            },
        }
    }
}

impl Default for NarrativeAnalytics {
    fn default() -> Self {
        Self {
            engagement_metrics: EngagementMetrics {
                average_session_story_time: 0.0,
                story_retention_rate: 0.0,
                narrative_skip_rate: 0.0,
                dialogue_engagement: 0.0,
                cutscene_completion_rate: 0.0,
                player_agency_satisfaction: 0.0,
            },
            choice_analytics: ChoiceAnalytics {
                choice_distribution: HashMap::new(),
                regret_indicators: HashMap::new(),
                choice_correlation_patterns: vec![],
                decision_time_analysis: HashMap::new(),
            },
            completion_statistics: CompletionStatistics {
                arc_completion_rates: HashMap::new(),
                average_completion_time: HashMap::new(),
                abandonment_points: HashMap::new(),
                replay_frequency: HashMap::new(),
            },
            emotional_response_data: EmotionalResponseData {
                emotional_peaks: vec![],
                player_emotional_range: HashMap::new(),
                narrative_emotional_effectiveness: HashMap::new(),
                emotional_contagion_patterns: vec![],
            },
            narrative_flow_analysis: NarrativeFlowAnalysis {
                pacing_effectiveness: HashMap::new(),
                narrative_bottlenecks: vec![],
                flow_optimization_suggestions: vec![],
                player_path_diversity: 0.0,
            },
        }
    }
}

impl Default for StoryProgressionTracker {
    fn default() -> Self {
        Self::new()
    }
}