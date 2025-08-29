// Additional race creation methods for different planets
// This file contains the implementation for the missing creation methods referenced in race_database.rs

use super::*;
use uuid::Uuid;
use std::collections::HashMap;

impl RaceDatabase {
    // Pyros (volcanic) planet race creation methods
    pub fn create_phoenix_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Phoenix Kin {}", i + 1), "Phoenix-blooded fire spirits", RaceCategory::PhoenixHybrid, planet, None,
                (1.7, 2.0), BodyBuild::Ethereal, vec!["Flame-touched", "Golden"], vec!["Fire colors", "Phoenix feathers"], 
                vec!["Golden".to_string(), "Fire orange".to_string()], 50, 300, 500, AgingRate::Cyclical,
                vec![CulturalTrait::Mystical, CulturalTrait::Protective], SpiritualAlignment::Renewal, "Phoenix Rebirth".to_string(), vec!["Fire Immunity".to_string(), "Resurrection".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalFlight], vec![ElementalAffinity::Fire, ElementalAffinity::Light], vec![MagicalInclination::Healing, MagicalInclination::Evocation],
                SocialStructure::Hierarchy, vec![Environment::Volcanic], ArchitecturalStyle::Towering, vec!["Phoenix".to_string()]
            )
        }).collect()
    }

    pub fn create_lava_elemental_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Lava Touched {}", i + 1), "Lava-infused humanoids", RaceCategory::FireTouched, planet, None,
                (1.6, 1.9), BodyBuild::Powerful, vec!["Molten rock", "Lava-veined"], vec!["Molten glow"], 
                vec!["Lava red".to_string(), "Molten orange".to_string()], 40, 200, 350, AgingRate::Slow,
                vec![CulturalTrait::Warrior, CulturalTrait::Independent], SpiritualAlignment::Courage, "Molten Heart".to_string(), vec!["Lava Immunity".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Fire, ElementalAffinity::Earth], vec![MagicalInclination::Evocation, MagicalInclination::Transmutation],
                SocialStructure::Tribal, vec![Environment::Volcanic], ArchitecturalStyle::Stone, vec!["Elemental".to_string()]
            )
        }).collect()
    }

    pub fn create_ember_fae_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Ember Sprite {}", i + 1), "Tiny fire spirits", RaceCategory::Sylvan, planet, None,
                (0.2, 0.4), BodyBuild::Ethereal, vec!["Flickering flame", "Ember glow"], vec!["Fire colors"], 
                vec!["Flame".to_string()], 5, 200, 400, AgingRate::Slow,
                vec![CulturalTrait::Playful, CulturalTrait::Mystical], SpiritualAlignment::Joy, "Living Flame".to_string(), vec!["Fire Form".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::TemperatureResistance], vec![ElementalAffinity::Fire], vec![MagicalInclination::Evocation, MagicalInclination::Illusion],
                SocialStructure::Collective, vec![Environment::Volcanic], ArchitecturalStyle::Magical, vec!["Elemental".to_string()]
            )
        }).collect()
    }

    // Aquatica planet race creation methods (additional to existing aquatic variants)
    pub fn create_coral_people_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Coral People {}", i + 1), "Living coral reef humanoids", RaceCategory::Aquatic, planet, None,
                (1.5, 1.8), BodyBuild::Athletic, vec!["Coral textured", "Reef colors"], vec!["Coral patterns"], 
                vec!["Ocean blue".to_string(), "Coral pink".to_string()], 40, 400, 600, AgingRate::Slow,
                vec![CulturalTrait::Communal, CulturalTrait::Protective], SpiritualAlignment::Harmony, "Reef Guardian".to_string(), vec!["Coral Growth".to_string()],
                vec![NaturalAbility::NaturalSwim, NaturalAbility::PressureAdaptation], vec![ElementalAffinity::Water, ElementalAffinity::Nature], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Collective, vec![Environment::Ocean], ArchitecturalStyle::Coral, vec!["Aquatic".to_string()]
            )
        }).collect()
    }

    pub fn create_sea_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Sea Elf {}", i + 1), "Ocean-dwelling elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Graceful, vec!["Sea-touched", "Pearl-like"], vec!["Ocean colors"], 
                vec!["Sea blue".to_string(), "Pearl".to_string()], 100, 800, 1200, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Harmonious], SpiritualAlignment::Flow, "Ocean's Grace".to_string(), vec!["Tidal Magic".to_string()],
                vec![NaturalAbility::NaturalSwim, NaturalAbility::EnhancedSenses("Echolocation".to_string())], vec![ElementalAffinity::Water], vec![MagicalInclination::Elemental(ElementalAffinity::Water), MagicalInclination::Divination],
                SocialStructure::Council, vec![Environment::Ocean], ArchitecturalStyle::Flowing, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_tide_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Tide Human {}", i + 1), "Sea-adapted humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Sea-tanned", "Salt-touched"], vec!["Ocean colors"], 
                vec!["Sea green".to_string(), "Ocean blue".to_string()], 18, 100, 150, AgingRate::Normal,
                vec![CulturalTrait::Pragmatic, CulturalTrait::Exploratory], SpiritualAlignment::Balance, "Tidal Adaptation".to_string(), vec!["Water Breathing".to_string()],
                vec![NaturalAbility::NaturalSwim, NaturalAbility::PressureAdaptation], vec![ElementalAffinity::Water], vec![MagicalInclination::Elemental(ElementalAffinity::Water)],
                SocialStructure::Democracy, vec![Environment::Ocean, Environment::Coast], ArchitecturalStyle::Flowing, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_whale_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Whale Kin {}", i + 1), "Whale-song blessed people", RaceCategory::Aquatic, planet, None,
                (2.0, 2.5), BodyBuild::Robust, vec!["Whale-like", "Deep blue"], vec!["Whale patterns"], 
                vec!["Deep blue".to_string(), "Whale gray".to_string()], 50, 300, 500, AgingRate::Slow,
                vec![CulturalTrait::Wise, CulturalTrait::Peaceful], SpiritualAlignment::Wisdom, "Whale Song".to_string(), vec!["Deep Communication".to_string()],
                vec![NaturalAbility::NaturalSwim, NaturalAbility::PressureAdaptation, NaturalAbility::Telepathy], vec![ElementalAffinity::Water, ElementalAffinity::Sound], vec![MagicalInclination::Divination, MagicalInclination::Telepathic],
                SocialStructure::Council, vec![Environment::Ocean], ArchitecturalStyle::Organic, vec!["Whale Song".to_string()]
            )
        }).collect()
    }

    // Lumina (crystal) planet race creation methods
    pub fn create_crystal_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Crystal Kin {}", i + 1), "Living crystal beings", RaceCategory::CrystalkinHybrid, planet, None,
                (1.8, 2.2), BodyBuild::Ethereal, vec!["Crystalline", "Translucent"], vec!["Crystal colors"], 
                vec!["Prism".to_string(), "Crystal clear".to_string()], 60, 800, 1200, AgingRate::Eternal,
                vec![CulturalTrait::Wise, CulturalTrait::Harmonious], SpiritualAlignment::Clarity, "Crystal Resonance".to_string(), vec!["Light Refraction".to_string()],
                vec![NaturalAbility::MagicDetection, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Crystal, ElementalAffinity::Light], vec![MagicalInclination::Divination, MagicalInclination::Transmutation],
                SocialStructure::Hierarchy, vec![Environment::Crystal], ArchitecturalStyle::Crystal, vec!["Harmonic".to_string()]
            )
        }).collect()
    }

    pub fn create_gem_dwarf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Gem Dwarf {}", i + 1), "Crystal-mining dwarfs", RaceCategory::Dwarven, planet, Some("Dwarf".to_string()),
                (1.2, 1.4), BodyBuild::Robust, vec!["Gem-touched", "Crystalline"], vec!["Gem colors"], 
                vec!["Gem colors".to_string()], 120, 500, 700, AgingRate::Slow,
                vec![CulturalTrait::Artisan, CulturalTrait::Scholarly], SpiritualAlignment::Perfection, "Gem Mastery".to_string(), vec!["Crystal Sight".to_string()],
                vec![NaturalAbility::MagicDetection, NaturalAbility::EnhancedSenses("Sight".to_string())], vec![ElementalAffinity::Crystal, ElementalAffinity::Earth], vec![MagicalInclination::Transmutation, MagicalInclination::Enchantment],
                SocialStructure::Meritocracy, vec![Environment::Underground, Environment::Crystal], ArchitecturalStyle::Crystal, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }

    pub fn create_prism_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Prism Elf {}", i + 1), "Light-refracting elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Ethereal, vec!["Prism-like", "Light-touched"], vec!["Rainbow"], 
                vec!["All colors".to_string()], 100, 900, 1300, AgingRate::Slow,
                vec![CulturalTrait::Artistic, CulturalTrait::Mystical], SpiritualAlignment::Beauty, "Light Mastery".to_string(), vec!["Rainbow Vision".to_string()],
                vec![NaturalAbility::TrueSeeing, NaturalAbility::MagicDetection], vec![ElementalAffinity::Light, ElementalAffinity::Crystal], vec![MagicalInclination::Illusion, MagicalInclination::Divination],
                SocialStructure::Council, vec![Environment::Crystal], ArchitecturalStyle::Crystal, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_resonance_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Resonance Human {}", i + 1), "Sound-crystal attuned humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Crystal-veined", "Harmonic"], vec!["Resonant colors"], 
                vec!["Clear".to_string(), "Harmonic".to_string()], 18, 120, 180, AgingRate::Normal,
                vec![CulturalTrait::Scholarly, CulturalTrait::Harmonious], SpiritualAlignment::Harmony, "Crystal Resonance".to_string(), vec!["Sound Magic".to_string()],
                vec![NaturalAbility::EnhancedSenses("Hearing".to_string()), NaturalAbility::MagicDetection], vec![ElementalAffinity::Sound, ElementalAffinity::Crystal], vec![MagicalInclination::Enchantment, MagicalInclination::Transmutation],
                SocialStructure::Meritocracy, vec![Environment::Crystal], ArchitecturalStyle::Harmonic, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_light_fae_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Light Sprite {}", i + 1), "Radiant light spirits", RaceCategory::Sylvan, planet, None,
                (0.3, 0.5), BodyBuild::Ethereal, vec!["Pure light", "Radiant"], vec!["Light spectrum"], 
                vec!["Radiant".to_string()], 8, 400, 600, AgingRate::Eternal,
                vec![CulturalTrait::Joyful, CulturalTrait::Mystical], SpiritualAlignment::Light, "Pure Radiance".to_string(), vec!["Light Form".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::TrueSeeing], vec![ElementalAffinity::Light], vec![MagicalInclination::Illusion, MagicalInclination::Healing],
                SocialStructure::Collective, vec![Environment::Crystal], ArchitecturalStyle::Magical, vec!["Light".to_string()]
            )
        }).collect()
    }

    // Umbra (underground) planet race creation methods
    pub fn create_deep_dwarf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Deep Dwarf {}", i + 1), "Deep cave dwelling dwarfs", RaceCategory::Dwarven, planet, Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust, vec!["Stone-gray", "Cave-adapted"], vec!["Stone colors"], 
                vec!["Gray".to_string(), "Black".to_string()], 120, 400, 600, AgingRate::Slow,
                vec![CulturalTrait::Traditionalist, CulturalTrait::Independent], SpiritualAlignment::Endurance, "Stone Sight".to_string(), vec!["Dark Vision".to_string()],
                vec![NaturalAbility::Darkvision, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Earth, ElementalAffinity::Shadow], vec![MagicalInclination::Transmutation, MagicalInclination::Warding],
                SocialStructure::Clan, vec![Environment::Underground], ArchitecturalStyle::Stone, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }

    pub fn create_shadow_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Shadow Elf {}", i + 1), "Divine shadow-dwelling elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Graceful, vec!["Shadow-touched", "Dusky"], vec!["Dark colors"], 
                vec!["Silver".to_string(), "Violet".to_string()], 100, 800, 1200, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Independent], SpiritualAlignment::Mystery, "Shadow Step".to_string(), vec!["Shadow Blend".to_string()],
                vec![NaturalAbility::Darkvision, NaturalAbility::Camouflage], vec![ElementalAffinity::Shadow], vec![MagicalInclination::Illusion, MagicalInclination::Divination],
                SocialStructure::Council, vec![Environment::Underground], ArchitecturalStyle::Flowing, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_cave_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Cave Human {}", i + 1), "Underground-adapted humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.5, 1.7), BodyBuild::Athletic, vec!["Pale", "Cave-adapted"], vec!["Dark colors"], 
                vec!["Pale eyes".to_string()], 18, 90, 130, AgingRate::Normal,
                vec![CulturalTrait::Communal, CulturalTrait::Pragmatic], SpiritualAlignment::Survival, "Cave Navigation".to_string(), vec!["Echo Location".to_string()],
                vec![NaturalAbility::Darkvision, NaturalAbility::EnhancedSenses("Touch".to_string())], vec![ElementalAffinity::Earth], vec![MagicalInclination::Warding],
                SocialStructure::Tribal, vec![Environment::Underground], ArchitecturalStyle::Stone, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_mushroom_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Mushroom Kin {}", i + 1), "Fungal humanoids", RaceCategory::Sylvan, planet, None,
                (1.4, 1.8), BodyBuild::Robust, vec!["Mushroom-textured", "Spore-touched"], vec!["Fungal colors"], 
                vec!["Spore".to_string(), "Earthy".to_string()], 30, 200, 300, AgingRate::Normal,
                vec![CulturalTrait::Communal, CulturalTrait::Wise], SpiritualAlignment::Cycle, "Spore Network".to_string(), vec!["Fungal Communication".to_string()],
                vec![NaturalAbility::PoisonResistance, NaturalAbility::DiseaseResistance], vec![ElementalAffinity::Nature, ElementalAffinity::Earth], vec![MagicalInclination::Nature, MagicalInclination::Healing],
                SocialStructure::Collective, vec![Environment::Underground, Environment::Swamp], ArchitecturalStyle::Organic, vec!["Spore".to_string()]
            )
        }).collect()
    }

    pub fn create_crystal_moth_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Crystal Moth {}", i + 1), "Luminous cave dwellers", RaceCategory::Sylvan, planet, None,
                (1.0, 1.4), BodyBuild::Ethereal, vec!["Bioluminescent", "Moth-like"], vec!["Glowing patterns"], 
                vec!["Bioluminescent".to_string()], 15, 80, 120, AgingRate::Normal,
                vec![CulturalTrait::Mystical, CulturalTrait::Peaceful], SpiritualAlignment::Light, "Bio-luminescence".to_string(), vec!["Light Generation".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::Darkvision], vec![ElementalAffinity::Light, ElementalAffinity::Air], vec![MagicalInclination::Illusion, MagicalInclination::Light],
                SocialStructure::Collective, vec![Environment::Underground], ArchitecturalStyle::Organic, vec!["Light Patterns".to_string()]
            )
        }).collect()
    }

    // Glacialis (ice world) race creation methods
    pub fn create_frost_giant_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Frost Giant {}", i + 1), "Ice-dwelling giants", RaceCategory::IceTouched, planet, None,
                (2.5, 3.2), BodyBuild::Powerful, vec!["Ice-blue", "Frost-white"], vec!["Ice colors"], 
                vec!["Ice blue".to_string(), "Frost white".to_string()], 80, 400, 600, AgingRate::Slow,
                vec![CulturalTrait::Warrior, CulturalTrait::Traditional], SpiritualAlignment::Endurance, "Frost Immunity".to_string(), vec!["Ice Shaping".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalWeapons("Ice Fists".to_string())], vec![ElementalAffinity::Ice], vec![MagicalInclination::Elemental(ElementalAffinity::Ice), MagicalInclination::Evocation],
                SocialStructure::Tribal, vec![Environment::Ice], ArchitecturalStyle::Ice, vec!["Giant".to_string()]
            )
        }).collect()
    }

    pub fn create_ice_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Ice Elf {}", i + 1), "Frost-dwelling elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Ethereal, vec!["Ice-pale", "Crystal-clear"], vec!["Frost colors"], 
                vec!["Ice blue".to_string(), "Crystal clear".to_string()], 100, 900, 1300, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Harmonious], SpiritualAlignment::Serenity, "Ice Magic".to_string(), vec!["Frost Walking".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::Camouflage], vec![ElementalAffinity::Ice, ElementalAffinity::Air], vec![MagicalInclination::Elemental(ElementalAffinity::Ice), MagicalInclination::Illusion],
                SocialStructure::Council, vec![Environment::Ice], ArchitecturalStyle::Ice, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_snow_dwarf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Snow Dwarf {}", i + 1), "Arctic-dwelling dwarfs", RaceCategory::Dwarven, planet, Some("Dwarf".to_string()),
                (1.3, 1.5), BodyBuild::Robust, vec!["Snow-white", "Ice-touched"], vec!["White", "Ice blue"], 
                vec!["Ice blue".to_string(), "White".to_string()], 120, 450, 650, AgingRate::Slow,
                vec![CulturalTrait::Artisan, CulturalTrait::Enduring], SpiritualAlignment::Perseverance, "Ice Crafting".to_string(), vec!["Cold Immunity".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::NaturalArmor], vec![ElementalAffinity::Ice, ElementalAffinity::Metal], vec![MagicalInclination::Transmutation, MagicalInclination::Enchantment],
                SocialStructure::Clan, vec![Environment::Ice, Environment::Mountains], ArchitecturalStyle::Ice, vec!["Dwarvish".to_string()]
            )
        }).collect()
    }

    pub fn create_ice_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Ice Human {}", i + 1), "Cold-adapted humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Pale", "Frost-kissed"], vec!["White", "Pale blonde"], 
                vec!["Ice blue".to_string(), "Pale".to_string()], 18, 100, 150, AgingRate::Normal,
                vec![CulturalTrait::Survivor, CulturalTrait::Communal], SpiritualAlignment::Endurance, "Cold Adaptation".to_string(), vec!["Hypothermia Resistance".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::EnhancedSenses("Touch".to_string())], vec![ElementalAffinity::Ice], vec![MagicalInclination::Elemental(ElementalAffinity::Ice)],
                SocialStructure::Tribal, vec![Environment::Ice], ArchitecturalStyle::Stone, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_winter_fae_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Winter Sprite {}", i + 1), "Frost spirits", RaceCategory::Sylvan, planet, None,
                (0.3, 0.5), BodyBuild::Ethereal, vec!["Crystalline", "Snow-white"], vec!["Ice crystal"], 
                vec!["Frost".to_string()], 8, 400, 600, AgingRate::Eternal,
                vec![CulturalTrait::Playful, CulturalTrait::Mystical], SpiritualAlignment::Winter, "Frost Magic".to_string(), vec!["Snow Creation".to_string()],
                vec![NaturalAbility::NaturalFlight, NaturalAbility::TemperatureResistance], vec![ElementalAffinity::Ice, ElementalAffinity::Air], vec![MagicalInclination::Illusion, MagicalInclination::Elemental(ElementalAffinity::Ice)],
                SocialStructure::Collective, vec![Environment::Ice], ArchitecturalStyle::Ice, vec!["Frost".to_string()]
            )
        }).collect()
    }

    // Tempest (desert world) race creation methods
    pub fn create_sand_nomad_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Sand Nomad {}", i + 1), "Desert-wandering humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.7, 1.9), BodyBuild::Athletic, vec!["Sun-bronzed", "Desert-tanned"], vec!["Dark brown", "Black"], 
                vec!["Dark brown".to_string(), "Amber".to_string()], 16, 90, 130, AgingRate::Normal,
                vec![CulturalTrait::Nomadic, CulturalTrait::Independent], SpiritualAlignment::Freedom, "Desert Navigation".to_string(), vec!["Heat Resistance".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::EnhancedSenses("Smell".to_string())], vec![ElementalAffinity::Earth, ElementalAffinity::Fire], vec![MagicalInclination::Divination, MagicalInclination::Elemental(ElementalAffinity::Earth)],
                SocialStructure::Nomadic, vec![Environment::Desert], ArchitecturalStyle::Organic, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_dune_elf_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Dune Elf {}", i + 1), "Sand-dwelling elves", RaceCategory::Elven, planet, Some("Elf".to_string()),
                (1.7, 1.9), BodyBuild::Graceful, vec!["Sand-colored", "Dune-touched"], vec!["Sandy blonde", "Desert gold"], 
                vec!["Amber".to_string(), "Gold".to_string()], 100, 800, 1200, AgingRate::Slow,
                vec![CulturalTrait::Mystical, CulturalTrait::Nomadic], SpiritualAlignment::Mirage, "Sand Walking".to_string(), vec!["Mirage Sight".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::Camouflage], vec![ElementalAffinity::Earth, ElementalAffinity::Fire], vec![MagicalInclination::Illusion, MagicalInclination::Divination],
                SocialStructure::Council, vec![Environment::Desert], ArchitecturalStyle::Organic, vec!["Elvish".to_string()]
            )
        }).collect()
    }

    pub fn create_cactus_kin_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Cactus Kin {}", i + 1), "Plant-desert hybrids", RaceCategory::Sylvan, planet, None,
                (1.8, 2.2), BodyBuild::Robust, vec!["Cactus-green", "Spined"], vec!["Desert plant"], 
                vec!["Desert green".to_string(), "Cactus flower".to_string()], 30, 300, 500, AgingRate::Slow,
                vec![CulturalTrait::Enduring, CulturalTrait::Protective], SpiritualAlignment::Survival, "Water Storage".to_string(), vec!["Spine Defense".to_string()],
                vec![NaturalAbility::NaturalWeapons("Spines".to_string()), NaturalAbility::TemperatureResistance], vec![ElementalAffinity::Nature, ElementalAffinity::Earth], vec![MagicalInclination::Nature, MagicalInclination::Warding],
                SocialStructure::Collective, vec![Environment::Desert], ArchitecturalStyle::Organic, vec!["Plant Speech".to_string()]
            )
        }).collect()
    }

    pub fn create_mirage_human_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Mirage Human {}", i + 1), "Heat-adapted illusion humans", RaceCategory::Human, planet, Some("Human".to_string()),
                (1.6, 1.8), BodyBuild::Athletic, vec!["Shimmering", "Heat-wavered"], vec!["Mirage-touched"], 
                vec!["Shifting".to_string(), "Mirage".to_string()], 18, 95, 140, AgingRate::Normal,
                vec![CulturalTrait::Mystical, CulturalTrait::Adaptive], SpiritualAlignment::Illusion, "Mirage Form".to_string(), vec!["Heat Immunity".to_string()],
                vec![NaturalAbility::TemperatureResistance, NaturalAbility::Camouflage], vec![ElementalAffinity::Fire, ElementalAffinity::Air], vec![MagicalInclination::Illusion, MagicalInclination::Elemental(ElementalAffinity::Fire)],
                SocialStructure::Nomadic, vec![Environment::Desert], ArchitecturalStyle::Magical, vec!["Common".to_string()]
            )
        }).collect()
    }

    pub fn create_sand_djinn_variants(&self, planet: &str, count: usize) -> Vec<RaceDefinition> {
        (0..count).map(|i| {
            self.create_base_race(
                &format!("Sand Djinn {}", i + 1), "Desert spirits", RaceCategory::Celestial, planet, None,
                (2.0, 2.5), BodyBuild::Ethereal, vec!["Sandstorm", "Desert wind"], vec!["Sand-formed"], 
                vec!["Desert gold".to_string(), "Sandstorm".to_string()], 100, 1000, 2000, AgingRate::Eternal,
                vec![CulturalTrait::Mystical, CulturalTrait::Independent], SpiritualAlignment::Freedom, "Sand Form".to_string(), vec!["Sandstorm Control".to_string()],
                vec![NaturalAbility::ShapeMinorChanges, NaturalAbility::NaturalFlight], vec![ElementalAffinity::Earth, ElementalAffinity::Air], vec![MagicalInclination::Transmutation, MagicalInclination::Elemental(ElementalAffinity::Earth)],
                SocialStructure::Independent, vec![Environment::Desert], ArchitecturalStyle::Magical, vec!["Djinn".to_string()]
            )
        }).collect()
    }

    // Seraphim (floating world) race creation methods - will continue with more methods...
}