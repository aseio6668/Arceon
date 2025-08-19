use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Game events that can occur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    PlayerJoined(String),
    PlayerLeft(String),
    PlayerMoved { player_id: String, from: String, to: String },
    PlayerSaid { player_id: String, message: String },
    NpcSpawned(String),
    NpcDied(String),
    ItemDropped { item_id: String, location: String },
    SkillGained { player_id: String, skill: String, level: u32 },
    
    // Area-specific events
    PlayerEnteredArea { player_id: Uuid, area_id: Uuid },
    PlayerLeftArea { player_id: Uuid, area_id: Uuid },
    PlayerApproachedNpc { player_id: Uuid, npc_id: Uuid, area_id: Uuid },
    NpcAction { npc_id: Uuid, action: String, area_id: Uuid },
    AreaGenerated { area_id: Uuid, area_name: String },
    RouteTraversed { player_id: Uuid, from_location: String, to_location: String, description: String },
}

/// Event system for handling game events
pub struct EventSystem {
    events: Vec<GameEvent>,
}

impl EventSystem {
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
        }
    }
    
    pub fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);
    }
    
    pub fn get_recent_events(&self, limit: usize) -> &[GameEvent] {
        let start = if self.events.len() > limit {
            self.events.len() - limit
        } else {
            0
        };
        &self.events[start..]
    }
}
