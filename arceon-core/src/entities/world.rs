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

/// Universal coordinate system for the Arceon universe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalCoordinates {
    pub galaxy_id: Uuid,
    pub solar_system_id: Uuid,
    pub planet_id: Uuid,
    pub area_id: Option<Uuid>,
}

/// Represents a galaxy containing multiple solar systems
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Galaxy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub solar_systems: Vec<Uuid>,
    pub galaxy_type: GalaxyType,
    pub central_star_mass: f64, // Solar masses
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GalaxyType {
    Spiral,
    Elliptical,
    Irregular,
    Dwarf,
}

/// Represents a solar system containing planets
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct SolarSystem {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub galaxy_id: Uuid,
    pub planets: Vec<Uuid>,
    pub star_type: StarType,
    pub star_mass: f64, // Solar masses
    pub habitable_zone_inner: f64, // AU from star
    pub habitable_zone_outer: f64, // AU from star
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StarType {
    MainSequence,
    RedGiant,
    WhiteDwarf,
    NeutronStar,
    BinarySystem,
}

/// Represents a planet containing areas and locations
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub solar_system_id: Uuid,
    pub areas: Vec<Uuid>,
    pub planet_type: PlanetType,
    pub habitability: HabitabilityType,
    pub dominant_species: Vec<super::being::Race>,
    pub orbital_distance: f64, // AU from star
    pub gravity_multiplier: f32, // Relative to Earth
    pub atmosphere_composition: AtmosphereType,
    pub surface_conditions: SurfaceConditions,
    pub technology_level: TechnologyLevel,
    pub can_interact_with: Vec<Uuid>, // Other planets this one can communicate/travel to
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanetType {
    Terrestrial,    // Earth-like with solid surface
    Ocean,          // Mostly water worlds
    Desert,         // Arid worlds
    Ice,            // Frozen worlds
    Forest,         // Heavily forested worlds
    Mountain,       // Mountainous terrain dominant
    Volcanic,       // Active geological activity
    Crystal,        // Rich in magical crystals
    Floating,       // Sky islands and floating landmasses
    Underground,    // Cave systems and underground cities
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HabitabilityType {
    HighlyHabitable,    // Ideal conditions for most species
    EarthLike,          // Similar to Earth conditions
    Exotic,             // Unique but livable conditions
    Harsh,              // Difficult but survivable
    Uninhabitable,      // Requires life support
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AtmosphereType {
    Breathable,         // No equipment needed
    Filtered,           // Light breathing apparatus needed
    Hostile,            // Full life support required
    Magical,            // Atmosphere enhanced with magic
    Thin,               // Low oxygen, some difficulty breathing
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceConditions {
    pub temperature_range: (i32, i32), // Min, Max Celsius
    pub weather_patterns: Vec<WeatherCondition>,
    pub natural_hazards: Vec<String>,
    pub magical_phenomena: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TechnologyLevel {
    Primitive,          // Stone age, basic tools
    Medieval,           // Current Espan level - fantasy medieval
    Renaissance,        // Early scientific revolution
    Industrial,         // Steam and early machinery
    Modern,             // Earth-like current technology
    Advanced,           // Beyond current Earth tech
    Magical,            // Magic-based technology
    Hybrid,             // Mix of magic and technology
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

impl Planet {
    pub fn new(
        name: String,
        solar_system_id: Uuid,
        planet_type: PlanetType,
        habitability: HabitabilityType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            solar_system_id,
            areas: Vec::new(),
            planet_type,
            habitability,
            dominant_species: Vec::new(),
            orbital_distance: 1.0,
            gravity_multiplier: 1.0,
            atmosphere_composition: AtmosphereType::Breathable,
            surface_conditions: SurfaceConditions {
                temperature_range: (10, 30),
                weather_patterns: vec![WeatherCondition::Clear, WeatherCondition::Cloudy],
                natural_hazards: Vec::new(),
                magical_phenomena: Vec::new(),
            },
            technology_level: TechnologyLevel::Medieval,
            can_interact_with: Vec::new(),
        }
    }
    
    pub fn can_communicate_with(&self, other_planet_id: Uuid) -> bool {
        self.can_interact_with.contains(&other_planet_id)
    }
    
    pub fn discover_communication(&mut self, planet_id: Uuid) {
        if !self.can_interact_with.contains(&planet_id) {
            self.can_interact_with.push(planet_id);
        }
    }
}

impl SolarSystem {
    pub fn new(name: String, galaxy_id: Uuid, star_type: StarType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            galaxy_id,
            planets: Vec::new(),
            star_type,
            star_mass: 1.0,
            habitable_zone_inner: 0.95,
            habitable_zone_outer: 1.37,
        }
    }
}

impl Galaxy {
    pub fn new(name: String, galaxy_type: GalaxyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description: String::new(),
            solar_systems: Vec::new(),
            galaxy_type,
            central_star_mass: 4_000_000.0, // Typical supermassive black hole
        }
    }
}
