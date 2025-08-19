use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::genesis::Position;

/// Enhanced targeting system for NPCs, objects, and players
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetingSystem {
    pub npc_targets: HashMap<Uuid, NpcTarget>,      // player_id -> current NPC target
    pub object_targets: HashMap<Uuid, ObjectTarget>, // player_id -> current object target
    pub area_entities: HashMap<String, AreaEntities>, // area_name -> entities in area
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcTarget {
    pub target_id: Uuid,
    pub target_name: String,
    pub target_type: NpcType,
    pub position: Position,
    pub can_interact: bool,
    pub can_attack: bool,
    pub relationship: RelationshipStatus,
    pub last_interaction: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTarget {
    pub target_id: Uuid,
    pub target_name: String,
    pub object_type: ObjectType,
    pub position: Position,
    pub can_pickup: bool,
    pub can_use: bool,
    pub can_examine: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcType {
    Player,
    Npc,
    Merchant,
    Guard,
    Scholar,
    Crafter,
    Leader,
    Hostile,
    Neutral,
    Friendly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectType {
    Weapon,
    Armor,
    Tool,
    Consumable,
    Material,
    Book,
    Container,
    Furniture,
    Structure,
    TerrainFeature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStatus {
    Allied,
    Friendly,
    Neutral,
    Suspicious,
    Hostile,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaEntities {
    pub area_name: String,
    pub npcs: Vec<NpcInArea>,
    pub objects: Vec<ObjectInArea>,
    pub players: Vec<PlayerInArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcInArea {
    pub npc_id: Uuid,
    pub name: String,
    pub npc_type: NpcType,
    pub position: Position,
    pub status: NpcStatus,
    pub can_interact: bool,
    pub can_attack: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInArea {
    pub object_id: Uuid,
    pub name: String,
    pub object_type: ObjectType,
    pub position: Position,
    pub can_pickup: bool,
    pub can_use: bool,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInArea {
    pub player_id: Uuid,
    pub name: String,
    pub position: Position,
    pub status: PlayerStatus,
    pub can_interact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NpcStatus {
    Idle,
    Working,
    Trading,
    Talking,
    Fighting,
    Moving,
    Sleeping,
    Dead,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerStatus {
    Online,
    Away,
    Busy,
    Offline,
}

impl TargetingSystem {
    pub fn new() -> Self {
        Self {
            npc_targets: HashMap::new(),
            object_targets: HashMap::new(),
            area_entities: HashMap::new(),
        }
    }

    /// Set NPC target for a player
    pub fn set_npc_target(&mut self, player_id: Uuid, target_id: Uuid, area_name: &str) -> Result<String, String> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            if let Some(npc) = area_entities.npcs.iter().find(|n| n.npc_id == target_id) {
                let target = NpcTarget {
                    target_id,
                    target_name: npc.name.clone(),
                    target_type: npc.npc_type.clone(),
                    position: npc.position.clone(),
                    can_interact: npc.can_interact,
                    can_attack: npc.can_attack,
                    relationship: RelationshipStatus::Unknown,
                    last_interaction: None,
                };
                
                self.npc_targets.insert(player_id, target);
                Ok(format!("Now targeting NPC: {}", npc.name))
            } else if let Some(player) = area_entities.players.iter().find(|p| p.player_id == target_id) {
                let target = NpcTarget {
                    target_id,
                    target_name: player.name.clone(),
                    target_type: NpcType::Player,
                    position: player.position.clone(),
                    can_interact: player.can_interact,
                    can_attack: false, // Players can't attack other players by default
                    relationship: RelationshipStatus::Neutral,
                    last_interaction: None,
                };
                
                self.npc_targets.insert(player_id, target);
                Ok(format!("Now targeting player: {}", player.name))
            } else {
                Err("Target not found in current area".to_string())
            }
        } else {
            Err("Area not found".to_string())
        }
    }

    /// Set object target for a player
    pub fn set_object_target(&mut self, player_id: Uuid, target_id: Uuid, area_name: &str) -> Result<String, String> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            if let Some(object) = area_entities.objects.iter().find(|o| o.object_id == target_id) {
                let target = ObjectTarget {
                    target_id,
                    target_name: object.name.clone(),
                    object_type: object.object_type.clone(),
                    position: object.position.clone(),
                    can_pickup: object.can_pickup,
                    can_use: object.can_use,
                    can_examine: true,
                };
                
                self.object_targets.insert(player_id, target);
                Ok(format!("Now targeting object: {}", object.name))
            } else {
                Err("Object not found in current area".to_string())
            }
        } else {
            Err("Area not found".to_string())
        }
    }

    /// Clear NPC target for a player
    pub fn clear_npc_target(&mut self, player_id: Uuid) -> String {
        if let Some(target) = self.npc_targets.remove(&player_id) {
            format!("No longer targeting {}", target.target_name)
        } else {
            "No NPC target to clear".to_string()
        }
    }

    /// Clear object target for a player
    pub fn clear_object_target(&mut self, player_id: Uuid) -> String {
        if let Some(target) = self.object_targets.remove(&player_id) {
            format!("No longer targeting {}", target.target_name)
        } else {
            "No object target to clear".to_string()
        }
    }

    /// Get current NPC target for a player
    pub fn get_npc_target(&self, player_id: Uuid) -> Option<&NpcTarget> {
        self.npc_targets.get(&player_id)
    }

    /// Get current object target for a player
    pub fn get_object_target(&self, player_id: Uuid) -> Option<&ObjectTarget> {
        self.object_targets.get(&player_id)
    }

    /// List all NPCs in current area
    pub fn list_npcs_in_area(&self, area_name: &str) -> Vec<&NpcInArea> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            area_entities.npcs.iter().collect()
        } else {
            Vec::new()
        }
    }

    /// List all objects in current area
    pub fn list_objects_in_area(&self, area_name: &str) -> Vec<&ObjectInArea> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            area_entities.objects.iter().collect()
        } else {
            Vec::new()
        }
    }

    /// List all players in current area
    pub fn list_players_in_area(&self, area_name: &str) -> Vec<&PlayerInArea> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            area_entities.players.iter().collect()
        } else {
            Vec::new()
        }
    }

    /// Update area entities (called when entities move or spawn)
    pub fn update_area_entities(&mut self, area_name: String, entities: AreaEntities) {
        self.area_entities.insert(area_name, entities);
    }

    /// Find entity by name in area
    pub fn find_entity_by_name(&self, area_name: &str, name: &str) -> Option<FoundEntity> {
        if let Some(area_entities) = self.area_entities.get(area_name) {
            // Check NPCs
            for npc in &area_entities.npcs {
                if npc.name.to_lowercase().contains(&name.to_lowercase()) {
                    return Some(FoundEntity::Npc(npc.npc_id, npc.name.clone()));
                }
            }
            
            // Check players
            for player in &area_entities.players {
                if player.name.to_lowercase().contains(&name.to_lowercase()) {
                    return Some(FoundEntity::Player(player.player_id, player.name.clone()));
                }
            }
            
            // Check objects
            for object in &area_entities.objects {
                if object.name.to_lowercase().contains(&name.to_lowercase()) {
                    return Some(FoundEntity::Object(object.object_id, object.name.clone()));
                }
            }
        }
        None
    }

    /// Calculate distance between two positions
    pub fn calculate_distance(&self, pos1: &Position, pos2: &Position) -> f64 {
        let dx = pos1.x - pos2.x;
        let dy = pos1.y - pos2.y;
        let dz = pos1.z - pos2.z;
        (dx*dx + dy*dy + dz*dz).sqrt()
    }

    /// Check if target is within interaction range
    pub fn is_within_interaction_range(&self, player_pos: &Position, target_pos: &Position) -> bool {
        self.calculate_distance(player_pos, target_pos) <= 5.0 // 5 unit interaction range
    }

    /// Check if target is within attack range
    pub fn is_within_attack_range(&self, player_pos: &Position, target_pos: &Position) -> bool {
        self.calculate_distance(player_pos, target_pos) <= 2.0 // 2 unit attack range
    }
}

#[derive(Debug, Clone)]
pub enum FoundEntity {
    Npc(Uuid, String),
    Player(Uuid, String),
    Object(Uuid, String),
}