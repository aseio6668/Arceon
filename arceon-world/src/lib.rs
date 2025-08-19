pub mod area_generator;
pub mod name_generator;
pub mod area_manager;
pub mod genesis;
pub mod runestone_knowledge;
pub mod command_engine;
pub mod targeting;
pub mod enhanced_commands;
pub mod trading_system;

use anyhow::Result;
use arceon_core::entities::world::{Area, AreaType, AreaSize, CultureType};
use arceon_core::entities::being::Race;
pub use area_generator::AreaGenerator;
pub use area_manager::AreaManager;
pub use genesis::*;
pub use runestone_knowledge::*;
pub use command_engine::*;

/// World generation and management systems
pub struct WorldManager {
    area_generator: AreaGenerator,
}

impl WorldManager {
    pub fn new() -> Self {
        Self {
            area_generator: AreaGenerator::new(),
        }
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
        crystal_desert.description = "An endless expanse of golden sand dotted with mysterious crystal formations that hum with magical energy. Mirages dance on the horizon, and ancient ruins peek through the dunes, holding secrets of civilizations long past.".to_string();
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
