/*!
# Archetype System

Dynamic character archetype calculation based on skill combinations.
Archetypes emerge naturally from player actions and represent their playstyle patterns.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::skill_trees::SkillLevel;
use arceon_core::entities::being::SkillType;

/// Archetype calculation and tracking system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeSystem {
    pub archetype_definitions: Vec<ArchetypeDefinition>,
    pub player_archetypes: HashMap<Uuid, PlayerArchetypes>,
    pub archetype_bonuses: HashMap<ArchetypeType, ArchetypeBonus>,
    pub progression_history: HashMap<Uuid, Vec<ArchetypeSnapshot>>,
}

/// Player's current archetype levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerArchetypes {
    pub player_id: Uuid,
    pub archetype_levels: HashMap<ArchetypeType, f64>,
    pub dominant_archetype: Option<ArchetypeType>,
    pub secondary_archetype: Option<ArchetypeType>,
    pub last_calculated: DateTime<Utc>,
    pub total_archetype_points: f64,
    pub archetype_progression_rate: f64,
}

/// Definition of how an archetype is calculated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeDefinition {
    pub archetype_type: ArchetypeType,
    pub display_name: String,
    pub description: String,
    pub skill_formula: SkillFormula,
    pub minimum_threshold: f64,
    pub maximum_cap: Option<f64>,
    pub synergy_bonuses: Vec<SynergyBonus>,
    pub philosophy: ArchetypePhilosophy,
}

/// Comprehensive archetype types covering all playstyles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchetypeType {
    // Combat Archetypes
    Warrior,        // Physical combat mastery
    Guardian,       // Defensive protection specialist
    Berserker,      // Aggressive combat fury
    Duelist,        // Precise, skillful combat
    
    // Magic Archetypes
    Mage,           // Pure arcane magic
    Sorcerer,       // Innate magical power
    Wizard,         // Scholarly magical study
    Elementalist,   // Elemental magic mastery
    
    // Hybrid Combat-Magic
    BattleMage,     // Combat + Magic balance
    Spellsword,     // Weapon enchantment focus
    MysticWarrior,  // Spiritual combat magic
    
    // Support Archetypes
    Healer,         // Life/restoration magic
    Priest,         // Divine/holy magic
    Druid,          // Nature magic harmony
    Shaman,         // Spirit world connection
    
    // Stealth Archetypes
    Rogue,          // Stealth and cunning
    Assassin,       // Lethal stealth specialist
    Scout,          // Exploration and tracking
    Infiltrator,    // Social stealth mastery
    
    // Crafting Archetypes - Traditional
    Blacksmith,     // Metal tool crafting
    Weaponsmith,    // Weapon crafting specialist
    Armorsmith,     // Armor crafting specialist
    Jeweler,        // Precious item crafting
    
    // Crafting Archetypes - Magical
    Artificer,      // Magic-infused crafting
    Enchanter,      // Item enchantment specialist
    Alchemist,      // Potion and transmutation
    Runesmith,      // Runic magic crafting
    
    // Gathering Archetypes - Traditional
    Miner,          // Physical mining expertise
    Herbalist,      // Plant gathering knowledge
    Lumberjack,     // Tree harvesting skill
    Fisher,         // Aquatic resource gathering
    
    // Gathering Archetypes - Magical
    GeomancerMiner, // Magical earth communion
    VerdantMage,    // Magical plant affinity
    SilvianAdept,   // Magical forest connection
    AquaticMystic,  // Magical water affinity
    
    // Hybrid Gathering-Crafting
    MysticSmith,    // Magic + Traditional smithing
    NatureCrafter,  // Nature magic + crafting
    ElementalForge, // Elemental magic crafting
    
    // Knowledge Archetypes
    Scholar,        // Lore and knowledge
    Researcher,     // Discovery and experimentation
    Librarian,      // Information organization
    Sage,           // Wisdom and counsel
    
    // Social Archetypes
    Diplomat,       // Social negotiation
    Merchant,       // Trade and commerce
    Leader,         // Group coordination
    Entertainer,    // Performance and charisma
    
    // Exploration Archetypes
    Explorer,       // Discovery and adventure
    Cartographer,   // Mapping and navigation
    TreasureHunter, // Valuable discovery
    Wanderer,       // Nomadic lifestyle
    
    // Specialized Archetypes
    BeastMaster,    // Animal companionship
    Summoner,       // Entity summoning magic
    Necromancer,    // Death magic specialist
    TimeKeeper,     // Temporal magic focus
}

/// How archetype level is calculated from skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFormula {
    pub primary_skills: Vec<SkillWeight>,     // Core skills for this archetype
    pub secondary_skills: Vec<SkillWeight>,   // Supporting skills
    pub synergy_multipliers: Vec<SkillSynergy>, // Skill combination bonuses
    pub penalty_skills: Vec<SkillWeight>,     // Skills that reduce this archetype
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillWeight {
    pub skill_type: SkillType,
    pub weight: f64,          // How much this skill contributes
    pub power: f64,           // Exponential scaling (1.0 = linear, 2.0 = quadratic)
    pub threshold: f64,       // Minimum skill level to contribute
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergy {
    pub skill_a: SkillType,
    pub skill_b: SkillType,
    pub multiplier: f64,      // Bonus when both skills are high
    pub synergy_type: SynergyType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynergyType {
    Multiplicative,    // skill_a * skill_b * multiplier
    Additive,         // (skill_a + skill_b) * multiplier
    Geometric,        // sqrt(skill_a * skill_b) * multiplier
    Harmonic,         // 2 * skill_a * skill_b / (skill_a + skill_b) * multiplier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynergyBonus {
    pub with_archetype: ArchetypeType,
    pub bonus_multiplier: f64,
    pub description: String,
}

/// Philosophical approach of the archetype
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypePhilosophy {
    pub core_philosophy: Philosophy,
    pub gathering_approach: GatheringApproach,
    pub crafting_approach: CraftingApproach,
    pub combat_approach: CombatApproach,
    pub magical_alignment: MagicalAlignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Philosophy {
    Physical,      // Relies on physical strength and tools
    Magical,       // Relies on arcane power and spells
    Spiritual,     // Relies on connection to higher powers
    Natural,       // Relies on harmony with nature
    Intellectual,  // Relies on knowledge and study
    Balanced,      // Combines multiple approaches
    Chaotic,       // Unpredictable, situational
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GatheringApproach {
    ToolBased,     // Traditional tools (pickaxe, herbs pouch, axe)
    MagicBased,    // Innate magical gathering spells
    SpiritBased,   // Communication with nature spirits
    Hybrid,        // Combines tool and magic approaches
    Symbiotic,     // Takes only what nature offers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingApproach {
    Traditional,   // Physical tools and techniques
    Enchanted,     // Magic-enhanced traditional crafting
    PureMagic,     // Creation through pure magical power
    Alchemical,    // Transformation through magical formulae
    Living,        // Growth and cultivation methods
    Elemental,     // Elemental force manipulation
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CombatApproach {
    Melee,         // Close physical combat
    Ranged,        // Distance physical combat
    Magical,       // Spellcasting combat
    Hybrid,        // Combines physical and magical
    Defensive,     // Protection and support focus
    Tactical,      // Strategy and positioning
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MagicalAlignment {
    Arcane,        // Raw magical energy
    Divine,        // Holy/unholy powers
    Elemental,     // Fire, water, earth, air
    Nature,        // Life, growth, harmony
    Spirit,        // Soul, astral, ethereal
    Void,          // Darkness, entropy, negation
    Time,          // Temporal manipulation
    Space,         // Dimensional manipulation
}

/// Bonuses granted by archetype mastery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeBonus {
    pub archetype_type: ArchetypeType,
    pub level_bonuses: Vec<LevelBonus>,
    pub mastery_abilities: Vec<MasteryAbility>,
    pub special_interactions: Vec<SpecialInteraction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelBonus {
    pub required_level: f64,
    pub bonus_type: BonusType,
    pub magnitude: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BonusType {
    SkillGainRate,     // Faster skill learning
    ResourceYield,     // Better gathering results
    CraftingQuality,   // Higher quality creations
    CombatDamage,      // Increased damage
    MagicPower,        // Stronger spells
    SocialInfluence,   // Better NPC interactions
    ExplorationSpeed,  // Faster movement/discovery
    EconomicBonus,     // Better prices/trades
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryAbility {
    pub name: String,
    pub required_level: f64,
    pub ability_type: AbilityType,
    pub description: String,
    pub cooldown: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityType {
    ActiveSkill,       // Player-activated ability
    PassiveBonus,      // Always-active enhancement
    TriggeredEffect,   // Activates under conditions
    SocialOption,      // New dialogue/interaction options
    CraftingRecipe,    // Unlocks new recipes
    GatheringTechnique, // New gathering methods
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialInteraction {
    pub interaction_type: InteractionType,
    pub description: String,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    NPCDialogue,       // Special dialogue options
    WorldObject,       // Interact with objects differently
    QuestUnlock,       // Access to archetype-specific quests
    LocationAccess,    // Entry to restricted areas
    ItemCreation,      // Unique crafting possibilities
    MagicalEffect,     // Environmental magical interactions
}

/// Historical snapshot of archetype progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeSnapshot {
    pub timestamp: DateTime<Utc>,
    pub archetype_levels: HashMap<ArchetypeType, f64>,
    pub trigger_event: ProgressionTrigger,
    pub significant_changes: Vec<ArchetypeChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionTrigger {
    SkillLevelUp,
    QuestCompletion,
    CraftingSuccess,
    CombatVictory,
    MagicalDiscovery,
    SocialAchievement,
    ExplorationMilestone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchetypeChange {
    pub archetype_type: ArchetypeType,
    pub old_level: f64,
    pub new_level: f64,
    pub change_magnitude: f64,
    pub rank_change: Option<RankChange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RankChange {
    BecameDominant,    // Became highest archetype
    BecameSecondary,   // Became second highest
    LostDominance,     // No longer highest
    LostSecondary,     // No longer second
    MajorIncrease,     // Significant level gain
    MajorDecrease,     // Significant level loss
}

impl ArchetypeSystem {
    /// Create new archetype system with all definitions
    pub fn new() -> Self {
        let mut system = Self {
            archetype_definitions: vec![],
            player_archetypes: HashMap::new(),
            archetype_bonuses: HashMap::new(),
            progression_history: HashMap::new(),
        };
        
        system.initialize_archetype_definitions();
        system.initialize_archetype_bonuses();
        system
    }

    /// Initialize all archetype calculation formulas
    fn initialize_archetype_definitions(&mut self) {
        // Warrior Archetype
        let warrior = ArchetypeDefinition {
            archetype_type: ArchetypeType::Warrior,
            display_name: "Warrior".to_string(),
            description: "Master of physical combat and martial prowess".to_string(),
            skill_formula: SkillFormula {
                primary_skills: vec![
                    SkillWeight { skill_type: SkillType::OneHandedSword, weight: 1.5, power: 1.2, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::Defense, weight: 1.3, power: 1.1, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::Athletics, weight: 1.0, power: 1.0, threshold: 0.0 },
                ],
                secondary_skills: vec![
                    SkillWeight { skill_type: SkillType::Leadership, weight: 0.8, power: 1.0, threshold: 5.0 },
                    SkillWeight { skill_type: SkillType::Smithing, weight: 0.5, power: 0.9, threshold: 10.0 },
                ],
                synergy_multipliers: vec![
                    SkillSynergy {
                        skill_a: SkillType::OneHandedSword,
                        skill_b: SkillType::Defense,
                        multiplier: 1.25,
                        synergy_type: SynergyType::Geometric,
                    },
                ],
                penalty_skills: vec![
                    SkillWeight { skill_type: SkillType::FireMagic, weight: -0.1, power: 1.0, threshold: 20.0 },
                ],
            },
            minimum_threshold: 1.0,
            maximum_cap: None,
            synergy_bonuses: vec![
                SynergyBonus {
                    with_archetype: ArchetypeType::Blacksmith,
                    bonus_multiplier: 1.15,
                    description: "Warriors who understand smithing craft superior weapons".to_string(),
                },
            ],
            philosophy: ArchetypePhilosophy {
                core_philosophy: Philosophy::Physical,
                gathering_approach: GatheringApproach::ToolBased,
                crafting_approach: CraftingApproach::Traditional,
                combat_approach: CombatApproach::Melee,
                magical_alignment: MagicalAlignment::Arcane, // Least magical
            },
        };

        // Battle-Mage Archetype (Hybrid)
        let battle_mage = ArchetypeDefinition {
            archetype_type: ArchetypeType::BattleMage,
            display_name: "Battle-Mage".to_string(),
            description: "Warrior-scholar who blends steel and sorcery".to_string(),
            skill_formula: SkillFormula {
                primary_skills: vec![
                    SkillWeight { skill_type: SkillType::Defense, weight: 1.0, power: 1.0, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::FireMagic, weight: 1.0, power: 1.0, threshold: 0.0 },
                ],
                secondary_skills: vec![
                    SkillWeight { skill_type: SkillType::OneHandedSword, weight: 0.7, power: 1.0, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::Enchanting, weight: 0.8, power: 1.1, threshold: 5.0 },
                ],
                synergy_multipliers: vec![
                    SkillSynergy {
                        skill_a: SkillType::Defense,
                        skill_b: SkillType::FireMagic,
                        multiplier: 1.5, // Strong synergy bonus
                        synergy_type: SynergyType::Geometric,
                    },
                ],
                penalty_skills: vec![],
            },
            minimum_threshold: 2.0, // Requires both combat and magic
            maximum_cap: None,
            synergy_bonuses: vec![],
            philosophy: ArchetypePhilosophy {
                core_philosophy: Philosophy::Balanced,
                gathering_approach: GatheringApproach::Hybrid,
                crafting_approach: CraftingApproach::Enchanted,
                combat_approach: CombatApproach::Hybrid,
                magical_alignment: MagicalAlignment::Arcane,
            },
        };

        // Geomancer-Miner Archetype (Magical Gatherer)
        let geomancer_miner = ArchetypeDefinition {
            archetype_type: ArchetypeType::GeomancerMiner,
            display_name: "Geomancer-Miner".to_string(),
            description: "One who communes with earth spirits to gather precious materials".to_string(),
            skill_formula: SkillFormula {
                primary_skills: vec![
                    SkillWeight { skill_type: SkillType::Mining, weight: 1.3, power: 1.1, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::FireMagic, weight: 1.0, power: 1.2, threshold: 5.0 },
                ],
                secondary_skills: vec![
                    SkillWeight { skill_type: SkillType::Alchemy, weight: 0.6, power: 1.0, threshold: 10.0 },
                    SkillWeight { skill_type: SkillType::Geology, weight: 0.8, power: 1.0, threshold: 0.0 },
                ],
                synergy_multipliers: vec![
                    SkillSynergy {
                        skill_a: SkillType::Mining,
                        skill_b: SkillType::FireMagic,
                        multiplier: 1.4,
                        synergy_type: SynergyType::Multiplicative,
                    },
                ],
                penalty_skills: vec![],
            },
            minimum_threshold: 1.5,
            maximum_cap: None,
            synergy_bonuses: vec![],
            philosophy: ArchetypePhilosophy {
                core_philosophy: Philosophy::Spiritual,
                gathering_approach: GatheringApproach::MagicBased,
                crafting_approach: CraftingApproach::Alchemical,
                combat_approach: CombatApproach::Magical,
                magical_alignment: MagicalAlignment::Elemental,
            },
        };

        // Healer Archetype
        let healer = ArchetypeDefinition {
            archetype_type: ArchetypeType::Healer,
            display_name: "Healer".to_string(),
            description: "Guardian of life who mends wounds and nurtures growth".to_string(),
            skill_formula: SkillFormula {
                primary_skills: vec![
                    SkillWeight { skill_type: SkillType::FireMagic, weight: 1.2, power: 1.1, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::Alchemy, weight: 1.0, power: 1.0, threshold: 0.0 },
                ],
                secondary_skills: vec![
                    SkillWeight { skill_type: SkillType::Herbalism, weight: 0.8, power: 1.0, threshold: 0.0 },
                    SkillWeight { skill_type: SkillType::Health, weight: 0.4, power: 0.9, threshold: 0.0 },
                ],
                synergy_multipliers: vec![
                    SkillSynergy {
                        skill_a: SkillType::FireMagic,
                        skill_b: SkillType::Alchemy,
                        multiplier: 1.3,
                        synergy_type: SynergyType::Harmonic,
                    },
                ],
                penalty_skills: vec![
                    SkillWeight { skill_type: SkillType::Defense, weight: -0.05, power: 1.0, threshold: 30.0 },
                ],
            },
            minimum_threshold: 1.0,
            maximum_cap: None,
            synergy_bonuses: vec![],
            philosophy: ArchetypePhilosophy {
                core_philosophy: Philosophy::Spiritual,
                gathering_approach: GatheringApproach::Symbiotic,
                crafting_approach: CraftingApproach::Living,
                combat_approach: CombatApproach::Defensive,
                magical_alignment: MagicalAlignment::Divine,
            },
        };

        // Add more archetypes...
        self.archetype_definitions.extend(vec![
            warrior,
            battle_mage,
            geomancer_miner,
            healer,
        ]);
    }

    /// Initialize archetype bonuses and abilities
    fn initialize_archetype_bonuses(&mut self) {
        // Warrior bonuses
        let warrior_bonus = ArchetypeBonus {
            archetype_type: ArchetypeType::Warrior,
            level_bonuses: vec![
                LevelBonus {
                    required_level: 10.0,
                    bonus_type: BonusType::CombatDamage,
                    magnitude: 1.1,
                    description: "10% increased melee damage".to_string(),
                },
                LevelBonus {
                    required_level: 25.0,
                    bonus_type: BonusType::SkillGainRate,
                    magnitude: 1.15,
                    description: "15% faster combat skill learning".to_string(),
                },
            ],
            mastery_abilities: vec![
                MasteryAbility {
                    name: "Battle Fury".to_string(),
                    required_level: 30.0,
                    ability_type: AbilityType::ActiveSkill,
                    description: "Temporary combat enhancement that increases damage and speed".to_string(),
                    cooldown: Some(300),
                },
            ],
            special_interactions: vec![
                SpecialInteraction {
                    interaction_type: InteractionType::NPCDialogue,
                    description: "Warriors are respected by guards and soldiers".to_string(),
                    conditions: vec!["Military NPCs".to_string()],
                },
            ],
        };

        // GeomancerMiner bonuses
        let geomancer_bonus = ArchetypeBonus {
            archetype_type: ArchetypeType::GeomancerMiner,
            level_bonuses: vec![
                LevelBonus {
                    required_level: 15.0,
                    bonus_type: BonusType::ResourceYield,
                    magnitude: 1.2,
                    description: "20% increased rare mineral discovery".to_string(),
                },
            ],
            mastery_abilities: vec![
                MasteryAbility {
                    name: "Stone Sense".to_string(),
                    required_level: 20.0,
                    ability_type: AbilityType::PassiveBonus,
                    description: "Automatically detect valuable mineral deposits nearby".to_string(),
                    cooldown: None,
                },
                MasteryAbility {
                    name: "Earth Communion".to_string(),
                    required_level: 35.0,
                    ability_type: AbilityType::GatheringTechnique,
                    description: "Gather materials through pure magical communion with earth spirits".to_string(),
                    cooldown: None,
                },
            ],
            special_interactions: vec![
                SpecialInteraction {
                    interaction_type: InteractionType::WorldObject,
                    description: "Can mine without tools using earth magic".to_string(),
                    conditions: vec!["Mineral deposits".to_string()],
                },
            ],
        };

        self.archetype_bonuses.insert(ArchetypeType::Warrior, warrior_bonus);
        self.archetype_bonuses.insert(ArchetypeType::GeomancerMiner, geomancer_bonus);
    }

    /// Calculate player's current archetype levels
    pub fn calculate_archetypes(&mut self, player_id: Uuid, skills: &HashMap<SkillType, SkillLevel>) -> Result<PlayerArchetypes> {
        let mut archetype_levels = HashMap::new();
        
        // Calculate each archetype level
        for definition in &self.archetype_definitions {
            let level = self.calculate_archetype_level(definition, skills)?;
            if level >= definition.minimum_threshold {
                archetype_levels.insert(definition.archetype_type.clone(), level);
            }
        }

        // Find dominant and secondary archetypes
        let mut sorted_archetypes: Vec<(&ArchetypeType, &f64)> = archetype_levels.iter().collect();
        sorted_archetypes.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

        let dominant_archetype = sorted_archetypes.first().map(|(at, _)| (*at).clone());
        let secondary_archetype = sorted_archetypes.get(1).map(|(at, _)| (*at).clone());

        let total_points: f64 = archetype_levels.values().sum();

        let player_archetypes = PlayerArchetypes {
            player_id,
            archetype_levels,
            dominant_archetype,
            secondary_archetype,
            last_calculated: chrono::Utc::now(),
            total_archetype_points: total_points,
            archetype_progression_rate: 1.0, // Will be calculated based on recent changes
        };

        // Store historical snapshot
        self.record_archetype_snapshot(player_id, &player_archetypes, ProgressionTrigger::SkillLevelUp);

        // Update player archetypes
        self.player_archetypes.insert(player_id, player_archetypes.clone());

        Ok(player_archetypes)
    }

    /// Calculate individual archetype level from skills
    fn calculate_archetype_level(&self, definition: &ArchetypeDefinition, skills: &HashMap<SkillType, SkillLevel>) -> Result<f64> {
        let mut total_score: f64 = 0.0;

        // Primary skills contribution
        for skill_weight in &definition.skill_formula.primary_skills {
            if let Some(skill_level) = skills.get(&skill_weight.skill_type) {
                let level = skill_level.level as f64;
                if level >= skill_weight.threshold {
                    let contribution = level.powf(skill_weight.power) * skill_weight.weight;
                    total_score += contribution;
                }
            }
        }

        // Secondary skills contribution
        for skill_weight in &definition.skill_formula.secondary_skills {
            if let Some(skill_level) = skills.get(&skill_weight.skill_type) {
                let level = skill_level.level as f64;
                if level >= skill_weight.threshold {
                    let contribution = level.powf(skill_weight.power) * skill_weight.weight;
                    total_score += contribution;
                }
            }
        }

        // Synergy bonuses
        for synergy in &definition.skill_formula.synergy_multipliers {
            if let (Some(skill_a), Some(skill_b)) = (
                skills.get(&synergy.skill_a),
                skills.get(&synergy.skill_b)
            ) {
                let level_a = skill_a.level as f64;
                let level_b = skill_b.level as f64;
                
                let synergy_bonus = match synergy.synergy_type {
                    SynergyType::Multiplicative => level_a * level_b * synergy.multiplier,
                    SynergyType::Additive => (level_a + level_b) * synergy.multiplier,
                    SynergyType::Geometric => (level_a * level_b).sqrt() * synergy.multiplier,
                    SynergyType::Harmonic => {
                        if level_a + level_b > 0.0 {
                            2.0 * level_a * level_b / (level_a + level_b) * synergy.multiplier
                        } else {
                            0.0
                        }
                    },
                };
                
                total_score += synergy_bonus;
            }
        }

        // Penalty skills
        for skill_weight in &definition.skill_formula.penalty_skills {
            if let Some(skill_level) = skills.get(&skill_weight.skill_type) {
                let level = skill_level.level as f64;
                if level >= skill_weight.threshold {
                    let penalty = level.powf(skill_weight.power) * skill_weight.weight.abs();
                    total_score -= penalty;
                }
            }
        }

        // Apply cap if defined
        if let Some(cap) = definition.maximum_cap {
            total_score = total_score.min(cap);
        }

        // Ensure non-negative
        Ok(total_score.max(0.0))
    }

    /// Record archetype progression snapshot
    fn record_archetype_snapshot(&mut self, player_id: Uuid, archetypes: &PlayerArchetypes, trigger: ProgressionTrigger) {
        let snapshot = ArchetypeSnapshot {
            timestamp: chrono::Utc::now(),
            archetype_levels: archetypes.archetype_levels.clone(),
            trigger_event: trigger,
            significant_changes: vec![], // Would calculate changes from previous snapshot
        };

        self.progression_history.entry(player_id).or_default().push(snapshot);
    }

    /// Get player's archetype information
    pub fn get_player_archetypes(&self, player_id: Uuid) -> Option<&PlayerArchetypes> {
        self.player_archetypes.get(&player_id)
    }

    /// Get archetype bonuses for player
    pub fn get_applicable_bonuses(&self, player_id: Uuid) -> Vec<&ArchetypeBonus> {
        if let Some(player_archetypes) = self.player_archetypes.get(&player_id) {
            player_archetypes.archetype_levels.iter()
                .filter_map(|(archetype_type, level)| {
                    self.archetype_bonuses.get(archetype_type)
                        .filter(|_| *level >= 1.0) // Minimum level for bonuses
                })
                .collect()
        } else {
            vec![]
        }
    }

    /// Get archetype progression history
    pub fn get_progression_history(&self, player_id: Uuid) -> Option<&Vec<ArchetypeSnapshot>> {
        self.progression_history.get(&player_id)
    }

    /// Get archetype display information
    pub fn get_archetype_display(&self, archetype_type: &ArchetypeType) -> Option<&ArchetypeDefinition> {
        self.archetype_definitions.iter()
            .find(|def| def.archetype_type == *archetype_type)
    }
}

impl Default for ArchetypeSystem {
    fn default() -> Self {
        Self::new()
    }
}