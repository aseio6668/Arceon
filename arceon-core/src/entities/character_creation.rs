use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use super::being::{Being, BeingType, Race, Skill, Vital, VitalSystem, SkillSystem, BeingCapability, Incarnation, InventorySystem, EquipmentSystem}; // Removed unused EvolutionContribution
use super::skills::SkillRegistry;
use super::archetypes::ArchetypeSystem;

/// Character creation system with default skills and vitals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCreationSystem {
    pub default_vitals: DefaultVitals,
    pub default_skills: DefaultSkills,
    pub racial_bonuses: HashMap<Race, RacialBonuses>,
    pub item_type_skills: HashMap<String, String>, // item_type -> skill_name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultVitals {
    pub hitpoints: VitalStats,
    pub energy_points: VitalStats,
    pub mana_points: VitalStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalStats {
    pub base_value: f64,
    pub max_value: f64,
    pub regeneration_rate: f64,
    pub vital_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultSkills {
    pub core_attributes: Vec<String>,
    pub survival_skills: Vec<String>,
    pub armor_skills: Vec<String>,
    pub combat_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RacialBonuses {
    pub race: Race,
    pub skill_bonuses: HashMap<String, f64>,
    pub vital_bonuses: HashMap<String, f64>,
    pub starting_skills: Vec<String>,
    pub special_abilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedCharacter {
    pub being: Being,
    pub starting_skills: HashMap<String, Skill>,
    pub starting_vitals: HashMap<String, Vital>,
    pub unlocked_skills: Vec<String>,
}

impl CharacterCreationSystem {
    pub fn new() -> Self {
        Self {
            default_vitals: DefaultVitals::new(),
            default_skills: DefaultSkills::new(),
            racial_bonuses: Self::create_racial_bonuses(),
            item_type_skills: Self::create_item_type_mappings(),
        }
    }

    /// Create a new character with default skills and vitals
    pub fn create_character(&self, name: String, race: Race, _starting_area: String) -> CreatedCharacter {
        let being_id = Uuid::new_v4();
        
        // Create base being
        let mut being = Being {
            id: being_id,
            name: name.clone(),
            being_type: BeingType::Player,
            race: race.clone(),
            is_feral: false,
            vitals: VitalSystem::new(),
            skills: SkillSystem::new(),
            archetypes: ArchetypeSystem::new(),
            capabilities: vec![
                BeingCapability::CanMove,
                BeingCapability::CanCommunicate,
                BeingCapability::CanLearn,
                BeingCapability::CanFight,
            ],
            incarnation_history: Vec::new(),
            current_incarnation: Incarnation {
                id: Uuid::new_v4(),
                being_id: being_id,
                incarnation_number: 1,
                race: race.clone(),
                being_type: BeingType::Player,
                birth_timestamp: chrono::Utc::now(),
                death_timestamp: None,
                cause_of_death: None,
                chose_reincarnation: false,
                afterlife_plane: None,
                skill_achievements: Vec::new(),
                evolution_contributions: Vec::new(),
                is_final_death: false,
            },
            discovered_skills: Vec::new(),
            evolution_contributions: Vec::new(),
            inventory: Some(InventorySystem {
                items: HashMap::new(),
                max_weight: None,
                max_slots: None,
            }),
            equipment: Some(EquipmentSystem {
                head: None,
                shoulders: None,
                arms: None,
                chest: None,
                hands: None,
                waist: None,
                legs: None,
                feet: None,
                back: None,
                rings: Vec::new(),
                neck: None,
                main_hand: None,
                off_hand: None,
            }),
        };

        // Apply default vitals with racial bonuses
        self.apply_default_vitals(&mut being, &race);

        // Apply default skills with racial bonuses
        let skills = self.create_default_skills(&race);
        for (skill_name, skill) in skills.iter() {
            being.skills.skills.insert(skill_name.clone(), skill.clone());
        }

        CreatedCharacter {
            being,
            starting_skills: skills,
            starting_vitals: HashMap::new(), // Will be filled by apply_default_vitals
            unlocked_skills: self.default_skills.core_attributes.clone(),
        }
    }

    /// Apply default vitals with racial bonuses to a character
    fn apply_default_vitals(&self, being: &mut Being, race: &Race) {
        // Get racial bonuses if they exist
        let racial_bonus = self.racial_bonuses.get(race);

        // Apply racial bonuses to health
        let racial_hp_bonus = racial_bonus
            .and_then(|bonus| bonus.vital_bonuses.get("Hitpoints"))
            .unwrap_or(&0.0);
        being.vitals.health.current += racial_hp_bonus;
        being.vitals.health.maximum += racial_hp_bonus;

        // Apply racial bonuses to energy
        let racial_energy_bonus = racial_bonus
            .and_then(|bonus| bonus.vital_bonuses.get("Energy"))
            .unwrap_or(&0.0);
        being.vitals.energy.current += racial_energy_bonus;
        being.vitals.energy.maximum += racial_energy_bonus;

        // Apply racial bonuses to mana
        let racial_mana_bonus = racial_bonus
            .and_then(|bonus| bonus.vital_bonuses.get("Mana"))
            .unwrap_or(&0.0);
        being.vitals.mana.current += racial_mana_bonus;
        being.vitals.mana.maximum += racial_mana_bonus;
    }

    /// Create default skills for a character
    fn create_default_skills(&self, race: &Race) -> HashMap<String, Skill> {
        let mut skills = HashMap::new();
        let skill_registry = SkillRegistry::new();
        
        // Get racial bonuses if they exist
        let racial_bonus = self.racial_bonuses.get(race);

        // Core Attributes - everyone starts with these
        for skill_name in &self.default_skills.core_attributes {
            if let Some(mut skill) = skill_registry.create_skill(skill_name) {
                // Apply racial bonus if exists
                if let Some(bonus_amount) = racial_bonus
                    .and_then(|bonus| bonus.skill_bonuses.get(skill_name)) {
                    skill.level += bonus_amount;
                    skill.experience += bonus_amount * 100.0; // Experience scales with bonus
                }
                skills.insert(skill_name.clone(), skill);
            }
        }

        // Armor Skills - everyone starts with basic armor knowledge
        for skill_name in &self.default_skills.armor_skills {
            if let Some(mut skill) = skill_registry.create_skill(skill_name) {
                // Apply racial bonus if exists
                if let Some(bonus_amount) = racial_bonus
                    .and_then(|bonus| bonus.skill_bonuses.get(skill_name)) {
                    skill.level += bonus_amount;
                    skill.experience += bonus_amount * 100.0;
                }
                skills.insert(skill_name.clone(), skill);
            }
        }

        // Combat Skills - everyone starts with unarmed combat
        if let Some(mut unarmed_skill) = skill_registry.create_skill("Unarmed Combat") {
            if let Some(bonus_amount) = racial_bonus
                .and_then(|bonus| bonus.skill_bonuses.get("Unarmed Combat")) {
                unarmed_skill.level += bonus_amount;
                unarmed_skill.experience += bonus_amount * 100.0;
            }
            skills.insert("Unarmed Combat".to_string(), unarmed_skill);
        }

        // Add racial starting skills
        if let Some(racial_bonus) = racial_bonus {
            for skill_name in &racial_bonus.starting_skills {
                if !skills.contains_key(skill_name) {
                    if let Some(skill) = skill_registry.create_skill(skill_name) {
                        skills.insert(skill_name.clone(), skill);
                    }
                }
            }
        }

        skills
    }

    /// Unlock skills when player picks up items for the first time
    pub fn unlock_skills_for_item(&self, item_type: &str, character: &mut CreatedCharacter) -> Vec<String> {
        let mut unlocked_skills = Vec::new();
        
        if let Some(skill_name) = self.item_type_skills.get(item_type) {
            // Check if skill is already unlocked
            if !character.starting_skills.contains_key(skill_name) {
                let skill_registry = SkillRegistry::new();
                if let Some(skill) = skill_registry.create_skill(skill_name) {
                    character.starting_skills.insert(skill_name.clone(), skill.clone());
                    character.being.skills.skills.insert(skill_name.clone(), skill);
                    character.unlocked_skills.push(skill_name.clone());
                    unlocked_skills.push(skill_name.clone());
                }
            }
        }

        unlocked_skills
    }

    /// Get skill recommendations based on what items the player has found
    pub fn get_recommended_skills(&self, found_items: &[String]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for item in found_items {
            if let Some(skill) = self.item_type_skills.get(item) {
                if !recommendations.contains(skill) {
                    recommendations.push(skill.clone());
                }
            }
        }

        recommendations
    }

    /// Create racial bonuses for different races
    fn create_racial_bonuses() -> HashMap<Race, RacialBonuses> {
        let mut bonuses = HashMap::new();

        // Human - Versatile and social
        bonuses.insert(Race::Human, RacialBonuses {
            race: Race::Human,
            skill_bonuses: HashMap::from([
                ("Charisma".to_string(), 2.0),
                ("Intelligence".to_string(), 1.0),
                ("Tool Making".to_string(), 1.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Energy".to_string(), 10.0),
            ]),
            starting_skills: vec![
                "Tool Making".to_string(),
                "Gathering".to_string(),
            ],
            special_abilities: vec!["Versatile Learning".to_string()],
        });

        // Elf - Magical and dexterous
        bonuses.insert(Race::Elf, RacialBonuses {
            race: Race::Elf,
            skill_bonuses: HashMap::from([
                ("Intelligence".to_string(), 3.0),
                ("Wisdom".to_string(), 2.0),
                ("Dexterity".to_string(), 2.0),
                ("Nature Magic".to_string(), 2.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Mana".to_string(), 25.0),
            ]),
            starting_skills: vec![
                "Nature Magic".to_string(),
                "Archery".to_string(),
            ],
            special_abilities: vec!["Magical Affinity".to_string()],
        });

        // Dwarf - Strong and crafty
        bonuses.insert(Race::Dwarf, RacialBonuses {
            race: Race::Dwarf,
            skill_bonuses: HashMap::from([
                ("Strength".to_string(), 3.0),
                ("Vitality".to_string(), 2.0),
                ("Blacksmithing".to_string(), 3.0),
                ("Mining".to_string(), 2.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Hitpoints".to_string(), 25.0),
            ]),
            starting_skills: vec![
                "Blacksmithing".to_string(),
                "Mining".to_string(),
                "Hammers".to_string(),
            ],
            special_abilities: vec!["Forge Mastery".to_string()],
        });

        // Gnome - Intelligent and inventive
        bonuses.insert(Race::Gnome, RacialBonuses {
            race: Race::Gnome,
            skill_bonuses: HashMap::from([
                ("Intelligence".to_string(), 4.0),
                ("Dexterity".to_string(), 2.0),
                ("Engineering".to_string(), 3.0),
                ("Illusion Magic".to_string(), 2.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Mana".to_string(), 20.0),
            ]),
            starting_skills: vec![
                "Engineering".to_string(),
                "Illusion Magic".to_string(),
                "Tool Making".to_string(),
            ],
            special_abilities: vec!["Tinker Mastery".to_string()],
        });

        // Halfling - Lucky and agile
        bonuses.insert(Race::Halfling, RacialBonuses {
            race: Race::Halfling,
            skill_bonuses: HashMap::from([
                ("Dexterity".to_string(), 3.0),
                ("Charisma".to_string(), 2.0),
                ("Stealth".to_string(), 2.0),
                ("Gathering".to_string(), 2.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Energy".to_string(), 15.0),
            ]),
            starting_skills: vec![
                "Stealth".to_string(),
                "Gathering".to_string(),
                "Dagger".to_string(),
            ],
            special_abilities: vec!["Lucky".to_string()],
        });

        // Orc - Strong and fierce
        bonuses.insert(Race::Orc, RacialBonuses {
            race: Race::Orc,
            skill_bonuses: HashMap::from([
                ("Strength".to_string(), 4.0),
                ("Courage".to_string(), 3.0),
                ("Unarmed Combat".to_string(), 3.0),
                ("Intimidation".to_string(), 2.0),
            ]),
            vital_bonuses: HashMap::from([
                ("Hitpoints".to_string(), 30.0),
                ("Energy".to_string(), 10.0),
            ]),
            starting_skills: vec![
                "Clubs".to_string(),
                "Intimidation".to_string(),
                "Heavy Armor".to_string(),
            ],
            special_abilities: vec!["Berserker Rage".to_string()],
        });

        bonuses
    }

    /// Create mappings from item types to skills that should be unlocked
    fn create_item_type_mappings() -> HashMap<String, String> {
        HashMap::from([
            // Weapons
            ("sword".to_string(), "Sword".to_string()),
            ("club".to_string(), "Clubs".to_string()),
            ("hammer".to_string(), "Hammers".to_string()),
            ("dagger".to_string(), "Dagger".to_string()),
            ("knuckles".to_string(), "Knuckles".to_string()),
            ("polearm".to_string(), "Polearm".to_string()),
            ("stave".to_string(), "Stave".to_string()),
            ("bow".to_string(), "Archery".to_string()),
            ("crossbow".to_string(), "Archery".to_string()),
            ("shield".to_string(), "Shield".to_string()),
            
            // Tools and crafting items
            ("pickaxe".to_string(), "Mining".to_string()),
            ("forge".to_string(), "Blacksmithing".to_string()),
            ("anvil".to_string(), "Blacksmithing".to_string()),
            ("saw".to_string(), "Carpentry".to_string()),
            ("chisel".to_string(), "Masonry".to_string()),
            ("needle".to_string(), "Weaving".to_string()),
            ("loom".to_string(), "Weaving".to_string()),
            ("pottery_wheel".to_string(), "Pottery".to_string()),
            
            // Books and scrolls
            ("spellbook".to_string(), "Intelligence".to_string()),
            ("scroll".to_string(), "Intelligence".to_string()),
            ("tome".to_string(), "Wisdom".to_string()),
            
            // Instruments
            ("lute".to_string(), "Musical Performance".to_string()),
            ("flute".to_string(), "Musical Performance".to_string()),
            ("drum".to_string(), "Musical Performance".to_string()),
        ])
    }
}

impl DefaultVitals {
    pub fn new() -> Self {
        Self {
            hitpoints: VitalStats {
                base_value: 100.0,
                max_value: 100.0,
                regeneration_rate: 1.0,
                vital_type: "Health".to_string(),
            },
            energy_points: VitalStats {
                base_value: 100.0,
                max_value: 100.0,
                regeneration_rate: 2.0,
                vital_type: "Energy".to_string(),
            },
            mana_points: VitalStats {
                base_value: 50.0,
                max_value: 50.0,
                regeneration_rate: 0.5,
                vital_type: "Mana".to_string(),
            },
        }
    }
}

impl DefaultSkills {
    pub fn new() -> Self {
        Self {
            core_attributes: vec![
                "Strength".to_string(),
                "Vitality".to_string(),
                "Intelligence".to_string(),
                "Wisdom".to_string(),
                "Dexterity".to_string(),
                "Charisma".to_string(),
                "Courage".to_string(),
            ],
            survival_skills: vec![
                "Gathering".to_string(),
            ],
            armor_skills: vec![
                "Light Armor".to_string(),
                "Medium Armor".to_string(),
                "Heavy Armor".to_string(),
            ],
            combat_skills: vec![
                "Unarmed Combat".to_string(),
            ],
        }
    }
}