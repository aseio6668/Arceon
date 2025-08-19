use std::collections::HashMap;
use uuid::Uuid;
use anyhow::Result;
use arceon_core::entities::world::Area;
use arceon_core::entities::being::Race;
use arceon_core::events::{GameEvent, EventSystem};

/// Manages all areas in the game world and player movement
pub struct AreaManager {
    areas: HashMap<Uuid, Area>,
    player_locations: HashMap<Uuid, Uuid>, // player_id -> area_id
    event_system: EventSystem,
}

impl AreaManager {
    pub fn new() -> Self {
        Self {
            areas: HashMap::new(),
            player_locations: HashMap::new(),
            event_system: EventSystem::new(),
        }
    }
    
    /// Add an area to the world
    pub fn add_area(&mut self, area: Area) {
        let area_id = area.id;
        self.areas.insert(area_id, area);
        self.event_system.add_event(GameEvent::AreaGenerated {
            area_id,
            area_name: self.areas[&area_id].name.clone(),
        });
    }
    
    /// Get an area by ID
    pub fn get_area(&self, area_id: &Uuid) -> Option<&Area> {
        self.areas.get(area_id)
    }
    
    /// Get a mutable reference to an area
    pub fn get_area_mut(&mut self, area_id: &Uuid) -> Option<&mut Area> {
        self.areas.get_mut(area_id)
    }
    
    /// Find the starting area for a race
    pub fn find_starting_area_for_race(&self, race: &Race) -> Option<&Area> {
        self.areas.values().find(|area| {
            if let Some(affinity) = &area.race_affinity {
                affinity.primary_race == *race
            } else {
                false
            }
        })
    }
    
    /// Move a player to a specific area
    pub fn move_player_to_area(&mut self, player_id: Uuid, target_area_id: Uuid) -> Result<String> {
        // Check if area exists
        if !self.areas.contains_key(&target_area_id) {
            return Err(anyhow::anyhow!("Target area does not exist"));
        }
        
        // Get previous location
        let previous_area = self.player_locations.get(&player_id).copied();
        
        // Update player location
        self.player_locations.insert(player_id, target_area_id);
        
        // Update area player lists
        if let Some(prev_area_id) = previous_area {
            if let Some(prev_area) = self.areas.get_mut(&prev_area_id) {
                prev_area.current_players.retain(|&id| id != player_id);
            }
            self.event_system.add_event(GameEvent::PlayerLeftArea {
                player_id,
                area_id: prev_area_id,
            });
        }
        
        if let Some(new_area) = self.areas.get_mut(&target_area_id) {
            new_area.current_players.push(player_id);
            self.event_system.add_event(GameEvent::PlayerEnteredArea {
                player_id,
                area_id: target_area_id,
            });
            
            Ok(format!("You have entered {}. {}", new_area.name, new_area.description))
        } else {
            Err(anyhow::anyhow!("Failed to update area"))
        }
    }
    
    /// Get the current area of a player
    pub fn get_player_area(&self, player_id: &Uuid) -> Option<&Area> {
        if let Some(area_id) = self.player_locations.get(player_id) {
            self.areas.get(area_id)
        } else {
            None
        }
    }
    
    /// Generate a travel description when moving within an area
    pub fn approach_location(&mut self, player_id: Uuid, target_location: &str) -> Result<String> {
        let area_id = self.player_locations.get(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player not in any area"))?;
        
        let area = self.areas.get(area_id)
            .ok_or_else(|| anyhow::anyhow!("Area not found"))?;
        
        if let Some(location) = area.get_location(target_location) {
            let travel_description = self.generate_travel_narrative(&area.name, target_location, &location.location_type);
            
            self.event_system.add_event(GameEvent::RouteTraversed {
                player_id,
                from_location: "your current position".to_string(),
                to_location: target_location.to_string(),
                description: travel_description.clone(),
            });
            
            Ok(format!("{}\n\nYou arrive at {}. {}", 
                travel_description, 
                location.name, 
                location.description
            ))
        } else {
            Err(anyhow::anyhow!("Location '{}' not found in this area", target_location))
        }
    }
    
    /// Find an NPC in the current area
    pub fn find_npc_in_area(&self, player_id: &Uuid, npc_name: &str) -> Result<String> {
        let area_id = self.player_locations.get(player_id)
            .ok_or_else(|| anyhow::anyhow!("Player not in any area"))?;
        
        let area = self.areas.get(area_id)
            .ok_or_else(|| anyhow::anyhow!("Area not found"))?;
        
        // Search through all locations in the area for NPCs
        for (location_name, location) in &area.locations {
            if !location.npcs_present.is_empty() {
                // For now, we'll use a simple search - in the future this would check actual NPC names
                if location_name.to_lowercase().contains(&npc_name.to_lowercase()) {
                    return Ok(format!("You find references to {} near {}. {}", 
                        npc_name, location_name, location.description));
                }
            }
        }
        
        Err(anyhow::anyhow!("Could not find '{}' in this area", npc_name))
    }
    
    /// Approach a specific NPC
    pub fn approach_npc(&mut self, player_id: Uuid, npc_name: &str) -> Result<String> {
        let area_id = self.player_locations.get(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player not in any area"))?;
        
        let area = self.areas.get(area_id)
            .ok_or_else(|| anyhow::anyhow!("Area not found"))?;
        
        // Find which location the NPC might be in based on name patterns
        let npc_location = self.find_likely_npc_location(npc_name, area);
        
        if let Some(location_name) = npc_location {
            let travel_description = self.generate_approach_narrative(&area.name, &location_name, npc_name);
            
            self.event_system.add_event(GameEvent::RouteTraversed {
                player_id,
                from_location: "your current position".to_string(),
                to_location: location_name.clone(),
                description: travel_description.clone(),
            });
            
            Ok(format!("{}\n\nYou approach {} and find them ready to speak with you.", 
                travel_description, npc_name))
        } else {
            Err(anyhow::anyhow!("Could not locate {} in this area", npc_name))
        }
    }
    
    /// Generate immersive travel narrative
    pub fn generate_travel_narrative(&self, area_name: &str, destination: &str, location_type: &arceon_core::entities::world::LocationType) -> String {
        use arceon_core::entities::world::LocationType;
        
        let base_narrative = match location_type {
            LocationType::Bank => format!("You make your way through the streets of {}, passing merchants and citizens going about their daily business. The impressive stone facade of {} comes into view.", area_name, destination),
            LocationType::Tavern => format!("You walk through the winding streets of {}, following the sound of laughter and the warm glow of lanterns that leads you to {}.", area_name, destination),
            LocationType::Market => format!("The bustling sounds of commerce guide you through {} as you head toward {}. Vendors call out their wares and the aroma of fresh goods fills the air.", area_name, destination),
            LocationType::Stable => format!("You follow the cobblestone paths of {}, hearing the gentle nickering of horses and the creak of leather that signals your approach to {}.", area_name, destination),
            LocationType::Temple => format!("A sense of peace settles over you as you walk the quiet paths of {} toward {}. The air seems lighter here, filled with a gentle reverence.", area_name, destination),
            LocationType::Harbor => format!("The salty breeze carries you through {} toward {}. You can hear the creaking of ship masts and the calls of sailors preparing for their next voyage.", area_name, destination),
            _ => format!("You make your way through the paths of {} toward {}, taking in the sights and sounds of this vibrant area.", area_name, destination),
        };
        
        base_narrative
    }
    
    /// Generate simple approach narrative for terminal usage
    pub fn generate_simple_approach_narrative(&self, from: &str, to: &str) -> String {
        format!("You make your way through {} toward {}. The path winds gently, and you take in the sights and sounds around you.", from, to)
    }
    
    /// Generate narrative for approaching an NPC
    fn generate_approach_narrative(&self, area_name: &str, location: &str, npc_name: &str) -> String {
        format!("You navigate through the bustling streets of {} toward {}. As you approach, you spot {} among the crowd. They notice your approach and turn to greet you.", 
            area_name, location, npc_name)
    }
    
    /// Find the most likely location for an NPC based on their name/type
    fn find_likely_npc_location(&self, npc_name: &str, area: &Area) -> Option<String> {
        let lower_name = npc_name.to_lowercase();
        
        // Simple heuristics for where NPCs might be found
        if lower_name.contains("stable") || lower_name.contains("horse") {
            area.locations.keys().find(|k| k.to_lowercase().contains("stable")).cloned()
        } else if lower_name.contains("bank") || lower_name.contains("gold") {
            area.locations.keys().find(|k| k.to_lowercase().contains("bank")).cloned()
        } else if lower_name.contains("priest") || lower_name.contains("cleric") {
            area.locations.keys().find(|k| k.to_lowercase().contains("temple")).cloned()
        } else if lower_name.contains("merchant") || lower_name.contains("trader") {
            area.locations.keys().find(|k| k.to_lowercase().contains("market")).cloned()
        } else if lower_name.contains("guard") || lower_name.contains("captain") {
            area.locations.keys().find(|k| k.to_lowercase().contains("guard")).cloned()
        } else {
            // Default to market or first available location
            area.locations.keys().find(|k| k.to_lowercase().contains("market"))
                .or_else(|| area.locations.keys().next())
                .cloned()
        }
    }
    
    /// List all areas
    pub fn list_areas(&self) -> Vec<&Area> {
        self.areas.values().collect()
    }
    
    /// Get recent events
    pub fn get_recent_events(&self, limit: usize) -> &[GameEvent] {
        self.event_system.get_recent_events(limit)
    }
}
