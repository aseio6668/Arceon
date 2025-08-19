/*!
# Dialogue System

Advanced dialogue management with dynamic conversations,
character relationships, and narrative branching.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, NPCId, QuestId, DialogueId};

/// Main dialogue system managing conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSystem {
    pub dialogue_trees: HashMap<NPCId, DialogueTree>,
    pub active_conversations: HashMap<PlayerId, ActiveConversation>,
    pub dialogue_templates: Vec<DialogueTemplate>,
    pub conversation_history: HashMap<PlayerId, ConversationHistory>,
    pub dynamic_responses: DynamicResponseEngine,
}

/// Tree structure for character dialogues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTree {
    pub tree_id: DialogueId,
    pub character_id: NPCId,
    pub root_nodes: HashMap<String, DialogueNode>,
    pub conditional_branches: Vec<ConditionalBranch>,
    pub context_modifiers: Vec<ContextModifier>,
    pub personality_influences: PersonalityInfluences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueNode {
    pub node_id: String,
    pub text: String,
    pub speaker: Speaker,
    pub emotion: Emotion,
    pub animation_hints: Vec<String>,
    pub voice_modifiers: VoiceModifiers,
    pub conditions: Vec<DialogueCondition>,
    pub responses: Vec<DialogueResponse>,
    pub effects: Vec<DialogueEffect>,
    pub timeout: Option<u32>, // Seconds before auto-advance
    pub repeatable: bool,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Speaker {
    NPC(NPCId),
    Player,
    Narrator,
    System,
    Environment(String), // Environmental storytelling
    Memory(String), // Flashback or memory
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
    Suspicious,
    Romantic,
    Fearful,
    Proud,
    Ashamed,
    Nostalgic,
    Hopeful,
    Desperate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceModifiers {
    pub volume: f64,      // 0.0 to 1.0
    pub pitch: f64,       // 0.0 to 2.0 (1.0 = normal)
    pub speed: f64,       // 0.0 to 2.0 (1.0 = normal)
    pub emphasis: Vec<WordEmphasis>,
    pub accent: Option<String>,
    pub dialect: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEmphasis {
    pub word_index: usize,
    pub emphasis_type: EmphasisType,
    pub intensity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmphasisType {
    Stress,      // Louder/stronger
    Whisper,     // Quieter
    Pause,       // Pause before word
    Elongation,  // Stretch the word
    Tone,        // Change in tone
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueCondition {
    FirstMeeting,
    RepeatVisitor,
    QuestActive(QuestId),
    QuestCompleted(QuestId),
    QuestFailed(QuestId),
    PlayerLevel(u32, Option<u32>), // min, max
    TimeOfDay(u32, u32), // start hour, end hour
    Weather(String),
    Location(crate::LocationId),
    Item(crate::ItemId),
    Reputation(String, i32),
    Relationship(RelationshipCondition),
    Flag(String, bool),
    Random(f64), // 0.0 to 1.0 probability
    Seasonal(String),
    PlayerChoice(String, String), // choice_id, selected_option
    WorldState(String, String), // state_name, required_value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipCondition {
    pub aspect: RelationshipAspect,
    pub comparison: ComparisonOperator,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipAspect {
    Trust,
    Affection,
    Respect,
    Fear,
    Familiarity,
    Overall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    EqualTo,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub response_id: String,
    pub text: String,
    pub requirements: Vec<ResponseRequirement>,
    pub consequences: Vec<DialogueConsequence>,
    pub emotion: Option<Emotion>,
    pub leads_to: Vec<String>, // Next possible dialogue nodes
    pub interrupts_speaker: bool,
    pub delay: Option<u32>, // Delay before showing option in seconds
    pub expires: Option<u32>, // Seconds before option disappears
    pub cooldown: Option<u32>, // Seconds before can be used again
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseRequirement {
    Level(u32),
    Skill(String, u32),
    Item(crate::ItemId),
    Reputation(String, i32),
    Quest(QuestId),
    Choice(String, String), // previous_choice_id, required_option
    Attribute(String, u32), // player attribute requirement
    Knowledge(String), // lore knowledge requirement
    Class(String), // player class requirement
    Achievement(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueConsequence {
    pub consequence_type: ConsequenceType,
    pub target: String,
    pub value: i32,
    pub description: String,
    pub permanent: bool,
    pub hidden: bool, // Player doesn't see immediate effect
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    ReputationChange,
    ItemGain,
    ItemLoss,
    GoldGain,
    GoldLoss,
    ExperienceGain,
    QuestStart,
    QuestProgress,
    QuestComplete,
    QuestFail,
    RelationshipChange,
    FlagSet,
    WorldStateChange,
    SkillGain,
    EmotionChange,
    LocationUnlock,
    DialogueUnlock,
    CharacterStateChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueEffect {
    ChangeEmotion(Emotion),
    PlaySound(String),
    ShowAnimation(String),
    CameraFocus(String),
    EnvironmentalChange(String),
    TimeSkip(u32), // Skip forward in time
    Flashback(String), // Show flashback sequence
    QuestHint(QuestId),
    NPCStateChange(NPCId, String, String),
    WorldEventTrigger(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalBranch {
    pub branch_id: String,
    pub conditions: Vec<DialogueCondition>,
    pub alternative_nodes: Vec<String>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextModifier {
    pub modifier_id: String,
    pub context_type: ContextType,
    pub text_modifications: Vec<TextModification>,
    pub response_filters: Vec<ResponseFilter>,
    pub emotion_influences: Vec<EmotionInfluence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextType {
    Location(crate::LocationId),
    Weather(String),
    TimeOfDay(u32, u32),
    PlayerMood(Emotion),
    RecentEvent(String),
    CompanionPresent(NPCId),
    InCombat,
    Injured,
    Wealthy,
    Poor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextModification {
    pub original_text: String,
    pub modified_text: String,
    pub modification_type: ModificationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Replace,
    Append,
    Prepend,
    Insert(usize), // Insert at position
    Conditional, // Show only if conditions met
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFilter {
    pub response_id: String,
    pub filter_action: FilterAction,
    pub conditions: Vec<DialogueCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterAction {
    Hide,
    Disable,
    Modify(String), // Change text
    Highlight,
    AddIcon(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionInfluence {
    pub base_emotion: Emotion,
    pub influenced_emotion: Emotion,
    pub intensity_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityInfluences {
    pub personality_traits: HashMap<String, f64>,
    pub speech_patterns: SpeechPatterns,
    pub topic_preferences: TopicPreferences,
    pub conversation_style: ConversationStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechPatterns {
    pub formality_level: f64, // 0.0 = very casual, 1.0 = very formal
    pub verbosity: f64, // 0.0 = terse, 1.0 = very verbose
    pub directness: f64, // 0.0 = indirect/cryptic, 1.0 = very direct
    pub emotional_expressiveness: f64,
    pub humor_tendency: f64,
    pub interruption_likelihood: f64,
    pub common_phrases: Vec<String>,
    pub speech_quirks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicPreferences {
    pub favorite_topics: Vec<String>,
    pub avoided_topics: Vec<String>,
    pub expertise_areas: Vec<String>,
    pub gossip_tendency: f64,
    pub philosophical_inclination: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationStyle {
    pub patience_level: f64,
    pub attention_span: f64,
    pub empathy_level: f64,
    pub assertiveness: f64,
    pub conversation_dominance: f64, // How much they control the conversation
}

/// Active conversation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveConversation {
    pub conversation_id: Uuid,
    pub player_id: PlayerId,
    pub npc_id: NPCId,
    pub current_node: String,
    pub conversation_state: ConversationState,
    pub context: ConversationContext,
    pub history: Vec<ConversationTurn>,
    pub start_time: DateTime<Utc>,
    pub last_interaction: DateTime<Utc>,
    pub options_presented: Vec<String>,
    pub timeout_timer: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationState {
    Starting,
    WaitingForResponse,
    NPCSpeaking,
    PlayerSpeaking,
    Processing,
    Ending,
    Paused,
    Interrupted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub location: Option<crate::LocationId>,
    pub time_of_day: u32,
    pub weather: Option<String>,
    pub nearby_npcs: Vec<NPCId>,
    pub player_state: PlayerConversationState,
    pub interruption_sources: Vec<InterruptionSource>,
    pub privacy_level: PrivacyLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerConversationState {
    pub current_emotion: Option<Emotion>,
    pub energy_level: f64,
    pub attention_span: f64,
    pub recent_actions: Vec<String>,
    pub equipped_items: Vec<crate::ItemId>,
    pub active_quests: Vec<QuestId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterruptionSource {
    Combat,
    NPCApproach(NPCId),
    QuestUpdate(QuestId),
    EnvironmentalHazard,
    TimeLimit,
    PlayerAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyLevel {
    Public,     // Many people around
    SemiPrivate, // Few people around
    Private,    // Just the two characters
    Intimate,   // Very private setting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub turn_id: Uuid,
    pub speaker: Speaker,
    pub text: String,
    pub emotion: Emotion,
    pub timestamp: DateTime<Utc>,
    pub player_choice_id: Option<String>,
    pub effects_triggered: Vec<String>,
}

/// Historical record of conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationHistory {
    pub player_id: PlayerId,
    pub conversations: Vec<CompletedConversation>,
    pub relationship_changes: Vec<RelationshipChange>,
    pub memorable_moments: Vec<MemorableMoment>,
    pub conversation_patterns: ConversationPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedConversation {
    pub conversation_id: Uuid,
    pub npc_id: NPCId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub turn_count: u32,
    pub outcome: ConversationOutcome,
    pub topics_discussed: Vec<String>,
    pub emotions_expressed: Vec<Emotion>,
    pub choices_made: Vec<String>,
    pub consequences: Vec<DialogueConsequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationOutcome {
    Successful,
    Neutral,
    Negative,
    Interrupted,
    Abandoned,
    QuestStarted,
    InformationGained,
    RelationshipChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipChange {
    pub npc_id: NPCId,
    pub aspect: RelationshipAspect,
    pub old_value: f64,
    pub new_value: f64,
    pub cause: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorableMoment {
    pub moment_id: Uuid,
    pub npc_id: NPCId,
    pub description: String,
    pub emotional_impact: f64,
    pub timestamp: DateTime<Utc>,
    pub context: String,
    pub referenced_count: u32, // How often NPC mentions this
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationPatterns {
    pub preferred_topics: HashMap<String, f64>,
    pub conversation_length_preference: f64,
    pub politeness_level: f64,
    pub humor_appreciation: f64,
    pub directness_preference: f64,
    pub emotional_engagement: f64,
}

/// Template for generating dialogue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub template_type: DialogueTemplateType,
    pub structure: DialogueStructure,
    pub variable_slots: Vec<VariableSlot>,
    pub emotional_arc: EmotionalArc,
    pub branching_options: BranchingOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DialogueTemplateType {
    QuestGiver,
    Merchant,
    Gossip,
    Lore,
    Romance,
    Conflict,
    Information,
    Casual,
    Dramatic,
    Comedy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueStructure {
    pub opening: Vec<String>,
    pub body_patterns: Vec<BodyPattern>,
    pub closing: Vec<String>,
    pub transition_phrases: Vec<String>,
    pub fallback_responses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPattern {
    pub pattern_type: PatternType,
    pub text_templates: Vec<String>,
    pub variable_requirements: Vec<String>,
    pub conditional_variants: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Question,
    Statement,
    Request,
    Offer,
    Warning,
    Explanation,
    Story,
    Reaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableSlot {
    pub slot_name: String,
    pub slot_type: VariableType,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation_rules: Vec<ValidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    PlayerName,
    NPCName,
    LocationName,
    QuestName,
    ItemName,
    Number,
    CustomText,
    RandomChoice(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRule {
    MinLength(usize),
    MaxLength(usize),
    AllowedValues(Vec<String>),
    NumericRange(f64, f64),
    RequiredFormat(String), // Regex pattern
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalArc {
    pub starting_emotion: Emotion,
    pub emotional_journey: Vec<EmotionalWaypoint>,
    pub ending_emotion: Emotion,
    pub intensity_curve: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalWaypoint {
    pub position: f64, // 0.0 to 1.0 through conversation
    pub emotion: Emotion,
    pub intensity: f64,
    pub trigger_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchingOptions {
    pub branch_points: Vec<BranchPoint>,
    pub merge_opportunities: Vec<MergeOpportunity>,
    pub escape_routes: Vec<EscapeRoute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchPoint {
    pub position: f64,
    pub decision_prompt: String,
    pub options: Vec<BranchOption>,
    pub consequences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchOption {
    pub option_text: String,
    pub requirements: Vec<ResponseRequirement>,
    pub leads_to: String,
    pub emotional_impact: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeOpportunity {
    pub position: f64,
    pub merge_conditions: Vec<String>,
    pub transition_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscapeRoute {
    pub trigger: EscapeTrigger,
    pub exit_text: String,
    pub consequences: Vec<DialogueConsequence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscapeTrigger {
    PlayerInitiated,
    TimeLimit,
    Interruption,
    LowRelationship,
    SpecificResponse(String),
}

/// Dynamic response generation engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicResponseEngine {
    pub response_generators: HashMap<String, ResponseGenerator>,
    pub context_analyzers: Vec<ContextAnalyzer>,
    pub personality_modelers: HashMap<NPCId, PersonalityModel>,
    pub conversation_memory: ConversationMemory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseGenerator {
    pub generator_name: String,
    pub input_patterns: Vec<String>,
    pub output_templates: Vec<String>,
    pub context_requirements: Vec<String>,
    pub personality_influences: Vec<PersonalityInfluence>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityInfluence {
    pub trait_name: String,
    pub influence_strength: f64,
    pub text_modifications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextAnalyzer {
    pub analyzer_name: String,
    pub context_factors: Vec<ContextFactor>,
    pub influence_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContextFactor {
    RecentPlayerActions,
    CurrentLocation,
    TimeOfDay,
    Weather,
    NPCMood,
    RelationshipLevel,
    QuestStatus,
    WorldEvents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityModel {
    pub npc_id: NPCId,
    pub traits: HashMap<String, f64>,
    pub speech_patterns: SpeechPatterns,
    pub learned_preferences: HashMap<PlayerId, PlayerPreferences>,
    pub conversation_history: Vec<ConversationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPreferences {
    pub preferred_conversation_style: String,
    pub topics_of_interest: Vec<String>,
    pub response_patterns: HashMap<String, f64>,
    pub relationship_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    pub player_id: PlayerId,
    pub date: DateTime<Utc>,
    pub topics: Vec<String>,
    pub emotional_tone: Emotion,
    pub outcome_satisfaction: f64,
    pub key_points: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMemory {
    pub short_term: HashMap<NPCId, Vec<RecentInteraction>>,
    pub long_term: HashMap<NPCId, Vec<SignificantMemory>>,
    pub relationship_context: HashMap<(NPCId, PlayerId), RelationshipContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentInteraction {
    pub player_id: PlayerId,
    pub timestamp: DateTime<Utc>,
    pub interaction_summary: String,
    pub emotional_impact: f64,
    pub topics_discussed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificantMemory {
    pub memory_id: Uuid,
    pub player_id: PlayerId,
    pub description: String,
    pub emotional_significance: f64,
    pub referenced_frequency: f64,
    pub contextual_triggers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipContext {
    pub relationship_stage: RelationshipStage,
    pub shared_experiences: Vec<String>,
    pub inside_jokes: Vec<String>,
    pub unresolved_tensions: Vec<String>,
    pub future_plans: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStage {
    FirstMeeting,
    Acquaintance,
    Friend,
    CloseFriend,
    Enemy,
    Rival,
    Romantic,
    Family,
    Mentor,
    Student,
}

impl DialogueSystem {
    /// Create new dialogue system
    pub fn new() -> Self {
        Self {
            dialogue_trees: HashMap::new(),
            active_conversations: HashMap::new(),
            dialogue_templates: vec![],
            conversation_history: HashMap::new(),
            dynamic_responses: DynamicResponseEngine::new(),
        }
    }

    /// Start conversation between player and NPC
    pub fn start_conversation(&mut self, player_id: PlayerId, npc_id: NPCId) -> Result<ConversationId> {
        let conversation_id = Uuid::new_v4();
        
        // Get dialogue tree for NPC
        let dialogue_tree = self.dialogue_trees.get(&npc_id)
            .ok_or_else(|| anyhow::anyhow!("No dialogue tree found for NPC"))?;

        // Determine starting node based on conditions
        let starting_node = self.determine_starting_node(player_id, dialogue_tree)?;

        let conversation = ActiveConversation {
            conversation_id,
            player_id,
            npc_id,
            current_node: starting_node,
            conversation_state: ConversationState::Starting,
            context: self.build_conversation_context(player_id, npc_id)?,
            history: vec![],
            start_time: Utc::now(),
            last_interaction: Utc::now(),
            options_presented: vec![],
            timeout_timer: None,
        };

        self.active_conversations.insert(player_id, conversation);
        tracing::info!("Started conversation {} between player {} and NPC {}", 
                      conversation_id, player_id, npc_id);

        Ok(conversation_id)
    }

    /// Determine appropriate starting dialogue node
    fn determine_starting_node(&self, player_id: PlayerId, dialogue_tree: &DialogueTree) -> Result<String> {
        // Check for conditional branches that might override default starting node
        for branch in &dialogue_tree.conditional_branches {
            if self.evaluate_branch_conditions(player_id, &branch.conditions)? {
                if let Some(node) = branch.alternative_nodes.first() {
                    return Ok(node.clone());
                }
            }
        }

        // Default to first root node or greeting
        dialogue_tree.root_nodes.keys()
            .find(|&node_id| node_id == "greeting" || node_id == "default")
            .or_else(|| dialogue_tree.root_nodes.keys().next())
            .map(|s| s.clone())
            .ok_or_else(|| anyhow::anyhow!("No starting node available"))
    }

    /// Build conversation context
    fn build_conversation_context(&self, _player_id: PlayerId, _npc_id: NPCId) -> Result<ConversationContext> {
        Ok(ConversationContext {
            location: None, // Would be populated from player's current location
            time_of_day: 12, // Would get actual time
            weather: None,
            nearby_npcs: vec![],
            player_state: PlayerConversationState {
                current_emotion: None,
                energy_level: 1.0,
                attention_span: 1.0,
                recent_actions: vec![],
                equipped_items: vec![],
                active_quests: vec![],
            },
            interruption_sources: vec![],
            privacy_level: PrivacyLevel::Public,
        })
    }

    /// Process player response to dialogue
    pub fn process_player_response(&mut self, player_id: PlayerId, response_id: String) -> Result<DialogueNode> {
        // First, gather all the data we need without holding mutable references
        let (npc_id, selected_response, next_node_id) = {
            let conversation = self.active_conversations.get(&player_id)
                .ok_or_else(|| anyhow::anyhow!("No active conversation found"))?;

            let dialogue_tree = self.dialogue_trees.get(&conversation.npc_id)
                .ok_or_else(|| anyhow::anyhow!("Dialogue tree not found"))?;

            let current_node = dialogue_tree.root_nodes.get(&conversation.current_node)
                .ok_or_else(|| anyhow::anyhow!("Current dialogue node not found"))?;

            // Find the selected response
            let selected_response = current_node.responses.iter()
                .find(|response| response.response_id == response_id)
                .ok_or_else(|| anyhow::anyhow!("Response not found"))?
                .clone();

            // Check response requirements (using immutable reference)
            if !self.check_response_requirements(player_id, &selected_response.requirements)? {
                return Err(anyhow::anyhow!("Player doesn't meet response requirements"));
            }

            // Determine next dialogue node (using immutable reference)
            let next_node_id = self.determine_next_node(conversation, &selected_response)?;
            
            (conversation.npc_id, selected_response, next_node_id)
        };
        
        // Apply consequences
        for consequence in &selected_response.consequences {
            self.apply_dialogue_consequence(player_id, consequence)?;
        }

        // Update conversation state
        let conversation = self.active_conversations.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("No active conversation found"))?;

        // Record conversation turn
        let turn = ConversationTurn {
            turn_id: Uuid::new_v4(),
            speaker: Speaker::Player,
            text: selected_response.text.clone(),
            emotion: selected_response.emotion.unwrap_or(Emotion::Neutral),
            timestamp: Utc::now(),
            player_choice_id: Some(response_id.clone()),
            effects_triggered: vec![],
        };

        conversation.history.push(turn);
        conversation.last_interaction = Utc::now();
        conversation.current_node = next_node_id.clone();

        // Get next dialogue node
        let dialogue_tree = self.dialogue_trees.get(&npc_id)
            .ok_or_else(|| anyhow::anyhow!("Dialogue tree not found"))?;
        let next_node = dialogue_tree.root_nodes.get(&next_node_id)
            .ok_or_else(|| anyhow::anyhow!("Next dialogue node not found"))?;

        Ok(next_node.clone())
    }

    /// Check if player meets response requirements
    fn check_response_requirements(&self, _player_id: PlayerId, requirements: &[ResponseRequirement]) -> Result<bool> {
        // For now, assume all requirements are met
        // In a real implementation, this would check player level, items, quests, etc.
        for _requirement in requirements {
            match _requirement {
                ResponseRequirement::Level(_level) => {
                    // Check if player level >= required level
                },
                ResponseRequirement::Item(_item_id) => {
                    // Check if player has the item
                },
                _ => {
                    // Handle other requirements
                },
            }
        }
        
        Ok(true)
    }

    /// Apply dialogue consequence
    fn apply_dialogue_consequence(&mut self, player_id: PlayerId, consequence: &DialogueConsequence) -> Result<()> {
        match consequence.consequence_type {
            ConsequenceType::ReputationChange => {
                tracing::info!("Player {} reputation change: {} by {}", 
                              player_id, consequence.target, consequence.value);
            },
            ConsequenceType::QuestStart => {
                tracing::info!("Player {} started quest: {}", player_id, consequence.target);
            },
            ConsequenceType::RelationshipChange => {
                tracing::info!("Player {} relationship change with {}: {}", 
                              player_id, consequence.target, consequence.value);
            },
            ConsequenceType::ItemGain => {
                tracing::info!("Player {} gained item: {} x{}", 
                              player_id, consequence.target, consequence.value);
            },
            _ => {
                tracing::debug!("Applied dialogue consequence: {:?}", consequence.consequence_type);
            },
        }

        Ok(())
    }

    /// Determine next dialogue node based on response
    fn determine_next_node(&self, _conversation: &ActiveConversation, response: &DialogueResponse) -> Result<String> {
        if response.leads_to.is_empty() {
            return Ok("end".to_string());
        }

        // For now, take the first lead_to option
        // In a real implementation, this might consider conditions, randomness, etc.
        Ok(response.leads_to[0].clone())
    }

    /// End conversation
    pub fn end_conversation(&mut self, player_id: PlayerId) -> Result<ConversationOutcome> {
        let conversation = self.active_conversations.remove(&player_id)
            .ok_or_else(|| anyhow::anyhow!("No active conversation found"))?;

        let outcome = self.determine_conversation_outcome(&conversation);

        // Record completed conversation
        let completed_conversation = CompletedConversation {
            conversation_id: conversation.conversation_id,
            npc_id: conversation.npc_id,
            start_time: conversation.start_time,
            end_time: Utc::now(),
            turn_count: conversation.history.len() as u32,
            outcome: outcome.clone(),
            topics_discussed: vec![], // Would extract from conversation history
            emotions_expressed: conversation.history.iter()
                .map(|turn| turn.emotion.clone())
                .collect(),
            choices_made: conversation.history.iter()
                .filter_map(|turn| turn.player_choice_id.clone())
                .collect(),
            consequences: vec![], // Would collect applied consequences
        };

        self.conversation_history.entry(player_id)
            .or_insert_with(|| ConversationHistory {
                player_id,
                conversations: vec![],
                relationship_changes: vec![],
                memorable_moments: vec![],
                conversation_patterns: ConversationPatterns {
                    preferred_topics: HashMap::new(),
                    conversation_length_preference: 0.5,
                    politeness_level: 0.5,
                    humor_appreciation: 0.5,
                    directness_preference: 0.5,
                    emotional_engagement: 0.5,
                },
            })
            .conversations.push(completed_conversation);

        tracing::info!("Ended conversation {} with outcome: {:?}", 
                      conversation.conversation_id, outcome);

        Ok(outcome)
    }

    /// Determine conversation outcome
    fn determine_conversation_outcome(&self, conversation: &ActiveConversation) -> ConversationOutcome {
        // Analyze conversation to determine outcome
        // For now, return a default outcome
        if conversation.history.len() > 5 {
            ConversationOutcome::Successful
        } else {
            ConversationOutcome::Neutral
        }
    }

    /// Evaluate branch conditions
    fn evaluate_branch_conditions(&self, _player_id: PlayerId, conditions: &[DialogueCondition]) -> Result<bool> {
        // For now, assume all conditions are met
        // In a real implementation, this would check each condition
        for _condition in conditions {
            match _condition {
                DialogueCondition::FirstMeeting => {
                    // Check if this is first meeting with NPC
                },
                DialogueCondition::QuestActive(_quest_id) => {
                    // Check if quest is active
                },
                DialogueCondition::PlayerLevel(_min_level, _max_level) => {
                    // Check player level range
                },
                _ => {
                    // Handle other conditions
                },
            }
        }
        
        Ok(true)
    }

    /// Get current dialogue options for player
    pub fn get_dialogue_options(&self, player_id: PlayerId) -> Result<Vec<DialogueResponse>> {
        let conversation = self.active_conversations.get(&player_id)
            .ok_or_else(|| anyhow::anyhow!("No active conversation found"))?;

        let dialogue_tree = self.dialogue_trees.get(&conversation.npc_id)
            .ok_or_else(|| anyhow::anyhow!("Dialogue tree not found"))?;

        let current_node = dialogue_tree.root_nodes.get(&conversation.current_node)
            .ok_or_else(|| anyhow::anyhow!("Current dialogue node not found"))?;

        // Filter responses based on requirements and conditions
        let available_responses: Vec<DialogueResponse> = current_node.responses.iter()
            .filter(|response| {
                self.check_response_requirements(player_id, &response.requirements)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        Ok(available_responses)
    }

    /// Add dialogue tree for NPC
    pub fn add_dialogue_tree(&mut self, npc_id: NPCId, dialogue_tree: DialogueTree) {
        self.dialogue_trees.insert(npc_id, dialogue_tree);
        tracing::info!("Added dialogue tree for NPC {}", npc_id);
    }
}

type ConversationId = Uuid;

impl DynamicResponseEngine {
    fn new() -> Self {
        Self {
            response_generators: HashMap::new(),
            context_analyzers: vec![],
            personality_modelers: HashMap::new(),
            conversation_memory: ConversationMemory {
                short_term: HashMap::new(),
                long_term: HashMap::new(),
                relationship_context: HashMap::new(),
            },
        }
    }
}

impl Default for DialogueSystem {
    fn default() -> Self {
        Self::new()
    }
}