/*!
# Arceon Quest and Narrative Engine

Dynamic quest generation, storytelling systems, and narrative progression
for the Arceon MMORPG world.
*/

pub mod quest_generation;
pub mod narrative_engine;
pub mod quest_chains;
pub mod dialogue_system;
pub mod world_events;
pub mod story_progression;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// Re-export key types
pub use quest_generation::QuestGenerator;
pub use narrative_engine::NarrativeEngine;
pub use quest_chains::QuestChainManager;
pub use dialogue_system::DialogueSystem;
pub use world_events::WorldEventCoordinator;
pub use story_progression::StoryProgressionTracker;

/// Type aliases for better code readability
pub type QuestId = Uuid;
pub type PlayerId = Uuid;
pub type NPCId = Uuid;
pub type LocationId = Uuid;
pub type ItemId = Uuid;
pub type FactionId = Uuid;
pub type DialogueId = Uuid;
pub type EventId = Uuid;
pub type ChainId = Uuid;

/// Main quest and narrative system coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestSystem {
    pub quest_generator: quest_generation::QuestGenerator,
    pub narrative_engine: narrative_engine::NarrativeEngine,
    pub chain_manager: quest_chains::QuestChainManager,
    pub dialogue_system: dialogue_system::DialogueSystem,
    pub event_coordinator: world_events::WorldEventCoordinator,
    pub story_tracker: story_progression::StoryProgressionTracker,
}

/// Core quest structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub quest_id: QuestId,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub objectives: Vec<QuestObjective>,
    pub rewards: QuestRewards,
    pub requirements: QuestRequirements,
    pub narrative_context: NarrativeContext,
    pub status: QuestStatus,
    pub metadata: QuestMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QuestType {
    Main,           // Primary story quests
    Side,           // Optional side quests
    Daily,          // Repeatable daily quests
    Weekly,         // Weekly challenges
    Chain,          // Part of a quest chain
    Discovery,      // Exploration-based
    Crafting,       // Creation/gathering focused
    Combat,         // Battle-oriented
    Social,         // Player interaction
    Event,          // World event related
    Personal,       // Character-specific story
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestObjective {
    pub objective_id: Uuid,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub target: ObjectiveTarget,
    pub current_progress: u32,
    pub required_progress: u32,
    pub optional: bool,
    pub completion_order: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    Kill,
    Collect,
    Interact,
    Escort,
    Deliver,
    Explore,
    Craft,
    Talk,
    Survive,
    Protect,
    Solve,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveTarget {
    NPC(NPCId),
    Location(LocationId),
    Item(ItemId),
    Monster(String),
    Player(PlayerId),
    Area(String),
    Multiple(Vec<Box<ObjectiveTarget>>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewards {
    pub experience: u64,
    pub gold: u64,
    pub items: Vec<ItemReward>,
    pub reputation: HashMap<FactionId, i32>,
    pub titles: Vec<String>,
    pub abilities: Vec<String>,
    pub narrative_unlocks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReward {
    pub item_id: ItemId,
    pub quantity: u32,
    pub quality: Option<String>,
    pub bind_type: BindType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindType {
    None,
    OnPickup,
    OnEquip,
    OnUse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRequirements {
    pub level: Option<u32>,
    pub completed_quests: Vec<QuestId>,
    pub items: Vec<ItemRequirement>,
    pub reputation: HashMap<FactionId, i32>,
    pub skills: HashMap<String, u32>,
    pub location: Option<LocationId>,
    pub time_restrictions: Option<TimeRestriction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRequirement {
    pub item_id: ItemId,
    pub quantity: u32,
    pub consumed_on_start: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub recurring: Option<RecurringPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecurringPattern {
    Daily,
    Weekly,
    Monthly,
    Seasonal,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeContext {
    pub story_arc: String,
    pub mood: NarrativeMood,
    pub characters: Vec<NPCId>,
    pub themes: Vec<String>,
    pub branching_paths: HashMap<String, QuestId>,
    pub player_choices: Vec<PlayerChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NarrativeMood {
    Heroic,
    Mysterious,
    Tragic,
    Comedic,
    Dark,
    Uplifting,
    Tense,
    Peaceful,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerChoice {
    pub choice_id: String,
    pub description: String,
    pub consequences: Vec<QuestConsequence>,
    pub made_choice: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestConsequence {
    pub consequence_type: ConsequenceType,
    pub target: String,
    pub magnitude: i32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsequenceType {
    ReputationChange,
    ItemGain,
    ItemLoss,
    QuestUnlock,
    QuestBlock,
    NPCRelationship,
    WorldState,
    DialogueChange,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QuestStatus {
    Available,
    Active,
    Completed,
    Failed,
    Abandoned,
    Locked,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestMetadata {
    pub created_at: DateTime<Utc>,
    pub creator: QuestCreator,
    pub difficulty: QuestDifficulty,
    pub estimated_duration: u32, // minutes
    pub tags: Vec<String>,
    pub region: Option<String>,
    pub seasonal: bool,
    pub repeatable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestCreator {
    System,
    Player(PlayerId),
    Event,
    AI,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestDifficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
    Elite,
    Legendary,
}

impl QuestSystem {
    /// Create new quest system
    pub fn new() -> Self {
        Self {
            quest_generator: QuestGenerator::new(),
            narrative_engine: NarrativeEngine::new(),
            chain_manager: QuestChainManager::new(),
            dialogue_system: DialogueSystem::new(),
            event_coordinator: WorldEventCoordinator::new(),
            story_tracker: StoryProgressionTracker::new(),
        }
    }

    /// Generate a new quest for a player
    pub fn generate_quest(&mut self, player_id: PlayerId, context: GenerationContext) -> anyhow::Result<Quest> {
        let quest = self.quest_generator.generate_quest(player_id, context)?;
        
        // Register quest with narrative engine
        self.narrative_engine.register_quest(&quest)?;
        
        // Check for chain connections
        if let Some(chain_id) = self.chain_manager.find_suitable_chain(&quest) {
            self.chain_manager.add_quest_to_chain(chain_id, quest.quest_id)?;
        }
        
        Ok(quest)
    }

    /// Accept a quest for a player
    pub fn accept_quest(&mut self, player_id: PlayerId, quest_id: QuestId) -> anyhow::Result<()> {
        // Validate quest availability and requirements
        self.quest_generator.validate_quest_acceptance(player_id, quest_id)?;
        
        // Update narrative state
        self.narrative_engine.on_quest_accepted(player_id, quest_id)?;
        
        // Track story progression
        self.story_tracker.record_quest_start(player_id, quest_id)?;
        
        Ok(())
    }

    /// Update quest progress
    pub fn update_progress(&mut self, player_id: PlayerId, quest_id: QuestId, objective_id: Uuid, progress: u32) -> anyhow::Result<()> {
        let quest_updated = self.quest_generator.update_objective_progress(quest_id, objective_id, progress)?;
        
        if quest_updated {
            // Check for quest completion
            if self.quest_generator.is_quest_complete(quest_id)? {
                self.complete_quest(player_id, quest_id)?;
            } else {
                // Update narrative based on progress
                self.narrative_engine.on_quest_progress(player_id, quest_id, objective_id)?;
            }
        }
        
        Ok(())
    }

    /// Complete a quest
    pub fn complete_quest(&mut self, player_id: PlayerId, quest_id: QuestId) -> anyhow::Result<QuestRewards> {
        let rewards = self.quest_generator.complete_quest(quest_id)?;
        
        // Update narrative state
        self.narrative_engine.on_quest_completed(player_id, quest_id)?;
        
        // Check for chain progression
        self.chain_manager.on_quest_completed(quest_id)?;
        
        // Update story progression
        self.story_tracker.record_quest_completion(player_id, quest_id)?;
        
        // Generate follow-up content
        self.generate_follow_up_content(player_id, quest_id)?;
        
        Ok(rewards)
    }

    /// Generate follow-up content after quest completion
    fn generate_follow_up_content(&mut self, player_id: PlayerId, completed_quest_id: QuestId) -> anyhow::Result<()> {
        // Check for narrative branches
        if let Some(branches) = self.narrative_engine.get_narrative_branches(completed_quest_id)? {
            for branch in branches {
                let context = GenerationContext {
                    trigger: QuestTrigger::NarrativeBranch(completed_quest_id),
                    player_level: None, // Will be fetched by generator
                    location: None,
                    story_context: Some(branch),
                };
                
                if let Ok(_follow_up) = self.generate_quest(player_id, context) {
                    // Follow-up quest generated successfully
                }
            }
        }
        
        Ok(())
    }

    /// Get active quests for player
    pub fn get_active_quests(&self, player_id: PlayerId) -> Vec<&Quest> {
        self.quest_generator.get_player_quests(player_id, QuestStatus::Active)
    }

    /// Get available quests for player at location
    pub fn get_available_quests(&self, player_id: PlayerId, location_id: LocationId) -> anyhow::Result<Vec<Quest>> {
        self.quest_generator.get_available_quests(player_id, Some(location_id))
    }
}

/// Context for quest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationContext {
    pub trigger: QuestTrigger,
    pub player_level: Option<u32>,
    pub location: Option<LocationId>,
    pub story_context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestTrigger {
    PlayerInteraction(NPCId),
    LocationDiscovery(LocationId),
    ItemFound(ItemId),
    LevelUp,
    TimeEvent,
    WorldEvent(EventId),
    NarrativeBranch(QuestId),
    PlayerRequest,
    SystemGenerated,
}

impl Default for QuestSystem {
    fn default() -> Self {
        Self::new()
    }
}