use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Non-Player Character component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Npc {
    pub npc_type: NpcType,
    pub faction: String,
    pub disposition: i32,      // -100 to 100, how they feel about player
    pub can_respawn: bool,
    pub respawn_timer: Option<u64>,
    pub last_death: Option<i64>,
}

/// Types of NPCs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcType {
    Avatar,          // Friendly, story-driven NPCs
    Merchant,        // Traders and shopkeepers
    Guard,           // City/area protection
    Monster,         // Hostile creatures
    Neutral,         // Non-hostile, non-interactive
    QuestGiver,      // NPCs that provide quests
    Crafter,         // Specialized crafting NPCs
}

/// NPC personality and behavior
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NpcPersonality {
    pub traits: HashMap<String, f32>, // Personality traits (0.0 to 1.0)
    pub mood: Mood,
    pub interests: Vec<String>,
    pub relationships: HashMap<String, f32>, // NPC/Player ID -> relationship strength
    pub memory_capacity: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Mood {
    Happy,
    Sad,
    Angry,
    Neutral,
    Excited,
    Fearful,
    Contemplative,
}

/// NPC dialogue and interaction system
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NpcDialogue {
    pub conversation_trees: HashMap<String, ConversationNode>,
    pub current_topic: Option<String>,
    pub last_interaction: Option<i64>,
    pub known_information: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationNode {
    pub text: String,
    pub responses: Vec<DialogueResponse>,
    pub conditions: Vec<String>, // Conditions that must be met
    pub effects: Vec<String>,    // Effects when this node is reached
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub next_node: Option<String>,
    pub requirements: Vec<String>, // Player requirements to use this response
}

/// NPC AI behavior state
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NpcAiState {
    pub current_goal: Goal,
    pub goal_stack: Vec<Goal>,
    pub last_action: Option<String>,
    pub action_cooldown: f32,
    pub awareness_range: f32,
    pub memory: Vec<Memory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Goal {
    Idle,
    Patrol(Vec<String>),      // Patrol between locations
    Guard(String),            // Guard a specific location
    Craft(String),           // Work on crafting
    Socialize(String),       // Interact with specific entity
    Flee(String),            // Run from threat
    Attack(String),          // Combat target
    Trade,                   // Looking for trading opportunities
    Study(String),           // Study/research something
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Memory {
    pub event: String,
    pub timestamp: i64,
    pub importance: f32,
    pub entities_involved: Vec<String>,
    pub location: String,
}

/// NPC skills and abilities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NpcSkills {
    pub combat_skills: HashMap<String, u32>,
    pub crafting_skills: HashMap<String, u32>,
    pub social_skills: HashMap<String, u32>,
    pub magic_schools: HashMap<String, u32>,
    pub knowledge_areas: HashMap<String, u32>,
}

/// NPC story progression
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct NpcStory {
    pub current_chapter: String,
    pub completed_events: Vec<String>,
    pub available_quests: Vec<String>,
    pub personal_goals: Vec<String>,
    pub life_events: Vec<LifeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifeEvent {
    pub event_type: String,
    pub description: String,
    pub timestamp: i64,
    pub impact_on_personality: HashMap<String, f32>,
}
