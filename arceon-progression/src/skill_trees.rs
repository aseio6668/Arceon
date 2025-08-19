/*!
# Skill Trees System

Advanced skill tree implementation with multiple progression paths, 
prerequisites, synergies, and dynamic skill unlocking.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;
use arceon_core::entities::being::SkillType;

use crate::SkillNodeId;

/// Type alias for skill level information  
pub type SkillLevel = arceon_core::entities::SkillLevel;


/// Manages all skill trees and character skill progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTreeManager {
    pub skill_trees: HashMap<SkillTreeType, SkillTree>,
    pub skill_nodes: HashMap<SkillNodeId, SkillNode>,
    pub skill_synergies: Vec<SkillSynergy>,
    pub progression_paths: HashMap<String, ProgressionPath>,
}

/// Different types of skill trees available
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum SkillTreeType {
    Combat,
    Magic,
    Crafting,
    Social,
    Exploration,
    Leadership,
    Survival,
    Technology,
    Spiritual,
    Mercantile,
}

/// A complete skill tree for a category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub tree_id: Uuid,
    pub tree_type: SkillTreeType,
    pub name: String,
    pub description: String,
    pub root_nodes: Vec<SkillNodeId>,
    pub all_nodes: Vec<SkillNodeId>,
    pub tree_bonuses: Vec<TreeBonus>,
    pub mastery_thresholds: Vec<MasteryThreshold>,
}

/// Individual skill node in a tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNode {
    pub node_id: SkillNodeId,
    pub skill_type: SkillType,
    pub name: String,
    pub description: String,
    pub tier: u32,
    pub cost: u32,
    pub prerequisites: Vec<SkillNodeId>,
    pub unlocks: Vec<SkillNodeId>,
    pub skill_effects: Vec<SkillEffect>,
    pub position: SkillNodePosition,
    pub node_type: SkillNodeType,
    pub max_rank: u32,
}

/// Position in the skill tree for UI display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillNodePosition {
    pub x: f32,
    pub y: f32,
    pub tree_section: String,
}

/// Type of skill node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillNodeType {
    Active,      // Activatable abilities
    Passive,     // Permanent bonuses
    Toggle,      // Can be turned on/off
    Trigger,     // Activates under conditions
    Mastery,     // Tree capstone skills
    Synergy,     // Combines multiple skills
}

/// Effects granted by a skill
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEffect {
    pub effect_type: SkillEffectType,
    pub magnitude: f64,
    pub duration: Option<f64>,
    pub conditions: Vec<EffectCondition>,
    pub scaling: EffectScaling,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SkillEffectType {
    DamageBonus,
    DefenseBonus,
    HealthBonus,
    ManaBonus,
    SpeedBonus,
    CriticalChance,
    ResourceGeneration,
    SkillCooldownReduction,
    ExperienceBonus,
    CraftingEfficiency,
    TradingBonus,
    LeadershipRange,
    SocialInfluence,
    ExplorationSpeed,
    SurvivalResistance,
    TechnologyProficiency,
    SpiritualConnection,
    ElementalMastery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectCondition {
    pub condition_type: String,
    pub value: f64,
    pub operator: ComparisonOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equal,
    GreaterThan,
    LessThan,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectScaling {
    Linear { per_level: f64 },
    Exponential { base: f64, exponent: f64 },
    Logarithmic { base: f64 },
    Custom { formula: String },
}

/// Bonuses for mastering tree sections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeBonus {
    pub bonus_id: Uuid,
    pub name: String,
    pub description: String,
    pub required_nodes: u32,
    pub required_tier: u32,
    pub bonus_effects: Vec<SkillEffect>,
}

/// Mastery thresholds for progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryThreshold {
    pub threshold_id: Uuid,
    pub nodes_required: u32,
    pub tier_required: u32,
    pub rewards: Vec<MasteryReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryReward {
    pub reward_type: MasteryRewardType,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MasteryRewardType {
    SkillPoints,
    TalentPoints,
    SpecialAbility,
    Title,
    Customization,
}

/// Synergies between different skills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergy {
    pub synergy_id: Uuid,
    pub name: String,
    pub description: String,
    pub required_skills: Vec<SkillNodeId>,
    pub synergy_effects: Vec<SkillEffect>,
    pub activation_type: SynergyActivationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynergyActivationType {
    Passive,     // Always active when skills are learned
    Active,      // Must be manually activated
    Conditional, // Activates under specific conditions
}

/// Predefined progression paths through skill trees
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionPath {
    pub path_id: Uuid,
    pub name: String,
    pub description: String,
    pub recommended_order: Vec<SkillNodeId>,
    pub path_bonuses: Vec<PathBonus>,
    pub difficulty_rating: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathBonus {
    pub milestone: u32,
    pub bonus_effect: SkillEffect,
}

impl SkillTreeManager {
    /// Create a new skill tree manager with default trees
    pub fn new() -> Self {
        let mut manager = Self {
            skill_trees: HashMap::new(),
            skill_nodes: HashMap::new(),
            skill_synergies: vec![],
            progression_paths: HashMap::new(),
        };

        manager.initialize_default_trees();
        manager
    }

    /// Initialize default skill trees
    fn initialize_default_trees(&mut self) {
        self.create_combat_tree();
        self.create_magic_tree();
        self.create_crafting_tree();
        self.create_social_tree();
        self.create_exploration_tree();
        self.setup_skill_synergies();
        self.create_progression_paths();
    }

    /// Create the combat skill tree
    fn create_combat_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        let mut nodes = vec![];

        // Tier 1 - Basic Combat
        let basic_attack = self.create_skill_node(
            "Basic Attack Mastery".to_string(),
            "Improves basic attack damage and accuracy".to_string(),
            SkillType::Defense,
            1, 1, vec![], 
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::DamageBonus,
                magnitude: 0.1,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.05 },
            }],
            SkillNodePosition { x: 0.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(basic_attack);

        let defensive_stance = self.create_skill_node(
            "Defensive Stance".to_string(),
            "Reduces damage taken but decreases attack speed".to_string(),
            SkillType::Defense,
            1, 1, vec![],
            SkillNodeType::Toggle,
            vec![SkillEffect {
                effect_type: SkillEffectType::DefenseBonus,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 1.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(defensive_stance);

        // Tier 2 - Specialized Combat
        let weapon_mastery = self.create_skill_node(
            "Weapon Mastery".to_string(),
            "Specialized training with specific weapon types".to_string(),
            SkillType::OneHandedSword,
            2, 2, vec![basic_attack],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::CriticalChance,
                magnitude: 0.05,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.02 },
            }],
            SkillNodePosition { x: 0.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(weapon_mastery);

        let combat_reflexes = self.create_skill_node(
            "Combat Reflexes".to_string(),
            "Improves dodge chance and reaction speed".to_string(),
            SkillType::Athletics,
            2, 2, vec![defensive_stance],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::SpeedBonus,
                magnitude: 0.15,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.08 },
            }],
            SkillNodePosition { x: 1.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(combat_reflexes);

        // Tier 3 - Advanced Combat
        let berserker_rage = self.create_skill_node(
            "Berserker Rage".to_string(),
            "Temporary massive damage boost at cost of defense".to_string(),
            SkillType::Fury,
            3, 3, vec![weapon_mastery, combat_reflexes],
            SkillNodeType::Active,
            vec![
                SkillEffect {
                    effect_type: SkillEffectType::DamageBonus,
                    magnitude: 0.5,
                    duration: Some(10.0),
                    conditions: vec![],
                    scaling: EffectScaling::Linear { per_level: 0.1 },
                },
            ],
            SkillNodePosition { x: 0.5, y: 2.0, tree_section: "mastery".to_string() }
        );
        nodes.push(berserker_rage);

        let tree = SkillTree {
            tree_id,
            tree_type: SkillTreeType::Combat,
            name: "Combat Mastery".to_string(),
            description: "Master the arts of warfare and combat".to_string(),
            root_nodes: vec![basic_attack, defensive_stance],
            all_nodes: nodes.clone(),
            tree_bonuses: vec![
                TreeBonus {
                    bonus_id: Uuid::new_v4(),
                    name: "Combat Expert".to_string(),
                    description: "Bonus for mastering multiple combat skills".to_string(),
                    required_nodes: 3,
                    required_tier: 2,
                    bonus_effects: vec![SkillEffect {
                        effect_type: SkillEffectType::DamageBonus,
                        magnitude: 0.1,
                        duration: None,
                        conditions: vec![],
                        scaling: EffectScaling::Linear { per_level: 0.0 },
                    }],
                }
            ],
            mastery_thresholds: vec![
                MasteryThreshold {
                    threshold_id: Uuid::new_v4(),
                    nodes_required: 5,
                    tier_required: 3,
                    rewards: vec![MasteryReward {
                        reward_type: MasteryRewardType::SpecialAbility,
                        amount: 1,
                    }],
                }
            ],
        };

        self.skill_trees.insert(SkillTreeType::Combat, tree);
    }

    /// Create the magic skill tree
    fn create_magic_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        let mut nodes = vec![];

        // Tier 1 - Basic Magic
        let mana_channeling = self.create_skill_node(
            "Mana Channeling".to_string(),
            "Improves mana efficiency and regeneration".to_string(),
            SkillType::Mana,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::ManaBonus,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 0.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(mana_channeling);

        let elemental_affinity = self.create_skill_node(
            "Elemental Affinity".to_string(),
            "Increases effectiveness with elemental magic".to_string(),
            SkillType::FireMagic,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::ElementalMastery,
                magnitude: 0.15,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.08 },
            }],
            SkillNodePosition { x: 1.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(elemental_affinity);

        // Tier 2 - Specialized Magic
        let spell_power = self.create_skill_node(
            "Spell Power".to_string(),
            "Increases the damage and effectiveness of all spells".to_string(),
            SkillType::FireMagic,
            2, 2, vec![mana_channeling],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::DamageBonus,
                magnitude: 0.25,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 0.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(spell_power);

        let arcane_knowledge = self.create_skill_node(
            "Arcane Knowledge".to_string(),
            "Reduces spell cooldowns and increases mana efficiency".to_string(),
            SkillType::Intellect,
            2, 2, vec![elemental_affinity],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::SkillCooldownReduction,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.05 },
            }],
            SkillNodePosition { x: 1.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(arcane_knowledge);

        // Tier 3 - Master Magic
        let archmage_ascension = self.create_skill_node(
            "Archmage Ascension".to_string(),
            "Ultimate magical mastery - unlocks legendary spells".to_string(),
            SkillType::Focus,
            3, 4, vec![spell_power, arcane_knowledge],
            SkillNodeType::Mastery,
            vec![
                SkillEffect {
                    effect_type: SkillEffectType::DamageBonus,
                    magnitude: 0.4,
                    duration: None,
                    conditions: vec![],
                    scaling: EffectScaling::Exponential { base: 1.1, exponent: 2.0 },
                },
                SkillEffect {
                    effect_type: SkillEffectType::ManaBonus,
                    magnitude: 0.5,
                    duration: None,
                    conditions: vec![],
                    scaling: EffectScaling::Linear { per_level: 0.2 },
                },
            ],
            SkillNodePosition { x: 0.5, y: 2.0, tree_section: "mastery".to_string() }
        );
        nodes.push(archmage_ascension);

        let tree = SkillTree {
            tree_id,
            tree_type: SkillTreeType::Magic,
            name: "Arcane Mastery".to_string(),
            description: "Harness the fundamental forces of magic".to_string(),
            root_nodes: vec![mana_channeling, elemental_affinity],
            all_nodes: nodes.clone(),
            tree_bonuses: vec![
                TreeBonus {
                    bonus_id: Uuid::new_v4(),
                    name: "Magical Synergy".to_string(),
                    description: "Bonus for learning diverse magical disciplines".to_string(),
                    required_nodes: 3,
                    required_tier: 2,
                    bonus_effects: vec![SkillEffect {
                        effect_type: SkillEffectType::ElementalMastery,
                        magnitude: 0.15,
                        duration: None,
                        conditions: vec![],
                        scaling: EffectScaling::Linear { per_level: 0.0 },
                    }],
                }
            ],
            mastery_thresholds: vec![
                MasteryThreshold {
                    threshold_id: Uuid::new_v4(),
                    nodes_required: 4,
                    tier_required: 3,
                    rewards: vec![MasteryReward {
                        reward_type: MasteryRewardType::Title,
                        amount: 1,
                    }],
                }
            ],
        };

        self.skill_trees.insert(SkillTreeType::Magic, tree);
    }

    /// Create the crafting skill tree
    fn create_crafting_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        let mut nodes = vec![];

        // Tier 1 - Basic Crafting
        let resource_gathering = self.create_skill_node(
            "Resource Gathering".to_string(),
            "Improves efficiency in gathering raw materials".to_string(),
            SkillType::Mining,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::ResourceGeneration,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 0.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(resource_gathering);

        let basic_craftsmanship = self.create_skill_node(
            "Basic Craftsmanship".to_string(),
            "Fundamental crafting skills and tool proficiency".to_string(),
            SkillType::Smithing,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::CraftingEfficiency,
                magnitude: 0.15,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.08 },
            }],
            SkillNodePosition { x: 1.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(basic_craftsmanship);

        // Tier 2 - Specialized Crafting  
        let advanced_smithing = self.create_skill_node(
            "Advanced Smithing".to_string(),
            "Master metalworking and weapon/armor creation".to_string(),
            SkillType::Smithing,
            2, 2, vec![basic_craftsmanship],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::CraftingEfficiency,
                magnitude: 0.3,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.15 },
            }],
            SkillNodePosition { x: 0.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(advanced_smithing);

        let enchanting_mastery = self.create_skill_node(
            "Enchanting Mastery".to_string(),
            "Imbue items with magical properties".to_string(),
            SkillType::Enchanting,
            2, 2, vec![basic_craftsmanship],
            SkillNodeType::Active,
            vec![SkillEffect {
                effect_type: SkillEffectType::ElementalMastery,
                magnitude: 0.25,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 1.0, y: 1.0, tree_section: "specialization".to_string() }
        );
        nodes.push(enchanting_mastery);

        // Tier 3 - Master Crafting
        let legendary_artisan = self.create_skill_node(
            "Legendary Artisan".to_string(),
            "Ability to create legendary items and artifacts".to_string(),
            SkillType::Carpentry,
            3, 4, vec![advanced_smithing, enchanting_mastery],
            SkillNodeType::Mastery,
            vec![
                SkillEffect {
                    effect_type: SkillEffectType::CraftingEfficiency,
                    magnitude: 0.5,
                    duration: None,
                    conditions: vec![],
                    scaling: EffectScaling::Exponential { base: 1.2, exponent: 1.5 },
                },
            ],
            SkillNodePosition { x: 0.5, y: 2.0, tree_section: "mastery".to_string() }
        );
        nodes.push(legendary_artisan);

        let tree = SkillTree {
            tree_id,
            tree_type: SkillTreeType::Crafting,
            name: "Master Artisan".to_string(),
            description: "Create wonders through skill and dedication".to_string(),
            root_nodes: vec![resource_gathering, basic_craftsmanship],
            all_nodes: nodes.clone(),
            tree_bonuses: vec![],
            mastery_thresholds: vec![],
        };

        self.skill_trees.insert(SkillTreeType::Crafting, tree);
    }

    /// Create the social skill tree
    fn create_social_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        let mut nodes = vec![];

        // Tier 1 - Basic Social
        let persuasion = self.create_skill_node(
            "Persuasion".to_string(),
            "Improves ability to influence others through words".to_string(),
            SkillType::Leadership,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::SocialInfluence,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 0.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(persuasion);

        let networking = self.create_skill_node(
            "Networking".to_string(),
            "Build and maintain beneficial relationships".to_string(),
            SkillType::Trading,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::TradingBonus,
                magnitude: 0.15,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.08 },
            }],
            SkillNodePosition { x: 1.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(networking);

        let tree = SkillTree {
            tree_id,
            tree_type: SkillTreeType::Social,
            name: "Social Mastery".to_string(),
            description: "Master the art of human interaction".to_string(),
            root_nodes: vec![persuasion, networking],
            all_nodes: nodes.clone(),
            tree_bonuses: vec![],
            mastery_thresholds: vec![],
        };

        self.skill_trees.insert(SkillTreeType::Social, tree);
    }

    /// Create the exploration skill tree
    fn create_exploration_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        let mut nodes = vec![];

        // Tier 1 - Basic Exploration
        let pathfinding = self.create_skill_node(
            "Pathfinding".to_string(),
            "Navigate terrain efficiently and avoid getting lost".to_string(),
            SkillType::Athletics,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::ExplorationSpeed,
                magnitude: 0.2,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.1 },
            }],
            SkillNodePosition { x: 0.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(pathfinding);

        let survival_instincts = self.create_skill_node(
            "Survival Instincts".to_string(),
            "Resist environmental hazards and find resources".to_string(),
            SkillType::Stealth,
            1, 1, vec![],
            SkillNodeType::Passive,
            vec![SkillEffect {
                effect_type: SkillEffectType::SurvivalResistance,
                magnitude: 0.25,
                duration: None,
                conditions: vec![],
                scaling: EffectScaling::Linear { per_level: 0.12 },
            }],
            SkillNodePosition { x: 1.0, y: 0.0, tree_section: "foundation".to_string() }
        );
        nodes.push(survival_instincts);

        let tree = SkillTree {
            tree_id,
            tree_type: SkillTreeType::Exploration,
            name: "Explorer's Path".to_string(),
            description: "Venture into the unknown and survive".to_string(),
            root_nodes: vec![pathfinding, survival_instincts],
            all_nodes: nodes.clone(),
            tree_bonuses: vec![],
            mastery_thresholds: vec![],
        };

        self.skill_trees.insert(SkillTreeType::Exploration, tree);
    }

    /// Helper function to create skill nodes
    fn create_skill_node(
        &mut self,
        name: String,
        description: String,
        skill_type: SkillType,
        tier: u32,
        cost: u32,
        prerequisites: Vec<SkillNodeId>,
        node_type: SkillNodeType,
        effects: Vec<SkillEffect>,
        position: SkillNodePosition,
    ) -> SkillNodeId {
        let node_id = Uuid::new_v4();
        let node = SkillNode {
            node_id,
            skill_type,
            name,
            description,
            tier,
            cost,
            prerequisites,
            unlocks: vec![], // Will be populated based on prerequisites
            skill_effects: effects,
            position,
            node_type,
            max_rank: 1,
        };

        self.skill_nodes.insert(node_id, node);
        node_id
    }

    /// Setup synergies between skills
    fn setup_skill_synergies(&mut self) {
        // Combat + Magic synergy
        let combat_mage_synergy = SkillSynergy {
            synergy_id: Uuid::new_v4(),
            name: "Spellsword".to_string(),
            description: "Combine physical combat with magical enhancement".to_string(),
            required_skills: vec![], // Would reference actual skill IDs
            synergy_effects: vec![
                SkillEffect {
                    effect_type: SkillEffectType::DamageBonus,
                    magnitude: 0.2,
                    duration: None,
                    conditions: vec![],
                    scaling: EffectScaling::Linear { per_level: 0.0 },
                },
            ],
            activation_type: SynergyActivationType::Passive,
        };
        self.skill_synergies.push(combat_mage_synergy);

        // Crafting + Magic synergy
        let artificer_synergy = SkillSynergy {
            synergy_id: Uuid::new_v4(),
            name: "Artificer".to_string(),
            description: "Craft magical items with enhanced properties".to_string(),
            required_skills: vec![], // Would reference actual skill IDs
            synergy_effects: vec![
                SkillEffect {
                    effect_type: SkillEffectType::CraftingEfficiency,
                    magnitude: 0.3,
                    duration: None,
                    conditions: vec![],
                    scaling: EffectScaling::Linear { per_level: 0.0 },
                },
            ],
            activation_type: SynergyActivationType::Passive,
        };
        self.skill_synergies.push(artificer_synergy);
    }

    /// Create suggested progression paths
    fn create_progression_paths(&mut self) {
        // Warrior path
        let warrior_path = ProgressionPath {
            path_id: Uuid::new_v4(),
            name: "Warrior".to_string(),
            description: "Pure combat focused build emphasizing physical prowess".to_string(),
            recommended_order: vec![], // Would contain ordered skill IDs
            path_bonuses: vec![
                PathBonus {
                    milestone: 5,
                    bonus_effect: SkillEffect {
                        effect_type: SkillEffectType::HealthBonus,
                        magnitude: 0.2,
                        duration: None,
                        conditions: vec![],
                        scaling: EffectScaling::Linear { per_level: 0.0 },
                    },
                },
            ],
            difficulty_rating: 2,
        };
        self.progression_paths.insert("warrior".to_string(), warrior_path);

        // Mage path
        let mage_path = ProgressionPath {
            path_id: Uuid::new_v4(),
            name: "Archmage".to_string(),
            description: "Pure magical build focusing on spell power and mana efficiency".to_string(),
            recommended_order: vec![], // Would contain ordered skill IDs
            path_bonuses: vec![
                PathBonus {
                    milestone: 5,
                    bonus_effect: SkillEffect {
                        effect_type: SkillEffectType::ManaBonus,
                        magnitude: 0.3,
                        duration: None,
                        conditions: vec![],
                        scaling: EffectScaling::Linear { per_level: 0.0 },
                    },
                },
            ],
            difficulty_rating: 3,
        };
        self.progression_paths.insert("mage".to_string(), mage_path);

        // Hybrid path
        let spellsword_path = ProgressionPath {
            path_id: Uuid::new_v4(),
            name: "Spellsword".to_string(),
            description: "Balanced combat and magic build with unique synergies".to_string(),
            recommended_order: vec![], // Would contain ordered skill IDs
            path_bonuses: vec![
                PathBonus {
                    milestone: 7,
                    bonus_effect: SkillEffect {
                        effect_type: SkillEffectType::ElementalMastery,
                        magnitude: 0.25,
                        duration: None,
                        conditions: vec![],
                        scaling: EffectScaling::Linear { per_level: 0.0 },
                    },
                },
            ],
            difficulty_rating: 4,
        };
        self.progression_paths.insert("spellsword".to_string(), spellsword_path);
    }

    /// Check if a skill can be unlocked
    pub fn can_unlock_skill(&self, _character_id: Uuid, skill_id: SkillNodeId, unlocked_skills: &[SkillNodeId]) -> Result<bool> {
        let skill_node = self.skill_nodes.get(&skill_id)
            .ok_or_else(|| anyhow::anyhow!("Skill not found"))?;

        // Check if all prerequisites are met
        for prereq in &skill_node.prerequisites {
            if !unlocked_skills.contains(prereq) {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get the cost to unlock a skill
    pub fn get_skill_cost(&self, skill_id: SkillNodeId) -> Result<u32> {
        let skill_node = self.skill_nodes.get(&skill_id)
            .ok_or_else(|| anyhow::anyhow!("Skill not found"))?;
        Ok(skill_node.cost)
    }

    /// Get all available skill trees
    pub fn get_skill_trees(&self) -> &HashMap<SkillTreeType, SkillTree> {
        &self.skill_trees
    }

    /// Get details of a specific skill node
    pub fn get_skill_node(&self, skill_id: SkillNodeId) -> Option<&SkillNode> {
        self.skill_nodes.get(&skill_id)
    }

    /// Get active synergies for a character
    pub fn get_active_synergies(&self, unlocked_skills: &[SkillNodeId]) -> Vec<&SkillSynergy> {
        self.skill_synergies.iter()
            .filter(|synergy| {
                synergy.required_skills.iter()
                    .all(|skill_id| unlocked_skills.contains(skill_id))
            })
            .collect()
    }

    /// Get suggested progression paths
    pub fn get_progression_paths(&self) -> &HashMap<String, ProgressionPath> {
        &self.progression_paths
    }

    /// Calculate total skill effects for a character
    pub fn calculate_total_effects(&self, unlocked_skills: &[SkillNodeId]) -> HashMap<SkillEffectType, f64> {
        let mut total_effects = HashMap::new();

        // Add effects from unlocked skills
        for skill_id in unlocked_skills {
            if let Some(skill_node) = self.skill_nodes.get(skill_id) {
                for effect in &skill_node.skill_effects {
                    *total_effects.entry(effect.effect_type.clone()).or_insert(0.0) += effect.magnitude;
                }
            }
        }

        // Add synergy effects
        for synergy in self.get_active_synergies(unlocked_skills) {
            for effect in &synergy.synergy_effects {
                *total_effects.entry(effect.effect_type.clone()).or_insert(0.0) += effect.magnitude;
            }
        }

        total_effects
    }
}

impl Default for SkillTreeManager {
    fn default() -> Self {
        Self::new()
    }
}