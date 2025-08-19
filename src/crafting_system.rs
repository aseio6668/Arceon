use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{info, warn, debug};
use uuid::Uuid;

/// Advanced Elemental Crafting System for Arceon
/// Supports complex combinations of Fire, Water, Air, Earth, Spirit, Light/Dark elements
#[derive(Debug, Clone)]
pub struct CraftingSystem {
    pub crafting_config: CraftingConfig,
    pub elemental_forge: ElementalForge,
    pub recipe_database: RecipeDatabase,
    pub crafting_stations: HashMap<Uuid, CraftingStation>,
    pub material_analyzer: MaterialAnalyzer,
    pub transmutation_engine: TransmutationEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingConfig {
    pub max_recipe_complexity: u32,
    pub base_crafting_time: Duration,
    pub elemental_stability_threshold: f64,
    pub critical_success_chance: f64,
    pub catastrophic_failure_chance: f64,
    pub experience_multiplier: f64,
    pub master_crafter_bonus: f64,
    pub elemental_resonance_bonus: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalForge {
    pub elemental_chambers: HashMap<ElementType, ElementalChamber>,
    pub fusion_matrix: FusionMatrix,
    pub harmony_calculator: HarmonyCalculator,
    pub elemental_storage: ElementalStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ElementType {
    Fire,
    Water,
    Air,
    Earth,
    Spirit,
    Light,
    Dark,
    // Compound elements
    Steam,      // Fire + Water
    Lightning,  // Air + Fire
    Ice,        // Water + Air
    Crystal,    // Earth + Spirit
    Shadow,     // Dark + Spirit
    Radiance,   // Light + Spirit
    Void,       // Dark + Light (rare)
    Essence,    // All elements combined (legendary)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalChamber {
    pub element_type: ElementType,
    pub purity_level: f64,          // 0.0 - 1.0
    pub charge_level: f64,          // 0.0 - 1.0
    pub stability: f64,             // 0.0 - 1.0
    pub resonance_frequency: f64,
    pub last_activation: SystemTime,
    pub chamber_upgrades: Vec<ChamberUpgrade>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChamberUpgrade {
    pub upgrade_type: UpgradeType,
    pub efficiency_bonus: f64,
    pub stability_bonus: f64,
    pub capacity_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpgradeType {
    ElementalFocus,     // Increases purity
    StabilityMatrix,    // Increases stability
    ChargeAmplifier,    // Increases charge capacity
    ResonanceHarmonizer, // Improves element mixing
    QuantumStabilizer,  // Prevents catastrophic failures
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionMatrix {
    pub fusion_recipes: HashMap<ElementCombination, FusionResult>,
    pub active_fusions: Vec<ActiveFusion>,
    pub fusion_history: Vec<FusionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ElementCombination {
    pub primary_element: ElementType,
    pub secondary_element: ElementType,
    pub catalyst_element: Option<ElementType>,
    pub required_ratio: ElementRatio,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ElementRatio {
    pub primary_amount: u32,
    pub secondary_amount: u32,
    pub catalyst_amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionResult {
    pub result_element: ElementType,
    pub efficiency: f64,
    pub stability_requirement: f64,
    pub fusion_time: Duration,
    pub byproducts: Vec<ElementType>,
    pub special_conditions: Vec<FusionCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionCondition {
    TimeOfDay(u8),              // Specific hour required
    MoonPhase(MoonPhase),       // Specific moon phase
    ElementalAlignment(f64),     // Minimum alignment score
    MasterCrafterRequired,       // Needs expert skill
    SacredLocation,             // Must be at special place
    MultipleStations,           // Requires station synchronization
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MoonPhase {
    NewMoon,
    WaxingCrescent,
    FirstQuarter,
    WaxingGibbous,
    FullMoon,
    WaningGibbous,
    ThirdQuarter,
    WaningCrescent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveFusion {
    pub fusion_id: Uuid,
    pub combination: ElementCombination,
    pub start_time: SystemTime,
    pub completion_time: SystemTime,
    pub current_progress: f64,
    pub stability_score: f64,
    pub crafter_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionRecord {
    pub fusion_id: Uuid,
    pub crafter_id: Uuid,
    pub combination: ElementCombination,
    pub result: FusionOutcome,
    pub timestamp: SystemTime,
    pub experience_gained: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionOutcome {
    Success {
        result_element: ElementType,
        quality: CraftingQuality,
        bonus_effects: Vec<BonusEffect>,
    },
    PartialSuccess {
        result_element: ElementType,
        quality: CraftingQuality,
        impurities: Vec<ElementType>,
    },
    Failure {
        resources_lost: Vec<(ElementType, u32)>,
        equipment_damage: f64,
    },
    CatastrophicFailure {
        explosion_damage: f64,
        area_contamination: ElementType,
        equipment_destroyed: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingQuality {
    Poor,
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BonusEffect {
    IncreasedPotency(f64),
    ElementalResonance(ElementType),
    DurabilityBonus(f64),
    MagicalInfusion,
    TemporalStability,
    SpiritualEssence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonyCalculator {
    pub elemental_affinities: HashMap<(ElementType, ElementType), f64>,
    pub harmonic_frequencies: HashMap<ElementType, f64>,
    pub dissonance_penalties: HashMap<ElementCombination, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalStorage {
    pub stored_elements: HashMap<ElementType, StoredElement>,
    pub storage_capacity: u32,
    pub containment_quality: f64,
    pub preservation_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredElement {
    pub element_type: ElementType,
    pub quantity: u32,
    pub purity: f64,
    pub stability: f64,
    pub storage_time: SystemTime,
    pub decay_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeDatabase {
    pub known_recipes: HashMap<Uuid, CraftingRecipe>,
    pub recipe_categories: HashMap<RecipeCategory, Vec<Uuid>>,
    pub discovery_conditions: HashMap<Uuid, DiscoveryCondition>,
    pub mastery_requirements: HashMap<Uuid, MasteryRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingRecipe {
    pub recipe_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: RecipeCategory,
    pub required_elements: HashMap<ElementType, u32>,
    pub required_materials: HashMap<String, u32>,
    pub required_tools: Vec<CraftingTool>,
    pub complexity_level: u32,
    pub base_success_chance: f64,
    pub crafting_time: Duration,
    pub result_item: CraftedItem,
    pub alternative_results: Vec<AlternativeResult>,
    pub elemental_conditions: Vec<ElementalCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecipeCategory {
    Weapons,
    Armor,
    Tools,
    Consumables,
    Enchantments,
    Constructs,
    Transmutation,
    Ritual,
    Experimental,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingTool {
    ElementalForge,
    InfusionCrucible,
    HarmonyBells,
    SpiritualAltar,
    QuantumAnvil,
    CrystalLens,
    VoidChamber,
    EssenceDistiller,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftedItem {
    pub item_id: Uuid,
    pub name: String,
    pub item_type: ItemType,
    pub elemental_properties: HashMap<ElementType, f64>,
    pub magical_effects: Vec<MagicalEffect>,
    pub durability: f64,
    pub quality: CraftingQuality,
    pub creator_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemType {
    Weapon {
        damage_type: DamageType,
        attack_power: f64,
        elemental_damage: HashMap<ElementType, f64>,
    },
    Armor {
        protection_value: f64,
        elemental_resistance: HashMap<ElementType, f64>,
        weight: f64,
    },
    Tool {
        tool_efficiency: f64,
        special_abilities: Vec<ToolAbility>,
        energy_requirement: f64,
    },
    Consumable {
        effect_duration: Duration,
        potency: f64,
        side_effects: Vec<SideEffect>,
    },
    Enchantment {
        target_types: Vec<String>,
        enhancement_power: f64,
        duration: Option<Duration>,
    },
    Construct {
        autonomy_level: f64,
        task_capabilities: Vec<TaskCapability>,
        maintenance_requirement: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DamageType {
    Physical,
    Elemental(ElementType),
    Spiritual,
    Temporal,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolAbility {
    ElementalManipulation(ElementType),
    MaterialTransmutation,
    EnergyChanneling,
    SpiritualCommunication,
    DimensionalAccess,
    TimeDistortion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SideEffect {
    ElementalImbalance(ElementType),
    TemporaryWeakness,
    SpiritualDrain,
    MagicalResonance,
    EnhancedSenses,
    TimeDilation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskCapability {
    ResourceGathering,
    CraftingAssistance,
    ElementalManipulation,
    GuardianProtection,
    InformationGathering,
    SpiritualGuidance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeResult {
    pub result_chance: f64,
    pub required_conditions: Vec<CraftingCondition>,
    pub alternative_item: CraftedItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingCondition {
    CrafterLevel(u32),
    ElementalMastery(ElementType, u32),
    SpecialLocation(String),
    TimeRestriction(TimeRestriction),
    GroupCrafting(u32),
    RareComponents,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeRestriction {
    DayOnly,
    NightOnly,
    SpecificHour(u8),
    CelestialEvent(CelestialEvent),
    SeasonalWindow(Season),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CelestialEvent {
    SolarEclipse,
    LunarEclipse,
    PlanetaryAlignment,
    CometApproach,
    StarfallNight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementalCondition {
    pub required_element: ElementType,
    pub minimum_purity: f64,
    pub minimum_charge: f64,
    pub required_harmony: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MagicalEffect {
    pub effect_type: EffectType,
    pub potency: f64,
    pub duration: Option<Duration>,
    pub activation_condition: ActivationCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    ElementalBoost(ElementType),
    ProtectiveWard(ElementType),
    HealingAura,
    StrengthEnhancement,
    SpeedBoost,
    MagicalSight,
    SpiritualConnection,
    TemporalShift,
    DimensionalAnchor,
    VoidResistance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationCondition {
    OnUse,
    Continuous,
    CombatOnly,
    ElementalTrigger(ElementType),
    EmotionalState(String),
    LocationBased(String),
    TimeBasedCycle(Duration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryCondition {
    pub discovery_method: DiscoveryMethod,
    pub required_knowledge: Vec<String>,
    pub prerequisite_recipes: Vec<Uuid>,
    pub discovery_chance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Experimentation,
    AncientKnowledge,
    SpiritualRevelation,
    ElementalResonance,
    CommunityCollaboration,
    AccidentalDiscovery,
    MasterTeaching,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryRequirement {
    pub elemental_masteries: HashMap<ElementType, u32>,
    pub total_crafting_experience: f64,
    pub successful_crafts: u32,
    pub rare_material_access: Vec<String>,
    pub mentor_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingStation {
    pub station_id: Uuid,
    pub station_type: StationType,
    pub location: StationLocation,
    pub upgrade_level: u32,
    pub available_tools: Vec<CraftingTool>,
    pub elemental_attunement: HashMap<ElementType, f64>,
    pub operation_status: OperationStatus,
    pub maintenance_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StationType {
    BasicForge,
    ElementalFoundry,
    SpiritualAltar,
    CrystalWorkshop,
    VoidLaboratory,
    HarmonyPavilion,
    TransmutationChamber,
    MasterWorkstation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationLocation {
    pub world_coordinates: (f64, f64, f64),
    pub area_name: String,
    pub elemental_influences: HashMap<ElementType, f64>,
    pub ambient_energy_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationStatus {
    Idle,
    Active {
        current_recipe: Uuid,
        progress: f64,
        estimated_completion: SystemTime,
    },
    Maintenance,
    Malfunctioning {
        error_type: String,
        repair_requirement: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialAnalyzer {
    pub analysis_database: HashMap<String, MaterialProperties>,
    pub scanning_accuracy: f64,
    pub detection_range: f64,
    pub elemental_sensitivity: HashMap<ElementType, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub material_name: String,
    pub elemental_composition: HashMap<ElementType, f64>,
    pub purity_level: f64,
    pub stability_rating: f64,
    pub rarity_class: RarityClass,
    pub special_properties: Vec<SpecialProperty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RarityClass {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythical,
    Cosmic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpecialProperty {
    ElementalAmplification(ElementType),
    StabilityEnhancement,
    ResonanceInduction,
    PurityPurification,
    QuantumEntanglement,
    SpiritualConductivity,
    TemporalAnchor,
    VoidResistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmutationEngine {
    pub transmutation_recipes: HashMap<TransmutationPath, TransmutationResult>,
    pub active_transmutations: Vec<ActiveTransmutation>,
    pub energy_requirements: HashMap<String, f64>,
    pub success_modifiers: HashMap<ElementType, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransmutationPath {
    pub source_material: String,
    pub target_material: String,
    pub catalyst_elements: Vec<ElementType>,
    pub required_conditions: Vec<TransmutationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmutationResult {
    pub conversion_efficiency: f64,
    pub energy_cost: f64,
    pub time_required: Duration,
    pub byproducts: Vec<String>,
    pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransmutationCondition {
    HighTemperature(String), // Store as string for eq/hash compatibility
    SpecificPressure(String),
    ElementalField(ElementType, String),
    SpiritualAlignment(String),
    CosmicTiming(CelestialEvent),
    MasterSupervision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTransmutation {
    pub transmutation_id: Uuid,
    pub path: TransmutationPath,
    pub start_time: SystemTime,
    pub completion_time: SystemTime,
    pub current_progress: f64,
    pub energy_invested: f64,
    pub operator_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskFactor {
    ExplosionRisk(f64),
    ToxicByproducts,
    ElementalInstability,
    SpiritualContamination,
    QuantumFluctuation,
    TemporalAnomalies,
    VoidIncursion,
}

impl CraftingSystem {
    pub fn new() -> Self {
        Self {
            crafting_config: CraftingConfig::default(),
            elemental_forge: ElementalForge::new(),
            recipe_database: RecipeDatabase::new(),
            crafting_stations: HashMap::new(),
            material_analyzer: MaterialAnalyzer::new(),
            transmutation_engine: TransmutationEngine::new(),
        }
    }

    /// Initiate a crafting process
    pub async fn start_crafting(&mut self, crafter_id: Uuid, recipe_id: Uuid, station_id: Uuid) -> Result<CraftingSession> {
        info!("🔥 Starting crafting session for crafter: {}", crafter_id);

        // Validate recipe exists
        let recipe = match self.recipe_database.known_recipes.get(&recipe_id) {
            Some(recipe) => recipe.clone(),
            None => return Err(anyhow::anyhow!("Recipe not found")),
        };

        // Validate station exists and is available
        let station = match self.crafting_stations.get(&station_id) {
            Some(station) => station.clone(),
            None => return Err(anyhow::anyhow!("Crafting station not found")),
        };

        // Check if station is idle
        match station.operation_status {
            OperationStatus::Idle => {},
            _ => return Err(anyhow::anyhow!("Crafting station is not available")),
        }

        // Calculate success probability
        let success_chance = self.calculate_success_chance(&recipe, &station, crafter_id).await?;

        // Check elemental requirements
        self.validate_elemental_requirements(&recipe).await?;

        // Create crafting session
        let session = CraftingSession {
            session_id: Uuid::new_v4(),
            crafter_id,
            recipe_id,
            station_id,
            start_time: SystemTime::now(),
            estimated_completion: SystemTime::now() + recipe.crafting_time,
            current_phase: CraftingPhase::Preparation,
            success_probability: success_chance,
            elemental_stability: 1.0,
            accumulated_experience: 0.0,
        };

        // Update station status
        if let Some(station) = self.crafting_stations.get_mut(&station_id) {
            station.operation_status = OperationStatus::Active {
                current_recipe: recipe_id,
                progress: 0.0,
                estimated_completion: session.estimated_completion,
            };
        }

        info!("⚒️ Crafting session started - Success chance: {:.1}%", success_chance * 100.0);

        Ok(session)
    }

    /// Process elemental fusion
    pub async fn fuse_elements(&mut self, combination: ElementCombination, crafter_id: Uuid) -> Result<FusionOutcome> {
        info!("🌀 Starting elemental fusion: {:?} + {:?}", combination.primary_element, combination.secondary_element);

        // Check if fusion recipe exists
        let fusion_result = match self.elemental_forge.fusion_matrix.fusion_recipes.get(&combination) {
            Some(result) => result.clone(),
            None => return Err(anyhow::anyhow!("Unknown elemental combination")),
        };

        // Calculate harmony between elements
        let harmony_score = self.calculate_elemental_harmony(&combination).await?;

        // Check stability requirements
        if harmony_score < fusion_result.stability_requirement {
            warn!("⚠️ Elemental harmony too low: {:.2} < {:.2}", harmony_score, fusion_result.stability_requirement);
            return Ok(FusionOutcome::Failure {
                resources_lost: vec![
                    (combination.primary_element, combination.required_ratio.primary_amount),
                    (combination.secondary_element, combination.required_ratio.secondary_amount),
                ],
                equipment_damage: 0.1,
            });
        }

        // Start active fusion
        let fusion_id = Uuid::new_v4();
        let active_fusion = ActiveFusion {
            fusion_id,
            combination: combination.clone(),
            start_time: SystemTime::now(),
            completion_time: SystemTime::now() + fusion_result.fusion_time,
            current_progress: 0.0,
            stability_score: harmony_score,
            crafter_id,
        };

        self.elemental_forge.fusion_matrix.active_fusions.push(active_fusion);

        // Simulate fusion completion for immediate result
        self.complete_fusion(fusion_id).await
    }

    async fn complete_fusion(&mut self, fusion_id: Uuid) -> Result<FusionOutcome> {
        // Find the active fusion
        let fusion_index = self.elemental_forge.fusion_matrix.active_fusions
            .iter()
            .position(|f| f.fusion_id == fusion_id)
            .ok_or_else(|| anyhow::anyhow!("Fusion not found"))?;

        let fusion = self.elemental_forge.fusion_matrix.active_fusions.remove(fusion_index);

        // Get fusion result template
        let fusion_result = self.elemental_forge.fusion_matrix.fusion_recipes
            .get(&fusion.combination)
            .ok_or_else(|| anyhow::anyhow!("Fusion recipe not found"))?
            .clone();

        // Calculate final outcome based on stability
        let outcome = if fusion.stability_score >= 0.9 {
            // High stability = success with bonus
            FusionOutcome::Success {
                result_element: fusion_result.result_element.clone(),
                quality: CraftingQuality::Epic,
                bonus_effects: vec![
                    BonusEffect::IncreasedPotency(1.5),
                    BonusEffect::ElementalResonance(fusion_result.result_element.clone()),
                ],
            }
        } else if fusion.stability_score >= 0.7 {
            // Medium stability = normal success
            FusionOutcome::Success {
                result_element: fusion_result.result_element.clone(),
                quality: CraftingQuality::Rare,
                bonus_effects: vec![BonusEffect::IncreasedPotency(1.2)],
            }
        } else if fusion.stability_score >= 0.5 {
            // Low stability = partial success
            FusionOutcome::PartialSuccess {
                result_element: fusion_result.result_element.clone(),
                quality: CraftingQuality::Common,
                impurities: fusion_result.byproducts.clone(),
            }
        } else {
            // Very low stability = failure
            FusionOutcome::Failure {
                resources_lost: vec![
                    (fusion.combination.primary_element.clone(), fusion.combination.required_ratio.primary_amount),
                    (fusion.combination.secondary_element.clone(), fusion.combination.required_ratio.secondary_amount),
                ],
                equipment_damage: 0.2,
            }
        };

        // Record fusion in history
        let fusion_record = FusionRecord {
            fusion_id,
            crafter_id: fusion.crafter_id,
            combination: fusion.combination,
            result: outcome.clone(),
            timestamp: SystemTime::now(),
            experience_gained: self.calculate_fusion_experience(&outcome),
        };

        self.elemental_forge.fusion_matrix.fusion_history.push(fusion_record);

        info!("🎆 Fusion completed with outcome: {:?}", outcome);

        Ok(outcome)
    }

    /// Analyze material composition
    pub async fn analyze_material(&self, material_name: &str) -> Result<MaterialProperties> {
        if let Some(properties) = self.material_analyzer.analysis_database.get(material_name) {
            info!("🔍 Material analysis: {} - Purity: {:.2}", material_name, properties.purity_level);
            Ok(properties.clone())
        } else {
            Err(anyhow::anyhow!("Unknown material: {}", material_name))
        }
    }

    /// Start material transmutation
    pub async fn transmute_material(&mut self, path: TransmutationPath, operator_id: Uuid) -> Result<TransmutationStatus> {
        info!("🧪 Starting transmutation: {} -> {}", path.source_material, path.target_material);

        // Check if transmutation path exists
        let transmutation_result = match self.transmutation_engine.transmutation_recipes.get(&path) {
            Some(result) => result.clone(),
            None => return Err(anyhow::anyhow!("Unknown transmutation path")),
        };

        // Validate conditions
        for condition in &path.required_conditions {
            self.validate_transmutation_condition(condition).await?;
        }

        // Calculate energy requirements
        let total_energy = transmutation_result.energy_cost;

        // Start active transmutation
        let transmutation_id = Uuid::new_v4();
        let active_transmutation = ActiveTransmutation {
            transmutation_id,
            path: path.clone(),
            start_time: SystemTime::now(),
            completion_time: SystemTime::now() + transmutation_result.time_required,
            current_progress: 0.0,
            energy_invested: 0.0,
            operator_id,
        };

        self.transmutation_engine.active_transmutations.push(active_transmutation);

        info!("⚡ Transmutation started - Energy required: {:.1}", total_energy);

        Ok(TransmutationStatus {
            transmutation_id,
            status: TransmutationState::InProgress,
            progress: 0.0,
            energy_invested: 0.0,
            estimated_completion: SystemTime::now() + transmutation_result.time_required,
        })
    }

    async fn calculate_success_chance(&self, recipe: &CraftingRecipe, station: &CraftingStation, _crafter_id: Uuid) -> Result<f64> {
        let mut success_chance = recipe.base_success_chance;

        // Station bonuses
        let station_bonus = match station.upgrade_level {
            0..=2 => 1.0,
            3..=5 => 1.1,
            6..=8 => 1.2,
            _ => 1.3,
        };

        success_chance *= station_bonus;

        // Elemental attunement bonuses
        for (element, _amount) in &recipe.required_elements {
            if let Some(attunement) = station.elemental_attunement.get(element) {
                success_chance *= 1.0 + (attunement * 0.1);
            }
        }

        // Apply configuration modifiers
        success_chance *= self.crafting_config.master_crafter_bonus;

        Ok(success_chance.min(0.95)) // Cap at 95%
    }

    async fn validate_elemental_requirements(&self, recipe: &CraftingRecipe) -> Result<()> {
        for (element_type, required_amount) in &recipe.required_elements {
            if let Some(stored) = self.elemental_forge.elemental_storage.stored_elements.get(element_type) {
                if stored.quantity < *required_amount {
                    return Err(anyhow::anyhow!("Insufficient {} element: need {}, have {}", 
                        format!("{:?}", element_type), required_amount, stored.quantity));
                }
            } else {
                return Err(anyhow::anyhow!("Missing required element: {:?}", element_type));
            }
        }

        Ok(())
    }

    async fn calculate_elemental_harmony(&self, combination: &ElementCombination) -> Result<f64> {
        // Get base affinity between elements
        let affinity = self.elemental_forge.harmony_calculator.elemental_affinities
            .get(&(combination.primary_element.clone(), combination.secondary_element.clone()))
            .unwrap_or(&0.5);

        // Check for dissonance
        let dissonance = self.elemental_forge.harmony_calculator.dissonance_penalties
            .get(combination)
            .unwrap_or(&0.0);

        // Calculate harmonic frequencies
        let primary_freq = self.elemental_forge.harmony_calculator.harmonic_frequencies
            .get(&combination.primary_element)
            .unwrap_or(&1.0);

        let secondary_freq = self.elemental_forge.harmony_calculator.harmonic_frequencies
            .get(&combination.secondary_element)
            .unwrap_or(&1.0);

        let frequency_harmony = if (primary_freq - secondary_freq).abs() < 0.1 {
            1.2 // Similar frequencies create harmony
        } else if (primary_freq - secondary_freq).abs() > 0.5 {
            0.8 // Very different frequencies create dissonance
        } else {
            1.0 // Neutral
        };

        let total_harmony = (affinity * frequency_harmony) - dissonance;

        Ok(total_harmony.max(0.0).min(1.0))
    }

    fn calculate_fusion_experience(&self, outcome: &FusionOutcome) -> f64 {
        match outcome {
            FusionOutcome::Success { quality, .. } => {
                match quality {
                    CraftingQuality::Common => 10.0,
                    CraftingQuality::Uncommon => 15.0,
                    CraftingQuality::Rare => 25.0,
                    CraftingQuality::Epic => 40.0,
                    CraftingQuality::Legendary => 70.0,
                    CraftingQuality::Mythical => 120.0,
                    _ => 5.0,
                }
            },
            FusionOutcome::PartialSuccess { .. } => 8.0,
            FusionOutcome::Failure { .. } => 3.0,
            FusionOutcome::CatastrophicFailure { .. } => 1.0,
        }
    }

    async fn validate_transmutation_condition(&self, condition: &TransmutationCondition) -> Result<()> {
        match condition {
            TransmutationCondition::HighTemperature(required_temp) => {
                // In a real implementation, check actual temperature
                info!("🌡️ Temperature requirement: {} degrees", required_temp);
                Ok(())
            },
            TransmutationCondition::SpecificPressure(required_pressure) => {
                info!("🔧 Pressure requirement: {} atm", required_pressure);
                Ok(())
            },
            TransmutationCondition::ElementalField(element, strength) => {
                // Check if required elemental field is present
                info!("🌀 Elemental field requirement: {:?} at strength {}", element, strength);
                Ok(())
            },
            TransmutationCondition::SpiritualAlignment(alignment) => {
                info!("🔮 Spiritual alignment requirement: {}", alignment);
                Ok(())
            },
            TransmutationCondition::CosmicTiming(event) => {
                info!("⭐ Cosmic timing requirement: {:?}", event);
                Ok(())
            },
            TransmutationCondition::MasterSupervision => {
                // Check if a master is present
                info!("👨‍🏫 Master supervision required");
                Ok(())
            },
        }
    }

    /// Get station information
    pub fn get_station_info(&self, station_id: Uuid) -> Option<&CraftingStation> {
        self.crafting_stations.get(&station_id)
    }

    /// Add a new crafting station
    pub fn add_crafting_station(&mut self, station: CraftingStation) -> Uuid {
        let station_id = station.station_id;
        self.crafting_stations.insert(station_id, station);
        info!("🏭 New crafting station added: {}", station_id);
        station_id
    }

    /// Get available recipes for a crafter
    pub fn get_available_recipes(&self, _crafter_id: Uuid) -> Vec<&CraftingRecipe> {
        // In a real implementation, filter by crafter's knowledge and mastery
        self.recipe_database.known_recipes.values().collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingSession {
    pub session_id: Uuid,
    pub crafter_id: Uuid,
    pub recipe_id: Uuid,
    pub station_id: Uuid,
    pub start_time: SystemTime,
    pub estimated_completion: SystemTime,
    pub current_phase: CraftingPhase,
    pub success_probability: f64,
    pub elemental_stability: f64,
    pub accumulated_experience: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CraftingPhase {
    Preparation,
    ElementalInfusion,
    MaterialShaping,
    SpiritualBinding,
    Finalization,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransmutationStatus {
    pub transmutation_id: Uuid,
    pub status: TransmutationState,
    pub progress: f64,
    pub energy_invested: f64,
    pub estimated_completion: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransmutationState {
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

// Default implementations and constructors
impl Default for CraftingConfig {
    fn default() -> Self {
        Self {
            max_recipe_complexity: 10,
            base_crafting_time: Duration::from_secs(300),
            elemental_stability_threshold: 0.7,
            critical_success_chance: 0.05,
            catastrophic_failure_chance: 0.02,
            experience_multiplier: 1.0,
            master_crafter_bonus: 1.2,
            elemental_resonance_bonus: 1.1,
        }
    }
}

impl ElementalForge {
    fn new() -> Self {
        let mut elemental_chambers = HashMap::new();
        
        // Initialize basic elemental chambers
        for element in [ElementType::Fire, ElementType::Water, ElementType::Air, ElementType::Earth, ElementType::Spirit, ElementType::Light, ElementType::Dark] {
            elemental_chambers.insert(element.clone(), ElementalChamber {
                element_type: element,
                purity_level: 0.8,
                charge_level: 1.0,
                stability: 0.9,
                resonance_frequency: 1.0,
                last_activation: SystemTime::now(),
                chamber_upgrades: Vec::new(),
            });
        }

        Self {
            elemental_chambers,
            fusion_matrix: FusionMatrix::new(),
            harmony_calculator: HarmonyCalculator::new(),
            elemental_storage: ElementalStorage::new(),
        }
    }
}

impl FusionMatrix {
    fn new() -> Self {
        let mut fusion_recipes = HashMap::new();

        // Define basic fusion recipes
        fusion_recipes.insert(
            ElementCombination {
                primary_element: ElementType::Fire,
                secondary_element: ElementType::Water,
                catalyst_element: None,
                required_ratio: ElementRatio { primary_amount: 1, secondary_amount: 1, catalyst_amount: 0 },
            },
            FusionResult {
                result_element: ElementType::Steam,
                efficiency: 0.8,
                stability_requirement: 0.6,
                fusion_time: Duration::from_secs(60),
                byproducts: vec![],
                special_conditions: vec![],
            }
        );

        fusion_recipes.insert(
            ElementCombination {
                primary_element: ElementType::Air,
                secondary_element: ElementType::Fire,
                catalyst_element: None,
                required_ratio: ElementRatio { primary_amount: 2, secondary_amount: 1, catalyst_amount: 0 },
            },
            FusionResult {
                result_element: ElementType::Lightning,
                efficiency: 0.9,
                stability_requirement: 0.8,
                fusion_time: Duration::from_secs(30),
                byproducts: vec![],
                special_conditions: vec![FusionCondition::TimeOfDay(12)],
            }
        );

        Self {
            fusion_recipes,
            active_fusions: Vec::new(),
            fusion_history: Vec::new(),
        }
    }
}

impl HarmonyCalculator {
    fn new() -> Self {
        let mut elemental_affinities = HashMap::new();
        let mut harmonic_frequencies = HashMap::new();

        // Define elemental affinities (0.0 = incompatible, 1.0 = perfect harmony)
        elemental_affinities.insert((ElementType::Fire, ElementType::Air), 0.8);
        elemental_affinities.insert((ElementType::Water, ElementType::Earth), 0.8);
        elemental_affinities.insert((ElementType::Fire, ElementType::Water), 0.3);
        elemental_affinities.insert((ElementType::Air, ElementType::Earth), 0.4);
        elemental_affinities.insert((ElementType::Light, ElementType::Dark), 0.1);
        elemental_affinities.insert((ElementType::Spirit, ElementType::Fire), 0.7);
        elemental_affinities.insert((ElementType::Spirit, ElementType::Water), 0.7);
        elemental_affinities.insert((ElementType::Spirit, ElementType::Air), 0.7);
        elemental_affinities.insert((ElementType::Spirit, ElementType::Earth), 0.7);

        // Define harmonic frequencies
        harmonic_frequencies.insert(ElementType::Fire, 1.5);
        harmonic_frequencies.insert(ElementType::Water, 0.8);
        harmonic_frequencies.insert(ElementType::Air, 1.2);
        harmonic_frequencies.insert(ElementType::Earth, 0.6);
        harmonic_frequencies.insert(ElementType::Spirit, 1.0);
        harmonic_frequencies.insert(ElementType::Light, 2.0);
        harmonic_frequencies.insert(ElementType::Dark, 0.5);

        Self {
            elemental_affinities,
            harmonic_frequencies,
            dissonance_penalties: HashMap::new(),
        }
    }
}

impl ElementalStorage {
    fn new() -> Self {
        let mut stored_elements = HashMap::new();

        // Initialize with basic elements
        for element in [ElementType::Fire, ElementType::Water, ElementType::Air, ElementType::Earth] {
            stored_elements.insert(element.clone(), StoredElement {
                element_type: element,
                quantity: 10,
                purity: 0.8,
                stability: 0.9,
                storage_time: SystemTime::now(),
                decay_rate: 0.01,
            });
        }

        Self {
            stored_elements,
            storage_capacity: 1000,
            containment_quality: 0.9,
            preservation_rate: 0.95,
        }
    }
}

impl RecipeDatabase {
    fn new() -> Self {
        Self {
            known_recipes: HashMap::new(),
            recipe_categories: HashMap::new(),
            discovery_conditions: HashMap::new(),
            mastery_requirements: HashMap::new(),
        }
    }
}

impl MaterialAnalyzer {
    fn new() -> Self {
        let mut analysis_database = HashMap::new();

        // Add some basic materials
        analysis_database.insert("Iron Ore".to_string(), MaterialProperties {
            material_name: "Iron Ore".to_string(),
            elemental_composition: {
                let mut composition = HashMap::new();
                composition.insert(ElementType::Earth, 0.8);
                composition.insert(ElementType::Fire, 0.2);
                composition
            },
            purity_level: 0.7,
            stability_rating: 0.9,
            rarity_class: RarityClass::Common,
            special_properties: vec![SpecialProperty::StabilityEnhancement],
        });

        analysis_database.insert("Crystal Shard".to_string(), MaterialProperties {
            material_name: "Crystal Shard".to_string(),
            elemental_composition: {
                let mut composition = HashMap::new();
                composition.insert(ElementType::Spirit, 0.6);
                composition.insert(ElementType::Light, 0.4);
                composition
            },
            purity_level: 0.95,
            stability_rating: 0.8,
            rarity_class: RarityClass::Rare,
            special_properties: vec![SpecialProperty::ElementalAmplification(ElementType::Spirit), SpecialProperty::ResonanceInduction],
        });

        Self {
            analysis_database,
            scanning_accuracy: 0.9,
            detection_range: 100.0,
            elemental_sensitivity: HashMap::new(),
        }
    }
}

impl TransmutationEngine {
    fn new() -> Self {
        let mut transmutation_recipes = HashMap::new();

        // Basic transmutation: Iron to Gold
        transmutation_recipes.insert(
            TransmutationPath {
                source_material: "Iron Ore".to_string(),
                target_material: "Gold Ore".to_string(),
                catalyst_elements: vec![ElementType::Fire, ElementType::Spirit],
                required_conditions: vec![
                    TransmutationCondition::HighTemperature("1500.0".to_string()),
                    TransmutationCondition::ElementalField(ElementType::Fire, "0.8".to_string()),
                ],
            },
            TransmutationResult {
                conversion_efficiency: 0.3,
                energy_cost: 100.0,
                time_required: Duration::from_secs(600),
                byproducts: vec!["Slag".to_string()],
                risk_factors: vec![RiskFactor::ExplosionRisk(0.1)],
            }
        );

        Self {
            transmutation_recipes,
            active_transmutations: Vec::new(),
            energy_requirements: HashMap::new(),
            success_modifiers: HashMap::new(),
        }
    }
}