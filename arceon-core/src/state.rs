use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::entities::{world::Area, being::Being, quests::QuestSystem};

/// Global game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub world_time: u64,
    pub online_players: HashMap<String, PlayerData>, // player_id -> player data
    pub world_events: Vec<String>,
    pub global_announcements: Vec<String>,
    pub areas: HashMap<String, Area>, // area_id -> area
    pub beings: HashMap<String, Being>, // being_id -> being
    pub quest_system: QuestSystem, // Quest and reputation system
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerData {
    pub being_id: String,
    pub current_area_id: String,
    pub last_activity: u64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            world_time: 0,
            online_players: HashMap::new(),
            world_events: Vec::new(),
            global_announcements: Vec::new(),
            areas: HashMap::new(),
            beings: HashMap::new(),
            quest_system: QuestSystem::new(),
        }
    }
    
    pub fn add_area(&mut self, area: Area) {
        self.areas.insert(area.id.to_string(), area);
    }
    
    pub fn add_being(&mut self, being: Being) {
        self.beings.insert(being.id.to_string(), being);
    }
    
    pub fn move_player_to_area(&mut self, player_id: &str, area_id: &str) -> Result<(), String> {
        if !self.areas.contains_key(area_id) {
            return Err(format!("Area {} not found", area_id));
        }
        
        if let Some(player_data) = self.online_players.get_mut(player_id) {
            player_data.current_area_id = area_id.to_string();
            player_data.last_activity = self.world_time;
            Ok(())
        } else {
            Err(format!("Player {} not found", player_id))
        }
    }
    
    pub fn get_players_in_area(&self, area_id: &str) -> Vec<&str> {
        self.online_players
            .iter()
            .filter_map(|(player_id, data)| {
                if data.current_area_id == area_id {
                    Some(player_id.as_str())
                } else {
                    None
                }
            })
            .collect()
    }
}
