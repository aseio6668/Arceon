use bevy_ecs::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Large area that can contain multiple locations
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub area_type: AreaType,
    pub size: AreaSize,
    pub locations: HashMap<String, Location>,
    pub connected_areas: Vec<AreaConnection>,
    pub resident_npcs: Vec<Uuid>,
    pub current_players: Vec<Uuid>,
    pub race_affinity: Option<RaceAffinity>,
    pub climate: Climate,
    pub danger_level: u32, // 0-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AreaType {
    City,
    Capital,
    Village,
    Port,
    Forest,
    Plains,
    Mountains,
    Desert,
    Swamp,
    Island,
    Underground,
    Magical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AreaSize {
    Tiny,       // 1-5 locations
    Small,      // 1-10 locations
    Medium,     // 10-25 locations  
    Large,      // 25-50 locations
    Massive,    // 50+ locations
}

/// Specific location within an area
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub description: String,
    pub location_type: LocationType,
    pub npcs_present: Vec<Uuid>,
    pub items_present: Vec<Uuid>,
    pub connections: Vec<String>, // Names of other locations in this area
    pub special_features: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LocationType {
    Bank,
    Tavern,
    Shop,
    Stable,
    GuardPost,
    Temple,
    Library,
    CraftingHall,
    Residential,
    Market,
    Harbor,
    Gate,
    Park,
    TrainingGround,
    Wilderness,
    // New location types for diverse areas
    Clearing,       // Forest clearings, open areas
    Ruins,          // Ancient structures, abandoned buildings
    Camp,           // Temporary settlements, outposts
    Natural,        // Natural features like springs, viewpoints
    Cave,           // Underground areas, caverns
    Landmark,
}

/// Connection between areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaConnection {
    pub target_area_id: Uuid,
    pub connection_type: ConnectionType,
    pub travel_time: u32, // in minutes
    pub description: String,
    pub requirements: Vec<String>, // What's needed to use this route
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
    Road,
    Highway,        // Major roads between capitals
    ShipRoute,
    SeaRoute,       // Alias for ShipRoute  
    Portal,
    MountainPass,
    UndergroundTunnel,
    Tunnel,         // Alias for UndergroundTunnel
    Bridge,
    Ferry,
}

/// Race affinity for starting areas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaceAffinity {
    pub primary_race: super::being::Race,
    pub secondary_races: Vec<super::being::Race>,
    pub culture_type: CultureType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CultureType {
    Traditional,
    Cosmopolitan,
    Isolated,
    TradingHub,
    Military,
    Scholarly,
    Artisan,
    Ancient,        // For ruins and lost civilizations
    Mystical,       // For magical and otherworldly places
}

/// Weather and climate system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Climate {
    pub base_temperature: i32, // Celsius
    pub humidity: f32,         // 0.0 to 1.0
    pub current_weather: Weather,
    pub seasonal_variation: f32,
}

/// Weather system component
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Weather {
    pub condition: WeatherCondition,
    pub temperature: i32, // in Celsius
    pub wind_speed: u32,  // km/h
    pub visibility: f32,  // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
    Foggy,
    Windy,
}

impl Area {
    pub fn new(name: String, area_type: AreaType, race_affinity: Option<RaceAffinity>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            area_type,
            size: AreaSize::Medium,
            locations: HashMap::new(),
            connected_areas: Vec::new(),
            resident_npcs: Vec::new(),
            current_players: Vec::new(),
            race_affinity,
            climate: Climate {
                base_temperature: 20,
                humidity: 0.5,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 20,
                    wind_speed: 5,
                    visibility: 1.0,
                },
                seasonal_variation: 0.3,
            },
            danger_level: 1,
        }
    }
    
    pub fn add_location(&mut self, name: String, location: Location) {
        self.locations.insert(name, location);
    }
    
    pub fn get_location(&self, name: &str) -> Option<&Location> {
        self.locations.get(name)
    }
    
    pub fn find_npc_location(&self, npc_id: Uuid) -> Option<&str> {
        for (location_name, location) in &self.locations {
            if location.npcs_present.contains(&npc_id) {
                return Some(location_name);
            }
        }
        None
    }
}
