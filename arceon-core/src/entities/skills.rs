use super::being::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Extended skill system with comprehensive skill definitions
/// This module contains all skill definitions including the new comprehensive system

/// Core attribute skills that influence other abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreAttributes {
    pub dexterity: f64,    // Critical hits, ranged accuracy and damage
    pub courage: f64,      // Melee attack rating
    pub wisdom: f64,       // Healing potency, mana regen, slight max mana
    pub strength: f64,     // Slight max HP, melee damage
    pub vitality: f64,     // Health regen, healing received
    pub charisma: f64,     // Social actions, illusion magic influence
    pub intelligence: f64, // Spell unlocks, mana capacity
}

/// Magic school skills
pub fn create_illusion_magic_skill() -> Skill {
    Skill {
        name: "Illusion Magic".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Illusion Mastery".to_string(),
            description: "Enhances all illusion spells and deception abilities".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Charisma".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Minor Illusion".to_string(),
            description: "Create a small visual or auditory illusion".to_string(),
            activation_cost: ActivationCost::Mana(15.0),
            cooldown: SkillCooldown {
                base_duration: 8.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.05),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("stealth".to_string(), 25.0, 30.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Charisma".to_string(), 15.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 10.0),
        ],
        evolution_potential: vec!["Mesmerize".to_string(), "Invisibility".to_string()],
        synergy_skills: vec!["Charisma".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Charisma".to_string()],
    }
}

/// Nature Magic spells
pub fn create_nature_magic_skill() -> Skill {
    Skill {
        name: "Nature Magic".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Nature's Bond".to_string(),
            description: "Improves interaction with natural environments and creatures".to_string(),
            effects: vec![
                PassiveEffect::IncreaseRegeneration("health".to_string(), 0.05),
                PassiveEffect::GrantResistance("poison".to_string(), 1.0),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Nature's Touch".to_string(),
            description: "Channel natural energy to heal wounds".to_string(),
            activation_cost: ActivationCost::Mana(20.0),
            cooldown: SkillCooldown {
                base_duration: 12.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.08),
            },
            effects: vec![
                ActiveEffect::HealSelf(40.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Wisdom".to_string(), 12.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 8.0),
        ],
        evolution_potential: vec!["Bark Skin".to_string(), "Entangle".to_string(), "Animal Speech".to_string()],
        synergy_skills: vec!["Wisdom".to_string(), "Taming".to_string()],
        prerequisite_skills: vec!["Wisdom".to_string()],
    }
}

/// Healing Magic spells
pub fn create_healing_magic_skill() -> Skill {
    Skill {
        name: "Healing Magic".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Healing Focus".to_string(),
            description: "Enhances all healing abilities and recovery rates".to_string(),
            effects: vec![
                PassiveEffect::IncreaseRegeneration("health".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Vitality".to_string(), 0.2),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Heal".to_string(),
            description: "Restore health to self or ally".to_string(),
            activation_cost: ActivationCost::Mana(25.0),
            cooldown: SkillCooldown {
                base_duration: 3.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.02),
            },
            effects: vec![
                ActiveEffect::HealSelf(50.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Wisdom".to_string(), 10.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 8.0),
        ],
        evolution_potential: vec!["Greater Heal".to_string(), "Cure Disease".to_string(), "Resurrection".to_string()],
        synergy_skills: vec!["Wisdom".to_string(), "Vitality".to_string()],
        prerequisite_skills: vec!["Wisdom".to_string()],
    }
}

/// Death Magic spells
pub fn create_death_magic_skill() -> Skill {
    Skill {
        name: "Death Magic".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Death's Touch".to_string(),
            description: "Understanding of death magic grants resistance to life drain".to_string(),
            effects: vec![
                PassiveEffect::GrantResistance("necrotic".to_string(), 2.0),
                PassiveEffect::IncreaseDamage(1.5), // When using death spells
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Drain Life".to_string(),
            description: "Siphon life force from target to heal yourself".to_string(),
            activation_cost: ActivationCost::Combination(vec![
                ActivationCost::Mana(30.0),
                ActivationCost::Health(5.0),
            ]),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::DealDamage(35.0),
                ActiveEffect::HealSelf(20.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Intelligence".to_string(), 15.0),
            SkillRequirement::SkillLevel("Courage".to_string(), 12.0),
        ],
        evolution_potential: vec!["Soul Burn".to_string(), "Animate Dead".to_string(), "Death Ray".to_string()],
        synergy_skills: vec!["Intelligence".to_string()],
        prerequisite_skills: vec!["Intelligence".to_string()],
    }
}

/// Bardic skills
pub fn create_song_skill() -> Skill {
    Skill {
        name: "Song".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Persuasion,
        passive_trait: PassiveTrait {
            name: "Musical Aptitude".to_string(),
            description: "Natural musical ability enhances social interactions".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Charisma".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Inspire".to_string(),
            description: "Sing an inspiring song to boost nearby allies".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.2),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(5.0, vec![
                    ActiveEffect::TemporaryBuff("damage".to_string(), 15.0, 60.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Charisma".to_string(), 8.0),
        ],
        evolution_potential: vec!["Battle Hymn".to_string(), "Song of Rest".to_string()],
        synergy_skills: vec!["Charisma".to_string(), "Instruments".to_string()],
        prerequisite_skills: vec!["Charisma".to_string()],
    }
}

/// Taming skill
pub fn create_taming_skill() -> Skill {
    Skill {
        name: "Taming".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Survival,
        passive_trait: PassiveTrait {
            name: "Animal Kinship".to_string(),
            description: "Natural affinity with beasts and creatures".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Nature Magic".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Tame Beast".to_string(),
            description: "Attempt to tame a wild creature".to_string(),
            activation_cost: ActivationCost::Combination(vec![
                ActivationCost::Energy(40.0),
                ActivationCost::Mana(15.0),
            ]),
            cooldown: SkillCooldown {
                base_duration: 60.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.5),
            },
            effects: vec![
                // Placeholder - would need special taming logic
                ActiveEffect::TemporaryBuff("animal_friendship".to_string(), 100.0, 300.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Charisma".to_string(), 10.0),
            SkillRequirement::SkillLevel("Wisdom".to_string(), 8.0),
        ],
        evolution_potential: vec!["Beast Master".to_string(), "Pack Leader".to_string()],
        synergy_skills: vec!["Charisma".to_string(), "Nature Magic".to_string()],
        prerequisite_skills: vec!["Charisma".to_string()],
    }
}

/// Instrument skills
pub fn create_instrument_lute_skill() -> Skill {
    Skill {
        name: "Instrument: Lute".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Persuasion,
        passive_trait: PassiveTrait {
            name: "Lute Mastery".to_string(),
            description: "Skill with the lute enhances song effects".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Song".to_string(), 0.2),
                PassiveEffect::ReduceSkillCooldowns("Persuasion".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Melodic Performance".to_string(),
            description: "Play a captivating melody on the lute".to_string(),
            activation_cost: ActivationCost::Energy(15.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(8.0, vec![
                    ActiveEffect::TemporaryBuff("charm".to_string(), 20.0, 45.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Song".to_string(), 5.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 8.0),
        ],
        evolution_potential: vec!["Song of Courage".to_string(), "Lullaby".to_string()],
        synergy_skills: vec!["Song".to_string(), "Dexterity".to_string()],
        prerequisite_skills: vec!["Song".to_string()],
    }
}

/// Abjuration passive skills
pub fn create_abjuration_skill() -> Skill {
    Skill {
        name: "Abjuration".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Magical Protection".to_string(),
            description: "Understanding of protective magic grants natural defenses".to_string(),
            effects: vec![
                PassiveEffect::GrantResistance("magic".to_string(), 2.0),
                PassiveEffect::IncreaseDefense(0.5),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Magic Shield".to_string(),
            description: "Create a barrier that absorbs magical damage".to_string(),
            activation_cost: ActivationCost::Mana(35.0),
            cooldown: SkillCooldown {
                base_duration: 45.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.3),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("magic_shield".to_string(), 100.0, 120.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Intelligence".to_string(), 12.0),
            SkillRequirement::SkillLevel("Wisdom".to_string(), 10.0),
        ],
        evolution_potential: vec!["Dispel Magic".to_string(), "Counterspell".to_string()],
        synergy_skills: vec!["Intelligence".to_string(), "Wisdom".to_string()],
        prerequisite_skills: vec!["Intelligence".to_string()],
    }
}

/// Core attribute skill creators
pub fn create_dexterity_skill() -> Skill {
    Skill {
        name: "Dexterity".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Acrobatics,
        passive_trait: PassiveTrait {
            name: "Nimble".to_string(),
            description: "Improves critical hit chance, ranged accuracy and damage".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(0.5), // For ranged attacks
                PassiveEffect::IncreaseParryChance(1.0),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Precise Strike".to_string(),
            description: "Execute a precise attack with increased critical chance".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::DealDamage(45.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Archery".to_string(), "Sleight of Hand".to_string()],
        synergy_skills: vec!["Stealth".to_string(), "Acrobatics".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_courage_skill() -> Skill {
    Skill {
        name: "Courage".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Combat,
        passive_trait: PassiveTrait {
            name: "Brave Heart".to_string(),
            description: "Improves melee attack rating and resistance to fear".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.0), // For melee attacks
                PassiveEffect::GrantResistance("fear".to_string(), 2.0),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Battle Cry".to_string(),
            description: "Rally yourself and nearby allies with a inspiring shout".to_string(),
            activation_cost: ActivationCost::Energy(30.0),
            cooldown: SkillCooldown {
                base_duration: 60.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.4),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(6.0, vec![
                    ActiveEffect::TemporaryBuff("courage".to_string(), 25.0, 120.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Fearless".to_string(), "Intimidation".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Leadership".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_wisdom_skill() -> Skill {
    Skill {
        name: "Wisdom".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Wisdom,
        passive_trait: PassiveTrait {
            name: "Inner Knowledge".to_string(),
            description: "Enhances healing potency, mana regeneration, and slightly increases max mana".to_string(),
            effects: vec![
                PassiveEffect::IncreaseMaxMana(2.0),
                PassiveEffect::IncreaseRegeneration("mana".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Insight".to_string(),
            description: "Gain deep understanding of a situation or being".to_string(),
            activation_cost: ActivationCost::Mana(20.0),
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.2),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("perception".to_string(), 50.0, 180.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Healing Magic".to_string(), "Nature Magic".to_string()],
        synergy_skills: vec!["Intelligence".to_string(), "Vitality".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_strength_skill() -> Skill {
    Skill {
        name: "Strength".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Athletics,
        passive_trait: PassiveTrait {
            name: "Powerful Build".to_string(),
            description: "Slightly increases max hitpoints and melee damage".to_string(),
            effects: vec![
                PassiveEffect::IncreaseMaxHealth(3.0),
                PassiveEffect::IncreaseDamage(1.0), // For melee attacks
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: Some(ActiveTrait {
            name: "Power Attack".to_string(),
            description: "Channel your strength into a devastating blow".to_string(),
            activation_cost: ActivationCost::Energy(35.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::DealDamage(80.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Athletics".to_string(), "Two-Handed Weapons".to_string()],
        synergy_skills: vec!["Courage".to_string(), "Athletics".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_vitality_skill() -> Skill {
    Skill {
        name: "Vitality".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Health,
        passive_trait: PassiveTrait {
            name: "Hardy Constitution".to_string(),
            description: "Improves natural health regeneration and healing received".to_string(),
            effects: vec![
                PassiveEffect::IncreaseRegeneration("health".to_string(), 0.1),
                PassiveEffect::IncreaseMaxHealth(2.0),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Second Wind".to_string(),
            description: "Rapidly recover health and energy through force of will".to_string(),
            activation_cost: ActivationCost::None,
            cooldown: SkillCooldown {
                base_duration: 300.0, // 5 minutes
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(2.0),
            },
            effects: vec![
                ActiveEffect::HealSelf(50.0),
                ActiveEffect::RestoreVital("energy".to_string(), 40.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Healing Magic".to_string(), "Disease Resistance".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Wisdom".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_charisma_skill() -> Skill {
    Skill {
        name: "Charisma".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Persuasion,
        passive_trait: PassiveTrait {
            name: "Natural Charm".to_string(),
            description: "Improves social interactions and has some illusion magic influence".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Song".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Illusion Magic".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Persuade".to_string(),
            description: "Use your natural charm to influence others".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 45.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.3),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("social_influence".to_string(), 30.0, 300.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Illusion Magic".to_string(), "Song".to_string(), "Taming".to_string()],
        synergy_skills: vec!["Song".to_string(), "Illusion Magic".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

pub fn create_intelligence_skill() -> Skill {
    Skill {
        name: "Intelligence".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Intellect,
        passive_trait: PassiveTrait {
            name: "Quick Mind".to_string(),
            description: "Enhances learning speed and magical understanding".to_string(),
            effects: vec![
                PassiveEffect::IncreaseMaxMana(5.0),
                PassiveEffect::BoostSkillExperience("Arcane".to_string(), 0.2),
            ],
            formula: SkillFormula::Linear(0.05),
        },
        active_trait: Some(ActiveTrait {
            name: "Analyze".to_string(),
            description: "Quickly analyze a situation or object for weaknesses".to_string(),
            activation_cost: ActivationCost::Mana(15.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("analysis".to_string(), 40.0, 60.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: true,
        unlock_requirements: Vec::new(),
        evolution_potential: vec!["Illusion Magic".to_string(), "Death Magic".to_string(), "Abjuration".to_string()],
        synergy_skills: vec!["Wisdom".to_string(), "Abjuration".to_string()],
        prerequisite_skills: Vec::new(),
    }
}

/// Combat archetype skills - Fighter
pub fn create_fighter_skills() -> Vec<Skill> {
    vec![
        Skill {
            name: "Weapon Mastery".to_string(),
            level: 1.0,
            experience: 0.0,
            category: SkillCategory::Weapons,
            passive_trait: PassiveTrait {
                name: "Combat Expertise".to_string(),
                description: "Mastery with all weapon types".to_string(),
                effects: vec![
                    PassiveEffect::IncreaseDamage(2.0),
                    PassiveEffect::IncreaseParryChance(1.5),
                ],
                formula: SkillFormula::Linear(0.02),
            },
            active_trait: Some(ActiveTrait {
                name: "Flurry of Blows".to_string(),
                description: "Unleash a series of rapid strikes".to_string(),
                activation_cost: ActivationCost::Energy(40.0),
                cooldown: SkillCooldown {
                    base_duration: 30.0,
                    current_remaining: 0.0,
                    reduction_formula: SkillFormula::Linear(0.2),
                },
                effects: vec![
                    ActiveEffect::DealDamage(30.0), // Multiple hits
                    ActiveEffect::DealDamage(30.0),
                    ActiveEffect::DealDamage(30.0),
                ],
                can_be_hotkeyed: true,
            }),
            discovered_by: Vec::new(),
            is_innate: false,
            unlock_requirements: vec![
                SkillRequirement::SkillLevel("Strength".to_string(), 15.0),
                SkillRequirement::SkillLevel("Courage".to_string(), 15.0),
            ],
            evolution_potential: vec!["Weapon Specialist".to_string()],
            synergy_skills: vec!["Strength".to_string(), "Courage".to_string()],
            prerequisite_skills: vec!["Strength".to_string()],
        },
        // Additional fighter skills would go here...
    ]
}

/// Additional spell skills for magic schools

// Illusion Magic Spells
pub fn create_mesmerize_skill() -> Skill {
    Skill {
        name: "Mesmerize".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Hypnotic Presence".to_string(),
            description: "Your presence naturally charms and disorients enemies".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Charisma".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Mesmerize".to_string(),
            description: "Entrance an enemy, making them unable to act".to_string(),
            activation_cost: ActivationCost::Mana(40.0),
            cooldown: SkillCooldown {
                base_duration: 25.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::Stun(8.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Illusion Magic".to_string(), 20.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 15.0),
            SkillRequirement::SkillLevel("Charisma".to_string(), 18.0),
        ],
        evolution_potential: vec!["Mass Hypnosis".to_string()],
        synergy_skills: vec!["Illusion Magic".to_string(), "Charisma".to_string()],
        prerequisite_skills: vec!["Illusion Magic".to_string()],
    }
}

// Nature Magic Spells
pub fn create_bark_skin_skill() -> Skill {
    Skill {
        name: "Bark Skin".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Natural Armor".to_string(),
            description: "Your skin becomes tough like tree bark".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(1.0),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Bark Skin".to_string(),
            description: "Transform your skin into natural armor".to_string(),
            activation_cost: ActivationCost::Mana(30.0),
            cooldown: SkillCooldown {
                base_duration: 45.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.3),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("natural_armor".to_string(), 50.0, 300.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Nature Magic".to_string(), 15.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 12.0),
        ],
        evolution_potential: vec!["Stone Skin".to_string()],
        synergy_skills: vec!["Nature Magic".to_string(), "Vitality".to_string()],
        prerequisite_skills: vec!["Nature Magic".to_string()],
    }
}

pub fn create_entangle_skill() -> Skill {
    Skill {
        name: "Entangle".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Plant Control".to_string(),
            description: "Natural affinity with plant life".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Nature Magic".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Entangle".to_string(),
            description: "Cause vines and roots to emerge and restrain enemies".to_string(),
            activation_cost: ActivationCost::Mana(35.0),
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.2),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(4.0, vec![
                    ActiveEffect::Stun(6.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Nature Magic".to_string(), 18.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 12.0),
        ],
        evolution_potential: vec!["Thorn Wall".to_string()],
        synergy_skills: vec!["Nature Magic".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Nature Magic".to_string()],
    }
}

// Healing Magic Spells
pub fn create_greater_heal_skill() -> Skill {
    Skill {
        name: "Greater Heal".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Enhanced Healing".to_string(),
            description: "All healing effects are more potent".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Healing Magic".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Greater Heal".to_string(),
            description: "Powerful healing magic that restores significant health".to_string(),
            activation_cost: ActivationCost::Mana(50.0),
            cooldown: SkillCooldown {
                base_duration: 8.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.05),
            },
            effects: vec![
                ActiveEffect::HealSelf(120.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Healing Magic".to_string(), 20.0),
            SkillRequirement::SkillLevel("Wisdom".to_string(), 18.0),
        ],
        evolution_potential: vec!["Mass Heal".to_string()],
        synergy_skills: vec!["Healing Magic".to_string(), "Wisdom".to_string()],
        prerequisite_skills: vec!["Healing Magic".to_string()],
    }
}

pub fn create_cure_disease_skill() -> Skill {
    Skill {
        name: "Cure Disease".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Divine,
        passive_trait: PassiveTrait {
            name: "Disease Resistance".to_string(),
            description: "Natural immunity to diseases and poisons".to_string(),
            effects: vec![
                PassiveEffect::GrantResistance("disease".to_string(), 3.0),
                PassiveEffect::GrantResistance("poison".to_string(), 2.0),
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: Some(ActiveTrait {
            name: "Cure Disease".to_string(),
            description: "Remove diseases and poisons from target".to_string(),
            activation_cost: ActivationCost::Mana(30.0),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::HealSelf(40.0), // Also heals
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Healing Magic".to_string(), 15.0),
            SkillRequirement::SkillLevel("Wisdom".to_string(), 15.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 10.0),
        ],
        evolution_potential: vec!["Purification".to_string()],
        synergy_skills: vec!["Healing Magic".to_string(), "Wisdom".to_string()],
        prerequisite_skills: vec!["Healing Magic".to_string()],
    }
}

// Death Magic Spells
pub fn create_soul_burn_skill() -> Skill {
    Skill {
        name: "Soul Burn".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Soul Sight".to_string(),
            description: "Ability to see and affect spiritual essence".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.5), // Spiritual damage
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Soul Burn".to_string(),
            description: "Directly damage the target's soul".to_string(),
            activation_cost: ActivationCost::Combination(vec![
                ActivationCost::Mana(45.0),
                ActivationCost::Health(10.0),
            ]),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.12),
            },
            effects: vec![
                ActiveEffect::DealDamage(80.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Death Magic".to_string(), 22.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 20.0),
        ],
        evolution_potential: vec!["Soul Rend".to_string()],
        synergy_skills: vec!["Death Magic".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Death Magic".to_string()],
    }
}

pub fn create_animate_dead_skill() -> Skill {
    Skill {
        name: "Animate Dead".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Necromantic Mastery".to_string(),
            description: "Understanding of undeath and necromancy".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Death Magic".to_string(), 0.2),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Animate Dead".to_string(),
            description: "Raise a temporary undead minion".to_string(),
            activation_cost: ActivationCost::Combination(vec![
                ActivationCost::Mana(60.0),
                ActivationCost::Health(15.0),
            ]),
            cooldown: SkillCooldown {
                base_duration: 120.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(1.0),
            },
            effects: vec![
                // Placeholder - would need minion summoning system
                ActiveEffect::TemporaryBuff("undead_minion".to_string(), 1.0, 600.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Death Magic".to_string(), 25.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 22.0),
            SkillRequirement::SkillLevel("Charisma".to_string(), 15.0),
        ],
        evolution_potential: vec!["Create Undead".to_string()],
        synergy_skills: vec!["Death Magic".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Death Magic".to_string()],
    }
}

// Abjuration Spells
pub fn create_dispel_magic_skill() -> Skill {
    Skill {
        name: "Dispel Magic".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Magic Disruption".to_string(),
            description: "Natural ability to interfere with magical effects".to_string(),
            effects: vec![
                PassiveEffect::GrantResistance("magic".to_string(), 1.5),
            ],
            formula: SkillFormula::Linear(0.015),
        },
        active_trait: Some(ActiveTrait {
            name: "Dispel Magic".to_string(),
            description: "Remove magical effects from target".to_string(),
            activation_cost: ActivationCost::Mana(35.0),
            cooldown: SkillCooldown {
                base_duration: 18.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.12),
            },
            effects: vec![
                // Placeholder - would need dispel mechanics
                ActiveEffect::TemporaryBuff("dispel_aura".to_string(), 100.0, 30.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Abjuration".to_string(), 18.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 16.0),
            SkillRequirement::SkillLevel("Wisdom".to_string(), 14.0),
        ],
        evolution_potential: vec!["Greater Dispel".to_string()],
        synergy_skills: vec!["Abjuration".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Abjuration".to_string()],
    }
}

pub fn create_counterspell_skill() -> Skill {
    Skill {
        name: "Counterspell".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Arcane,
        passive_trait: PassiveTrait {
            name: "Spell Awareness".to_string(),
            description: "Heightened awareness of incoming magical attacks".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Abjuration".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Counterspell".to_string(),
            description: "Interrupt and counter an enemy's spell casting".to_string(),
            activation_cost: ActivationCost::Mana(25.0),
            cooldown: SkillCooldown {
                base_duration: 12.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.08),
            },
            effects: vec![
                ActiveEffect::Stun(3.0), // Interrupts casting
                ActiveEffect::TemporaryBuff("spell_reflect".to_string(), 50.0, 15.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Abjuration".to_string(), 20.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 18.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 12.0),
        ],
        evolution_potential: vec!["Spell Turning".to_string()],
        synergy_skills: vec!["Abjuration".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Abjuration".to_string()],
    }
}

// Crafting Skills
pub fn create_gathering_skill() -> Skill {
    Skill {
        name: "Gathering".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Survival,
        passive_trait: PassiveTrait {
            name: "Resource Sense".to_string(),
            description: "Ability to find and identify valuable resources".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Nature Magic".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Efficient Harvesting".to_string(),
            description: "Gather resources more effectively".to_string(),
            activation_cost: ActivationCost::Energy(15.0),
            cooldown: SkillCooldown {
                base_duration: 60.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.4),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("gathering_bonus".to_string(), 100.0, 300.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 5.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 3.0),
        ],
        evolution_potential: vec!["Master Gatherer".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Nature Magic".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

// Martial Arts Skills for Monk Archetype

/// Passive-only martial arts skills
pub fn create_iron_body_skill() -> Skill {
    Skill {
        name: "Iron Body".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Conditioned Body".to_string(),
            description: "Years of training have hardened your body like iron".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(1.5),
                PassiveEffect::GrantResistance("physical".to_string(), 2.0),
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: None,
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Vitality".to_string(), 15.0),
            SkillRequirement::SkillLevel("Strength".to_string(), 12.0),
        ],
        evolution_potential: vec!["Diamond Body".to_string()],
        synergy_skills: vec!["Vitality".to_string(), "Martial Arts".to_string()],
        prerequisite_skills: vec!["Vitality".to_string()],
    }
}

pub fn create_inner_peace_skill() -> Skill {
    Skill {
        name: "Inner Peace".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Focus,
        passive_trait: PassiveTrait {
            name: "Calm Mind".to_string(),
            description: "Mental discipline provides resistance to fear and confusion".to_string(),
            effects: vec![
                PassiveEffect::GrantResistance("mental".to_string(), 3.0),
                PassiveEffect::IncreaseRegeneration("mana".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: None,
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Wisdom".to_string(), 18.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 12.0),
        ],
        evolution_potential: vec!["Enlightenment".to_string()],
        synergy_skills: vec!["Wisdom".to_string(), "Martial Arts".to_string()],
        prerequisite_skills: vec!["Wisdom".to_string()],
    }
}

pub fn create_ki_flow_skill() -> Skill {
    Skill {
        name: "Ki Flow".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Energy,
        passive_trait: PassiveTrait {
            name: "Energy Circulation".to_string(),
            description: "Understanding of ki flow increases energy regeneration".to_string(),
            effects: vec![
                PassiveEffect::IncreaseRegeneration("energy".to_string(), 0.15),
                PassiveEffect::IncreaseMaxEnergy(5.0),
            ],
            formula: SkillFormula::Linear(0.05),
        },
        active_trait: None,
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Intelligence".to_string(), 15.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 12.0),
        ],
        evolution_potential: vec!["Ki Master".to_string()],
        synergy_skills: vec!["Intelligence".to_string(), "Martial Arts".to_string()],
        prerequisite_skills: vec!["Intelligence".to_string()],
    }
}

pub fn create_combat_reflexes_skill() -> Skill {
    Skill {
        name: "Combat Reflexes".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Lightning Reflexes".to_string(),
            description: "Enhanced reaction time in combat situations".to_string(),
            effects: vec![
                PassiveEffect::IncreaseParryChance(2.0),
                PassiveEffect::ModifyMovementSpeed(0.5),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: None,
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 18.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 10.0),
        ],
        evolution_potential: vec!["Precognition".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Martial Arts".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_pressure_points_skill() -> Skill {
    Skill {
        name: "Pressure Points".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Anatomical Knowledge".to_string(),
            description: "Knowledge of vital points increases critical hit chance".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.0), // Critical hits
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: None,
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Intelligence".to_string(), 20.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 15.0),
        ],
        evolution_potential: vec!["Death Touch".to_string()],
        synergy_skills: vec!["Intelligence".to_string(), "Martial Arts".to_string()],
        prerequisite_skills: vec!["Intelligence".to_string()],
    }
}

/// Active martial arts skills with passive traits
pub fn create_martial_arts_skill() -> Skill {
    Skill {
        name: "Martial Arts".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Unarmed Combat Mastery".to_string(),
            description: "Training in martial arts increases unarmed damage and technique".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(2.0), // Unarmed combat
                PassiveEffect::BoostSkillExperience("Dexterity".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Flying Kick".to_string(),
            description: "Launch a powerful aerial kick attack".to_string(),
            activation_cost: ActivationCost::Energy(30.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::DealDamage(60.0),
                ActiveEffect::Knockback(3.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 10.0),
            SkillRequirement::SkillLevel("Strength".to_string(), 8.0),
        ],
        evolution_potential: vec!["Iron Body".to_string(), "Ki Flow".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Strength".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_ki_strike_skill() -> Skill {
    Skill {
        name: "Ki Strike".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Ki Enhancement".to_string(),
            description: "Channel ki energy to enhance physical attacks".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.5),
            ],
            formula: SkillFormula::Linear(0.025),
        },
        active_trait: Some(ActiveTrait {
            name: "Ki Blast".to_string(),
            description: "Project ki energy as a ranged attack".to_string(),
            activation_cost: ActivationCost::Combination(vec![
                ActivationCost::Energy(25.0),
                ActivationCost::Mana(15.0),
            ]),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::DealDamage(55.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Martial Arts".to_string(), 15.0),
            SkillRequirement::SkillLevel("Ki Flow".to_string(), 10.0),
        ],
        evolution_potential: vec!["Ki Master".to_string()],
        synergy_skills: vec!["Martial Arts".to_string(), "Ki Flow".to_string()],
        prerequisite_skills: vec!["Martial Arts".to_string()],
    }
}

pub fn create_whirlwind_strike_skill() -> Skill {
    Skill {
        name: "Whirlwind Strike".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Spinning Technique".to_string(),
            description: "Mastery of spinning attacks increases area damage".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.0),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Spinning Assault".to_string(),
            description: "Attack all nearby enemies with a spinning strike".to_string(),
            activation_cost: ActivationCost::Energy(40.0),
            cooldown: SkillCooldown {
                base_duration: 25.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.2),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(3.0, vec![
                    ActiveEffect::DealDamage(45.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Martial Arts".to_string(), 12.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 15.0),
        ],
        evolution_potential: vec!["Hurricane Strike".to_string()],
        synergy_skills: vec!["Martial Arts".to_string(), "Dexterity".to_string()],
        prerequisite_skills: vec!["Martial Arts".to_string()],
    }
}

pub fn create_stunning_fist_skill() -> Skill {
    Skill {
        name: "Stunning Fist".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Unarmed,
        passive_trait: PassiveTrait {
            name: "Precise Strikes".to_string(),
            description: "Training in precise striking increases stun chance".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(0.5),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Paralyzing Strike".to_string(),
            description: "Strike nerve clusters to temporarily paralyze target".to_string(),
            activation_cost: ActivationCost::Energy(35.0),
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.25),
            },
            effects: vec![
                ActiveEffect::DealDamage(40.0),
                ActiveEffect::Stun(5.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Martial Arts".to_string(), 18.0),
            SkillRequirement::SkillLevel("Pressure Points".to_string(), 12.0),
        ],
        evolution_potential: vec!["Death Touch".to_string()],
        synergy_skills: vec!["Martial Arts".to_string(), "Pressure Points".to_string()],
        prerequisite_skills: vec!["Martial Arts".to_string()],
    }
}

pub fn create_meditation_skill() -> Skill {
    Skill {
        name: "Meditation".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Focus,
        passive_trait: PassiveTrait {
            name: "Mental Clarity".to_string(),
            description: "Regular meditation improves focus and energy recovery".to_string(),
            effects: vec![
                PassiveEffect::IncreaseRegeneration("mana".to_string(), 0.1),
                PassiveEffect::IncreaseRegeneration("energy".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Deep Meditation".to_string(),
            description: "Enter a deep meditative state to rapidly restore energy".to_string(),
            activation_cost: ActivationCost::None,
            cooldown: SkillCooldown {
                base_duration: 180.0, // 3 minutes
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(1.0),
            },
            effects: vec![
                ActiveEffect::RestoreVital("mana".to_string(), 80.0),
                ActiveEffect::RestoreVital("energy".to_string(), 60.0),
                ActiveEffect::HealSelf(30.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Wisdom".to_string(), 12.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 10.0),
        ],
        evolution_potential: vec!["Inner Peace".to_string()],
        synergy_skills: vec!["Wisdom".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Wisdom".to_string()],
    }
}

// Weapon Skills

pub fn create_sword_skill() -> Skill {
    Skill {
        name: "Sword".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Blade Mastery".to_string(),
            description: "Training with swords increases accuracy and damage".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.5), // When using swords
                PassiveEffect::IncreaseParryChance(1.0),
            ],
            formula: SkillFormula::Linear(0.015),
        },
        active_trait: Some(ActiveTrait {
            name: "Precision Cut".to_string(),
            description: "Execute a precise sword strike with increased critical chance".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 12.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.08),
            },
            effects: vec![
                ActiveEffect::DealDamage(50.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 8.0),
            SkillRequirement::SkillLevel("Strength".to_string(), 6.0),
        ],
        evolution_potential: vec!["Master Swordsman".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "One-Handed Weapon Mastery".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_clubs_skill() -> Skill {
    Skill {
        name: "Clubs".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Blunt Force Mastery".to_string(),
            description: "Training with clubs increases stunning power".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(2.0), // When using clubs
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Crushing Blow".to_string(),
            description: "Deliver a devastating blow that can stun enemies".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::DealDamage(55.0),
                ActiveEffect::Stun(3.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 10.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 6.0),
        ],
        evolution_potential: vec!["Warhammer Mastery".to_string()],
        synergy_skills: vec!["Strength".to_string(), "One-Handed Weapon Mastery".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

// Additional Weapon Skills

pub fn create_hammers_skill() -> Skill {
    Skill {
        name: "Hammers".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Hammer Mastery".to_string(),
            description: "Training with hammers increases armor penetration".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(2.5), // High armor pen
            ],
            formula: SkillFormula::Linear(0.025),
        },
        active_trait: Some(ActiveTrait {
            name: "Shatter Strike".to_string(),
            description: "A devastating blow that ignores armor".to_string(),
            activation_cost: ActivationCost::Energy(30.0),
            cooldown: SkillCooldown {
                base_duration: 18.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.12),
            },
            effects: vec![
                ActiveEffect::DealDamage(70.0), // High damage, ignores armor
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 12.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 8.0),
        ],
        evolution_potential: vec!["Thunder Hammer".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Two-Handed Weapon Mastery".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_dagger_skill() -> Skill {
    Skill {
        name: "Dagger".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Stealth Blade".to_string(),
            description: "Dagger training increases critical hit chance and stealth".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.0), // High crit
                PassiveEffect::BoostSkillExperience("Stealth".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Backstab".to_string(),
            description: "Strike from stealth with massive damage multiplier".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::DealDamage(80.0), // Very high stealth damage
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 12.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 8.0),
        ],
        evolution_potential: vec!["Assassinate".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Stealth".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_knuckles_skill() -> Skill {
    Skill {
        name: "Knuckles".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Brass Knuckle Mastery".to_string(),
            description: "Knuckle weapons enhance unarmed combat techniques".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.8),
                PassiveEffect::BoostSkillExperience("Martial Arts".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.018),
        },
        active_trait: Some(ActiveTrait {
            name: "Rapid Punches".to_string(),
            description: "Unleash a flurry of quick punches".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 10.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.06),
            },
            effects: vec![
                ActiveEffect::DealDamage(25.0),
                ActiveEffect::DealDamage(25.0),
                ActiveEffect::DealDamage(25.0), // Triple hit
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Martial Arts".to_string(), 8.0),
            SkillRequirement::SkillLevel("Strength".to_string(), 6.0),
        ],
        evolution_potential: vec!["Iron Fist".to_string()],
        synergy_skills: vec!["Martial Arts".to_string(), "Strength".to_string()],
        prerequisite_skills: vec!["Martial Arts".to_string()],
    }
}

pub fn create_shield_skill() -> Skill {
    Skill {
        name: "Shield".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Defense,
        passive_trait: PassiveTrait {
            name: "Shield Wall".to_string(),
            description: "Shield mastery greatly increases blocking ability".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(2.0),
                PassiveEffect::IncreaseParryChance(3.0),
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: Some(ActiveTrait {
            name: "Shield Bash".to_string(),
            description: "Strike with your shield to stun enemies".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 15.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.1),
            },
            effects: vec![
                ActiveEffect::DealDamage(35.0),
                ActiveEffect::Stun(3.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 10.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 8.0),
        ],
        evolution_potential: vec!["Tower Shield Mastery".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Defense".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_polearm_skill() -> Skill {
    Skill {
        name: "Polearm".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Reach Mastery".to_string(),
            description: "Polearm training provides extended reach and crowd control".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(2.0),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Sweeping Strike".to_string(),
            description: "Sweep in an arc to hit multiple enemies".to_string(),
            activation_cost: ActivationCost::Energy(35.0),
            cooldown: SkillCooldown {
                base_duration: 22.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.16),
            },
            effects: vec![
                ActiveEffect::AreaOfEffect(4.0, vec![
                    ActiveEffect::DealDamage(45.0),
                    ActiveEffect::Knockback(2.0),
                ]),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 12.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 10.0),
        ],
        evolution_potential: vec!["Halberd Mastery".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Two-Handed Weapon Mastery".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_stave_skill() -> Skill {
    Skill {
        name: "Stave".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Staff Combat".to_string(),
            description: "Staff training enhances both combat and magical abilities".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.2),
                PassiveEffect::BoostSkillExperience("Intelligence".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.015),
        },
        active_trait: Some(ActiveTrait {
            name: "Staff Whirl".to_string(),
            description: "Spin the staff defensively while attacking".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 18.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.12),
            },
            effects: vec![
                ActiveEffect::DealDamage(40.0),
                ActiveEffect::TemporaryBuff("defense".to_string(), 30.0, 15.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 10.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 8.0),
        ],
        evolution_potential: vec!["Arcane Staff Mastery".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

// Weapon Mastery Skills (Fighting Styles)

pub fn create_one_handed_weapon_mastery_skill() -> Skill {
    Skill {
        name: "One-Handed Weapon Mastery".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Single Weapon Focus".to_string(),
            description: "Specialization in one-handed weapons increases speed and accuracy".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(1.5), // For one-handed weapons
                PassiveEffect::ModifyMovementSpeed(0.3),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Perfect Strike".to_string(),
            description: "Execute a flawless one-handed weapon technique".to_string(),
            activation_cost: ActivationCost::Energy(22.0),
            cooldown: SkillCooldown {
                base_duration: 14.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.09),
            },
            effects: vec![
                ActiveEffect::DealDamage(55.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 15.0),
            SkillRequirement::CombinedSkills(vec![
                ("Sword".to_string(), 8.0),
            ]),
        ],
        evolution_potential: vec!["Weapon Master".to_string()],
        synergy_skills: vec!["Sword".to_string(), "Dagger".to_string(), "Clubs".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_dual_wield_mastery_skill() -> Skill {
    Skill {
        name: "Dual Wield Mastery".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Twin Weapon Harmony".to_string(),
            description: "Master coordination with two weapons increases attack rate".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(2.0), // When dual wielding
                PassiveEffect::IncreaseParryChance(1.5),
            ],
            formula: SkillFormula::Linear(0.025),
        },
        active_trait: Some(ActiveTrait {
            name: "Flurry".to_string(),
            description: "Unleash rapid strikes with both weapons".to_string(),
            activation_cost: ActivationCost::Energy(40.0),
            cooldown: SkillCooldown {
                base_duration: 25.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.18),
            },
            effects: vec![
                ActiveEffect::DealDamage(35.0),
                ActiveEffect::DealDamage(35.0),
                ActiveEffect::DealDamage(35.0),
                ActiveEffect::DealDamage(35.0), // Quad strike
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 18.0),
            SkillRequirement::SkillLevel("One-Handed Weapon Mastery".to_string(), 12.0),
        ],
        evolution_potential: vec!["Whirling Blades".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "One-Handed Weapon Mastery".to_string()],
        prerequisite_skills: vec!["One-Handed Weapon Mastery".to_string()],
    }
}

pub fn create_two_handed_weapon_mastery_skill() -> Skill {
    Skill {
        name: "Two-Handed Weapon Mastery".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Weapons,
        passive_trait: PassiveTrait {
            name: "Heavy Weapon Focus".to_string(),
            description: "Mastery of two-handed weapons increases power and reach".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDamage(3.0), // High damage for 2H weapons
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: Some(ActiveTrait {
            name: "Mighty Swing".to_string(),
            description: "Deliver a devastating two-handed strike".to_string(),
            activation_cost: ActivationCost::Energy(35.0),
            cooldown: SkillCooldown {
                base_duration: 20.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.15),
            },
            effects: vec![
                ActiveEffect::DealDamage(90.0), // Very high damage
                ActiveEffect::Knockback(4.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 18.0),
            SkillRequirement::CombinedSkills(vec![
                ("Hammers".to_string(), 10.0),
            ]),
        ],
        evolution_potential: vec!["Great Weapon Master".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Hammers".to_string(), "Polearm".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

// Armor Skills

pub fn create_light_armor_skill() -> Skill {
    Skill {
        name: "Light Armor".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Armor,
        passive_trait: PassiveTrait {
            name: "Mobility Focus".to_string(),
            description: "Light armor training maintains mobility while providing protection".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(0.8),
                PassiveEffect::ModifyMovementSpeed(0.2), // Speed bonus
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Evasive Maneuvers".to_string(),
            description: "Use armor flexibility to avoid attacks".to_string(),
            activation_cost: ActivationCost::Energy(15.0),
            cooldown: SkillCooldown {
                base_duration: 25.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.18),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("evasion".to_string(), 50.0, 10.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 8.0),
        ],
        evolution_potential: vec!["Shadow Armor".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Stealth".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_medium_armor_skill() -> Skill {
    Skill {
        name: "Medium Armor".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Armor,
        passive_trait: PassiveTrait {
            name: "Balanced Protection".to_string(),
            description: "Medium armor provides balanced defense and mobility".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(1.5),
                PassiveEffect::IncreaseMaxHealth(5.0),
            ],
            formula: SkillFormula::Linear(0.015),
        },
        active_trait: Some(ActiveTrait {
            name: "Armor Adaptation".to_string(),
            description: "Adjust armor positioning for maximum protection".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 45.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.3),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("defense".to_string(), 40.0, 60.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 10.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 8.0),
        ],
        evolution_potential: vec!["Adaptive Armor".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Vitality".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_heavy_armor_skill() -> Skill {
    Skill {
        name: "Heavy Armor".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Armor,
        passive_trait: PassiveTrait {
            name: "Fortress Defense".to_string(),
            description: "Heavy armor mastery provides maximum protection".to_string(),
            effects: vec![
                PassiveEffect::IncreaseDefense(3.0),
                PassiveEffect::IncreaseMaxHealth(10.0),
                PassiveEffect::GrantResistance("physical".to_string(), 2.0),
            ],
            formula: SkillFormula::Linear(0.03),
        },
        active_trait: Some(ActiveTrait {
            name: "Immovable Defense".to_string(),
            description: "Become an immovable defensive wall".to_string(),
            activation_cost: ActivationCost::Energy(40.0),
            cooldown: SkillCooldown {
                base_duration: 60.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.4),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("immobile_defense".to_string(), 100.0, 30.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 15.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 12.0),
        ],
        evolution_potential: vec!["Legendary Armor".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Vitality".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_mining_skill() -> Skill {
    Skill {
        name: "Mining".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Mineral Knowledge".to_string(),
            description: "Understanding of ores, gems, and underground resources".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Strength".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.01),
        },
        active_trait: Some(ActiveTrait {
            name: "Power Strike".to_string(),
            description: "Strike with enhanced force to break through tough materials".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 30.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.2),
            },
            effects: vec![
                ActiveEffect::TemporaryBuff("mining_power".to_string(), 200.0, 60.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 8.0),
            SkillRequirement::SkillLevel("Vitality".to_string(), 6.0),
        ],
        evolution_potential: vec!["Gemcrafting".to_string()],
        synergy_skills: vec!["Strength".to_string(), "Vitality".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

/// A comprehensive skill registry to manage all skills
pub struct SkillRegistry {
    pub skills: HashMap<String, fn() -> Skill>,
}

/// Construction and Crafting Skills for NPC autonomous building and creation

pub fn create_smelting_skill() -> Skill {
    Skill {
        name: "Smelting".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Smithing,
        passive_trait: PassiveTrait {
            name: "Metallurgy Knowledge".to_string(),
            description: "Understanding of metal properties and ore processing".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Blacksmithing".to_string(), 0.15),
                PassiveEffect::BoostSkillExperience("Engineering".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Refine Ore".to_string(),
            description: "Process raw ore into refined metal ingots".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 60.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.5),
            },
            effects: vec![
                ActiveEffect::CraftItem("Metal Ingot".to_string(), 1),
                ActiveEffect::GrantExperience("Smelting".to_string(), 15.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Gathering".to_string(), 8.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 12.0),
        ],
        evolution_potential: vec!["Advanced Metallurgy".to_string(), "Alloy Creation".to_string()],
        synergy_skills: vec!["Blacksmithing".to_string(), "Engineering".to_string()],
        prerequisite_skills: vec!["Gathering".to_string()],
    }
}

pub fn create_carpentry_skill() -> Skill {
    Skill {
        name: "Carpentry".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Woodworking Expertise".to_string(),
            description: "Knowledge of wood properties and construction techniques".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Architecture".to_string(), 0.2),
                PassiveEffect::BoostSkillExperience("Tool Making".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Craft Furniture".to_string(),
            description: "Create wooden furniture and structures".to_string(),
            activation_cost: ActivationCost::Energy(30.0),
            cooldown: SkillCooldown {
                base_duration: 90.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.8),
            },
            effects: vec![
                ActiveEffect::CraftItem("Wooden Furniture".to_string(), 1),
                ActiveEffect::GrantExperience("Carpentry".to_string(), 20.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Tool Making".to_string(), 5.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 10.0),
        ],
        evolution_potential: vec!["Fine Woodworking".to_string(), "Ship Building".to_string()],
        synergy_skills: vec!["Architecture".to_string(), "Tool Making".to_string()],
        prerequisite_skills: vec!["Tool Making".to_string()],
    }
}

pub fn create_masonry_skill() -> Skill {
    Skill {
        name: "Masonry".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Stone Shaping".to_string(),
            description: "Skill in working with stone and building lasting structures".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Architecture".to_string(), 0.25),
                PassiveEffect::BoostSkillExperience("Strength".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Shape Stone".to_string(),
            description: "Cut and shape stone blocks for construction".to_string(),
            activation_cost: ActivationCost::Energy(40.0),
            cooldown: SkillCooldown {
                base_duration: 120.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(1.0),
            },
            effects: vec![
                ActiveEffect::CraftItem("Stone Block".to_string(), 1),
                ActiveEffect::GrantExperience("Masonry".to_string(), 25.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Strength".to_string(), 15.0),
            SkillRequirement::SkillLevel("Tool Making".to_string(), 8.0),
        ],
        evolution_potential: vec!["Advanced Masonry".to_string(), "Sculptural Arts".to_string()],
        synergy_skills: vec!["Architecture".to_string(), "Strength".to_string()],
        prerequisite_skills: vec!["Strength".to_string()],
    }
}

pub fn create_architecture_skill() -> Skill {
    Skill {
        name: "Architecture".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Structural Design".to_string(),
            description: "Knowledge of structural engineering and building design".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Masonry".to_string(), 0.15),
                PassiveEffect::BoostSkillExperience("Carpentry".to_string(), 0.15),
                PassiveEffect::BoostSkillExperience("Intelligence".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Design Building".to_string(),
            description: "Create blueprints for complex structures".to_string(),
            activation_cost: ActivationCost::Energy(50.0),
            cooldown: SkillCooldown {
                base_duration: 300.0, // 5 minutes for complex design
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(2.0),
            },
            effects: vec![
                ActiveEffect::CraftItem("Building Blueprint".to_string(), 1),
                ActiveEffect::GrantExperience("Architecture".to_string(), 40.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Carpentry".to_string(), 12.0),
            SkillRequirement::SkillLevel("Masonry".to_string(), 12.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 18.0),
        ],
        evolution_potential: vec!["City Planning".to_string(), "Engineering Marvels".to_string()],
        synergy_skills: vec!["Carpentry".to_string(), "Masonry".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Carpentry".to_string(), "Masonry".to_string()],
    }
}

pub fn create_tool_making_skill() -> Skill {
    Skill {
        name: "Tool Making".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Smithing,
        passive_trait: PassiveTrait {
            name: "Tool Expertise".to_string(),
            description: "Knowledge of creating and maintaining various tools".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Carpentry".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Blacksmithing".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Dexterity".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Craft Basic Tool".to_string(),
            description: "Create simple tools for various tasks".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 45.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.4),
            },
            effects: vec![
                ActiveEffect::CraftItem("Basic Tool".to_string(), 1),
                ActiveEffect::GrantExperience("Tool Making".to_string(), 12.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Dexterity".to_string(), 8.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 6.0),
        ],
        evolution_potential: vec!["Advanced Tool Making".to_string(), "Specialized Tools".to_string()],
        synergy_skills: vec!["Dexterity".to_string(), "Blacksmithing".to_string()],
        prerequisite_skills: vec!["Dexterity".to_string()],
    }
}

pub fn create_blacksmithing_skill() -> Skill {
    Skill {
        name: "Blacksmithing".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Smithing,
        passive_trait: PassiveTrait {
            name: "Forge Mastery".to_string(),
            description: "Skill in metalworking and weapon/tool creation".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Smelting".to_string(), 0.15),
                PassiveEffect::BoostSkillExperience("Tool Making".to_string(), 0.2),
                PassiveEffect::BoostSkillExperience("Strength".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Forge Weapon".to_string(),
            description: "Create metal weapons and armor pieces".to_string(),
            activation_cost: ActivationCost::Energy(60.0),
            cooldown: SkillCooldown {
                base_duration: 180.0, // 3 minutes for weapon crafting
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(1.5),
            },
            effects: vec![
                ActiveEffect::CraftItem("Metal Weapon".to_string(), 1),
                ActiveEffect::GrantExperience("Blacksmithing".to_string(), 35.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Smelting".to_string(), 10.0),
            SkillRequirement::SkillLevel("Tool Making".to_string(), 8.0),
            SkillRequirement::SkillLevel("Strength".to_string(), 12.0),
        ],
        evolution_potential: vec!["Master Smithing".to_string(), "Enchanted Smithing".to_string()],
        synergy_skills: vec!["Smelting".to_string(), "Tool Making".to_string(), "Strength".to_string()],
        prerequisite_skills: vec!["Smelting".to_string(), "Tool Making".to_string()],
    }
}

pub fn create_leatherworking_skill() -> Skill {
    Skill {
        name: "Leatherworking".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Tailoring,
        passive_trait: PassiveTrait {
            name: "Hide Processing".to_string(),
            description: "Knowledge of leather preparation and crafting".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Gathering".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Dexterity".to_string(), 0.05),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Craft Leather Goods".to_string(),
            description: "Create leather armor, bags, and other items".to_string(),
            activation_cost: ActivationCost::Energy(35.0),
            cooldown: SkillCooldown {
                base_duration: 75.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.6),
            },
            effects: vec![
                ActiveEffect::CraftItem("Leather Item".to_string(), 1),
                ActiveEffect::GrantExperience("Leatherworking".to_string(), 18.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Gathering".to_string(), 6.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 10.0),
        ],
        evolution_potential: vec!["Master Leatherworking".to_string(), "Exotic Leathers".to_string()],
        synergy_skills: vec!["Gathering".to_string(), "Dexterity".to_string()],
        prerequisite_skills: vec!["Gathering".to_string()],
    }
}

pub fn create_weaving_skill() -> Skill {
    Skill {
        name: "Weaving".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Tailoring,
        passive_trait: PassiveTrait {
            name: "Textile Mastery".to_string(),
            description: "Skill in creating cloth and fabric items".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Gathering".to_string(), 0.1),
                PassiveEffect::BoostSkillExperience("Dexterity".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Weave Cloth".to_string(),
            description: "Create cloth and textile goods".to_string(),
            activation_cost: ActivationCost::Energy(25.0),
            cooldown: SkillCooldown {
                base_duration: 50.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.4),
            },
            effects: vec![
                ActiveEffect::CraftItem("Cloth".to_string(), 1),
                ActiveEffect::GrantExperience("Weaving".to_string(), 15.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Gathering".to_string(), 5.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 8.0),
        ],
        evolution_potential: vec!["Fine Weaving".to_string(), "Magical Textiles".to_string()],
        synergy_skills: vec!["Gathering".to_string(), "Dexterity".to_string()],
        prerequisite_skills: vec!["Gathering".to_string()],
    }
}

pub fn create_pottery_skill() -> Skill {
    Skill {
        name: "Pottery".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Clay Working".to_string(),
            description: "Knowledge of clay preparation and ceramic creation".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Gathering".to_string(), 0.05),
                PassiveEffect::BoostSkillExperience("Dexterity".to_string(), 0.1),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Shape Pottery".to_string(),
            description: "Create pottery and ceramic containers".to_string(),
            activation_cost: ActivationCost::Energy(20.0),
            cooldown: SkillCooldown {
                base_duration: 40.0,
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(0.3),
            },
            effects: vec![
                ActiveEffect::CraftItem("Pottery".to_string(), 1),
                ActiveEffect::GrantExperience("Pottery".to_string(), 12.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Gathering".to_string(), 3.0),
            SkillRequirement::SkillLevel("Dexterity".to_string(), 6.0),
        ],
        evolution_potential: vec!["Fine Ceramics".to_string(), "Artistic Pottery".to_string()],
        synergy_skills: vec!["Gathering".to_string(), "Dexterity".to_string()],
        prerequisite_skills: vec!["Gathering".to_string()],
    }
}

pub fn create_engineering_skill() -> Skill {
    Skill {
        name: "Engineering".to_string(),
        level: 1.0,
        experience: 0.0,
        category: SkillCategory::Engineering,
        passive_trait: PassiveTrait {
            name: "Mechanical Aptitude".to_string(),
            description: "Understanding of mechanical systems and complex construction".to_string(),
            effects: vec![
                PassiveEffect::BoostSkillExperience("Architecture".to_string(), 0.2),
                PassiveEffect::BoostSkillExperience("Blacksmithing".to_string(), 0.15),
                PassiveEffect::BoostSkillExperience("Intelligence".to_string(), 0.15),
            ],
            formula: SkillFormula::Linear(0.02),
        },
        active_trait: Some(ActiveTrait {
            name: "Design Mechanism".to_string(),
            description: "Create complex mechanical devices and systems".to_string(),
            activation_cost: ActivationCost::Energy(80.0),
            cooldown: SkillCooldown {
                base_duration: 240.0, // 4 minutes for complex engineering
                current_remaining: 0.0,
                reduction_formula: SkillFormula::Linear(2.0),
            },
            effects: vec![
                ActiveEffect::CraftItem("Mechanical Device".to_string(), 1),
                ActiveEffect::GrantExperience("Engineering".to_string(), 50.0),
            ],
            can_be_hotkeyed: true,
        }),
        discovered_by: Vec::new(),
        is_innate: false,
        unlock_requirements: vec![
            SkillRequirement::SkillLevel("Architecture".to_string(), 15.0),
            SkillRequirement::SkillLevel("Blacksmithing".to_string(), 12.0),
            SkillRequirement::SkillLevel("Intelligence".to_string(), 20.0),
        ],
        evolution_potential: vec!["Advanced Engineering".to_string(), "Siege Engineering".to_string()],
        synergy_skills: vec!["Architecture".to_string(), "Blacksmithing".to_string(), "Intelligence".to_string()],
        prerequisite_skills: vec!["Architecture".to_string(), "Blacksmithing".to_string()],
    }
}

impl SkillRegistry {
    pub fn new() -> Self {
        let mut skills = HashMap::new();
        
        // Core attributes
        skills.insert("Dexterity".to_string(), create_dexterity_skill as fn() -> Skill);
        skills.insert("Courage".to_string(), create_courage_skill as fn() -> Skill);
        skills.insert("Wisdom".to_string(), create_wisdom_skill as fn() -> Skill);
        skills.insert("Strength".to_string(), create_strength_skill as fn() -> Skill);
        skills.insert("Vitality".to_string(), create_vitality_skill as fn() -> Skill);
        skills.insert("Charisma".to_string(), create_charisma_skill as fn() -> Skill);
        skills.insert("Intelligence".to_string(), create_intelligence_skill as fn() -> Skill);
        
        // Magic schools - Base skills
        skills.insert("Illusion Magic".to_string(), create_illusion_magic_skill as fn() -> Skill);
        skills.insert("Nature Magic".to_string(), create_nature_magic_skill as fn() -> Skill);
        skills.insert("Healing Magic".to_string(), create_healing_magic_skill as fn() -> Skill);
        skills.insert("Death Magic".to_string(), create_death_magic_skill as fn() -> Skill);
        skills.insert("Abjuration".to_string(), create_abjuration_skill as fn() -> Skill);
        
        // Magic spells (unlocked via base schools)
        skills.insert("Mesmerize".to_string(), create_mesmerize_skill as fn() -> Skill);
        skills.insert("Bark Skin".to_string(), create_bark_skin_skill as fn() -> Skill);
        skills.insert("Entangle".to_string(), create_entangle_skill as fn() -> Skill);
        skills.insert("Greater Heal".to_string(), create_greater_heal_skill as fn() -> Skill);
        skills.insert("Cure Disease".to_string(), create_cure_disease_skill as fn() -> Skill);
        skills.insert("Soul Burn".to_string(), create_soul_burn_skill as fn() -> Skill);
        skills.insert("Animate Dead".to_string(), create_animate_dead_skill as fn() -> Skill);
        skills.insert("Dispel Magic".to_string(), create_dispel_magic_skill as fn() -> Skill);
        skills.insert("Counterspell".to_string(), create_counterspell_skill as fn() -> Skill);
        
        // Bardic skills
        skills.insert("Song".to_string(), create_song_skill as fn() -> Skill);
        skills.insert("Instrument: Lute".to_string(), create_instrument_lute_skill as fn() -> Skill);
        
        // Special skills
        skills.insert("Taming".to_string(), create_taming_skill as fn() -> Skill);
        
        // Crafting skills
        skills.insert("Gathering".to_string(), create_gathering_skill as fn() -> Skill);
        skills.insert("Mining".to_string(), create_mining_skill as fn() -> Skill);
        
        // Martial Arts skills (Monk archetype)
        skills.insert("Martial Arts".to_string(), create_martial_arts_skill as fn() -> Skill);
        skills.insert("Iron Body".to_string(), create_iron_body_skill as fn() -> Skill);
        skills.insert("Inner Peace".to_string(), create_inner_peace_skill as fn() -> Skill);
        skills.insert("Ki Flow".to_string(), create_ki_flow_skill as fn() -> Skill);
        skills.insert("Combat Reflexes".to_string(), create_combat_reflexes_skill as fn() -> Skill);
        skills.insert("Pressure Points".to_string(), create_pressure_points_skill as fn() -> Skill);
        skills.insert("Ki Strike".to_string(), create_ki_strike_skill as fn() -> Skill);
        skills.insert("Whirlwind Strike".to_string(), create_whirlwind_strike_skill as fn() -> Skill);
        skills.insert("Stunning Fist".to_string(), create_stunning_fist_skill as fn() -> Skill);
        skills.insert("Meditation".to_string(), create_meditation_skill as fn() -> Skill);
        
        // Weapon skills
        skills.insert("Sword".to_string(), create_sword_skill as fn() -> Skill);
        skills.insert("Clubs".to_string(), create_clubs_skill as fn() -> Skill);
        skills.insert("Hammers".to_string(), create_hammers_skill as fn() -> Skill);
        skills.insert("Dagger".to_string(), create_dagger_skill as fn() -> Skill);
        skills.insert("Knuckles".to_string(), create_knuckles_skill as fn() -> Skill);
        skills.insert("Shield".to_string(), create_shield_skill as fn() -> Skill);
        skills.insert("Polearm".to_string(), create_polearm_skill as fn() -> Skill);
        skills.insert("Stave".to_string(), create_stave_skill as fn() -> Skill);
        
        // Weapon mastery skills
        skills.insert("One-Handed Weapon Mastery".to_string(), create_one_handed_weapon_mastery_skill as fn() -> Skill);
        skills.insert("Dual Wield Mastery".to_string(), create_dual_wield_mastery_skill as fn() -> Skill);
        skills.insert("Two-Handed Weapon Mastery".to_string(), create_two_handed_weapon_mastery_skill as fn() -> Skill);
        
        // Armor skills
        skills.insert("Light Armor".to_string(), create_light_armor_skill as fn() -> Skill);
        skills.insert("Medium Armor".to_string(), create_medium_armor_skill as fn() -> Skill);
        skills.insert("Heavy Armor".to_string(), create_heavy_armor_skill as fn() -> Skill);
        
        // Construction and advanced crafting skills
        skills.insert("Smelting".to_string(), create_smelting_skill as fn() -> Skill);
        skills.insert("Carpentry".to_string(), create_carpentry_skill as fn() -> Skill);
        skills.insert("Masonry".to_string(), create_masonry_skill as fn() -> Skill);
        skills.insert("Architecture".to_string(), create_architecture_skill as fn() -> Skill);
        skills.insert("Tool Making".to_string(), create_tool_making_skill as fn() -> Skill);
        skills.insert("Blacksmithing".to_string(), create_blacksmithing_skill as fn() -> Skill);
        skills.insert("Leatherworking".to_string(), create_leatherworking_skill as fn() -> Skill);
        skills.insert("Weaving".to_string(), create_weaving_skill as fn() -> Skill);
        skills.insert("Pottery".to_string(), create_pottery_skill as fn() -> Skill);
        skills.insert("Engineering".to_string(), create_engineering_skill as fn() -> Skill);
        
        Self { skills }
    }
    
    pub fn create_skill(&self, name: &str) -> Option<Skill> {
        self.skills.get(name).map(|creator| creator())
    }
    
    pub fn get_all_skill_names(&self) -> Vec<String> {
        self.skills.keys().cloned().collect()
    }
}