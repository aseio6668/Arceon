use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::being::Race;

/// Player component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub wallet_address: String,
    pub connected: bool,
    pub last_login: i64,
    pub total_playtime: u64,
}

/// Player character information
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub race: Race,
    pub gender: Gender,
    pub appearance: Appearance,
    pub deity: Option<String>,
    pub backstory: String,
    pub created_at: i64,
}

// Race enum moved to being.rs for consistency across all beings

/// Player gender
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other(String),
}

/// Character appearance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub height: u32,        // in cm
    pub build: Build,
    pub hair_color: String,
    pub eye_color: String,
    pub skin_tone: String,
    pub distinguishing_marks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Build {
    Slim,
    Average,
    Muscular,
    Heavy,
    Frail,
}

/// Player statistics (skill-based system)
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStats {
    pub strength: u32,
    pub intelligence: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub wisdom: u32,
    pub charisma: u32,
    pub luck: u32,
}

impl PlayerStats {
    pub fn new() -> Self {
        Self {
            strength: 10,
            intelligence: 10,
            dexterity: 10,
            constitution: 10,
            wisdom: 10,
            charisma: 10,
            luck: 10,
        }
    }
}

/// Player inventory component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub items: HashMap<Uuid, u32>, // Item ID -> Quantity
    pub max_slots: u32,
    pub equipped: EquippedItems,
}

/// Equipment slots
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedItems {
    pub head: Option<Uuid>,
    pub shoulders: Option<Uuid>,
    pub arms: Option<Uuid>,
    pub chest: Option<Uuid>,
    pub hands: Option<Uuid>,
    pub waist: Option<Uuid>,
    pub legs: Option<Uuid>,
    pub feet: Option<Uuid>,
    pub back: Option<Uuid>,
    pub main_hand: Option<Uuid>,
    pub off_hand: Option<Uuid>,
    pub rings: Vec<Option<Uuid>>, // Multiple ring slots
    pub neck: Option<Uuid>,
}

impl EquippedItems {
    pub fn new() -> Self {
        Self {
            head: None,
            shoulders: None,
            arms: None,
            chest: None,
            hands: None,
            waist: None,
            legs: None,
            feet: None,
            back: None,
            main_hand: None,
            off_hand: None,
            rings: vec![None; 4], // 4 ring slots
            neck: None,
        }
    }
}

/// Player currency wallet
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub arcm_balance: u64,      // ArcM (Arceon Monetary) - blockchain currency
    pub local_currency: HashMap<String, u64>, // Local in-world currencies
}
