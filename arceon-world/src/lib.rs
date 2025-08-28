pub mod area_generator;
pub mod name_generator;
pub mod area_manager;
pub mod genesis;
pub mod runestone_knowledge;
pub mod command_engine;
pub mod targeting;
pub mod enhanced_commands;
pub mod trading_system;
pub mod race_database;

use anyhow::Result;
use arceon_core::entities::world::{
    Area, AreaType, AreaSize, CultureType, Planet, PlanetType, HabitabilityType, 
    SolarSystem, StarType, Galaxy, GalaxyType, AtmosphereType, SurfaceConditions, 
    TechnologyLevel, WeatherCondition, UniversalCoordinates
};
use arceon_core::entities::being::Race;
use uuid::Uuid;
pub use area_generator::AreaGenerator;
pub use area_manager::AreaManager;
pub use genesis::*;
pub use runestone_knowledge::*;
pub use command_engine::*;
pub use race_database::*;

/// Universe generation and management systems
pub struct UniverseManager {
    area_generator: AreaGenerator,
    pub galaxies: Vec<Galaxy>,
    pub solar_systems: Vec<SolarSystem>,
    pub planets: Vec<Planet>,
    pub race_database: RaceDatabase,
}

impl UniverseManager {
    pub fn new() -> Self {
        Self {
            area_generator: AreaGenerator::new(),
            galaxies: Vec::new(),
            solar_systems: Vec::new(),
            planets: Vec::new(),
            race_database: RaceDatabase::new(),
        }
    }
    
    /// Generate the entire Arceon universe with 13 planets across multiple solar systems and galaxies
    pub fn generate_universe(&mut self) -> Result<()> {
        // Create the primary galaxy (Ethereal Spiral)
        let mut primary_galaxy = Galaxy::new("Ethereal Spiral".to_string(), GalaxyType::Spiral);
        primary_galaxy.description = "The primary galaxy containing most of the inhabited worlds, including Espan. A massive spiral galaxy with billions of stars and countless worlds.".to_string();
        
        // Create the distant galaxy (Void's Edge)
        let mut distant_galaxy = Galaxy::new("Void's Edge".to_string(), GalaxyType::Elliptical);
        distant_galaxy.description = "A distant elliptical galaxy containing the mysterious planet Calypso. Its isolation makes communication extremely difficult.".to_string();
        
        let primary_galaxy_id = primary_galaxy.id;
        let distant_galaxy_id = distant_galaxy.id;
        
        // Solar System 1: Aethon System (contains 4 planets including Espan)
        let mut aethon_system = SolarSystem::new("Aethon System".to_string(), primary_galaxy_id, StarType::MainSequence);
        aethon_system.description = "A stable solar system with a golden sun and four diverse worlds. Home to Espan and three other inhabited planets.".to_string();
        aethon_system.star_mass = 1.1;
        let aethon_system_id = aethon_system.id;
        
        // Solar System 2: Nyx Binary System (contains 2 planets)
        let mut nyx_system = SolarSystem::new("Nyx Binary System".to_string(), primary_galaxy_id, StarType::BinarySystem);
        nyx_system.description = "A unique binary star system where two stars dance around each other, creating exotic day-night cycles on its planets.".to_string();
        nyx_system.star_mass = 1.5;
        let nyx_system_id = nyx_system.id;
        
        // Scattered systems for 4 isolated planets
        let mut isolated_systems = Vec::new();
        for i in 0..4 {
            let mut system = SolarSystem::new(
                format!("Solitary System {}", i + 1), 
                primary_galaxy_id, 
                StarType::MainSequence
            );
            system.description = format!("An isolated solar system far from other inhabited worlds, containing a single planet with unique characteristics.");
            isolated_systems.push(system);
        }
        
        // Distant galaxy system for Calypso
        let mut calypso_system = SolarSystem::new("Serenity System".to_string(), distant_galaxy_id, StarType::MainSequence);
        calypso_system.description = "The sole known system in the distant Void's Edge galaxy, containing the enigmatic planet Calypso.".to_string();
        let calypso_system_id = calypso_system.id;
        
        // Generate the 13 planets
        self.planets = self.generate_all_planets(
            aethon_system_id,
            nyx_system_id,
            &isolated_systems,
            calypso_system_id,
        )?;
        
        // Add planets to their respective systems
        aethon_system.planets = self.planets.iter()
            .filter(|p| p.solar_system_id == aethon_system_id)
            .map(|p| p.id)
            .collect();
            
        nyx_system.planets = self.planets.iter()
            .filter(|p| p.solar_system_id == nyx_system_id)
            .map(|p| p.id)
            .collect();
            
        calypso_system.planets = self.planets.iter()
            .filter(|p| p.solar_system_id == calypso_system_id)
            .map(|p| p.id)
            .collect();
            
        for (i, system) in isolated_systems.iter_mut().enumerate() {
            system.planets = self.planets.iter()
                .filter(|p| p.name == format!("Isolated World {}", i + 1))
                .map(|p| p.id)
                .collect();
        }
        
        // Store all systems
        self.solar_systems.push(aethon_system);
        self.solar_systems.push(nyx_system);
        self.solar_systems.extend(isolated_systems);
        self.solar_systems.push(calypso_system);
        
        // Update galaxies with their systems
        primary_galaxy.solar_systems = self.solar_systems.iter()
            .filter(|s| s.galaxy_id == primary_galaxy_id)
            .map(|s| s.id)
            .collect();
            
        distant_galaxy.solar_systems = self.solar_systems.iter()
            .filter(|s| s.galaxy_id == distant_galaxy_id)
            .map(|s| s.id)
            .collect();
        
        self.galaxies.push(primary_galaxy);
        self.galaxies.push(distant_galaxy);
        
        // Generate the comprehensive race database
        self.race_database.generate_complete_database()
            .map_err(|e| anyhow::anyhow!("Failed to generate race database: {}", e))?;
        
        Ok(())
    }
    
    /// Generate all 13 planets with their specified distribution
    fn generate_all_planets(
        &mut self,
        aethon_system_id: Uuid,
        nyx_system_id: Uuid,
        isolated_systems: &[SolarSystem],
        calypso_system_id: Uuid,
    ) -> Result<Vec<Planet>> {
        let mut planets = Vec::new();
        
        // 4 planets in Aethon System (including Espan)
        planets.push(self.create_espan(aethon_system_id)?);
        planets.push(self.create_planet("Verdania".to_string(), aethon_system_id, PlanetType::Forest, HabitabilityType::HighlyHabitable, vec![Race::Elf, Race::Halfling])?);
        planets.push(self.create_planet("Pyros".to_string(), aethon_system_id, PlanetType::Volcanic, HabitabilityType::Harsh, vec![Race::Dragonborn, Race::Dwarf])?);
        planets.push(self.create_planet("Aquatica".to_string(), aethon_system_id, PlanetType::Ocean, HabitabilityType::EarthLike, vec![Race::Human])?);
        
        // 2 planets in Nyx Binary System
        planets.push(self.create_planet("Lumina".to_string(), nyx_system_id, PlanetType::Crystal, HabitabilityType::Exotic, vec![Race::Gnome])?);
        planets.push(self.create_planet("Umbra".to_string(), nyx_system_id, PlanetType::Underground, HabitabilityType::Exotic, vec![Race::Orc, Race::Tiefling])?);
        
        // 4 isolated planets in separate systems
        for (i, system) in isolated_systems.iter().enumerate() {
            let planet_name = match i {
                0 => "Glacialis",
                1 => "Tempest",
                2 => "Seraphim",
                3 => "Nexus",
                _ => "Unknown",
            };
            
            let (planet_type, habitability, races) = match i {
                0 => (PlanetType::Ice, HabitabilityType::Harsh, vec![Race::Human]),
                1 => (PlanetType::Desert, HabitabilityType::EarthLike, vec![Race::Human]),
                2 => (PlanetType::Floating, HabitabilityType::Exotic, vec![Race::Human]),
                3 => (PlanetType::Terrestrial, HabitabilityType::HighlyHabitable, vec![Race::Human]),
                _ => (PlanetType::Terrestrial, HabitabilityType::EarthLike, vec![Race::Human]),
            };
            
            planets.push(self.create_planet(planet_name.to_string(), system.id, planet_type, habitability, races)?);
        }
        
        // The 13th planet: Calypso in distant galaxy
        planets.push(self.create_calypso(calypso_system_id)?);
        
        Ok(planets)
    }
    
    /// Create Espan as the primary starting planet
    fn create_espan(&mut self, solar_system_id: Uuid) -> Result<Planet> {
        let mut espan = Planet::new(
            "Espan".to_string(),
            solar_system_id,
            PlanetType::Terrestrial,
            HabitabilityType::HighlyHabitable,
        );
        
        espan.description = "A diverse world of kingdoms, forests, mountains, and mystical places. The birthplace of many races and the heart of civilization in the Aethon System. Medieval fantasy setting where magic and mystery intertwine with daily life.".to_string();
        espan.dominant_species = vec![Race::Human, Race::Elf, Race::Dwarf, Race::Gnome, Race::Halfling, Race::Orc, Race::Dragonborn, Race::Tiefling];
        espan.orbital_distance = 1.0;
        espan.gravity_multiplier = 1.0;
        espan.atmosphere_composition = AtmosphereType::Breathable;
        espan.technology_level = TechnologyLevel::Medieval;
        
        espan.surface_conditions = SurfaceConditions {
            temperature_range: (-10, 35),
            weather_patterns: vec![
                WeatherCondition::Clear, 
                WeatherCondition::Cloudy, 
                WeatherCondition::Rainy, 
                WeatherCondition::Snowy
            ],
            natural_hazards: vec!["Seasonal storms".to_string(), "Mountain avalanches".to_string()],
            magical_phenomena: vec!["Ley line convergences".to_string(), "Mystical auroras".to_string()],
        };
        
        // Generate areas for Espan using existing system
        let areas = self.generate_espan()?;
        espan.areas = areas.iter().map(|a| a.id).collect();
        
        Ok(espan)
    }
    
    /// Create Calypso as the distant 13th planet
    fn create_calypso(&mut self, solar_system_id: Uuid) -> Result<Planet> {
        let mut calypso = Planet::new(
            "Calypso".to_string(),
            solar_system_id,
            PlanetType::Floating,
            HabitabilityType::Exotic,
        );
        
        calypso.description = "A mysterious world of floating islands and sky cities, shrouded in perpetual twilight. Located in the distant Void's Edge galaxy, Calypso remains largely unexplored and cut off from all other worlds. Strange gravitational anomalies keep landmasses suspended in the atmosphere.".to_string();
        calypso.dominant_species = vec![Race::Human]; // Unique human subspecies adapted to floating world
        calypso.orbital_distance = 1.2;
        calypso.gravity_multiplier = 0.7;
        calypso.atmosphere_composition = AtmosphereType::Magical;
        calypso.technology_level = TechnologyLevel::Hybrid;
        
        calypso.surface_conditions = SurfaceConditions {
            temperature_range: (5, 25),
            weather_patterns: vec![WeatherCondition::Foggy, WeatherCondition::Windy],
            natural_hazards: vec!["Gravitational storms".to_string(), "Island drift".to_string()],
            magical_phenomena: vec!["Levitation fields".to_string(), "Temporal distortions".to_string()],
        };
        
        Ok(calypso)
    }
    
    /// Create a generic planet with specified characteristics
    fn create_planet(
        &mut self,
        name: String,
        solar_system_id: Uuid,
        planet_type: PlanetType,
        habitability: HabitabilityType,
        dominant_species: Vec<Race>,
    ) -> Result<Planet> {
        let mut planet = Planet::new(name.clone(), solar_system_id, planet_type.clone(), habitability);
        
        planet.dominant_species = dominant_species;
        planet.description = match planet_type {
            PlanetType::Forest => format!("{} is a world covered in vast ancient forests, where nature reigns supreme and magical creatures dwell in harmony with the dominant races.", name),
            PlanetType::Volcanic => format!("{} is a world of active volcanoes and molten landscapes, where hardy races have adapted to the harsh, fiery environment.", name),
            PlanetType::Ocean => format!("{} is primarily covered by vast oceans with scattered islands, where maritime civilizations have flourished.", name),
            PlanetType::Crystal => format!("{} is a world where massive crystal formations create a landscape of perpetual twilight and magical resonance.", name),
            PlanetType::Underground => format!("{} appears barren on the surface, but vast underground civilizations thrive in its extensive cave systems.", name),
            PlanetType::Ice => format!("{} is a frozen world where ice and snow dominate, requiring great resilience from its inhabitants.", name),
            PlanetType::Desert => format!("{} is a world of endless deserts and oasis settlements, where water is precious and survival requires cunning.", name),
            PlanetType::Floating => format!("{} defies conventional physics with floating landmasses and sky-bound civilizations.", name),
            _ => format!("{} is a diverse world with its own unique characteristics and challenges.", name),
        };
        
        // Set appropriate technology levels based on planet type and species
        planet.technology_level = match planet_type {
            PlanetType::Crystal | PlanetType::Floating => TechnologyLevel::Magical,
            PlanetType::Underground => TechnologyLevel::Industrial,
            _ => TechnologyLevel::Medieval,
        };
        
        Ok(planet)
    }
    
    /// Generate the world of Espan with race-specific starting areas
    pub fn generate_espan(&mut self) -> Result<Vec<Area>> {
        let mut areas = Vec::new();
        
        // Generate Human capitals
        let human_capital_1 = self.area_generator.generate_capital(
            Race::Human, 
            "Alderheart".to_string(),
            CultureType::TradingHub
        )?;
        
        let human_capital_2 = self.area_generator.generate_capital(
            Race::Human,
            "Goldenvale".to_string(), 
            CultureType::Traditional
        )?;
        
        // Generate Elven forest capital
        let elf_capital = self.area_generator.generate_capital(
            Race::Elf,
            "Silverleaf Enclave".to_string(),
            CultureType::Scholarly
        )?;
        
        // Generate Gnome underground capital
        let gnome_capital = self.area_generator.generate_capital(
            Race::Gnome,
            "Deepholm".to_string(),
            CultureType::Artisan
        )?;
        
        // Generate Dwarf mountain capital
        let dwarf_capital = self.area_generator.generate_capital(
            Race::Dwarf,
            "Ironforge".to_string(),
            CultureType::Military
        )?;
        
        // Generate other race capitals
        let halfling_capital = self.area_generator.generate_capital(
            Race::Halfling,
            "Greenhill".to_string(),
            CultureType::Traditional
        )?;
        
        let orc_capital = self.area_generator.generate_capital(
            Race::Orc,
            "Bloodrock Stronghold".to_string(),
            CultureType::Military
        )?;
        
        areas.extend([
            human_capital_1,
            human_capital_2,
            elf_capital,
            gnome_capital,
            dwarf_capital,
            halfling_capital,
            orc_capital,
        ]);
        
        // Generate wilderness and special areas
        let wilderness_areas = self.generate_wilderness_areas()?;
        areas.extend(wilderness_areas);
        
        // Generate trading posts and settlements
        let settlements = self.generate_settlements()?;
        areas.extend(settlements);
        
        // Generate magical and special locations
        let special_areas = self.generate_special_areas()?;
        areas.extend(special_areas);
        
        // Add connections between all areas
        self.area_generator.connect_areas(&mut areas)?;
        
        Ok(areas)
    }
    
    /// Generate wilderness areas between capitals
    fn generate_wilderness_areas(&mut self) -> Result<Vec<Area>> {
        let mut areas = Vec::new();
        
        // Ancient Forest
        let mut ancient_forest = self.area_generator.generate_wilderness(
            "Ancient Silverleaf Forest".to_string(),
            AreaType::Forest,
            AreaSize::Large,
            5, // Moderate danger
        )?;
        ancient_forest.description = "A vast ancient forest where elven magic still lingers in the air. Towering silverleaf trees create a canopy so thick that sunlight barely reaches the forest floor. Mysterious paths wind through groves where time seems to move differently.".to_string();
        areas.push(ancient_forest);
        
        // Mountain Range
        let mut dragon_peaks = self.area_generator.generate_wilderness(
            "Dragon Spine Mountains".to_string(),
            AreaType::Mountains,
            AreaSize::Massive,
            7, // High danger
        )?;
        dragon_peaks.description = "Jagged peaks that pierce the clouds, said to be the ancient resting place of dragons. Treacherous paths wind between snow-capped summits, and deep caverns echo with mysterious sounds. Only the bravest adventurers dare traverse these heights.".to_string();
        areas.push(dragon_peaks);
        
        // Desert
        let mut crystal_desert = self.area_generator.generate_wilderness(
            "Shifting Crystal Desert".to_string(),
            AreaType::Desert,
            AreaSize::Large,
            6, // High danger due to harsh conditions
        )?;
        crystal_desert.description = "An endless expanse of golden sand dotted with mysterious crystal formations that hum with magical energy. Mirages dance on the horizon, and ancient ruins peek through the dunes, holding secrets of civilizations long past on the planet Espan.".to_string();
        areas.push(crystal_desert);
        
        // Swampland
        let mut misty_swamp = self.area_generator.generate_wilderness(
            "Misty Marshlands".to_string(),
            AreaType::Swamp,
            AreaSize::Medium,
            4, // Moderate danger
        )?;
        misty_swamp.description = "A fog-shrouded wetland where ancient willows trail their branches in dark waters. Strange lights flicker through the mist, and the air is thick with the sound of croaking toads and splashing creatures unseen.".to_string();
        areas.push(misty_swamp);
        
        Ok(areas)
    }
    
    /// Generate trading posts and small settlements
    fn generate_settlements(&mut self) -> Result<Vec<Area>> {
        let mut areas = Vec::new();
        
        // Crossroads Trading Post
        let mut crossroads = self.area_generator.generate_settlement(
            "Crossroads Trading Post".to_string(),
            CultureType::TradingHub,
            2, // Low danger - well protected
        )?;
        crossroads.description = "A bustling trading post where merchants from all corners of Espan gather to exchange goods and stories. Well-maintained roads converge here, and the sound of hammering from the smithy mingles with the calls of traders hawking their wares.".to_string();
        areas.push(crossroads);
        
        // Coastal Fishing Village
        let mut fishing_village = self.area_generator.generate_settlement(
            "Seabreeze Harbor".to_string(),
            CultureType::Traditional,
            3, // Low-moderate danger
        )?;
        fishing_village.description = "A peaceful fishing village where weathered boats bob in the harbor and seabirds cry overhead. The smell of salt air and fish drying in the sun permeates everything, while fishermen mend their nets and share tales of the sea.".to_string();
        areas.push(fishing_village);
        
        // Mining Outpost
        let mut mining_outpost = self.area_generator.generate_settlement(
            "Ironvein Mining Outpost".to_string(),
            CultureType::Artisan,
            4, // Moderate danger due to mining hazards
        )?;
        mining_outpost.description = "A hardy mining settlement carved into the mountainside, where the ring of pickaxes echoes through stone tunnels. Ore carts rumble along tracks, and the forges burn day and night, processing the precious metals extracted from the deep earth.".to_string();
        areas.push(mining_outpost);
        
        Ok(areas)
    }
    
    /// Generate magical and special locations
    fn generate_special_areas(&mut self) -> Result<Vec<Area>> {
        let mut areas = Vec::new();
        
        // Magical Academy
        let mut academy = self.area_generator.generate_special_area(
            "Arcanum Academy of Mystical Arts".to_string(),
            AreaType::Magical,
            CultureType::Scholarly,
            1, // Very safe
        )?;
        academy.description = "A prestigious academy where mages and scholars study the arcane arts. Floating crystals provide light to the libraries, and practice yards echo with the sound of spells being cast. Ancient towers spiral into the sky, their tops wreathed in perpetual starlight.".to_string();
        areas.push(academy);
        
        // Underground Dungeon
        let mut ancient_ruins = self.area_generator.generate_special_area(
            "Lost Ruins of Vel'Andros".to_string(),
            AreaType::Underground,
            CultureType::Ancient,
            8, // Very dangerous
        )?;
        ancient_ruins.description = "The crumbling remains of an ancient civilization lie hidden beneath the earth. Strange runes glow with eldritch power, and the air hums with residual magic. Adventurers speak in hushed tones of the treasures and terrors that await in the deep chambers.".to_string();
        areas.push(ancient_ruins);
        
        // Floating Island
        let mut floating_island = self.area_generator.generate_special_area(
            "Skyhold Citadel".to_string(),
            AreaType::Magical,
            CultureType::Mystical,
            6, // High danger due to access difficulty
        )?;
        floating_island.description = "A massive fortress that floats high above the clouds, held aloft by ancient magical forces. Wind rushes through the crystal spires, and only those who can fly or master teleportation magic can reach this celestial stronghold.".to_string();
        areas.push(floating_island);
        
        Ok(areas)
    }
    
    /// Get the starting area for a specific race
    pub fn get_starting_area_for_race<'a>(&self, race: &Race, areas: &'a [Area]) -> Option<&'a Area> {
        areas.iter().find(|area| {
            if let Some(affinity) = &area.race_affinity {
                affinity.primary_race == *race
            } else {
                false
            }
        })
    }
    
    /// Generate a starting area for a specific race
    pub fn generate_starting_area(&mut self, race: &Race) -> Result<Area> {
        match race {
            Race::Human => self.area_generator.generate_capital(
                Race::Human,
                "Alderheart".to_string(),
                CultureType::TradingHub
            ),
            Race::Elf => self.area_generator.generate_capital(
                Race::Elf,
                "Silverleaf Enclave".to_string(),
                CultureType::Scholarly
            ),
            Race::Gnome => self.area_generator.generate_capital(
                Race::Gnome,
                "Deepholm".to_string(),
                CultureType::Artisan
            ),
            Race::Dwarf => self.area_generator.generate_capital(
                Race::Dwarf,
                "Ironforge".to_string(),
                CultureType::Military
            ),
            Race::Halfling => self.area_generator.generate_capital(
                Race::Halfling,
                "Greenhill".to_string(),
                CultureType::Traditional
            ),
            Race::Orc => self.area_generator.generate_capital(
                Race::Orc,
                "Bloodrock Stronghold".to_string(),
                CultureType::Military
            ),
            Race::Dragonborn => self.area_generator.generate_capital(
                Race::Dragonborn,
                "Drakmoor Citadel".to_string(),
                CultureType::Military
            ),
            Race::Tiefling => self.area_generator.generate_capital(
                Race::Tiefling,
                "Shadowhaven".to_string(),
                CultureType::TradingHub
            ),
            _ => self.area_generator.generate_capital(
                race.clone(),
                "Generic Settlement".to_string(),
                CultureType::Traditional
            ),
        }
    }
}
