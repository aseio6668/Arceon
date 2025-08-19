/*!
# Narrative Engine

Manages storytelling, character development, and narrative consistency
across the game world and quest systems.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{
    Quest, QuestId, PlayerId, NPCId, LocationId, NarrativeMood, ConsequenceType
};

/// Main narrative engine managing story flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEngine {
    pub story_graph: StoryGraph,
    pub character_database: CharacterDatabase,
    pub world_lore: WorldLore,
    pub narrative_state: HashMap<PlayerId, PlayerNarrativeState>,
    pub global_story_flags: HashMap<String, bool>,
    pub active_storylines: Vec<Storyline>,
    pub narrative_templates: Vec<NarrativeTemplate>,
}

/// Graph structure representing story connections and branches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryGraph {
    pub nodes: HashMap<Uuid, StoryNode>,
    pub connections: HashMap<Uuid, Vec<StoryConnection>>,
    pub current_nodes: HashMap<PlayerId, Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryNode {
    pub node_id: Uuid,
    pub name: String,
    pub node_type: StoryNodeType,
    pub content: StoryContent,
    pub conditions: Vec<NodeCondition>,
    pub effects: Vec<NodeEffect>,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StoryNodeType {
    QuestGiver,
    PlotPoint,
    CharacterDevelopment,
    WorldEvent,
    PlayerChoice,
    Resolution,
    Branching,
    Convergence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryContent {
    pub title: String,
    pub description: String,
    pub dialogue: Vec<DialogueEntry>,
    pub mood: NarrativeMood,
    pub themes: Vec<String>,
    pub characters: Vec<NPCId>,
    pub locations: Vec<LocationId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub speaker: Speaker,
    pub text: String,
    pub emotion: Emotion,
    pub choices: Vec<DialogueChoice>,
    pub conditions: Vec<DialogueCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Speaker {
    NPC(NPCId),
    Player,
    Narrator,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Emotion {
    Neutral,
    Happy,
    Sad,
    Angry,
    Excited,
    Worried,
    Mysterious,
    Determined,
    Confused,
    Grateful,
    Fearful,
    Proud,
    Hopeful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueChoice {
    pub choice_id: String,
    pub text: String,
    pub requirements: Vec<ChoiceRequirement>,
    pub consequences: Vec<DialogueConsequence>,
    pub leads_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChoiceRequirement {
    Level(u32),
    Reputation(String, i32),
    QuestCompleted(QuestId),
    Item(Uuid),
    Skill(String, u32),
    Flag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueConsequence {
    pub consequence_type: ConsequenceType,
    pub value: String,
    pub amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueCondition {
    FirstTime,
    RepeatVisit,
    QuestActive(QuestId),
    QuestCompleted(QuestId),
    PlayerLevel(u32, Option<u32>),
    TimeOfDay(u32, u32),
    Flag(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryConnection {
    pub connection_id: Uuid,
    pub from_node: Uuid,
    pub to_node: Uuid,
    pub connection_type: ConnectionType,
    pub requirements: Vec<ConnectionRequirement>,
    pub probability: f64,
    pub narrative_weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Sequential,      // Must happen after previous
    Parallel,        // Can happen alongside
    Conditional,     // Only if conditions met
    Choice,          // Player choice dependent
    Random,          // Randomly selected
    Triggered,       // External trigger required
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionRequirement {
    QuestCompleted(QuestId),
    ChoiceMade(String),
    TimeElapsed(u32),
    LevelReached(u32),
    LocationVisited(LocationId),
    ItemObtained(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeCondition {
    pub condition_id: String,
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    QuestStatus,
    PlayerLevel,
    Reputation,
    WorldState,
    TimeOfDay,
    Season,
    PlayerChoice,
    Random,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEffect {
    pub effect_id: String,
    pub effect_type: EffectType,
    pub target: String,
    pub value: i32,
    pub duration: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    SetFlag,
    ModifyReputation,
    UnlockQuest,
    ChangeWorldState,
    ModifyRelationship,
    GrantItem,
    TriggerEvent,
}

/// Database of characters and their development
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterDatabase {
    pub characters: HashMap<NPCId, Character>,
    pub relationships: HashMap<PlayerId, HashMap<NPCId, Relationship>>,
    pub character_arcs: HashMap<NPCId, CharacterArc>,
    pub dialogue_trees: HashMap<NPCId, DialogueTree>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub character_id: NPCId,
    pub name: String,
    pub background: String,
    pub personality: PersonalityTraits,
    pub motivations: Vec<String>,
    pub secrets: Vec<CharacterSecret>,
    pub current_state: CharacterState,
    pub involvement_in_stories: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub openness: f64,      // 0.0 to 1.0
    pub conscientiousness: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub neuroticism: f64,
    pub dominant_traits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSecret {
    pub secret_id: String,
    pub description: String,
    pub reveal_conditions: Vec<RevealCondition>,
    pub consequences_if_revealed: Vec<SecretConsequence>,
    pub known_by_players: Vec<PlayerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevealCondition {
    TrustLevel(f64),
    QuestProgression(QuestId),
    PlayerChoice(String),
    TimeElapsed(u32),
    SpecialEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretConsequence {
    pub target: SecretTarget,
    pub effect: String,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecretTarget {
    Character(NPCId),
    Player(PlayerId),
    WorldState(String),
    Story(Uuid),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterState {
    pub current_location: Option<LocationId>,
    pub mood: Emotion,
    pub available_for_quests: bool,
    pub recent_interactions: Vec<RecentInteraction>,
    pub temporary_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentInteraction {
    pub player_id: PlayerId,
    pub interaction_type: InteractionType,
    pub timestamp: DateTime<Utc>,
    pub outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    QuestGiven,
    QuestCompleted,
    Dialogue,
    Trade,
    Combat,
    Help,
    Betrayal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub character_id: NPCId,
    pub player_id: PlayerId,
    pub trust_level: f64,      // -1.0 to 1.0
    pub affection: f64,        // -1.0 to 1.0
    pub respect: f64,          // -1.0 to 1.0
    pub familiarity: f64,      // 0.0 to 1.0
    pub relationship_type: RelationshipType,
    pub history: Vec<RelationshipEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    Stranger,
    Acquaintance,
    Friend,
    Enemy,
    Ally,
    Romantic,
    Mentor,
    Student,
    Family,
    Rival,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipEvent {
    pub event_id: Uuid,
    pub event_type: InteractionType,
    pub impact: f64,
    pub timestamp: DateTime<Utc>,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterArc {
    pub arc_id: Uuid,
    pub character_id: NPCId,
    pub arc_type: CharacterArcType,
    pub stages: Vec<ArcStage>,
    pub current_stage: usize,
    pub completion_conditions: Vec<ArcCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterArcType {
    Growth,
    Redemption,
    Fall,
    Discovery,
    Sacrifice,
    Revenge,
    Romance,
    Mentor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStage {
    pub stage_id: String,
    pub name: String,
    pub description: String,
    pub entry_conditions: Vec<StageCondition>,
    pub duration: Option<u32>,
    pub character_changes: Vec<CharacterChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StageCondition {
    QuestCompleted(QuestId),
    PlayerChoice(String),
    TimeElapsed(u32),
    RelationshipLevel(PlayerId, f64),
    WorldEvent(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterChange {
    pub aspect: CharacterAspect,
    pub change_type: ChangeType,
    pub magnitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterAspect {
    Personality(String),
    Motivation(String),
    Location,
    Availability,
    Relationship(PlayerId),
    Appearance,
    Dialogue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    Increase,
    Decrease,
    Set,
    Add,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArcCondition {
    AllStagesCompleted,
    PlayerChoicesMade(Vec<String>),
    RelationshipThreshold(PlayerId, f64),
    QuestChainCompleted(Vec<QuestId>),
    TimeElapsed(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub tree_id: Uuid,
    pub character_id: NPCId,
    pub root_nodes: Vec<DialogueNode>,
    pub current_context: DialogueContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub node_id: String,
    pub text: String,
    pub speaker: Speaker,
    pub emotion: Emotion,
    pub conditions: Vec<DialogueCondition>,
    pub choices: Vec<DialogueChoice>,
    pub effects: Vec<DialogueEffect>,
    pub next_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEffect {
    pub effect_type: DialogueEffectType,
    pub target: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffectType {
    ModifyRelationship,
    SetFlag,
    GiveQuest,
    CompleteObjective,
    ChangeEmotion,
    UnlockDialogue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueContext {
    pub current_player: Option<PlayerId>,
    pub active_node: Option<String>,
    pub conversation_history: Vec<String>,
    pub session_flags: HashMap<String, bool>,
}

/// World lore and background information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldLore {
    pub factions: HashMap<String, Faction>,
    pub historical_events: Vec<HistoricalEvent>,
    pub myths_and_legends: Vec<Myth>,
    pub locations: HashMap<LocationId, LocationLore>,
    pub prophecies: Vec<Prophecy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub name: String,
    pub description: String,
    pub goals: Vec<String>,
    pub enemies: Vec<String>,
    pub allies: Vec<String>,
    pub reputation_levels: HashMap<String, String>,
    pub current_activities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEvent {
    pub event_id: Uuid,
    pub name: String,
    pub description: String,
    pub date: String, // In-world date
    pub participants: Vec<String>,
    pub consequences: Vec<String>,
    pub referenced_in_quests: Vec<QuestId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Myth {
    pub myth_id: Uuid,
    pub title: String,
    pub story: String,
    pub origin: String,
    pub truth_level: f64, // How much of it is true
    pub cultural_impact: f64,
    pub related_quests: Vec<QuestId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationLore {
    pub location_id: LocationId,
    pub history: String,
    pub legends: Vec<String>,
    pub significant_events: Vec<Uuid>,
    pub cultural_importance: f64,
    pub hidden_secrets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prophecy {
    pub prophecy_id: Uuid,
    pub text: String,
    pub origin: String,
    pub fulfillment_conditions: Vec<ProphecyCondition>,
    pub fulfillment_state: ProphecyState,
    pub related_players: Vec<PlayerId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProphecyCondition {
    PlayerAction(String),
    QuestCompletion(QuestId),
    WorldStateChange(String),
    TimeElapsed(u32),
    CharacterFate(NPCId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProphecyState {
    Dormant,
    Awakening,
    InProgress,
    NearFulfillment,
    Fulfilled,
    Subverted,
}

/// Player's narrative state and progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerNarrativeState {
    pub player_id: PlayerId,
    pub active_storylines: Vec<Uuid>,
    pub completed_storylines: Vec<Uuid>,
    pub story_flags: HashMap<String, bool>,
    pub reputation: HashMap<String, i32>,
    pub character_relationships: HashMap<NPCId, Relationship>,
    pub narrative_choices: Vec<NarrativeChoice>,
    pub current_mood: Option<NarrativeMood>,
    pub story_progression: f64, // Overall story completion 0.0-1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeChoice {
    pub choice_id: String,
    pub quest_id: Option<QuestId>,
    pub description: String,
    pub chosen_option: String,
    pub timestamp: DateTime<Utc>,
    pub consequences_applied: bool,
}

/// Active storylines in the world
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storyline {
    pub storyline_id: Uuid,
    pub name: String,
    pub description: String,
    pub storyline_type: StorylineType,
    pub participants: Vec<PlayerId>,
    pub key_characters: Vec<NPCId>,
    pub current_chapter: u32,
    pub chapters: Vec<StoryChapter>,
    pub branching_paths: HashMap<String, Uuid>,
    pub completion_criteria: Vec<CompletionCriterion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StorylineType {
    Personal,      // Individual player story
    Guild,         // Guild-based storyline
    World,         // World-affecting story
    Regional,      // Regional questline
    Seasonal,      // Time-limited story
    Dynamic,       // Emergent storyline
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryChapter {
    pub chapter_id: String,
    pub name: String,
    pub description: String,
    pub required_quests: Vec<QuestId>,
    pub optional_quests: Vec<QuestId>,
    pub key_events: Vec<String>,
    pub completion_conditions: Vec<ChapterCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChapterCondition {
    QuestsCompleted(Vec<QuestId>),
    ChoicesMade(Vec<String>),
    TimeLimit(u32),
    PlayerLevel(u32),
    CollectiveProgress(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionCriterion {
    AllChaptersCompleted,
    PlayerChoiceOutcome(String),
    CharacterFates(Vec<(NPCId, String)>),
    WorldStateAchieved(String),
    CollaborativeGoal(String),
}

/// Templates for generating narrative content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub template_type: TemplateType,
    pub mood: NarrativeMood,
    pub themes: Vec<String>,
    pub structure: NarrativeStructure,
    pub character_roles: Vec<CharacterRole>,
    pub plot_devices: Vec<PlotDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    HeroJourney,
    Redemption,
    Mystery,
    Tragedy,
    Romance,
    Revenge,
    Discovery,
    Sacrifice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeStructure {
    pub acts: Vec<NarrativeAct>,
    pub pacing: PacingRules,
    pub tension_curve: Vec<f64>,
    pub climax_position: f64, // 0.0-1.0 position in story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeAct {
    pub act_number: u32,
    pub purpose: String,
    pub key_elements: Vec<String>,
    pub duration_ratio: f64, // Percentage of total story
    pub tension_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacingRules {
    pub action_to_dialogue_ratio: f64,
    pub revelation_frequency: u32,
    pub conflict_escalation_rate: f64,
    pub resolution_timing: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRole {
    pub role_name: String,
    pub importance: CharacterImportance,
    pub arc_type: CharacterArcType,
    pub relationship_to_player: String,
    pub key_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CharacterImportance {
    Protagonist,
    Deuteragonist,
    Antagonist,
    Supporting,
    Minor,
    Background,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlotDevice {
    Foreshadowing,
    RedHerring,
    ChekhovsGun,
    DeusExMachina,
    Flashback,
    Revelation,
    Twist,
    Sacrifice,
}

impl NarrativeEngine {
    /// Create new narrative engine
    pub fn new() -> Self {
        Self {
            story_graph: StoryGraph::new(),
            character_database: CharacterDatabase::new(),
            world_lore: WorldLore::new(),
            narrative_state: HashMap::new(),
            global_story_flags: HashMap::new(),
            active_storylines: vec![],
            narrative_templates: vec![],
        }
    }

    /// Register a quest with the narrative engine
    pub fn register_quest(&mut self, quest: &Quest) -> Result<()> {
        // Create story node for the quest
        let story_node = StoryNode {
            node_id: Uuid::new_v4(),
            name: quest.title.clone(),
            node_type: match quest.quest_type {
                crate::QuestType::Main => StoryNodeType::PlotPoint,
                _ => StoryNodeType::QuestGiver,
            },
            content: StoryContent {
                title: quest.title.clone(),
                description: quest.description.clone(),
                dialogue: vec![],
                mood: quest.narrative_context.mood.clone(),
                themes: quest.narrative_context.themes.clone(),
                characters: quest.narrative_context.characters.clone(),
                locations: vec![],
            },
            conditions: vec![],
            effects: vec![],
            weight: 1.0,
        };

        self.story_graph.nodes.insert(story_node.node_id, story_node);
        Ok(())
    }

    /// Handle quest acceptance
    pub fn on_quest_accepted(&mut self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        self.update_player_narrative_state(player_id, |state| {
            state.story_flags.insert(format!("quest_accepted_{}", quest_id), true);
        });

        tracing::info!("Player {} accepted quest {}, narrative state updated", player_id, quest_id);
        Ok(())
    }

    /// Handle quest progress
    pub fn on_quest_progress(&mut self, player_id: PlayerId, quest_id: QuestId, objective_id: Uuid) -> Result<()> {
        self.update_player_narrative_state(player_id, |state| {
            state.story_flags.insert(format!("objective_{}_{}", quest_id, objective_id), true);
        });

        Ok(())
    }

    /// Handle quest completion
    pub fn on_quest_completed(&mut self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        self.update_player_narrative_state(player_id, |state| {
            state.story_flags.insert(format!("quest_completed_{}", quest_id), true);
            state.story_progression += 0.01; // Small progress increment
        });

        // Check for character development triggers
        self.check_character_development_triggers(player_id, quest_id)?;

        Ok(())
    }

    /// Get narrative branches available after quest completion
    pub fn get_narrative_branches(&self, quest_id: QuestId) -> Result<Option<Vec<String>>> {
        // Find story nodes connected to this quest
        let connected_branches: Vec<String> = self.story_graph.nodes.values()
            .filter(|node| {
                // Check if this node is connected to the completed quest
                node.content.title.contains(&quest_id.to_string()) ||
                node.conditions.iter().any(|cond| 
                    matches!(cond.condition_type, ConditionType::QuestStatus) &&
                    cond.parameters.get("quest_id") == Some(&quest_id.to_string())
                )
            })
            .map(|node| node.content.description.clone())
            .collect();

        if connected_branches.is_empty() {
            Ok(None)
        } else {
            Ok(Some(connected_branches))
        }
    }

    /// Update player narrative state
    fn update_player_narrative_state<F>(&mut self, player_id: PlayerId, updater: F)
    where
        F: FnOnce(&mut PlayerNarrativeState),
    {
        let state = self.narrative_state.entry(player_id).or_insert_with(|| {
            PlayerNarrativeState {
                player_id,
                active_storylines: vec![],
                completed_storylines: vec![],
                story_flags: HashMap::new(),
                reputation: HashMap::new(),
                character_relationships: HashMap::new(),
                narrative_choices: vec![],
                current_mood: None,
                story_progression: 0.0,
            }
        });

        updater(state);
    }

    /// Check for character development triggers
    fn check_character_development_triggers(&mut self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        // Check all character arcs for advancement conditions
        let character_ids: Vec<NPCId> = self.character_database.character_arcs.keys().cloned().collect();
        
        for character_id in character_ids {
            if let Some(arc) = self.character_database.character_arcs.get_mut(&character_id) {
                // Check if quest completion advances this character's arc
                if arc.current_stage < arc.stages.len() {
                    let current_stage = &arc.stages[arc.current_stage];
                    
                    let should_advance = current_stage.entry_conditions.iter().any(|condition| {
                        matches!(condition, StageCondition::QuestCompleted(qid) if *qid == quest_id)
                    });

                    if should_advance {
                        self.advance_character_arc(character_id, player_id)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Advance character arc to next stage
    fn advance_character_arc(&mut self, character_id: NPCId, _player_id: PlayerId) -> Result<()> {
        // Collect changes to apply without holding the mutable borrow
        let (changes, new_stage) = {
            if let Some(arc) = self.character_database.character_arcs.get_mut(&character_id) {
                if arc.current_stage + 1 < arc.stages.len() {
                    arc.current_stage += 1;
                    
                    // Get changes from new stage
                    let stage = &arc.stages[arc.current_stage];
                    (stage.character_changes.clone(), arc.current_stage)
                } else {
                    return Ok(());
                }
            } else {
                return Ok(());
            }
        };
        
        // Apply character changes without holding the arc borrow
        for change in &changes {
            self.apply_character_change(character_id, change)?;
        }

        tracing::info!("Advanced character {} to arc stage {}", character_id, new_stage);

        Ok(())
    }

    /// Apply character change
    fn apply_character_change(&mut self, character_id: NPCId, change: &CharacterChange) -> Result<()> {
        if let Some(character) = self.character_database.characters.get_mut(&character_id) {
            match &change.aspect {
                CharacterAspect::Personality(_trait_name) => {
                    // Modify personality trait (simplified)
                    match change.change_type {
                        ChangeType::Increase => {
                            // Would modify specific personality trait
                        },
                        ChangeType::Decrease => {
                            // Would decrease specific personality trait
                        },
                        _ => {},
                    }
                },
                CharacterAspect::Motivation(motivation) => {
                    match change.change_type {
                        ChangeType::Add => {
                            character.motivations.push(motivation.clone());
                        },
                        ChangeType::Remove => {
                            character.motivations.retain(|m| m != motivation);
                        },
                        _ => {},
                    }
                },
                CharacterAspect::Availability => {
                    character.current_state.available_for_quests = change.magnitude > 0.0;
                },
                _ => {
                    // Other aspects would be handled here
                },
            }
        }

        Ok(())
    }

    /// Generate dialogue for character interaction
    pub fn generate_dialogue(&mut self, character_id: NPCId, player_id: PlayerId, context: &str) -> Result<DialogueEntry> {
        let character = self.character_database.characters.get(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character not found"))?;

        let relationship = self.character_database.relationships
            .get(&player_id)
            .and_then(|rels| rels.get(&character_id));

        // Generate dialogue based on character personality and relationship
        let text = self.generate_dialogue_text(character, relationship, context)?;
        let emotion = self.determine_character_emotion(character, relationship);

        Ok(DialogueEntry {
            speaker: Speaker::NPC(character_id),
            text,
            emotion,
            choices: vec![],
            conditions: vec![],
        })
    }

    /// Generate dialogue text based on character and context
    fn generate_dialogue_text(&self, _character: &Character, relationship: Option<&Relationship>, context: &str) -> Result<String> {
        let base_greeting = match relationship {
            Some(rel) if rel.trust_level > 0.5 => "Hello, my trusted friend!",
            Some(rel) if rel.trust_level < -0.5 => "What do you want now?",
            Some(_) => "Greetings, traveler.",
            None => "I don't believe we've met...",
        };

        // In a real implementation, this would use the character's personality,
        // current mood, recent interactions, and story context to generate
        // appropriate dialogue using templates or AI generation
        
        Ok(format!("{} {}", base_greeting, context))
    }

    /// Determine character's emotion based on state and relationship
    fn determine_character_emotion(&self, character: &Character, relationship: Option<&Relationship>) -> Emotion {
        // Use character's current mood and relationship status
        match (&character.current_state.mood, relationship) {
            (Emotion::Happy, Some(rel)) if rel.affection > 0.7 => Emotion::Excited,
            (Emotion::Sad, _) => Emotion::Sad,
            (_, Some(rel)) if rel.trust_level < -0.5 => Emotion::Angry,
            (_, Some(rel)) if rel.trust_level > 0.7 => Emotion::Happy,
            _ => character.current_state.mood.clone(),
        }
    }

    /// Create or update character relationship
    pub fn update_relationship(&mut self, player_id: PlayerId, character_id: NPCId, interaction: InteractionType, impact: f64) -> Result<()> {
        let relationships = self.character_database.relationships.entry(player_id).or_default();
        
        let relationship = relationships.entry(character_id).or_insert_with(|| {
            Relationship {
                character_id,
                player_id,
                trust_level: 0.0,
                affection: 0.0,
                respect: 0.0,
                familiarity: 0.0,
                relationship_type: RelationshipType::Stranger,
                history: vec![],
            }
        });

        // Update relationship based on interaction
        match interaction {
            InteractionType::QuestCompleted => {
                relationship.trust_level += impact * 0.5;
                relationship.respect += impact * 0.3;
                relationship.familiarity += 0.1;
            },
            InteractionType::Help => {
                relationship.affection += impact * 0.4;
                relationship.trust_level += impact * 0.2;
                relationship.familiarity += 0.1;
            },
            InteractionType::Betrayal => {
                relationship.trust_level -= impact;
                relationship.affection -= impact * 0.8;
                relationship.respect -= impact * 0.5;
            },
            _ => {
                relationship.familiarity += 0.05;
            },
        }

        // Clamp values
        relationship.trust_level = relationship.trust_level.max(-1.0).min(1.0);
        relationship.affection = relationship.affection.max(-1.0).min(1.0);
        relationship.respect = relationship.respect.max(-1.0).min(1.0);
        relationship.familiarity = relationship.familiarity.max(0.0).min(1.0);

        // Update relationship type based on current values (inline logic to avoid borrow conflicts)
        relationship.relationship_type = if relationship.trust_level > 0.7 && relationship.affection > 0.6 {
            RelationshipType::Friend
        } else if relationship.trust_level < -0.5 || relationship.affection < -0.5 {
            RelationshipType::Enemy
        } else if relationship.familiarity > 0.3 {
            RelationshipType::Acquaintance
        } else {
            RelationshipType::Stranger
        };

        // Record event
        relationship.history.push(RelationshipEvent {
            event_id: Uuid::new_v4(),
            event_type: interaction,
            impact,
            timestamp: Utc::now(),
            context: "Quest interaction".to_string(),
        });

        Ok(())
    }

    /// Determine relationship type based on values
    #[allow(dead_code)]
    fn determine_relationship_type(&self, relationship: &Relationship) -> RelationshipType {
        if relationship.trust_level < -0.7 || relationship.affection < -0.7 {
            RelationshipType::Enemy
        } else if relationship.affection > 0.8 && relationship.trust_level > 0.6 {
            RelationshipType::Friend
        } else if relationship.trust_level > 0.7 && relationship.respect > 0.6 {
            RelationshipType::Ally
        } else if relationship.familiarity > 0.3 {
            RelationshipType::Acquaintance
        } else {
            RelationshipType::Stranger
        }
    }
}

impl StoryGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            current_nodes: HashMap::new(),
        }
    }
}

impl CharacterDatabase {
    fn new() -> Self {
        Self {
            characters: HashMap::new(),
            relationships: HashMap::new(),
            character_arcs: HashMap::new(),
            dialogue_trees: HashMap::new(),
        }
    }
}

impl WorldLore {
    fn new() -> Self {
        Self {
            factions: HashMap::new(),
            historical_events: vec![],
            myths_and_legends: vec![],
            locations: HashMap::new(),
            prophecies: vec![],
        }
    }
}

impl Default for NarrativeEngine {
    fn default() -> Self {
        Self::new()
    }
}