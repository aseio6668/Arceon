pub mod player;
pub mod npc;
pub mod world;
pub mod item;
pub mod skill;
pub mod being;
pub mod skills;
pub mod archetypes;
pub mod quests;
pub mod knowledge;
pub mod governance;
pub mod character_creation;

use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use player::{Player, Character, Gender, Appearance, Build, PlayerStats, Inventory, EquippedItems, Wallet};
pub use npc::*;
pub use world::*;
pub use item::*;
pub use skill::{Skill as SkillEnumType, SkillLevel, Skills, CombatSkills, CraftingSkills, MagicSkills, SurvivalSkills, SocialSkills};
pub use being::{Being, BeingType, Race, BeingCapability, ExperienceSource, DiscoveryMethod, 
    EvolutionRequest, RequestStatus, EvolutionEvidence, Skill, SkillType, SkillCategory, 
    PassiveTrait, ActiveTrait, SkillRequirement, Vital, ExperienceGain};

/// Base component for all entities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/// Position component for entities in the world
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub plane: String,      // Which plane of existence
    pub continent: String,  // Which continent
    pub region: String,     // Which region/area
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Health component for living entities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: u32,
    pub maximum: u32,
    pub regeneration_rate: f32,
}

impl Health {
    pub fn new(max_health: u32) -> Self {
        Self {
            current: max_health,
            maximum: max_health,
            regeneration_rate: 1.0,
        }
    }
    
    pub fn is_alive(&self) -> bool {
        self.current > 0
    }
    
    pub fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.maximum);
    }
    
    pub fn damage(&mut self, amount: u32) {
        self.current = self.current.saturating_sub(amount);
    }
}

/// Name component for entities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Name(pub String);

/// Description component for entities
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Description(pub String);
