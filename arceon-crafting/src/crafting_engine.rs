use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, debug};
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use arceon_core::entities::{SkillType, Race};
use arceon_ai::{AdaptiveBehaviorSystem, NeuralNetworkManager};
use arceon_economy::MarketplaceSystem;

/// Advanced crafting engine with complex dependency trees and skill integration
pub struct CraftingEngine {
    pub recipe_manager: Arc<RwLock<RecipeManager>>,
    pub crafting_sessions: Arc<RwLock<CraftingSessionManager>>,
    pub skill_integration: Arc<RwLock<SkillIntegration>>,
    pub discovery_system: Arc<RwLock<DiscoverySystem>>,
    pub quality_controller: Arc<RwLock<QualityController>>,
    pub ai_behavior_system: Arc<AdaptiveBehaviorSystem>,
    pub neural_network_manager: Arc<NeuralNetworkManager>,
    pub marketplace_system: Arc<MarketplaceSystem>,
}

/// Manages all crafting recipes and their complex dependency trees
#[derive(Debug, Default)]
pub struct RecipeManager {
    pub recipes: HashMap<Uuid, CraftingRecipe>,
    pub recipe_categories: HashMap<String, Vec<Uuid>>,
    pub dependency_graph: Graph<Uuid, RecipeDependency>,
    pub recipe_nodes: HashMap<Uuid, NodeIndex>,
    pub skill_unlocks: HashMap<SkillType, Vec<Uuid>>,
    pub cultural_recipes: HashMap<Race, Vec<Uuid>>,
    pub mastery_recipes: Vec<Uuid>, // Legendary recipes requiring mastery
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipe {
    pub recipe_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: CraftingCategory,
    pub subcategory: String,
    pub required_ingredients: Vec<IngredientRequirement>,
    pub optional_ingredients: Vec<OptionalIngredient>,
    pub required_tools: Vec<ToolRequirement>,
    pub required_facilities: Vec<FacilityRequirement>,
    pub skill_requirements: HashMap<SkillType, f64>,
    pub minimum_character_level: u32,
    pub crafting_time_base: f64, // Base time in seconds
    pub difficulty_rating: f64,
    pub failure_consequences: FailureConsequences,
    pub success_outcomes: Vec<CraftingOutcome>,
    pub discovery_method: DiscoveryMethod,
    pub cultural_affinity: Option<Race>,
    pub environmental_requirements: Vec<EnvironmentalRequirement>,
    pub batch_crafting: BatchCraftingInfo,
    pub learning_curve: LearningCurve,
    pub innovation_potential: f64, // Chance for variations/improvements
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum CraftingCategory {
    Weaponsmithing,
    Armorcraft,
    Alchemy,
    Enchanting,
    Cooking,
    Tailoring,
    Woodworking,
    Stonework,
    Jewelcraft,
    Engineering,
    Architecture,
    Farming,
    Brewing,
    Scribing,
    Tinkering,
    Ritualcraft,
    Herbalism,
    Leatherworking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientRequirement {
    pub ingredient_id: Uuid,
    pub ingredient_name: String,
    pub quantity_required: u32,
    pub quality_minimum: f64,
    pub quality_preference: f64, // Higher quality = better results
    pub substitutes: Vec<SubstituteIngredient>,
    pub preparation_required: Option<PreparationStep>,
    pub freshness_requirement: Option<FreshnessRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubstituteIngredient {
    pub substitute_id: Uuid,
    pub substitute_name: String,
    pub conversion_ratio: f64, // How much substitute per original
    pub quality_modifier: f64, // Effect on final quality
    pub skill_penalty: f64, // Additional difficulty when using substitute
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionalIngredient {
    pub ingredient_id: Uuid,
    pub ingredient_name: String,
    pub quantity_range: (u32, u32),
    pub effect_type: OptionalEffectType,
    pub potency: f64,
    pub conflicts_with: Vec<Uuid>, // Other optional ingredients that can't be used together
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionalEffectType {
    QualityBoost,
    QuantityIncrease,
    TimeReduction,
    SkillExpBonus,
    SpecialProperty(String),
    ElementalEnhancement(String),
    DurabilityIncrease,
    EfficiencyImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequirement {
    pub tool_type: ToolType,
    pub quality_minimum: f64,
    pub condition_minimum: f64,
    pub specialized_tools: Vec<String>, // Specific tool names for advanced recipes
    pub tool_mastery_bonus: f64, // Bonus for using masterwork tools
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ToolType {
    Hammer,
    Anvil,
    Forge,
    Tongs,
    File,
    Saw,
    Chisel,
    Needle,
    Cauldron,
    Distillery,
    Loom,
    Wheel,
    Knife,
    MortarPestle,
    Scales,
    Crucible,
    Workbench,
    EnchantingTable,
    Telescope,
    Microscope,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FacilityRequirement {
    pub facility_type: FacilityType,
    pub minimum_level: u32,
    pub special_features: Vec<String>,
    pub location_requirements: Vec<LocationRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum FacilityType {
    Smithy,
    Laboratory,
    Kitchen,
    Workshop,
    Studio,
    Greenhouse,
    Library,
    Observatory,
    Foundry,
    Mill,
    Brewery,
    Tannery,
    Scriptorium,
    EnchantingChamber,
    RitualCircle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationRequirement {
    pub requirement_type: LocationRequirementType,
    pub parameter: String,
    pub tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationRequirementType {
    NearWater,
    HighAltitude,
    Underground,
    ForestProximity,
    MagicalNexus,
    QuietEnvironment,
    WellVentilated,
    TemperatureRange,
    HumidityLevel,
}

/// Manages active crafting sessions for players
#[derive(Debug, Default)]
pub struct CraftingSessionManager {
    pub active_sessions: HashMap<Uuid, CraftingSession>,
    pub queued_crafts: HashMap<Uuid, Vec<QueuedCraft>>,
    pub collaboration_sessions: HashMap<Uuid, CollaborationSession>,
    pub batch_operations: HashMap<Uuid, BatchOperation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingSession {
    pub session_id: Uuid,
    pub crafter_id: Uuid,
    pub recipe_id: Uuid,
    pub start_time: SystemTime,
    pub estimated_completion: SystemTime,
    pub current_progress: f64,
    pub session_state: CraftingState,
    pub consumed_ingredients: Vec<ConsumedIngredient>,
    pub active_tools: Vec<Uuid>,
    pub facility_id: Option<Uuid>,
    pub skill_applications: Vec<SkillApplication>,
    pub quality_modifiers: Vec<QualityModifier>,
    pub interruptions: Vec<CraftingInterruption>,
    pub collaboration_bonus: f64,
    pub innovation_tracking: InnovationTracking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingState {
    Preparing,
    GatheringIngredients,
    SetupTools,
    ActiveCrafting,
    QualityControl,
    Finishing,
    Completed,
    Failed,
    Interrupted,
    AwaitingDecision, // For complex recipes with choices
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumedIngredient {
    pub ingredient_id: Uuid,
    pub quantity_used: u32,
    pub quality_used: f64,
    pub source_location: String,
    pub preparation_applied: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillApplication {
    pub skill_type: SkillType,
    pub skill_level_used: f64,
    pub success_roll: f64,
    pub contribution_weight: f64,
    pub experience_gained: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityModifier {
    pub modifier_type: QualityModifierType,
    pub impact_value: f64,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityModifierType {
    SkillBonus,
    ToolQuality,
    IngredientQuality,
    FacilityBonus,
    EnvironmentalBonus,
    CollaborationBonus,
    ConcentrationBonus,
    InnovationPenalty,
    TimeRushPenalty,
    PerfectConditions,
}

/// Integrates crafting with the skill system
#[derive(Debug, Default)]
pub struct SkillIntegration {
    pub skill_bonuses: HashMap<SkillType, SkillCraftingBonus>,
    pub cross_skill_synergies: Vec<SkillSynergy>,
    pub mastery_unlocks: HashMap<SkillType, Vec<MasteryUnlock>>,
    pub specialization_paths: HashMap<String, SpecializationPath>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCraftingBonus {
    pub skill_type: SkillType,
    pub quality_bonus_per_level: f64,
    pub time_reduction_per_level: f64,
    pub failure_reduction_per_level: f64,
    pub innovation_bonus_per_level: f64,
    pub applicable_categories: Vec<CraftingCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSynergy {
    pub primary_skill: SkillType,
    pub secondary_skill: SkillType,
    pub synergy_multiplier: f64,
    pub synergy_type: SynergyType,
    pub applicable_recipes: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynergyType {
    QualityMultiplier,
    TimeReduction,
    CriticalSuccessChance,
    InnovationBoost,
    ResourceEfficiency,
    SpecialTechniqueUnlock,
}

/// Manages the discovery of new recipes and techniques
#[derive(Debug, Default)]
pub struct DiscoverySystem {
    pub discovered_recipes: HashMap<Uuid, DiscoveredRecipe>,
    pub research_projects: HashMap<Uuid, ResearchProject>,
    pub experimentation_logs: HashMap<Uuid, Vec<ExperimentResult>>,
    pub innovation_seeds: Vec<InnovationSeed>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredRecipe {
    pub recipe_id: Uuid,
    pub discoverer_id: Uuid,
    pub discovery_method: DiscoveryMethod,
    pub discovery_date: SystemTime,
    pub knowledge_sharing: KnowledgeSharing,
    pub refinement_level: u32,
    pub cultural_significance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Experimentation,
    ResearchProject,
    AccidentalDiscovery,
    CulturalLearning,
    NaturalObservation,
    AncientKnowledge,
    CollaborativeInnovation,
    SkillMastery,
    EnvironmentalInspiration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchProject {
    pub project_id: Uuid,
    pub researcher_id: Uuid,
    pub project_name: String,
    pub target_discovery: ResearchTarget,
    pub required_knowledge: Vec<KnowledgeRequirement>,
    pub research_progress: f64,
    pub collaboration_network: Vec<Uuid>,
    pub funding_sources: Vec<FundingSource>,
    pub estimated_duration_days: u32,
    pub breakthrough_potential: f64,
}

impl CraftingEngine {
    /// Create a new crafting engine
    pub fn new(
        ai_behavior_system: Arc<AdaptiveBehaviorSystem>,
        neural_network_manager: Arc<NeuralNetworkManager>,
        marketplace_system: Arc<MarketplaceSystem>
    ) -> Self {
        Self {
            recipe_manager: Arc::new(RwLock::new(RecipeManager::default())),
            crafting_sessions: Arc::new(RwLock::new(CraftingSessionManager::default())),
            skill_integration: Arc::new(RwLock::new(SkillIntegration::default())),
            discovery_system: Arc::new(RwLock::new(DiscoverySystem::default())),
            quality_controller: Arc::new(RwLock::new(QualityController::default())),
            ai_behavior_system,
            neural_network_manager,
            marketplace_system,
        }
    }

    /// Initialize the crafting system with base recipes and skill integrations
    pub async fn initialize(&self) -> Result<()> {
        info!("🔨 Initializing advanced crafting system");

        // Initialize base recipes
        self.initialize_base_recipes().await?;
        
        // Setup skill integrations
        self.setup_skill_integrations().await?;
        
        // Initialize discovery system
        self.initialize_discovery_system().await?;
        
        // Setup quality system
        self.initialize_quality_system().await?;

        info!("✅ Advanced crafting system initialized");
        Ok(())
    }

    /// Initialize fundamental crafting recipes
    async fn initialize_base_recipes(&self) -> Result<()> {
        let mut recipe_manager = self.recipe_manager.write().await;
        
        // Basic Iron Sword - Fundamental weaponsmithing recipe
        let iron_sword_recipe = CraftingRecipe {
            recipe_id: Uuid::new_v4(),
            name: "Iron Sword".to_string(),
            description: "A sturdy iron sword suitable for combat and training".to_string(),
            category: CraftingCategory::Weaponsmithing,
            subcategory: "Bladed Weapons".to_string(),
            required_ingredients: vec![
                IngredientRequirement {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Iron Ingot".to_string(),
                    quantity_required: 3,
                    quality_minimum: 0.4,
                    quality_preference: 0.8,
                    substitutes: vec![
                        SubstituteIngredient {
                            substitute_id: Uuid::new_v4(),
                            substitute_name: "Steel Ingot".to_string(),
                            conversion_ratio: 0.8,
                            quality_modifier: 1.3,
                            skill_penalty: 0.1,
                        }
                    ],
                    preparation_required: Some(PreparationStep {
                        step_name: "Heat Treatment".to_string(),
                        time_required: 300.0,
                        skill_required: SkillType::Smithing,
                        tools_needed: vec![ToolType::Forge],
                    }),
                    freshness_requirement: None,
                },
                IngredientRequirement {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Leather Wrapping".to_string(),
                    quantity_required: 1,
                    quality_minimum: 0.3,
                    quality_preference: 0.6,
                    substitutes: vec![],
                    preparation_required: None,
                    freshness_requirement: None,
                },
            ],
            optional_ingredients: vec![
                OptionalIngredient {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Silver Wire".to_string(),
                    quantity_range: (1, 3),
                    effect_type: OptionalEffectType::QualityBoost,
                    potency: 0.15,
                    conflicts_with: vec![],
                }
            ],
            required_tools: vec![
                ToolRequirement {
                    tool_type: ToolType::Hammer,
                    quality_minimum: 0.4,
                    condition_minimum: 0.6,
                    specialized_tools: vec!["Smithing Hammer".to_string()],
                    tool_mastery_bonus: 0.2,
                },
                ToolRequirement {
                    tool_type: ToolType::Anvil,
                    quality_minimum: 0.5,
                    condition_minimum: 0.7,
                    specialized_tools: vec![],
                    tool_mastery_bonus: 0.15,
                },
            ],
            required_facilities: vec![
                FacilityRequirement {
                    facility_type: FacilityType::Smithy,
                    minimum_level: 1,
                    special_features: vec!["Hot Forge".to_string()],
                    location_requirements: vec![
                        LocationRequirement {
                            requirement_type: LocationRequirementType::WellVentilated,
                            parameter: "airflow".to_string(),
                            tolerance: 0.3,
                        }
                    ],
                }
            ],
            skill_requirements: {
                let mut skills = HashMap::new();
                skills.insert(SkillType::Smithing, 25.0);
                skills.insert(SkillType::Metalworking, 15.0);
                skills
            },
            minimum_character_level: 10,
            crafting_time_base: 1800.0, // 30 minutes base time
            difficulty_rating: 0.4,
            failure_consequences: FailureConsequences {
                ingredient_loss_chance: 0.2,
                tool_damage_chance: 0.1,
                facility_damage_chance: 0.05,
                experience_penalty: 0.1,
                possible_injuries: vec!["Minor Burns".to_string()],
            },
            success_outcomes: vec![
                CraftingOutcome {
                    outcome_type: OutcomeType::ItemCreation,
                    item_id: Uuid::new_v4(),
                    quantity: 1,
                    quality_range: (0.3, 0.9),
                    special_properties: vec![],
                    experience_reward: 150.0,
                    reputation_bonus: 5.0,
                }
            ],
            discovery_method: DiscoveryMethod::SkillMastery,
            cultural_affinity: None, // Universal recipe
            environmental_requirements: vec![],
            batch_crafting: BatchCraftingInfo {
                max_batch_size: 3,
                time_efficiency: 0.85, // 15% time savings for batches
                quality_consistency: 0.9,
                skill_requirement_increase: 10.0,
            },
            learning_curve: LearningCurve {
                initial_difficulty: 0.4,
                mastery_threshold: 100,
                efficiency_gain_per_craft: 0.02,
                quality_improvement_rate: 0.015,
            },
            innovation_potential: 0.1,
        };

        recipe_manager.recipes.insert(iron_sword_recipe.recipe_id, iron_sword_recipe.clone());
        recipe_manager.recipe_categories
            .entry("Weaponsmithing".to_string())
            .or_insert_with(Vec::new)
            .push(iron_sword_recipe.recipe_id);

        // Advanced Healing Potion - Complex alchemy recipe
        let healing_potion_recipe = CraftingRecipe {
            recipe_id: Uuid::new_v4(),
            name: "Greater Healing Potion".to_string(),
            description: "A potent healing elixir that rapidly restores health and vitality".to_string(),
            category: CraftingCategory::Alchemy,
            subcategory: "Restorative Potions".to_string(),
            required_ingredients: vec![
                IngredientRequirement {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Crystal Spring Water".to_string(),
                    quantity_required: 2,
                    quality_minimum: 0.7,
                    quality_preference: 0.9,
                    substitutes: vec![],
                    preparation_required: Some(PreparationStep {
                        step_name: "Purification Ritual".to_string(),
                        time_required: 600.0,
                        skill_required: SkillType::Alchemy,
                        tools_needed: vec![ToolType::Crucible],
                    }),
                    freshness_requirement: Some(FreshnessRequirement {
                        maximum_age_hours: 24,
                        degradation_rate: 0.1,
                    }),
                },
                IngredientRequirement {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Silverleaf Essence".to_string(),
                    quantity_required: 1,
                    quality_minimum: 0.6,
                    quality_preference: 0.85,
                    substitutes: vec![],
                    preparation_required: None,
                    freshness_requirement: Some(FreshnessRequirement {
                        maximum_age_hours: 72,
                        degradation_rate: 0.05,
                    }),
                },
            ],
            optional_ingredients: vec![
                OptionalIngredient {
                    ingredient_id: Uuid::new_v4(),
                    ingredient_name: "Moonstone Powder".to_string(),
                    quantity_range: (1, 2),
                    effect_type: OptionalEffectType::SpecialProperty("Enhanced Regeneration".to_string()),
                    potency: 0.3,
                    conflicts_with: vec![],
                }
            ],
            required_tools: vec![
                ToolRequirement {
                    tool_type: ToolType::Cauldron,
                    quality_minimum: 0.6,
                    condition_minimum: 0.8,
                    specialized_tools: vec!["Alchemical Cauldron".to_string()],
                    tool_mastery_bonus: 0.25,
                },
                ToolRequirement {
                    tool_type: ToolType::Distillery,
                    quality_minimum: 0.5,
                    condition_minimum: 0.7,
                    specialized_tools: vec![],
                    tool_mastery_bonus: 0.2,
                },
            ],
            required_facilities: vec![
                FacilityRequirement {
                    facility_type: FacilityType::Laboratory,
                    minimum_level: 2,
                    special_features: vec!["Magical Infusion Chamber".to_string()],
                    location_requirements: vec![
                        LocationRequirement {
                            requirement_type: LocationRequirementType::MagicalNexus,
                            parameter: "ley_line_proximity".to_string(),
                            tolerance: 0.2,
                        }
                    ],
                }
            ],
            skill_requirements: {
                let mut skills = HashMap::new();
                skills.insert(SkillType::Alchemy, 45.0);
                skills.insert(SkillType::Herbalism, 30.0);
                skills.insert(SkillType::Enchanting, 20.0);
                skills
            },
            minimum_character_level: 25,
            crafting_time_base: 3600.0, // 1 hour base time
            difficulty_rating: 0.7,
            failure_consequences: FailureConsequences {
                ingredient_loss_chance: 0.3,
                tool_damage_chance: 0.15,
                facility_damage_chance: 0.1,
                experience_penalty: 0.2,
                possible_injuries: vec!["Toxic Fumes Exposure".to_string(), "Magical Backlash".to_string()],
            },
            success_outcomes: vec![
                CraftingOutcome {
                    outcome_type: OutcomeType::ItemCreation,
                    item_id: Uuid::new_v4(),
                    quantity: 3, // Batch of 3 potions
                    quality_range: (0.5, 0.95),
                    special_properties: vec!["Rapid Healing".to_string(), "Vitality Boost".to_string()],
                    experience_reward: 300.0,
                    reputation_bonus: 15.0,
                }
            ],
            discovery_method: DiscoveryMethod::ResearchProject,
            cultural_affinity: Some(Race::Elf), // Elves have affinity for alchemical arts
            environmental_requirements: vec![
                EnvironmentalRequirement {
                    requirement_type: EnvironmentalFactorType::MagicalResonance,
                    minimum_value: 0.4,
                    optimal_value: 0.8,
                    tolerance: 0.2,
                }
            ],
            batch_crafting: BatchCraftingInfo {
                max_batch_size: 5,
                time_efficiency: 0.7, // Significant time savings for potion batches
                quality_consistency: 0.95,
                skill_requirement_increase: 15.0,
            },
            learning_curve: LearningCurve {
                initial_difficulty: 0.7,
                mastery_threshold: 200,
                efficiency_gain_per_craft: 0.015,
                quality_improvement_rate: 0.01,
            },
            innovation_potential: 0.3, // High potential for discovering variations
        };

        recipe_manager.recipes.insert(healing_potion_recipe.recipe_id, healing_potion_recipe.clone());
        recipe_manager.recipe_categories
            .entry("Alchemy".to_string())
            .or_insert_with(Vec::new)
            .push(healing_potion_recipe.recipe_id);

        info!("📜 Initialized {} base crafting recipes", recipe_manager.recipes.len());
        Ok(())
    }

    /// Setup skill integration bonuses and synergies
    async fn setup_skill_integrations(&self) -> Result<()> {
        let mut skill_integration = self.skill_integration.write().await;
        
        // Smithing skill bonuses
        let smithing_bonus = SkillCraftingBonus {
            skill_type: SkillType::Smithing,
            quality_bonus_per_level: 0.005, // 0.5% quality per level
            time_reduction_per_level: 0.002, // 0.2% time reduction per level
            failure_reduction_per_level: 0.003, // 0.3% failure reduction per level
            innovation_bonus_per_level: 0.001, // 0.1% innovation chance per level
            applicable_categories: vec![
                CraftingCategory::Weaponsmithing,
                CraftingCategory::Armorcraft,
                CraftingCategory::Engineering,
            ],
        };

        skill_integration.skill_bonuses.insert(SkillType::Smithing, smithing_bonus);

        // Alchemy skill bonuses
        let alchemy_bonus = SkillCraftingBonus {
            skill_type: SkillType::Alchemy,
            quality_bonus_per_level: 0.004,
            time_reduction_per_level: 0.003,
            failure_reduction_per_level: 0.004,
            innovation_bonus_per_level: 0.002,
            applicable_categories: vec![
                CraftingCategory::Alchemy,
                CraftingCategory::Cooking,
                CraftingCategory::Brewing,
                CraftingCategory::Herbalism,
            ],
        };

        skill_integration.skill_bonuses.insert(SkillType::Alchemy, alchemy_bonus);

        // Skill synergies
        let smithing_metalworking_synergy = SkillSynergy {
            primary_skill: SkillType::Smithing,
            secondary_skill: SkillType::Metalworking,
            synergy_multiplier: 1.5,
            synergy_type: SynergyType::QualityMultiplier,
            applicable_recipes: vec![], // All smithing recipes benefit
        };

        skill_integration.cross_skill_synergies.push(smithing_metalworking_synergy);

        let alchemy_herbalism_synergy = SkillSynergy {
            primary_skill: SkillType::Alchemy,
            secondary_skill: SkillType::Herbalism,
            synergy_multiplier: 1.3,
            synergy_type: SynergyType::InnovationBoost,
            applicable_recipes: vec![], // All alchemy recipes benefit
        };

        skill_integration.cross_skill_synergies.push(alchemy_herbalism_synergy);

        Ok(())
    }

    /// Initialize the discovery system with research capabilities
    async fn initialize_discovery_system(&self) -> Result<()> {
        let mut discovery_system = self.discovery_system.write().await;
        
        // Create some innovation seeds for players to discover
        let innovation_seeds = vec![
            InnovationSeed {
                seed_id: Uuid::new_v4(),
                seed_name: "Elemental Infusion Techniques".to_string(),
                discovery_trigger: DiscoveryTrigger::SkillCombination(vec![
                    SkillType::Enchanting,
                    SkillType::Smithing
                ]),
                potential_recipes: vec!["Flaming Sword".to_string(), "Frost Armor".to_string()],
                research_difficulty: 0.6,
                cultural_requirements: None,
            },
            InnovationSeed {
                seed_id: Uuid::new_v4(),
                seed_name: "Collaborative Crafting Methods".to_string(),
                discovery_trigger: DiscoveryTrigger::SocialInteraction,
                potential_recipes: vec!["Grand Tapestry".to_string(), "Community Forge".to_string()],
                research_difficulty: 0.4,
                cultural_requirements: None,
            },
        ];

        discovery_system.innovation_seeds = innovation_seeds;

        Ok(())
    }

    /// Initialize the quality control system
    async fn initialize_quality_system(&self) -> Result<()> {
        let mut quality_controller = self.quality_controller.write().await;
        
        // Setup quality calculation parameters
        quality_controller.base_quality_factors = QualityFactors {
            skill_contribution: 0.4,
            ingredient_contribution: 0.3,
            tool_contribution: 0.15,
            facility_contribution: 0.1,
            environmental_contribution: 0.05,
        };

        quality_controller.quality_thresholds = QualityThresholds {
            poor: 0.0,
            common: 0.3,
            uncommon: 0.5,
            rare: 0.7,
            epic: 0.85,
            legendary: 0.95,
        };

        Ok(())
    }

    /// Start a new crafting session for a player
    pub async fn start_crafting_session(&self, crafter_id: Uuid, recipe_id: Uuid, facility_id: Option<Uuid>) -> Result<Uuid> {
        info!("🎯 Starting crafting session for player {} with recipe {}", crafter_id, recipe_id);

        let recipe_manager = self.recipe_manager.read().await;
        let mut session_manager = self.crafting_sessions.write().await;

        if let Some(recipe) = recipe_manager.recipes.get(&recipe_id) {
            let session_id = Uuid::new_v4();
            let current_time = SystemTime::now();
            
            let session = CraftingSession {
                session_id,
                crafter_id,
                recipe_id,
                start_time: current_time,
                estimated_completion: current_time + std::time::Duration::from_secs(recipe.crafting_time_base as u64),
                current_progress: 0.0,
                session_state: CraftingState::Preparing,
                consumed_ingredients: Vec::new(),
                active_tools: Vec::new(),
                facility_id,
                skill_applications: Vec::new(),
                quality_modifiers: Vec::new(),
                interruptions: Vec::new(),
                collaboration_bonus: 0.0,
                innovation_tracking: InnovationTracking {
                    innovation_attempts: 0,
                    successful_innovations: 0,
                    innovation_points: 0.0,
                    discovered_variations: Vec::new(),
                },
            };

            session_manager.active_sessions.insert(session_id, session);
            
            // Use neural network to predict session success probability
            let success_prediction = self.predict_crafting_success(crafter_id, recipe_id).await?;
            debug!("🎲 Predicted crafting success probability: {:.2}%", success_prediction * 100.0);

            info!("✅ Crafting session {} started successfully", session_id);
            Ok(session_id)
        } else {
            Err(anyhow::anyhow!("Recipe not found: {}", recipe_id))
        }
    }

    /// Use neural networks to predict crafting success
    async fn predict_crafting_success(&self, crafter_id: Uuid, recipe_id: Uuid) -> Result<f64> {
        // Create input vector for neural network prediction
        let mut input_vector = Vec::new();
        
        // Recipe difficulty
        if let Some(recipe) = self.recipe_manager.read().await.recipes.get(&recipe_id) {
            input_vector.push(recipe.difficulty_rating);
            input_vector.push(recipe.minimum_character_level as f64 / 100.0);
            input_vector.push(recipe.crafting_time_base / 3600.0); // Normalize to hours
        }
        
        // Player experience factors (simplified)
        input_vector.push(0.5); // Player crafting experience placeholder
        input_vector.push(0.6); // Tool quality placeholder
        input_vector.push(0.7); // Facility quality placeholder
        
        // Environmental factors
        input_vector.push(0.8); // Environmental conditions placeholder
        
        // Pad to required input size
        while input_vector.len() < 48 {
            input_vector.push(0.0);
        }

        let prediction_output = self.neural_network_manager.process_decision(
            crafter_id,
            &input_vector,
            arceon_ai::NetworkType::DecisionMaking
        ).await?;

        // Extract success probability from neural network output
        let success_probability = prediction_output.get(0).unwrap_or(&0.5);
        Ok(*success_probability)
    }

    /// Get crafting system statistics
    pub async fn get_crafting_statistics(&self) -> CraftingStatistics {
        let recipe_manager = self.recipe_manager.read().await;
        let session_manager = self.crafting_sessions.read().await;
        let discovery_system = self.discovery_system.read().await;

        CraftingStatistics {
            total_recipes: recipe_manager.recipes.len(),
            active_sessions: session_manager.active_sessions.len(),
            total_discoveries: discovery_system.discovered_recipes.len(),
            recipe_categories: recipe_manager.recipe_categories.len(),
            research_projects: discovery_system.research_projects.len(),
        }
    }
}

// Supporting type definitions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDependency {
    pub dependency_type: DependencyType,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Prerequisite,      // Must complete first
    Synergistic,       // Benefits from completion
    Alternative,       // Can substitute
    Sequential,        // Part of a chain
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreparationStep {
    pub step_name: String,
    pub time_required: f64,
    pub skill_required: SkillType,
    pub tools_needed: Vec<ToolType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreshnessRequirement {
    pub maximum_age_hours: u64,
    pub degradation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureConsequences {
    pub ingredient_loss_chance: f64,
    pub tool_damage_chance: f64,
    pub facility_damage_chance: f64,
    pub experience_penalty: f64,
    pub possible_injuries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingOutcome {
    pub outcome_type: OutcomeType,
    pub item_id: Uuid,
    pub quantity: u32,
    pub quality_range: (f64, f64),
    pub special_properties: Vec<String>,
    pub experience_reward: f64,
    pub reputation_bonus: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutcomeType {
    ItemCreation,
    RecipeDiscovery,
    SkillUnlock,
    TechniqueImprovement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalRequirement {
    pub requirement_type: EnvironmentalFactorType,
    pub minimum_value: f64,
    pub optimal_value: f64,
    pub tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentalFactorType {
    Temperature,
    Humidity,
    MagicalResonance,
    AirPurity,
    Lighting,
    NoiseLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCraftingInfo {
    pub max_batch_size: u32,
    pub time_efficiency: f64,
    pub quality_consistency: f64,
    pub skill_requirement_increase: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningCurve {
    pub initial_difficulty: f64,
    pub mastery_threshold: u32,
    pub efficiency_gain_per_craft: f64,
    pub quality_improvement_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSharing {
    pub sharing_level: SharingLevel,
    pub cultural_restrictions: Vec<Race>,
    pub teaching_bonus: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingLevel {
    Secret,
    Guild,
    Cultural,
    Open,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResearchTarget {
    pub target_type: ResearchTargetType,
    pub specific_goal: String,
    pub complexity_rating: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResearchTargetType {
    NewRecipe,
    ImprovedTechnique,
    MaterialInnovation,
    ProcessOptimization,
    QualityEnhancement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeRequirement {
    pub knowledge_type: String,
    pub proficiency_required: f64,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FundingSource {
    pub source_type: String,
    pub amount: f64,
    pub requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentResult {
    pub experiment_id: Uuid,
    pub experimenter_id: Uuid,
    pub success: bool,
    pub results: String,
    pub discoveries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnovationSeed {
    pub seed_id: Uuid,
    pub seed_name: String,
    pub discovery_trigger: DiscoveryTrigger,
    pub potential_recipes: Vec<String>,
    pub research_difficulty: f64,
    pub cultural_requirements: Option<Race>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryTrigger {
    SkillCombination(Vec<SkillType>),
    MaterialCombination(Vec<String>),
    EnvironmentalCondition(String),
    SocialInteraction,
    AccidentalEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryUnlock {
    pub unlock_name: String,
    pub mastery_level_required: f64,
    pub unlock_type: UnlockType,
    pub benefits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockType {
    RecipeAccess,
    TechniqueImprovement,
    QualityBonus,
    TimeReduction,
    SpecialAbility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecializationPath {
    pub path_name: String,
    pub required_skills: HashMap<SkillType, f64>,
    pub exclusive_recipes: Vec<Uuid>,
    pub path_bonuses: Vec<PathBonus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathBonus {
    pub bonus_name: String,
    pub bonus_value: f64,
    pub bonus_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueuedCraft {
    pub craft_id: Uuid,
    pub recipe_id: Uuid,
    pub priority: u32,
    pub queue_position: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationSession {
    pub session_id: Uuid,
    pub participants: Vec<Uuid>,
    pub lead_crafter: Uuid,
    pub collaboration_type: CollaborationType,
    pub shared_workload: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollaborationType {
    SkillSharing,
    WorkloadDistribution,
    QualityControl,
    InnovationFocus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchOperation {
    pub operation_id: Uuid,
    pub recipe_id: Uuid,
    pub batch_size: u32,
    pub individual_sessions: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingInterruption {
    pub interruption_type: String,
    pub impact_severity: f64,
    pub recovery_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnovationTracking {
    pub innovation_attempts: u32,
    pub successful_innovations: u32,
    pub innovation_points: f64,
    pub discovered_variations: Vec<String>,
}

#[derive(Debug, Default)]
pub struct QualityController {
    pub base_quality_factors: QualityFactors,
    pub quality_thresholds: QualityThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityFactors {
    pub skill_contribution: f64,
    pub ingredient_contribution: f64,
    pub tool_contribution: f64,
    pub facility_contribution: f64,
    pub environmental_contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QualityThresholds {
    pub poor: f64,
    pub common: f64,
    pub uncommon: f64,
    pub rare: f64,
    pub epic: f64,
    pub legendary: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingStatistics {
    pub total_recipes: usize,
    pub active_sessions: usize,
    pub total_discoveries: usize,
    pub recipe_categories: usize,
    pub research_projects: usize,
}