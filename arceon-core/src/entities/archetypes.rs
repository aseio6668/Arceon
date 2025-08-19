use super::being::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Archetype system - dynamic class-like progression based on skill combinations
/// Archetypes represent the player's progression path and unlock automatically
/// as skill thresholds are met. Players can have multiple archetypes simultaneously.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeSystem {
    pub archetypes: HashMap<String, Archetype>,
    pub progression_log: Vec<ArchetypeProgression>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archetype {
    pub name: String,
    pub level: f64,           // Calculated level with 2 decimal precision
    pub description: String,
    pub requirements: Vec<ArchetypeRequirement>,
    pub bonuses: Vec<ArchetypeBonus>,
    pub tier: ArchetypeTier,
    pub is_unlocked: bool,
    pub unlock_timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchetypeTier {
    Basic,      // Fighter, Rogue, Mage, Healer
    Advanced,   // Druid, Shaman, Paladin, Deathknight, Necromancer
    Master,     // Summoner, Cleric, Warrior, Archer, Tamer, Berserker, Illusionist
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeRequirement {
    pub skill_name: String,
    pub minimum_level: f64,
    pub weight: f64,          // How much this skill contributes to archetype level
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArchetypeBonus {
    SkillExperienceMultiplier(String, f64),  // skill_name, multiplier
    SkillLevelBonus(String, f64),            // skill_name, bonus_levels
    VitalBonus(String, f64),                 // vital_name, bonus_amount
    SpecialAbility(String),                  // Special archetype ability
    PassiveEffect(String, f64),              // effect_name, strength
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeProgression {
    pub archetype_name: String,
    pub old_level: f64,
    pub new_level: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub unlocked: bool,
}

impl ArchetypeSystem {
    pub fn new() -> Self {
        Self {
            archetypes: HashMap::new(),
            progression_log: Vec::new(),
        }
    }

    /// Update all archetype levels based on current skills
    pub fn update_archetypes(&mut self, skills: &HashMap<String, Skill>) {
        // Initialize archetypes if not present
        self.initialize_archetypes();

        for (archetype_name, archetype) in &mut self.archetypes {
            let old_level = archetype.level;
            let new_level = Self::calculate_archetype_level(&archetype.requirements, skills);
            
            // Check if archetype should be unlocked
            if !archetype.is_unlocked && new_level >= 1.0 {
                archetype.is_unlocked = true;
                archetype.unlock_timestamp = Some(chrono::Utc::now());
                
                self.progression_log.push(ArchetypeProgression {
                    archetype_name: archetype_name.clone(),
                    old_level: 0.0,
                    new_level,
                    timestamp: chrono::Utc::now(),
                    unlocked: true,
                });
            }
            
            // Update level if changed significantly
            if (new_level - old_level).abs() > 0.01 {
                archetype.level = new_level;
                
                self.progression_log.push(ArchetypeProgression {
                    archetype_name: archetype_name.clone(),
                    old_level,
                    new_level,
                    timestamp: chrono::Utc::now(),
                    unlocked: false,
                });
            }
        }
    }

    /// Calculate archetype level based on skill requirements
    fn calculate_archetype_level(requirements: &[ArchetypeRequirement], skills: &HashMap<String, Skill>) -> f64 {
        let mut total_weighted_level = 0.0;
        let mut total_weight = 0.0;

        for req in requirements {
            if let Some(skill) = skills.get(&req.skill_name) {
                let skill_contribution = (skill.level * req.weight).max(0.0);
                total_weighted_level += skill_contribution;
                total_weight += req.weight;
            }
        }

        if total_weight > 0.0 {
            (total_weighted_level / total_weight).max(0.0)
        } else {
            0.0
        }
    }

    /// Initialize all archetype definitions
    fn initialize_archetypes(&mut self) {
        if self.archetypes.is_empty() {
            // Basic Archetypes
            self.archetypes.insert("Fighter".to_string(), Self::create_fighter_archetype());
            self.archetypes.insert("Rogue".to_string(), Self::create_rogue_archetype());
            self.archetypes.insert("Mage".to_string(), Self::create_mage_archetype());
            self.archetypes.insert("Healer".to_string(), Self::create_healer_archetype());
            
            // Advanced Archetypes
            self.archetypes.insert("Druid".to_string(), Self::create_druid_archetype());
            self.archetypes.insert("Shaman".to_string(), Self::create_shaman_archetype());
            self.archetypes.insert("Paladin".to_string(), Self::create_paladin_archetype());
            self.archetypes.insert("Deathknight".to_string(), Self::create_deathknight_archetype());
            self.archetypes.insert("Necromancer".to_string(), Self::create_necromancer_archetype());
            self.archetypes.insert("Monk".to_string(), Self::create_monk_archetype());
            
            // Master Archetypes
            self.archetypes.insert("Summoner".to_string(), Self::create_summoner_archetype());
            self.archetypes.insert("Cleric".to_string(), Self::create_cleric_archetype());
            self.archetypes.insert("Warrior".to_string(), Self::create_warrior_archetype());
            self.archetypes.insert("Archer".to_string(), Self::create_archer_archetype());
            self.archetypes.insert("Tamer".to_string(), Self::create_tamer_archetype());
            self.archetypes.insert("Berserker".to_string(), Self::create_berserker_archetype());
            self.archetypes.insert("Illusionist".to_string(), Self::create_illusionist_archetype());
        }
    }

    /// Get all unlocked archetypes for display
    pub fn get_unlocked_archetypes(&self) -> Vec<&Archetype> {
        self.archetypes.values()
            .filter(|archetype| archetype.is_unlocked)
            .collect()
    }

    /// Format archetype level for display (2 decimal places)
    pub fn format_archetype_level(level: f64) -> String {
        format!("{:.2}", level)
    }
}

// Archetype Definitions

impl ArchetypeSystem {
    fn create_fighter_archetype() -> Archetype {
        Archetype {
            name: "Fighter".to_string(),
            level: 0.0,
            description: "Master of weapons and combat, excelling in melee combat".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 10.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Courage".to_string(), minimum_level: 8.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Defense".to_string(), minimum_level: 5.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Weapon Mastery".to_string(), minimum_level: 1.0, weight: 0.25 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Strength".to_string(), 0.1),
                ArchetypeBonus::SkillExperienceMultiplier("Courage".to_string(), 0.1),
                ArchetypeBonus::VitalBonus("health".to_string(), 10.0),
            ],
            tier: ArchetypeTier::Basic,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_rogue_archetype() -> Archetype {
        Archetype {
            name: "Rogue".to_string(),
            level: 0.0,
            description: "Master of stealth, precision, and cunning tactics".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Dexterity".to_string(), minimum_level: 12.0, weight: 0.35 },
                ArchetypeRequirement { skill_name: "Stealth".to_string(), minimum_level: 8.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 5.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 6.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Dexterity".to_string(), 0.15),
                ArchetypeBonus::SkillExperienceMultiplier("Stealth".to_string(), 0.2),
                ArchetypeBonus::PassiveEffect("critical_chance".to_string(), 5.0),
            ],
            tier: ArchetypeTier::Basic,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_mage_archetype() -> Archetype {
        Archetype {
            name: "Mage".to_string(),
            level: 0.0,
            description: "Wielder of arcane forces and elemental magic".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 15.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 10.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Fire Magic".to_string(), minimum_level: 5.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Abjuration".to_string(), minimum_level: 3.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Intelligence".to_string(), 0.2),
                ArchetypeBonus::VitalBonus("mana".to_string(), 20.0),
                ArchetypeBonus::SpecialAbility("Arcane Focus".to_string()),
            ],
            tier: ArchetypeTier::Basic,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_healer_archetype() -> Archetype {
        Archetype {
            name: "Healer".to_string(),
            level: 0.0,
            description: "Master of restoration magic and protective arts".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 15.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Healing Magic".to_string(), minimum_level: 8.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Vitality".to_string(), minimum_level: 10.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 6.0, weight: 0.1 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Healing Magic".to_string(), 0.25),
                ArchetypeBonus::SkillExperienceMultiplier("Wisdom".to_string(), 0.15),
                ArchetypeBonus::PassiveEffect("healing_power".to_string(), 20.0),
            ],
            tier: ArchetypeTier::Basic,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_druid_archetype() -> Archetype {
        Archetype {
            name: "Druid".to_string(),
            level: 0.0,
            description: "Guardian of nature, master of natural magic and beast kinship".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Nature Magic".to_string(), minimum_level: 15.0, weight: 0.35 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 18.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Taming".to_string(), minimum_level: 12.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Healing Magic".to_string(), minimum_level: 8.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Nature Magic".to_string(), 0.3),
                ArchetypeBonus::SkillExperienceMultiplier("Taming".to_string(), 0.25),
                ArchetypeBonus::SpecialAbility("Wild Shape".to_string()),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_shaman_archetype() -> Archetype {
        Archetype {
            name: "Shaman".to_string(),
            level: 0.0,
            description: "Spiritual guide who bridges the natural and supernatural worlds".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 20.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Nature Magic".to_string(), minimum_level: 10.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Healing Magic".to_string(), minimum_level: 10.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 12.0, weight: 0.25 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Wisdom".to_string(), 0.2),
                ArchetypeBonus::SpecialAbility("Spirit Guide".to_string()),
                ArchetypeBonus::PassiveEffect("elemental_resistance".to_string(), 15.0),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_paladin_archetype() -> Archetype {
        Archetype {
            name: "Paladin".to_string(),
            level: 0.0,
            description: "Holy warrior combining martial prowess with divine magic".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Courage".to_string(), minimum_level: 20.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Healing Magic".to_string(), minimum_level: 15.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 15.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 12.0, weight: 0.15 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 12.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Healing Magic".to_string(), 0.2),
                ArchetypeBonus::SkillExperienceMultiplier("Courage".to_string(), 0.15),
                ArchetypeBonus::SpecialAbility("Divine Protection".to_string()),
                ArchetypeBonus::PassiveEffect("undead_damage".to_string(), 25.0),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_deathknight_archetype() -> Archetype {
        Archetype {
            name: "Deathknight".to_string(),
            level: 0.0,
            description: "Fallen warrior who wields death magic and martial might".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Death Magic".to_string(), minimum_level: 18.0, weight: 0.35 },
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 18.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Courage".to_string(), minimum_level: 15.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 12.0, weight: 0.2 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Death Magic".to_string(), 0.25),
                ArchetypeBonus::SkillExperienceMultiplier("Strength".to_string(), 0.15),
                ArchetypeBonus::SpecialAbility("Death Aura".to_string()),
                ArchetypeBonus::PassiveEffect("life_steal".to_string(), 10.0),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_necromancer_archetype() -> Archetype {
        Archetype {
            name: "Necromancer".to_string(),
            level: 0.0,
            description: "Master of death magic and undead minions".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Death Magic".to_string(), minimum_level: 25.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 22.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Abjuration".to_string(), minimum_level: 10.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 8.0, weight: 0.1 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Death Magic".to_string(), 0.3),
                ArchetypeBonus::SpecialAbility("Raise Undead".to_string()),
                ArchetypeBonus::PassiveEffect("undead_control".to_string(), 3.0),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_summoner_archetype() -> Archetype {
        Archetype {
            name: "Summoner".to_string(),
            level: 0.0,
            description: "Master of calling forth creatures and elemental beings".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 25.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 20.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Taming".to_string(), minimum_level: 18.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Nature Magic".to_string(), minimum_level: 12.0, weight: 0.2 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Taming".to_string(), 0.3),
                ArchetypeBonus::SpecialAbility("Summon Elemental".to_string()),
                ArchetypeBonus::PassiveEffect("summon_duration".to_string(), 50.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_cleric_archetype() -> Archetype {
        Archetype {
            name: "Cleric".to_string(),
            level: 0.0,
            description: "Divine spellcaster devoted to healing and protection".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Healing Magic".to_string(), minimum_level: 25.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 25.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Abjuration".to_string(), minimum_level: 15.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 15.0, weight: 0.1 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Healing Magic".to_string(), 0.35),
                ArchetypeBonus::SpecialAbility("Divine Intervention".to_string()),
                ArchetypeBonus::PassiveEffect("healing_radius".to_string(), 3.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_warrior_archetype() -> Archetype {
        Archetype {
            name: "Warrior".to_string(),
            level: 0.0,
            description: "Ultimate master of physical combat and tactical warfare".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 30.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Courage".to_string(), minimum_level: 25.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Weapon Mastery".to_string(), minimum_level: 20.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Defense".to_string(), minimum_level: 20.0, weight: 0.2 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Strength".to_string(), 0.25),
                ArchetypeBonus::SpecialAbility("Weapon Master".to_string()),
                ArchetypeBonus::PassiveEffect("damage_reduction".to_string(), 15.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_archer_archetype() -> Archetype {
        Archetype {
            name: "Archer".to_string(),
            level: 0.0,
            description: "Master of ranged combat and precision strikes".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Dexterity".to_string(), minimum_level: 28.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Archery".to_string(), minimum_level: 20.0, weight: 0.35 },
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 12.0, weight: 0.15 },
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 10.0, weight: 0.1 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Dexterity".to_string(), 0.25),
                ArchetypeBonus::SpecialAbility("Multi Shot".to_string()),
                ArchetypeBonus::PassiveEffect("range_bonus".to_string(), 25.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_tamer_archetype() -> Archetype {
        Archetype {
            name: "Tamer".to_string(),
            level: 0.0,
            description: "Supreme master of beast companionship and creature control".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Taming".to_string(), minimum_level: 30.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 25.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Nature Magic".to_string(), minimum_level: 18.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 15.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Taming".to_string(), 0.4),
                ArchetypeBonus::SpecialAbility("Beast Lord".to_string()),
                ArchetypeBonus::PassiveEffect("max_companions".to_string(), 2.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_berserker_archetype() -> Archetype {
        Archetype {
            name: "Berserker".to_string(),
            level: 0.0,
            description: "Wild warrior who fights with primal fury and unstoppable rage".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Fury".to_string(), minimum_level: 25.0, weight: 0.35 },
                ArchetypeRequirement { skill_name: "Strength".to_string(), minimum_level: 25.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Courage".to_string(), minimum_level: 20.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Vitality".to_string(), minimum_level: 15.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Fury".to_string(), 0.3),
                ArchetypeBonus::SpecialAbility("Unstoppable Rage".to_string()),
                ArchetypeBonus::PassiveEffect("rage_damage".to_string(), 40.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_monk_archetype() -> Archetype {
        Archetype {
            name: "Monk".to_string(),
            level: 0.0,
            description: "Master of martial arts and inner discipline".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Martial Arts".to_string(), minimum_level: 15.0, weight: 0.3 },
                ArchetypeRequirement { skill_name: "Wisdom".to_string(), minimum_level: 18.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Dexterity".to_string(), minimum_level: 15.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Ki Flow".to_string(), minimum_level: 8.0, weight: 0.15 },
                ArchetypeRequirement { skill_name: "Meditation".to_string(), minimum_level: 10.0, weight: 0.1 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Martial Arts".to_string(), 0.25),
                ArchetypeBonus::SkillExperienceMultiplier("Ki Flow".to_string(), 0.2),
                ArchetypeBonus::SpecialAbility("Ki Mastery".to_string()),
                ArchetypeBonus::PassiveEffect("unarmed_damage".to_string(), 30.0),
            ],
            tier: ArchetypeTier::Advanced,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }

    fn create_illusionist_archetype() -> Archetype {
        Archetype {
            name: "Illusionist".to_string(),
            level: 0.0,
            description: "Master of deception, reality manipulation, and mind magic".to_string(),
            requirements: vec![
                ArchetypeRequirement { skill_name: "Illusion Magic".to_string(), minimum_level: 30.0, weight: 0.4 },
                ArchetypeRequirement { skill_name: "Charisma".to_string(), minimum_level: 25.0, weight: 0.25 },
                ArchetypeRequirement { skill_name: "Intelligence".to_string(), minimum_level: 20.0, weight: 0.2 },
                ArchetypeRequirement { skill_name: "Dexterity".to_string(), minimum_level: 15.0, weight: 0.15 },
            ],
            bonuses: vec![
                ArchetypeBonus::SkillExperienceMultiplier("Illusion Magic".to_string(), 0.35),
                ArchetypeBonus::SpecialAbility("Reality Warp".to_string()),
                ArchetypeBonus::PassiveEffect("illusion_power".to_string(), 50.0),
            ],
            tier: ArchetypeTier::Master,
            is_unlocked: false,
            unlock_timestamp: None,
        }
    }
}