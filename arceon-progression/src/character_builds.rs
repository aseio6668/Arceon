/*!
# Character Builds System

Advanced character build templates, loadouts, and configuration management
for different playstyles and optimization strategies.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

use crate::{SkillNodeId, TalentTreeId, TalentId};

/// Character build template and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterBuild {
    pub build_id: Uuid,
    pub build_name: String,
    pub description: String,
    pub build_type: BuildType,
    pub playstyle: PlaystyleCategory,
    pub difficulty_rating: u32,
    pub skill_allocation: SkillAllocation,
    pub talent_configuration: TalentConfiguration,
    pub attribute_distribution: AttributeDistribution,
    pub equipment_loadout: EquipmentLoadout,
    pub ability_rotation: AbilityRotation,
    pub build_tags: Vec<String>,
    pub creator_id: Option<Uuid>,
    pub popularity_score: f64,
    pub effectiveness_rating: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Types of character builds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildType {
    PvE,           // Player vs Environment
    PvP,           // Player vs Player
    Hybrid,        // Mixed content
    Crafting,      // Production focused
    Social,        // Community/leadership focused
    Explorer,      // Discovery and adventure
    Economic,      // Trading and wealth
    Support,       // Group utility
    Solo,          // Independent gameplay
    Experimental,  // Testing new combinations
}

/// Playstyle categories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PlaystyleCategory {
    Aggressive,
    Defensive,
    Balanced,
    Specialist,
    Generalist,
    Tactical,
    Reactive,
    Supportive,
    Independent,
    Collaborative,
}

/// Skill point allocation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAllocation {
    pub primary_focus: Vec<SkillFocus>,
    pub secondary_focus: Vec<SkillFocus>,
    pub skill_priorities: HashMap<SkillNodeId, u32>,
    pub unlock_order: Vec<SkillNodeId>,
    pub milestone_targets: Vec<SkillMilestone>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFocus {
    pub category: String,
    pub allocation_percentage: f64,
    pub max_investment: u32,
    pub priority_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMilestone {
    pub level: u32,
    pub target_skills: Vec<SkillNodeId>,
    pub required_unlocks: u32,
    pub bonus_description: String,
}

/// Talent tree configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentConfiguration {
    pub talent_trees: HashMap<TalentTreeId, TalentTreeBuild>,
    pub talent_priorities: Vec<TalentPriority>,
    pub synergy_targets: Vec<TalentSynergy>,
    pub capstone_goals: Vec<TalentId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentTreeBuild {
    pub tree_id: TalentTreeId,
    pub selected_talents: Vec<TalentId>,
    pub investment_level: u32,
    pub completion_percentage: f64,
    pub build_strategy: TalentStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TalentStrategy {
    Focused,     // Deep investment in few trees
    Broad,       // Shallow investment across many trees
    Synergistic, // Target specific combinations
    Adaptive,    // Change based on needs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentPriority {
    pub talent_id: TalentId,
    pub priority_score: u32,
    pub unlock_condition: String,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentSynergy {
    pub synergy_name: String,
    pub required_talents: Vec<TalentId>,
    pub synergy_bonus: String,
    pub activation_level: u32,
}

/// Character attribute distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeDistribution {
    pub primary_attributes: HashMap<AttributeType, u32>,
    pub secondary_attributes: HashMap<AttributeType, u32>,
    pub attribute_priorities: Vec<AttributePriority>,
    pub scaling_strategy: AttributeScaling,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AttributeType {
    Strength,
    Dexterity,
    Intelligence,
    Wisdom,
    Constitution,
    Charisma,
    Perception,
    Luck,
    Willpower,
    Endurance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributePriority {
    pub attribute: AttributeType,
    pub target_value: u32,
    pub priority_level: u32,
    pub scaling_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeScaling {
    Linear,
    Exponential,
    Threshold,
    Balanced,
    MinMax,
}

/// Equipment loadout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentLoadout {
    pub primary_weapon: Option<EquipmentSpec>,
    pub secondary_weapon: Option<EquipmentSpec>,
    pub armor_set: ArmorConfiguration,
    pub accessories: Vec<AccessorySpec>,
    pub consumables: Vec<ConsumableSpec>,
    pub equipment_strategy: EquipmentStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentSpec {
    pub item_type: String,
    pub required_stats: HashMap<String, f64>,
    pub preferred_enchantments: Vec<String>,
    pub alternatives: Vec<String>,
    pub upgrade_priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmorConfiguration {
    pub armor_type: ArmorType,
    pub defense_priority: DefensePriority,
    pub stat_requirements: HashMap<String, f64>,
    pub set_bonuses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Magical,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefensePriority {
    Physical,
    Magical,
    Balanced,
    Evasion,
    Absorption,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessorySpec {
    pub accessory_type: String,
    pub stat_bonuses: HashMap<String, f64>,
    pub special_effects: Vec<String>,
    pub slot_priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumableSpec {
    pub consumable_type: String,
    pub usage_situation: String,
    pub quantity_recommended: u32,
    pub priority_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EquipmentStrategy {
    DamageMaximization,
    SurvivalFocused,
    UtilityOriented,
    CostEffective,
    EndgameOptimal,
    Leveling,
}

/// Ability rotation and usage strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityRotation {
    pub combat_rotation: Vec<AbilitySequence>,
    pub situational_abilities: HashMap<String, Vec<AbilityId>>,
    pub cooldown_management: CooldownStrategy,
    pub resource_management: ResourceStrategy,
    pub combo_chains: Vec<ComboChain>,
}

pub type AbilityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilitySequence {
    pub sequence_name: String,
    pub abilities: Vec<AbilityUsage>,
    pub trigger_condition: String,
    pub priority_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityUsage {
    pub ability_id: AbilityId,
    pub timing: AbilityTiming,
    pub conditions: Vec<String>,
    pub resource_cost: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityTiming {
    Immediate,
    Delayed(f64),
    Conditional(String),
    Reactive(String),
    Cooldown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooldownStrategy {
    pub priority_abilities: Vec<AbilityId>,
    pub cooldown_reduction_focus: bool,
    pub ability_substitutions: HashMap<AbilityId, Vec<AbilityId>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStrategy {
    pub resource_type: String,
    pub conservation_threshold: f64,
    pub regeneration_strategy: String,
    pub emergency_reserves: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboChain {
    pub chain_name: String,
    pub ability_sequence: Vec<AbilityId>,
    pub timing_windows: Vec<f64>,
    pub bonus_effects: Vec<String>,
    pub difficulty_rating: u32,
}

/// Build performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildMetrics {
    pub build_id: Uuid,
    pub damage_output: DamageMetrics,
    pub survivability: SurvivabilityMetrics,
    pub utility_value: UtilityMetrics,
    pub resource_efficiency: EfficiencyMetrics,
    pub versatility_score: f64,
    pub learning_curve: f64,
    pub equipment_requirements: EquipmentRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DamageMetrics {
    pub sustained_dps: f64,
    pub burst_damage: f64,
    pub aoe_effectiveness: f64,
    pub single_target_focus: f64,
    pub damage_consistency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivabilityMetrics {
    pub effective_health: f64,
    pub damage_mitigation: f64,
    pub mobility_rating: f64,
    pub recovery_speed: f64,
    pub threat_management: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityMetrics {
    pub group_support: f64,
    pub crowd_control: f64,
    pub buff_effectiveness: f64,
    pub debuff_capability: f64,
    pub situational_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyMetrics {
    pub mana_efficiency: f64,
    pub cooldown_optimization: f64,
    pub resource_generation: f64,
    pub action_economy: f64,
    pub skill_synergy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentRequirements {
    pub gear_dependency: f64,
    pub upgrade_priority: Vec<String>,
    pub budget_efficiency: f64,
    pub accessibility_rating: f64,
}

/// Build management system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildManager {
    pub user_builds: HashMap<Uuid, Vec<CharacterBuild>>,
    pub template_builds: HashMap<String, CharacterBuild>,
    pub popular_builds: Vec<CharacterBuild>,
    pub build_metrics: HashMap<Uuid, BuildMetrics>,
    pub build_categories: HashMap<String, Vec<Uuid>>,
}

impl CharacterBuild {
    /// Create a new default character build
    pub fn new_default() -> Self {
        Self {
            build_id: Uuid::new_v4(),
            build_name: "Default Build".to_string(),
            description: "Basic balanced build for new characters".to_string(),
            build_type: BuildType::Hybrid,
            playstyle: PlaystyleCategory::Balanced,
            difficulty_rating: 1,
            skill_allocation: SkillAllocation::default(),
            talent_configuration: TalentConfiguration::default(),
            attribute_distribution: AttributeDistribution::default(),
            equipment_loadout: EquipmentLoadout::default(),
            ability_rotation: AbilityRotation::default(),
            build_tags: vec!["beginner".to_string(), "balanced".to_string()],
            creator_id: None,
            popularity_score: 0.0,
            effectiveness_rating: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }

    /// Create a combat-focused build
    pub fn new_combat_build() -> Self {
        let mut build = Self::new_default();
        build.build_name = "Combat Specialist".to_string();
        build.description = "High damage output focused on physical combat".to_string();
        build.build_type = BuildType::PvE;
        build.playstyle = PlaystyleCategory::Aggressive;
        build.difficulty_rating = 2;
        build.build_tags = vec!["combat".to_string(), "damage".to_string(), "melee".to_string()];

        // Configure for combat focus
        build.skill_allocation.primary_focus = vec![
            SkillFocus {
                category: "Combat".to_string(),
                allocation_percentage: 60.0,
                max_investment: 100,
                priority_level: 1,
            },
            SkillFocus {
                category: "Athletics".to_string(),
                allocation_percentage: 25.0,
                max_investment: 50,
                priority_level: 2,
            },
        ];

        build.attribute_distribution.primary_attributes = [
            (AttributeType::Strength, 40),
            (AttributeType::Constitution, 30),
            (AttributeType::Dexterity, 20),
        ].into();

        build.equipment_loadout.equipment_strategy = EquipmentStrategy::DamageMaximization;
        build.equipment_loadout.armor_set.armor_type = ArmorType::Medium;
        build.equipment_loadout.armor_set.defense_priority = DefensePriority::Physical;

        build
    }

    /// Create a magic-focused build
    pub fn new_magic_build() -> Self {
        let mut build = Self::new_default();
        build.build_name = "Arcane Scholar".to_string();
        build.description = "High magical power with spell focus".to_string();
        build.build_type = BuildType::PvE;
        build.playstyle = PlaystyleCategory::Specialist;
        build.difficulty_rating = 3;
        build.build_tags = vec!["magic".to_string(), "spells".to_string(), "ranged".to_string()];

        // Configure for magic focus
        build.skill_allocation.primary_focus = vec![
            SkillFocus {
                category: "Magic".to_string(),
                allocation_percentage: 70.0,
                max_investment: 100,
                priority_level: 1,
            },
            SkillFocus {
                category: "Intellect".to_string(),
                allocation_percentage: 20.0,
                max_investment: 60,
                priority_level: 2,
            },
        ];

        build.attribute_distribution.primary_attributes = [
            (AttributeType::Intelligence, 50),
            (AttributeType::Wisdom, 30),
            (AttributeType::Constitution, 15),
        ].into();

        build.equipment_loadout.equipment_strategy = EquipmentStrategy::UtilityOriented;
        build.equipment_loadout.armor_set.armor_type = ArmorType::Light;
        build.equipment_loadout.armor_set.defense_priority = DefensePriority::Magical;

        build
    }

    /// Create a crafting-focused build
    pub fn new_crafting_build() -> Self {
        let mut build = Self::new_default();
        build.build_name = "Master Artisan".to_string();
        build.description = "Economic and production focused build".to_string();
        build.build_type = BuildType::Crafting;
        build.playstyle = PlaystyleCategory::Independent;
        build.difficulty_rating = 2;
        build.build_tags = vec!["crafting".to_string(), "economy".to_string(), "production".to_string()];

        // Configure for crafting focus
        build.skill_allocation.primary_focus = vec![
            SkillFocus {
                category: "Crafting".to_string(),
                allocation_percentage: 50.0,
                max_investment: 100,
                priority_level: 1,
            },
            SkillFocus {
                category: "Trading".to_string(),
                allocation_percentage: 30.0,
                max_investment: 75,
                priority_level: 2,
            },
            SkillFocus {
                category: "Resource Gathering".to_string(),
                allocation_percentage: 20.0,
                max_investment: 60,
                priority_level: 3,
            },
        ];

        build.attribute_distribution.primary_attributes = [
            (AttributeType::Intelligence, 35),
            (AttributeType::Dexterity, 30),
            (AttributeType::Wisdom, 25),
        ].into();

        build.equipment_loadout.equipment_strategy = EquipmentStrategy::CostEffective;
        build.equipment_loadout.armor_set.armor_type = ArmorType::Light;

        build
    }

    /// Create a social/leadership build
    pub fn new_social_build() -> Self {
        let mut build = Self::new_default();
        build.build_name = "Community Leader".to_string();
        build.description = "Leadership and social interaction focused".to_string();
        build.build_type = BuildType::Social;
        build.playstyle = PlaystyleCategory::Collaborative;
        build.difficulty_rating = 3;
        build.build_tags = vec!["social".to_string(), "leadership".to_string(), "support".to_string()];

        // Configure for social focus
        build.skill_allocation.primary_focus = vec![
            SkillFocus {
                category: "Leadership".to_string(),
                allocation_percentage: 40.0,
                max_investment: 100,
                priority_level: 1,
            },
            SkillFocus {
                category: "Social".to_string(),
                allocation_percentage: 35.0,
                max_investment: 85,
                priority_level: 1,
            },
            SkillFocus {
                category: "Trading".to_string(),
                allocation_percentage: 25.0,
                max_investment: 60,
                priority_level: 2,
            },
        ];

        build.attribute_distribution.primary_attributes = [
            (AttributeType::Charisma, 45),
            (AttributeType::Intelligence, 30),
            (AttributeType::Wisdom, 25),
        ].into();

        build.equipment_loadout.equipment_strategy = EquipmentStrategy::UtilityOriented;

        build
    }

    /// Validate build configuration for errors
    pub fn validate(&self) -> Result<()> {
        // Check skill allocation percentages
        let total_primary: f64 = self.skill_allocation.primary_focus.iter()
            .map(|focus| focus.allocation_percentage)
            .sum();
        
        if total_primary > 100.0 {
            return Err(anyhow::anyhow!("Primary skill allocation exceeds 100%"));
        }

        // Check attribute distribution
        let total_attributes: u32 = self.attribute_distribution.primary_attributes.values().sum();
        if total_attributes > 200 {
            return Err(anyhow::anyhow!("Attribute allocation too high"));
        }

        // Check for required fields
        if self.build_name.is_empty() {
            return Err(anyhow::anyhow!("Build name cannot be empty"));
        }

        Ok(())
    }

    /// Calculate build effectiveness score
    pub fn calculate_effectiveness(&self) -> f64 {
        let mut score = 0.0;

        // Skill allocation efficiency
        let allocation_efficiency = self.skill_allocation.primary_focus.iter()
            .map(|focus| focus.allocation_percentage * focus.priority_level as f64)
            .sum::<f64>() / 100.0;
        score += allocation_efficiency * 0.3;

        // Attribute synergy
        let attribute_synergy = self.calculate_attribute_synergy();
        score += attribute_synergy * 0.2;

        // Equipment compatibility
        let equipment_score = self.calculate_equipment_compatibility();
        score += equipment_score * 0.25;

        // Ability rotation complexity (inverse difficulty bonus)
        let rotation_complexity = self.ability_rotation.combo_chains.len() as f64 * 0.1;
        score += rotation_complexity * 0.15;

        // Versatility bonus
        let versatility = self.calculate_versatility();
        score += versatility * 0.1;

        score.min(10.0).max(0.0)
    }

    /// Calculate attribute synergy score
    fn calculate_attribute_synergy(&self) -> f64 {
        let mut synergy_score = 0.0;
        let total_points: u32 = self.attribute_distribution.primary_attributes.values().sum();
        
        if total_points == 0 {
            return 0.0;
        }

        // Check for focused vs balanced distribution
        let max_attribute = self.attribute_distribution.primary_attributes.values()
            .max().unwrap_or(&0);
        let focus_ratio = *max_attribute as f64 / total_points as f64;

        // Reward moderate focus (not too spread, not too concentrated)
        synergy_score += match focus_ratio {
            f if f >= 0.4 && f <= 0.6 => 1.0, // Sweet spot
            f if f >= 0.3 && f <= 0.7 => 0.8, // Good
            f if f >= 0.2 && f <= 0.8 => 0.6, // Acceptable
            _ => 0.3, // Too spread or too focused
        };

        synergy_score
    }

    /// Calculate equipment compatibility score
    fn calculate_equipment_compatibility(&self) -> f64 {
        let mut compatibility = 0.0;

        // Check if equipment strategy matches build type
        compatibility += match (&self.build_type, &self.equipment_loadout.equipment_strategy) {
            (BuildType::PvE, EquipmentStrategy::DamageMaximization) => 1.0,
            (BuildType::PvP, EquipmentStrategy::SurvivalFocused) => 1.0,
            (BuildType::Crafting, EquipmentStrategy::CostEffective) => 1.0,
            (BuildType::Social, EquipmentStrategy::UtilityOriented) => 1.0,
            _ => 0.5,
        } * 0.5;

        // Check armor type vs playstyle
        compatibility += match (&self.playstyle, &self.equipment_loadout.armor_set.armor_type) {
            (PlaystyleCategory::Aggressive, ArmorType::Medium | ArmorType::Heavy) => 0.5,
            (PlaystyleCategory::Defensive, ArmorType::Heavy) => 0.5,
            (PlaystyleCategory::Tactical, ArmorType::Light | ArmorType::Medium) => 0.5,
            _ => 0.25,
        };

        compatibility
    }

    /// Calculate build versatility
    fn calculate_versatility(&self) -> f64 {
        let mut versatility = 0.0;

        // Count different skill focus areas
        let focus_areas = self.skill_allocation.primary_focus.len() + 
                         self.skill_allocation.secondary_focus.len();
        versatility += (focus_areas as f64 * 0.1).min(0.5);

        // Count ability rotation complexity
        let rotation_variety = self.ability_rotation.situational_abilities.len() as f64 * 0.05;
        versatility += rotation_variety.min(0.3);

        // Build type factor
        versatility += match self.build_type {
            BuildType::Hybrid => 0.2,
            BuildType::Solo => 0.1,
            _ => 0.05,
        };

        versatility.min(1.0)
    }

    /// Get recommended next steps for build progression
    pub fn get_progression_recommendations(&self, current_level: u32) -> Vec<String> {
        let mut recommendations = vec![];

        // Skill recommendations based on level
        if current_level < 10 {
            recommendations.push("Focus on core skills in your primary category".to_string());
        } else if current_level < 25 {
            recommendations.push("Start investing in secondary skill areas".to_string());
        } else if current_level < 50 {
            recommendations.push("Look for skill synergies and combos".to_string());
        } else {
            recommendations.push("Consider specialization and mastery skills".to_string());
        }

        // Attribute recommendations
        let total_attributes: u32 = self.attribute_distribution.primary_attributes.values().sum();
        if total_attributes < current_level * 2 {
            recommendations.push("Allocate more attribute points to match your level".to_string());
        }

        // Equipment recommendations
        match self.equipment_loadout.equipment_strategy {
            EquipmentStrategy::Leveling if current_level > 25 => {
                recommendations.push("Consider upgrading to end-game equipment strategy".to_string());
            },
            _ => {},
        }

        recommendations
    }
}

impl Default for SkillAllocation {
    fn default() -> Self {
        Self {
            primary_focus: vec![
                SkillFocus {
                    category: "General".to_string(),
                    allocation_percentage: 100.0,
                    max_investment: 50,
                    priority_level: 1,
                },
            ],
            secondary_focus: vec![],
            skill_priorities: HashMap::new(),
            unlock_order: vec![],
            milestone_targets: vec![],
        }
    }
}

impl Default for TalentConfiguration {
    fn default() -> Self {
        Self {
            talent_trees: HashMap::new(),
            talent_priorities: vec![],
            synergy_targets: vec![],
            capstone_goals: vec![],
        }
    }
}

impl Default for AttributeDistribution {
    fn default() -> Self {
        Self {
            primary_attributes: [
                (AttributeType::Strength, 10),
                (AttributeType::Dexterity, 10),
                (AttributeType::Intelligence, 10),
                (AttributeType::Constitution, 10),
            ].into(),
            secondary_attributes: HashMap::new(),
            attribute_priorities: vec![],
            scaling_strategy: AttributeScaling::Balanced,
        }
    }
}

impl Default for EquipmentLoadout {
    fn default() -> Self {
        Self {
            primary_weapon: None,
            secondary_weapon: None,
            armor_set: ArmorConfiguration {
                armor_type: ArmorType::Light,
                defense_priority: DefensePriority::Balanced,
                stat_requirements: HashMap::new(),
                set_bonuses: vec![],
            },
            accessories: vec![],
            consumables: vec![],
            equipment_strategy: EquipmentStrategy::Leveling,
        }
    }
}

impl Default for AbilityRotation {
    fn default() -> Self {
        Self {
            combat_rotation: vec![],
            situational_abilities: HashMap::new(),
            cooldown_management: CooldownStrategy {
                priority_abilities: vec![],
                cooldown_reduction_focus: false,
                ability_substitutions: HashMap::new(),
            },
            resource_management: ResourceStrategy {
                resource_type: "mana".to_string(),
                conservation_threshold: 0.2,
                regeneration_strategy: "passive".to_string(),
                emergency_reserves: 0.1,
            },
            combo_chains: vec![],
        }
    }
}

impl BuildManager {
    /// Create a new build manager
    pub fn new() -> Self {
        Self {
            user_builds: HashMap::new(),
            template_builds: HashMap::new(),
            popular_builds: vec![],
            build_metrics: HashMap::new(),
            build_categories: HashMap::new(),
        }
    }

    /// Add a build template
    pub fn add_template_build(&mut self, category: String, build: CharacterBuild) {
        self.template_builds.insert(category.clone(), build.clone());
        self.build_categories.entry(category)
            .or_insert_with(Vec::new)
            .push(build.build_id);
    }

    /// Get recommended builds for a playstyle
    pub fn get_recommended_builds(&self, playstyle: &PlaystyleCategory) -> Vec<&CharacterBuild> {
        self.template_builds.values()
            .filter(|build| &build.playstyle == playstyle)
            .collect()
    }

    /// Save a user's build
    pub fn save_user_build(&mut self, user_id: Uuid, build: CharacterBuild) -> Result<()> {
        build.validate()?;
        
        self.user_builds.entry(user_id)
            .or_insert_with(Vec::new)
            .push(build);
        
        Ok(())
    }

    /// Get all builds for a user
    pub fn get_user_builds(&self, user_id: Uuid) -> Vec<&CharacterBuild> {
        self.user_builds.get(&user_id)
            .map(|builds| builds.iter().collect())
            .unwrap_or_default()
    }

    /// Calculate and cache build metrics
    pub fn calculate_build_metrics(&mut self, build: &CharacterBuild) -> BuildMetrics {
        let effectiveness = build.calculate_effectiveness();
        
        let metrics = BuildMetrics {
            build_id: build.build_id,
            damage_output: DamageMetrics {
                sustained_dps: effectiveness * 100.0,
                burst_damage: effectiveness * 150.0,
                aoe_effectiveness: effectiveness * 80.0,
                single_target_focus: effectiveness * 120.0,
                damage_consistency: effectiveness * 90.0,
            },
            survivability: SurvivabilityMetrics {
                effective_health: effectiveness * 1000.0,
                damage_mitigation: effectiveness * 0.3,
                mobility_rating: effectiveness * 0.8,
                recovery_speed: effectiveness * 0.6,
                threat_management: effectiveness * 0.7,
            },
            utility_value: UtilityMetrics {
                group_support: effectiveness * 0.5,
                crowd_control: effectiveness * 0.4,
                buff_effectiveness: effectiveness * 0.6,
                debuff_capability: effectiveness * 0.3,
                situational_value: effectiveness * 0.7,
            },
            resource_efficiency: EfficiencyMetrics {
                mana_efficiency: effectiveness * 0.8,
                cooldown_optimization: effectiveness * 0.7,
                resource_generation: effectiveness * 0.6,
                action_economy: effectiveness * 0.9,
                skill_synergy: effectiveness * 0.85,
            },
            versatility_score: build.calculate_versatility(),
            learning_curve: build.difficulty_rating as f64 / 5.0,
            equipment_requirements: EquipmentRequirements {
                gear_dependency: match build.equipment_loadout.equipment_strategy {
                    EquipmentStrategy::DamageMaximization => 0.9,
                    EquipmentStrategy::EndgameOptimal => 0.95,
                    EquipmentStrategy::CostEffective => 0.4,
                    EquipmentStrategy::Leveling => 0.2,
                    _ => 0.6,
                },
                upgrade_priority: vec!["weapon".to_string(), "armor".to_string()],
                budget_efficiency: 1.0 - ((build.difficulty_rating as f64 - 1.0) / 4.0),
                accessibility_rating: match build.difficulty_rating {
                    1 => 1.0,
                    2 => 0.8,
                    3 => 0.6,
                    4 => 0.4,
                    _ => 0.2,
                },
            },
        };

        self.build_metrics.insert(build.build_id, metrics.clone());
        metrics
    }

    /// Get popular builds sorted by effectiveness
    pub fn get_popular_builds(&self, limit: usize) -> Vec<&CharacterBuild> {
        let mut builds: Vec<&CharacterBuild> = self.template_builds.values().collect();
        builds.sort_by(|a, b| b.effectiveness_rating.partial_cmp(&a.effectiveness_rating).unwrap());
        builds.into_iter().take(limit).collect()
    }

    /// Search builds by tags
    pub fn search_builds_by_tags(&self, tags: &[String]) -> Vec<&CharacterBuild> {
        self.template_builds.values()
            .filter(|build| {
                tags.iter().any(|tag| build.build_tags.contains(tag))
            })
            .collect()
    }
}

impl Default for BuildManager {
    fn default() -> Self {
        Self::new()
    }
}