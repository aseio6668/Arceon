use anyhow::Result;
use arceon_core::entities::world::{
    Area, AreaType, Location, LocationType, AreaConnection, ConnectionType,
    RaceAffinity, CultureType, Climate, Weather, WeatherCondition, AreaSize
};
use arceon_core::entities::being::Race;
use crate::name_generator::NameGenerator;
use uuid::Uuid;

pub struct AreaGenerator {
    name_generator: NameGenerator,
    rng: rand::rngs::ThreadRng,
}

impl AreaGenerator {
    pub fn new() -> Self {
        Self {
            name_generator: NameGenerator::new(),
            rng: rand::thread_rng(),
        }
    }
    
    /// Generate a capital city for a specific race
    pub fn generate_capital(&mut self, race: Race, name: String, culture: CultureType) -> Result<Area> {
        let area_type = AreaType::Capital;
        let race_affinity = Some(RaceAffinity {
            primary_race: race.clone(),
            secondary_races: self.get_friendly_races(&race),
            culture_type: culture.clone(),
        });
        
        let mut area = Area::new(name.clone(), area_type, race_affinity);
        area.size = AreaSize::Large;
        area.description = self.generate_capital_description(&race, &culture);
        area.climate = self.generate_climate_for_race(&race);
        area.danger_level = 1; // Capitals are generally safe
        
        // Generate locations within the capital
        self.generate_capital_locations(&mut area, &race, &culture)?;
        
        Ok(area)
    }
    
    /// Connect areas with roads and travel routes
    pub fn connect_areas(&mut self, areas: &mut Vec<Area>) -> Result<()> {
        use uuid::Uuid;
        
        // Create connections between areas based on geographical and cultural proximity
        let area_ids: Vec<Uuid> = areas.iter().map(|a| a.id).collect();
        
        for i in 0..areas.len() {
            let connections = self.generate_connections_for_area(&areas[i], &area_ids, areas);
            areas[i].connected_areas = connections;
        }
        
        Ok(())
    }
    
    /// Generate connections for a specific area
    fn generate_connections_for_area(&mut self, area: &Area, all_area_ids: &[Uuid], all_areas: &[Area]) -> Vec<AreaConnection> {
        let mut connections = Vec::new();
        
        // Connect to nearby areas based on area type and culture
        for other_area in all_areas {
            if other_area.id == area.id {
                continue; // Don't connect to self
            }
            
            // Determine if areas should be connected
            if self.should_connect_areas(area, other_area) {
                let connection = AreaConnection {
                    target_area_id: other_area.id,
                    connection_type: self.determine_connection_type(area, other_area),
                    travel_time: self.calculate_travel_time(area, other_area),
                    description: self.generate_connection_description(area, other_area),
                    requirements: self.get_connection_requirements(area, other_area),
                };
                connections.push(connection);
            }
        }
        
        connections
    }
    
    /// Determine if two areas should be connected
    fn should_connect_areas(&self, area1: &Area, area2: &Area) -> bool {
        // Always connect capitals to each other (major trade routes)
        if matches!(area1.area_type, AreaType::Capital) && matches!(area2.area_type, AreaType::Capital) {
            return true;
        }
        
        // Connect areas with compatible races
        if let (Some(affinity1), Some(affinity2)) = (&area1.race_affinity, &area2.race_affinity) {
            // Same primary race - definitely connected
            if affinity1.primary_race == affinity2.primary_race {
                return true;
            }
            
            // Friendly races connected
            if affinity1.secondary_races.contains(&affinity2.primary_race) {
                return true;
            }
        }
        
        // Connect some areas randomly for trade routes
        // About 30% chance for non-hostile connections
        use rand::Rng;
        rand::thread_rng().gen_bool(0.3)
    }
    
    /// Determine the type of connection between areas
    fn determine_connection_type(&self, area1: &Area, area2: &Area) -> ConnectionType {
        match (&area1.area_type, &area2.area_type) {
            (AreaType::Capital, AreaType::Capital) => ConnectionType::Highway,
            (AreaType::City, AreaType::Capital) | (AreaType::Capital, AreaType::City) => ConnectionType::Road,
            (AreaType::Port, _) | (_, AreaType::Port) => ConnectionType::SeaRoute,
            (AreaType::Underground, _) | (_, AreaType::Underground) => ConnectionType::Tunnel,
            (AreaType::Mountains, _) | (_, AreaType::Mountains) => ConnectionType::MountainPass,
            _ => ConnectionType::Road,
        }
    }
    
    /// Calculate travel time between areas in minutes
    fn calculate_travel_time(&self, area1: &Area, area2: &Area) -> u32 {
        let base_time = match (&area1.area_type, &area2.area_type) {
            (AreaType::Capital, AreaType::Capital) => 60, // 1 hour between capitals
            (AreaType::City, AreaType::Capital) | (AreaType::Capital, AreaType::City) => 45,
            (AreaType::Port, _) | (_, AreaType::Port) => 90, // Sea travel takes longer
            (AreaType::Underground, _) | (_, AreaType::Underground) => 30, // Underground is faster
            (AreaType::Mountains, _) | (_, AreaType::Mountains) => 120, // Mountain travel is slow
            _ => 30,
        };
        
        // Add some variation
        use rand::Rng;
        let variation = rand::thread_rng().gen_range(-10..=10);
        (base_time + variation).max(10) as u32
    }
    
    /// Generate a description for the connection
    fn generate_connection_description(&self, area1: &Area, area2: &Area) -> String {
        match self.determine_connection_type(area1, area2) {
            ConnectionType::Highway => format!("A well-maintained highway connecting {} to {}", area1.name, area2.name),
            ConnectionType::Road => format!("A traveled road between {} and {}", area1.name, area2.name),
            ConnectionType::SeaRoute | ConnectionType::ShipRoute => format!("A sea route from {} to {}", area1.name, area2.name),
            ConnectionType::Tunnel | ConnectionType::UndergroundTunnel => format!("An underground passage linking {} and {}", area1.name, area2.name),
            ConnectionType::MountainPass => format!("A treacherous mountain pass between {} and {}", area1.name, area2.name),
            ConnectionType::Bridge => format!("A bridge spanning from {} to {}", area1.name, area2.name),
            ConnectionType::Portal => format!("A magical portal connecting {} to {}", area1.name, area2.name),
            ConnectionType::Ferry => format!("A ferry crossing between {} and {}", area1.name, area2.name),
        }
    }
    
    /// Get requirements for using this connection
    fn get_connection_requirements(&self, area1: &Area, area2: &Area) -> Vec<String> {
        let mut requirements = Vec::new();
        
        match self.determine_connection_type(area1, area2) {
            ConnectionType::MountainPass => {
                requirements.push("Cold weather gear recommended".to_string());
                requirements.push("Mountain climbing experience helpful".to_string());
            },
            ConnectionType::SeaRoute | ConnectionType::ShipRoute => {
                requirements.push("Sea-worthy vessel required".to_string());
                requirements.push("Basic sailing knowledge".to_string());
            },
            ConnectionType::Portal => {
                requirements.push("Magical aptitude required".to_string());
                requirements.push("Portal key needed".to_string());
            },
            ConnectionType::Tunnel | ConnectionType::UndergroundTunnel => {
                requirements.push("Underground navigation skills".to_string());
            },
            ConnectionType::Ferry => {
                requirements.push("Ferry fare required".to_string());
            },
            _ => {
                // Most routes have no special requirements
            }
        }
        
        // Add danger-based requirements
        let max_danger = area1.danger_level.max(area2.danger_level);
        if max_danger >= 5 {
            requirements.push("Armed escort recommended".to_string());
        }
        if max_danger >= 8 {
            requirements.push("Travel not recommended for inexperienced adventurers".to_string());
        }
        
        requirements
    }
    
    /// Generate a wilderness area
    pub fn generate_wilderness(&mut self, name: String, area_type: AreaType, size: AreaSize, danger_level: u32) -> Result<Area> {
        let mut area = Area::new(name.clone(), area_type.clone(), None);
        area.size = size;
        area.danger_level = danger_level;
        area.climate = self.generate_climate_for_area_type(&area_type);
        
        // Generate wilderness-specific locations
        self.generate_wilderness_locations(&mut area, &area_type)?;
        
        Ok(area)
    }
    
    /// Generate a settlement (trading post, village, etc.)
    pub fn generate_settlement(&mut self, name: String, culture: CultureType, danger_level: u32) -> Result<Area> {
        let area_type = match culture {
            CultureType::TradingHub => AreaType::City,
            _ => AreaType::City,
        };
        
        let mut area = Area::new(name.clone(), area_type, None);
        area.size = AreaSize::Small;
        area.danger_level = danger_level;
        area.climate = self.generate_temperate_climate();
        
        // Generate settlement-specific locations
        self.generate_settlement_locations(&mut area, &culture)?;
        
        Ok(area)
    }
    
    /// Generate a special area (magical, ancient, etc.)
    pub fn generate_special_area(&mut self, name: String, area_type: AreaType, culture: CultureType, danger_level: u32) -> Result<Area> {
        let mut area = Area::new(name.clone(), area_type.clone(), None);
        area.size = match area_type {
            AreaType::Magical => AreaSize::Medium,
            AreaType::Underground => AreaSize::Large,
            _ => AreaSize::Medium,
        };
        area.danger_level = danger_level;
        area.climate = self.generate_climate_for_area_type(&area_type);
        
        // Generate special locations
        self.generate_special_locations(&mut area, &area_type, &culture)?;
        
        Ok(area)
    }
    
    /// Generate locations for wilderness areas
    fn generate_wilderness_locations(&mut self, area: &mut Area, area_type: &AreaType) -> Result<()> {
        match area_type {
            AreaType::Forest => {
                self.add_location_to_area(area, LocationType::Clearing, "Ancient Grove")?;
                self.add_location_to_area(area, LocationType::Ruins, "Overgrown Shrine")?;
                self.add_location_to_area(area, LocationType::Camp, "Ranger Outpost")?;
                self.add_location_to_area(area, LocationType::Natural, "Crystal Spring")?;
            }
            AreaType::Mountains => {
                self.add_location_to_area(area, LocationType::Cave, "Dragon's Lair")?;
                self.add_location_to_area(area, LocationType::Ruins, "Abandoned Mine")?;
                self.add_location_to_area(area, LocationType::Natural, "Eagle's Perch")?;
                self.add_location_to_area(area, LocationType::Camp, "Mountain Shelter")?;
            }
            AreaType::Desert => {
                self.add_location_to_area(area, LocationType::Ruins, "Buried Temple")?;
                self.add_location_to_area(area, LocationType::Natural, "Oasis")?;
                self.add_location_to_area(area, LocationType::Cave, "Sand Tunnels")?;
                self.add_location_to_area(area, LocationType::Camp, "Nomad Camp")?;
            }
            AreaType::Swamp => {
                self.add_location_to_area(area, LocationType::Ruins, "Sunken Tower")?;
                self.add_location_to_area(area, LocationType::Natural, "Moonwell")?;
                self.add_location_to_area(area, LocationType::Camp, "Herbalist Hut")?;
                self.add_location_to_area(area, LocationType::Cave, "Mud Caves")?;
            }
            _ => {
                // Default wilderness locations
                self.add_location_to_area(area, LocationType::Camp, "Traveler's Rest")?;
                self.add_location_to_area(area, LocationType::Natural, "Scenic Overlook")?;
            }
        }
        Ok(())
    }
    
    /// Generate locations for settlements
    fn generate_settlement_locations(&mut self, area: &mut Area, culture: &CultureType) -> Result<()> {
        // Basic settlement needs
        self.add_location_to_area(area, LocationType::Tavern, "The Traveler's Rest")?;
        self.add_location_to_area(area, LocationType::Market, "Village Square")?;
        self.add_location_to_area(area, LocationType::Stable, "Stables")?;
        
        // Culture-specific additions
        match culture {
            CultureType::TradingHub => {
                self.add_location_to_area(area, LocationType::Shop, "Trading Post")?;
                self.add_location_to_area(area, LocationType::Bank, "Merchant Bank")?;
                self.add_location_to_area(area, LocationType::Harbor, "Trading Dock")?;
            }
            CultureType::Traditional => {
                self.add_location_to_area(area, LocationType::Temple, "Village Shrine")?;
                self.add_location_to_area(area, LocationType::Residential, "Fishing Quarter")?;
                self.add_location_to_area(area, LocationType::Harbor, "Harbor")?;
            }
            CultureType::Artisan => {
                self.add_location_to_area(area, LocationType::CraftingHall, "Smithy")?;
                self.add_location_to_area(area, LocationType::Shop, "Tool Shop")?;
                self.add_location_to_area(area, LocationType::Residential, "Miners' Quarter")?;
            }
            _ => {
                self.add_location_to_area(area, LocationType::Residential, "Residential Area")?;
            }
        }
        Ok(())
    }
    
    /// Generate locations for special areas
    fn generate_special_locations(&mut self, area: &mut Area, area_type: &AreaType, culture: &CultureType) -> Result<()> {
        match (area_type, culture) {
            (AreaType::Magical, CultureType::Scholarly) => {
                self.add_location_to_area(area, LocationType::Library, "Grand Library")?;
                self.add_location_to_area(area, LocationType::TrainingGround, "Practice Yards")?;
                self.add_location_to_area(area, LocationType::Temple, "Arcane Sanctum")?;
                self.add_location_to_area(area, LocationType::Residential, "Student Quarters")?;
            }
            (AreaType::Underground, CultureType::Ancient) => {
                self.add_location_to_area(area, LocationType::Ruins, "Ancient Throne Room")?;
                self.add_location_to_area(area, LocationType::Cave, "Crystal Caverns")?;
                self.add_location_to_area(area, LocationType::Ruins, "Ritual Chamber")?;
                self.add_location_to_area(area, LocationType::Cave, "Endless Passages")?;
            }
            (AreaType::Magical, CultureType::Mystical) => {
                self.add_location_to_area(area, LocationType::Temple, "Sky Temple")?;
                self.add_location_to_area(area, LocationType::TrainingGround, "Wind Platforms")?;
                self.add_location_to_area(area, LocationType::Natural, "Star Observatory")?;
                self.add_location_to_area(area, LocationType::Residential, "Cloud Chambers")?;
            }
            _ => {
                // Default special locations
                self.add_location_to_area(area, LocationType::Ruins, "Ancient Structure")?;
                self.add_location_to_area(area, LocationType::Natural, "Mysterious Formation")?;
            }
        }
        Ok(())
    }
    
    /// Helper method to add a location to an area
    fn add_location_to_area(&mut self, area: &mut Area, location_type: LocationType, name: &str) -> Result<()> {
        let location = self.generate_location(location_type, name, &Race::Human)?; // Default to human for non-racial areas
        area.add_location(name.to_string(), location);
        Ok(())
    }
    
    /// Generate climate for different area types
    fn generate_climate_for_area_type(&self, area_type: &AreaType) -> Climate {
        match area_type {
            AreaType::Desert => Climate {
                base_temperature: 35,
                humidity: 0.1,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 38,
                    wind_speed: 15,
                    visibility: 1.0,
                },
                seasonal_variation: 0.4,
            },
            AreaType::Mountains => Climate {
                base_temperature: 5,
                humidity: 0.3,
                current_weather: Weather {
                    condition: WeatherCondition::Snowy,
                    temperature: 2,
                    wind_speed: 25,
                    visibility: 0.7,
                },
                seasonal_variation: 0.6,
            },
            AreaType::Swamp => Climate {
                base_temperature: 18,
                humidity: 0.9,
                current_weather: Weather {
                    condition: WeatherCondition::Foggy,
                    temperature: 20,
                    wind_speed: 5,
                    visibility: 0.3,
                },
                seasonal_variation: 0.2,
            },
            AreaType::Forest => Climate {
                base_temperature: 15,
                humidity: 0.6,
                current_weather: Weather {
                    condition: WeatherCondition::Cloudy,
                    temperature: 18,
                    wind_speed: 8,
                    visibility: 0.8,
                },
                seasonal_variation: 0.3,
            },
            _ => self.generate_temperate_climate(),
        }
    }
    
    /// Generate a temperate climate
    fn generate_temperate_climate(&self) -> Climate {
        Climate {
            base_temperature: 20,
            humidity: 0.5,
            current_weather: Weather {
                condition: WeatherCondition::Clear,
                temperature: 22,
                wind_speed: 10,
                visibility: 1.0,
            },
            seasonal_variation: 0.3,
        }
    }
    
    /// Generate essential locations for a capital city
    fn generate_capital_locations(&mut self, area: &mut Area, race: &Race, culture: &CultureType) -> Result<()> {
        // Central locations every capital needs
        let essential_locations = vec![
            LocationType::Market,
            LocationType::Bank,
            LocationType::Tavern,
            LocationType::Temple,
            LocationType::GuardPost,
            LocationType::Stable,
        ];
        
        // Cultural specific locations
        let cultural_locations = match culture {
            CultureType::TradingHub => vec![LocationType::Harbor, LocationType::Shop, LocationType::Market],
            CultureType::Scholarly => vec![LocationType::Library, LocationType::Temple],
            CultureType::Artisan => vec![LocationType::CraftingHall, LocationType::Shop],
            CultureType::Military => vec![LocationType::TrainingGround, LocationType::GuardPost],
            _ => vec![LocationType::Park, LocationType::Residential],
        };
        
        // Generate all locations
        for location_type in essential_locations.into_iter().chain(cultural_locations) {
            let location_name = self.name_generator.generate_location_name(&location_type, race);
            let location = self.generate_location(location_type, &location_name, race)?;
            area.add_location(location_name, location);
        }
        
        // Add some residential areas
        for i in 1..=3 {
            let district_name = format!("{} District", self.get_district_name(i));
            let location = self.generate_location(LocationType::Residential, &district_name, race)?;
            area.add_location(district_name, location);
        }
        
        // Add gates for connections to other areas
        let gates = vec!["North Gate", "South Gate", "East Gate", "West Gate"];
        for gate_name in gates {
            let location = self.generate_location(LocationType::Gate, gate_name, race)?;
            area.add_location(gate_name.to_string(), location);
        }
        
        Ok(())
    }
    
    /// Generate a specific location within an area
    fn generate_location(&mut self, location_type: LocationType, name: &str, race: &Race) -> Result<Location> {
        let description = self.generate_location_description(&location_type, name, race);
        let connections = self.generate_location_connections(&location_type);
        let special_features = self.generate_special_features(&location_type, race);
        
        Ok(Location {
            name: name.to_string(),
            description,
            location_type,
            npcs_present: Vec::new(), // Will be populated later
            items_present: Vec::new(), // Will be populated later
            connections,
            special_features,
        })
    }
    
    
    /// Generate a description for a capital city
    fn generate_capital_description(&mut self, race: &Race, culture: &CultureType) -> String {
        match race {
            Race::Human => match culture {
                CultureType::TradingHub => "A bustling port city where merchants from across the realm come to trade. The harbor is filled with ships from distant lands, and the sound of commerce echoes through the streets.".to_string(),
                CultureType::Traditional => "A proud city with ancient stone walls and noble houses. Banners flutter from tall towers, and the streets are paved with cobblestones worn smooth by centuries of use.".to_string(),
                _ => "A grand human settlement that serves as a beacon of civilization in the realm.".to_string(),
            },
            Race::Elf => "An ethereal city nestled among ancient trees, where elegant spires of living wood reach toward the sky. Bridges of woven branches connect the various levels, and soft lights dance among the leaves.".to_string(),
            Race::Dwarf => "A mighty fortress carved into the living stone of the mountain. Great halls echo with the ring of hammers on anvils, and the warm glow of forges lights the underground passages.".to_string(),
            Race::Gnome => "An ingenious underground city filled with wonderful contraptions and glittering crystals. Steam pipes carry messages between districts, and the sound of tiny gears fills the air.".to_string(),
            Race::Halfling => "A cozy collection of round doors built into rolling green hills. Smoke rises from countless chimneys, and the scent of fresh bread and pipe-weed drifts on the breeze.".to_string(),
            Race::Orc => "A imposing stronghold of dark stone and iron spikes. War drums echo from the towers, and the air is thick with the smoke of forges crafting weapons of war.".to_string(),
            _ => "A unique settlement that reflects the character of its inhabitants.".to_string(),
        }
    }
    
    /// Generate appropriate climate for each race
    fn generate_climate_for_race(&mut self, race: &Race) -> Climate {
        match race {
            Race::Human => Climate {
                base_temperature: 15,
                humidity: 0.6,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 18,
                    wind_speed: 10,
                    visibility: 1.0,
                },
                seasonal_variation: 0.4,
            },
            Race::Elf => Climate {
                base_temperature: 12,
                humidity: 0.8,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 15,
                    wind_speed: 5,
                    visibility: 0.9,
                },
                seasonal_variation: 0.3,
            },
            Race::Dwarf => Climate {
                base_temperature: 8,
                humidity: 0.4,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 10,
                    wind_speed: 15,
                    visibility: 1.0,
                },
                seasonal_variation: 0.5,
            },
            _ => Climate {
                base_temperature: 15,
                humidity: 0.5,
                current_weather: Weather {
                    condition: WeatherCondition::Clear,
                    temperature: 15,
                    wind_speed: 8,
                    visibility: 1.0,
                },
                seasonal_variation: 0.4,
            },
        }
    }
    
    /// Get races that are friendly to the given race
    fn get_friendly_races(&self, race: &Race) -> Vec<Race> {
        match race {
            Race::Human => vec![Race::Halfling, Race::Dwarf],
            Race::Elf => vec![Race::Halfling, Race::Gnome],
            Race::Dwarf => vec![Race::Human, Race::Gnome],
            Race::Halfling => vec![Race::Human, Race::Elf, Race::Gnome],
            Race::Gnome => vec![Race::Elf, Race::Dwarf, Race::Halfling],
            Race::Orc => vec![], // Orcs are generally hostile
            _ => vec![Race::Human],
        }
    }
    
    /// Generate location description
    fn generate_location_description(&mut self, location_type: &LocationType, name: &str, _race: &Race) -> String {
        match location_type {
            LocationType::Market => format!("The {} bustles with activity as merchants display their wares and shoppers haggle over prices.", name),
            LocationType::Bank => format!("The {} is a secure stone building where the city's wealth is carefully guarded.", name),
            LocationType::Tavern => format!("{} welcomes travelers with warm food, cold drinks, and tales of adventure.", name),
            LocationType::Temple => format!("{} is a sacred place where the faithful come to pray and seek guidance.", name),
            LocationType::Stable => format!("The {} houses the finest mounts in the city, ready for travel or work.", name),
            LocationType::Harbor => "Ships from distant lands dock here, their holds full of exotic goods and stories.".to_string(),
            LocationType::Gate => format!("The {} provides passage to other parts of the realm, guarded by watchful sentries.", name),
            _ => format!("The {} serves the needs of the local community.", name),
        }
    }
    
    /// Generate connections between locations
    fn generate_location_connections(&mut self, location_type: &LocationType) -> Vec<String> {
        match location_type {
            LocationType::Market => vec!["Bank".to_string(), "Various Shops".to_string()],
            LocationType::Bank => vec!["Market".to_string()],
            LocationType::Tavern => vec!["Market".to_string(), "Residential Areas".to_string()],
            LocationType::Harbor => vec!["Market".to_string(), "Various Gates".to_string()],
            LocationType::Gate => vec!["Roads to other areas".to_string()],
            _ => vec!["Market".to_string()],
        }
    }
    
    /// Generate special features for locations
    fn generate_special_features(&mut self, location_type: &LocationType, race: &Race) -> Vec<String> {
        let mut features = Vec::new();
        
        match location_type {
            LocationType::Market => features.push("Merchant stalls".to_string()),
            LocationType::Temple => features.push("Sacred altar".to_string()),
            LocationType::Harbor => features.push("Docking facilities".to_string()),
            _ => {},
        }
        
        // Add race-specific features
        match race {
            Race::Elf => features.push("Elegant elven architecture".to_string()),
            Race::Dwarf => features.push("Masterful stonework".to_string()),
            Race::Gnome => features.push("Ingenious mechanisms".to_string()),
            _ => {},
        }
        
        features
    }
    
    /// Get district names
    fn get_district_name(&self, index: usize) -> &'static str {
        let names = vec!["Noble", "Merchant", "Artisan", "Common", "River", "Hill"];
        names.get(index).unwrap_or(&"Residential")
    }
}
