// Remaining race creation methods for Seraphim, Nexus, Calypso, and Shared races

use super::*;
use uuid::Uuid;
use std::collections::HashMap;

impl RaceDatabase {
    // Seraphim (floating world) race creation methods
    pub fn create_sky_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Sky Human {}", i + 1), "Cloud-dwelling humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Sky-touched", "Wind-kissed"], vec!["Cloud white", "Sky blue"], 
                vec!["Sky blue".to_string(), "Cloud white".to_string()], 18, 110, 160, AgingRate::Normal,
                vec![CulturalTrait::Exploratory, CulturalTrait::Free], SpiritualAlignment::Freedom, "Wind Walking".to_string(), vec!["Cloud Step".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::PressureAdaptation], vec![ElementalAffinity::Air], vec![MagicalInclination::Elemental(ElementalAffinity::Air), MagicalInclination::Divination],
                SocialStructure::Democracy, vec![Environment::Floating], ArchitecturalStyle::Floating, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_wind_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Wind Elf {}", i + 1), "Air-dwelling elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Ethereal, vec!["Wind-touched", "Sky-blue"], vec!["Wind colors"], 
                vec!["Sky blue".to_string(), "Silver".to_string()], 100, 900, 1400, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Free], SpiritualAlignment::Wind, "Wind Mastery".to_string(), vec!["Storm Calling".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::EnhancedSenses("Air Currents".to_string())], vec![ElementalAffinity::Air, ElementalAffinity::Lightning], vec![MagicalInclination::Elemental(ElementalAffinity::Air), MagicalInclination::Evocation],
                SocialStructure::Council, vec![Environment::Floating], ArchitecturalStyle::Flowing, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_cloud_giant_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Cloud Giant {}", i + 1), "Sky-dwelling giants", RaceCategory::AirTouched, planet, None,
                (3.0, 4.0), BodyBuild::Ethereal, vec!["Cloud-like", "Misty"], vec!["Cloud colors"], 
                vec!["Cloud white".to_string(), "Storm gray".to_string()], 100, 600, 800, AgingRate::Slow,
                vec![CulturalTrait::Wise, CulturalTrait::Protective], SpiritualAlignment::Sky, "Cloud Form".to_string(), vec!["Weather Control".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::ShapeMinorChanges], vec![ElementalAffinity::Air, ElementalAffinity::Water], vec![MagicalInclination::Elemental(ElementalAffinity::Air), MagicalInclination::Transmutation],
                SocialStructure::Hierarchy, vec![Environment::Floating], ArchitecturalStyle::Floating, vec!["Giant".to_string()]
            )
        }).collect()
    }

    pub fn create_storm_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Storm Kin {}", i + 1), "Lightning-touched people", RaceCategory::AirTouched, planet, None,
                (1.8, 2.1), BodyBuild::Athletic, vec!["Electric-charged", "Storm-marked"], vec!["Lightning colors"], 
                vec!["Electric blue".to_string(), "Storm gray".to_string()], 25, 120, 200, AgingRate::Normal,
                vec![CulturalTrait::Warrior, CulturalTrait::Independent], SpiritualAlignment::Storm, "Lightning Body".to_string(), vec!["Electric Touch".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::NaturalWeapons("Lightning".to_string())], vec![ElementalAffinity::Lightning, ElementalAffinity::Air], vec![MagicalInclination::Evocation, MagicalInclination::Elemental(ElementalAffinity::Lightning)],
                SocialStructure::Tribal, vec![Environment::Floating], ArchitecturalStyle::Towering, vec!["Storm Speech".to_string()]
            )
        }).collect()
    }

    pub fn create_air_sprite_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Air Sprite {}", i + 1), "Wind spirits", RaceCategory::Sylvan, planet, None,
                (0.2, 0.4), BodyBuild::Ethereal, vec!["Translucent", "Wind-formed"], vec!["Air colors"], 
                vec!["Translucent".to_string()], 5, 500, 800, AgingRate::Eternal,
                vec![CulturalTrait::Playful, CulturalTrait::Free], SpiritualAlignment::Wind, "Air Form".to_string(), vec!["Wind Control".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::ShapeMinorChanges], vec![ElementalAffinity::Air], vec![MagicalInclination::Illusion, MagicalInclination::Elemental(ElementalAffinity::Air)],
                SocialStructure::Collective, vec![Environment::Floating], ArchitecturalStyle::Magical, vec!["Wind".to_string()]
            )
        }).collect()
    }

    // Nexus (terrestrial world) race creation methods
    pub fn create_plains_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Plains Human {}", i + 1), "Grassland-dwelling humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Earth-toned", "Grass-touched"], vec!["Earth colors"], 
                vec!["Brown".to_string(), "Green".to_string()], 18, 85, 125, AgingRate::Normal,
                vec![CulturalTrait::Pragmatic, CulturalTrait::Communal], SpiritualAlignment::Balance, "Earth Connection".to_string(), vec!["Plant Growth".to_string()],
                vec![NaturalAbility::EnhancedSenses("Nature".to_string()), NaturalAbility::NaturalHealing], vec![ElementalAffinity::Earth, ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Democracy, vec![Environment::Plains], ArchitecturalStyle::Stone, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_hill_dwarf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Hill Dwarf {}", i + 1), "Rolling hill dwarfs", RaceCategory::Dwarven, planet, Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust, vec!["Hill-brown", "Earth-toned"], vec!["Earth colors"], 
                vec!["Brown".to_string(), "Green".to_string()], 120, 350, 550, AgingRate::Slow,
                vec![CulturalTrait::Artisan, CulturalTrait::Peaceful], SpiritualAlignment::Stability, "Hill Craft".to_string(), vec!["Stone Shaping".to_string()],
                vec![NaturalAbility::NaturalArmor, NaturalAbility::EnhancedSenses("Earth".to_string())], vec![ElementalAffinity::Earth], vec![MagicalInclination::Transmutation, MagicalInclination::Enchantment],
                SocialStructure::Clan, vec![Environment::Hills], ArchitecturalStyle::Stone, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }

    pub fn create_meadow_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Meadow Elf {}", i + 1), "Peaceful grassland elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Graceful, vec!["Meadow-green", "Flower-touched"], vec!["Meadow colors"], 
                vec!["Green".to_string(), "Gold".to_string()], 100, 750, 1100, AgingRate::Slow,
                vec![CulturalTrait::Peaceful, CulturalTrait::Harmonious], SpiritualAlignment::Serenity, "Meadow Harmony".to_string(), vec!["Flower Speech".to_string()],
                vec![NaturalAbility::LifeDetection, NaturalAbility::NaturalHealing], vec![ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Council, vec![Environment::Plains], ArchitecturalStyle::Organic, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_earth_giant_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Earth Giant {}", i + 1), "Stone-dwelling giants", RaceCategory::EarthTouched, planet, None,
                (2.8, 3.5), BodyBuild::Powerful, vec!["Stone-like", "Earth-toned"], vec!["Rock colors"], 
                vec!["Stone gray".to_string(), "Earth brown".to_string()], 90, 500, 700, AgingRate::Slow,
                vec![CulturalTrait::Protective, CulturalTrait::Wise], SpiritualAlignment::Stability, "Stone Body".to_string(), vec!["Earth Shaping".to_string()],
                vec![NaturalAbility::NaturalArmor, NaturalAbility::NaturalWeapons("Stone Fists".to_string())], vec![ElementalAffinity::Earth], vec![MagicalInclination::Transmutation, MagicalInclination::Elemental(ElementalAffinity::Earth)],
                SocialStructure::Hierarchy, vec![Environment::Mountains], ArchitecturalStyle::Stone, vec!["Giant".to_string()]
            )
        }).collect()
    }

    pub fn create_terra_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Terra Kin {}", i + 1), "Earth-bonded folk", RaceCategory::EarthTouched, planet, None,
                (1.7, 2.0), BodyBuild::Robust, vec!["Earth-veined", "Stone-touched"], vec!["Earth tones"], 
                vec!["Earth brown".to_string(), "Stone gray".to_string()], 40, 250, 400, AgingRate::Slow,
                vec![CulturalTrait::Stable, CulturalTrait::Protective], SpiritualAlignment::Grounding, "Earth Bond".to_string(), vec!["Seismic Sense".to_string()],
                vec![NaturalAbility::DangerSense, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Earth], vec![MagicalInclination::Warding, MagicalInclination::Elemental(ElementalAffinity::Earth)],
                SocialStructure::Collective, vec![Environment::Plains, Environment::Hills], ArchitecturalStyle::Stone, vec!["Earth Speech".to_string()]
            )
        }).collect()
    }

    // Calypso (mysterious floating world) race creation methods
    pub fn create_void_touched_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Void Touched {}", i + 1), "Space-touched beings", RaceCategory::VoidTouched, planet, None,
                (1.8, 2.2), BodyBuild::Ethereal, vec!["Star-speckled", "Void-black"], vec!["Cosmic colors"], 
                vec!["Void black".to_string(), "Star silver".to_string()], 60, 400, 800, AgingRate::Eternal,
                vec![CulturalTrait::Mystical, CulturalTrait::Detached], SpiritualAlignment::Void, "Void Walk".to_string(), vec!["Space Sense".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::TrueSeeing], vec![ElementalAffinity::Space, ElementalAffinity::Time], vec![MagicalInclination::Dimensional, MagicalInclination::Divination],
                SocialStructure::Independent, vec![Environment::Floating], ArchitecturalStyle::Geometric, vec!["Void".to_string()]
            )
        }).collect()
    }

    pub fn create_star_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Star Elf {}", i + 1), "Cosmic elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Ethereal, vec!["Starlight", "Cosmic"], vec!["Star colors"], 
                vec!["Star silver".to_string(), "Cosmic blue".to_string()], 100, 1200, 2000, AgingRate::Eternal,
                vec![CulturalTrait::Mystical, CulturalTrait::Wise], SpiritualAlignment::Cosmos, "Star Light".to_string(), vec!["Cosmic Vision".to_string()],
                vec![NaturalAbility::TrueSeeing, NaturalAbility::MagicDetection], vec![ElementalAffinity::Light, ElementalAffinity::Space], vec![MagicalInclination::Divination, MagicalInclination::Dimensional],
                SocialStructure::Council, vec![Environment::Floating], ArchitecturalStyle::Crystal, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_gravity_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Gravity Human {}", i + 1), "Weightless-adapted humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.8, 2.0), BodyBuild::Athletic, vec!["Weightless-pale", "Gravity-touched"], vec!["Space colors"], 
                vec!["Pale".to_string(), "Silver".to_string()], 18, 120, 180, AgingRate::Normal,
                vec![CulturalTrait::Adaptive, CulturalTrait::Exploratory], SpiritualAlignment::Freedom, "Gravity Control".to_string(), vec!["Weightless Movement".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::PressureAdaptation], vec![ElementalAffinity::Gravity], vec![MagicalInclination::Transmutation],
                SocialStructure::Democracy, vec![Environment::Floating], ArchitecturalStyle::Floating, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_cosmic_giant_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Cosmic Giant {}", i + 1), "Space-dwelling giants", RaceCategory::Starborn, planet, None,
                (4.0, 6.0), BodyBuild::Ethereal, vec!["Galaxy-pattern", "Nebula-swirled"], vec!["Cosmic patterns"], 
                vec!["Deep space".to_string(), "Nebula colors".to_string()], 200, 1000, 2000, AgingRate::Eternal,
                vec![CulturalTrait::Ancient, CulturalTrait::Wise], SpiritualAlignment::Universe, "Cosmic Awareness".to_string(), vec!["Galaxy Sight".to_string()],
                vec![NaturalAbility::TrueSeeing, NaturalAbility::NaturalFlight], vec![ElementalAffinity::Space, ElementalAffinity::Time], vec![MagicalInclination::Dimensional, MagicalInclination::Temporal],
                SocialStructure::Ancient, vec![Environment::Floating], ArchitecturalStyle::Cosmic, vec!["Cosmic".to_string()]
            )
        }).collect()
    }

    pub fn create_nebula_sprite_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Nebula Sprite {}", i + 1), "Cosmic gas spirits", RaceCategory::Starborn, planet, None,
                (0.3, 0.6), BodyBuild::Ethereal, vec!["Gas-like", "Nebula-colored"], vec!["Nebula colors"], 
                vec!["Nebula".to_string()], 10, 800, 1500, AgingRate::Eternal,
                vec![CulturalTrait::Mystical, CulturalTrait::Playful], SpiritualAlignment::Creation, "Nebula Form".to_string(), vec!["Gas Control".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::ShapeMinorChanges], vec![ElementalAffinity::Space, ElementalAffinity::Light], vec![MagicalInclination::Illusion, MagicalInclination::Transmutation],
                SocialStructure::Collective, vec![Environment::Floating], ArchitecturalStyle::Cosmic, vec!["Cosmic".to_string()]
            )
        }).collect()
    }

    // Shared races (inter-dimensional travelers)
    pub fn create_planar_human_variants(&self, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Planar Human {}", i + 1), "Dimension-hopping humans", RaceCategory::PlanarHybrid, "Shared", None,
                (1.6, 1.8), BodyBuild::Athletic, vec!["Reality-touched", "Planar-marked"], vec!["All colors"], 
                vec!["Shifting".to_string()], 18, 150, 300, AgingRate::Normal,
                vec![CulturalTrait::Exploratory, CulturalTrait::Adaptive], SpiritualAlignment::Unity, "Planar Sight".to_string(), vec!["Dimension Walk".to_string()],
                vec![NaturalAbility::MagicDetection, NaturalAbility::DangerSense], vec![ElementalAffinity::Space], vec![MagicalInclination::Dimensional],
                SocialStructure::Federation, vec![Environment::Urban], ArchitecturalStyle::Harmonious, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_cosmic_elf_variants(&self, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Cosmic Elf {}", i + 1), "Universe-traveling elves", RaceCategory::Elven, "Shared", Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Ethereal, vec!["Universe-touched", "Star-blessed"], vec!["Cosmic colors"], 
                vec!["All colors".to_string()], 100, 1500, 3000, AgingRate::Eternal,
                vec![CulturalTrait::Ancient, CulturalTrait::Wise], SpiritualAlignment::Transcendence, "Universal Knowledge".to_string(), vec!["Reality Navigation".to_string()],
                vec![NaturalAbility::TrueSeeing, NaturalAbility::MagicDetection], vec![ElementalAffinity::Space, ElementalAffinity::Time], vec![MagicalInclination::Dimensional, MagicalInclination::Temporal],
                SocialStructure::Council, vec![Environment::Urban], ArchitecturalStyle::Crystal, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_void_walker_variants(&self, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Void Walker {}", i + 1), "Inter-dimensional beings", RaceCategory::VoidTouched, "Shared", None,
                (1.9, 2.3), BodyBuild::Ethereal, vec!["Void-touched", "Reality-fluid"], vec!["Void colors"], 
                vec!["Void".to_string(), "Reality shift".to_string()], 80, 800, 1500, AgingRate::Eternal,
                vec![CulturalTrait::Detached, CulturalTrait::Mystical], SpiritualAlignment::Void, "Void Step".to_string(), vec!["Reality Phase".to_string()],
                vec![NaturalAbility::ShapeMinorChanges, NaturalAbility::TrueSeeing], vec![ElementalAffinity::Space, ElementalAffinity::Shadow], vec![MagicalInclination::Dimensional, MagicalInclination::Illusion],
                SocialStructure::Independent, vec![Environment::Urban], ArchitecturalStyle::Dimensional, vec!["Void".to_string()]
            )
        }).collect()
    }

    pub fn create_star_dwarf_variants(&self, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Star Dwarf {}", i + 1), "Space-faring dwarfs", RaceCategory::Dwarven, "Shared", Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust, vec!["Star-forged", "Cosmic-touched"], vec!["Metal colors"], 
                vec!["Star silver".to_string(), "Cosmic gold".to_string()], 120, 600, 900, AgingRate::Slow,
                vec![CulturalTrait::Explorer, CulturalTrait::Artisan], SpiritualAlignment::Discovery, "Star Forge".to_string(), vec!["Cosmic Crafting".to_string()],
                vec![NaturalAbility::NaturalArmor, NaturalAbility::MagicDetection], vec![ElementalAffinity::Metal, ElementalAffinity::Space], vec![MagicalInclination::Enchantment, MagicalInclination::Transmutation],
                SocialStructure::Federation, vec![Environment::Urban], ArchitecturalStyle::Geometric, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }

    pub fn create_portal_mage_variants(&self, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Portal Mage {}", i + 1), "Teleportation specialists", RaceCategory::HumanHybrid, "Shared", None,
                (1.6, 1.8), BodyBuild::Athletic, vec!["Portal-marked", "Magic-touched"], vec!["Magic colors"], 
                vec!["Arcane".to_string(), "Portal blue".to_string()], 25, 200, 400, AgingRate::Slow,
                vec![CulturalTrait::Scholarly, CulturalTrait::Helpful], SpiritualAlignment::Connection, "Portal Mastery".to_string(), vec!["Instant Travel".to_string()],
                vec![NaturalAbility::MagicDetection, NaturalAbility::Telepathy], vec![ElementalAffinity::Space], vec![MagicalInclination::Dimensional, MagicalInclination::Conjuration],
                SocialStructure::Meritocracy, vec![Environment::Urban], ArchitecturalStyle::Magical, vec!["Common".to_string()]
            )
        }).collect()
    }
}