use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, debug};
use arceon_core::entities::{SkillType, Race};

/// Advanced supply and demand engine with dynamic pricing
pub struct SupplyDemandEngine {
    pub supply_tracker: Arc<RwLock<SupplyTracker>>,
    pub demand_predictor: Arc<RwLock<DemandPredictor>>,
    pub price_calculator: Arc<RwLock<PriceCalculator>>,
    pub elasticity_analyzer: Arc<RwLock<ElasticityAnalyzer>>,
    pub seasonal_modeler: Arc<RwLock<SeasonalModeler>>,
}

/// Tracks supply levels across all markets and regions
#[derive(Debug, Default)]
pub struct SupplyTracker {
    pub resource_supplies: HashMap<Uuid, ResourceSupply>,
    pub production_chains: HashMap<Uuid, ProductionChain>,
    pub supply_forecasts: HashMap<Uuid, SupplyForecast>,
    pub bottleneck_analysis: BottleneckAnalysis,
    pub inventory_levels: HashMap<String, RegionalInventory>, // market_id -> inventory
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSupply {
    pub resource_id: Uuid,
    pub total_available: u64,
    pub regional_distribution: HashMap<String, u64>, // market_id -> quantity
    pub production_rate: f64, // units per hour
    pub consumption_rate: f64, // units per hour
    pub spoilage_rate: Option<f64>, // for perishable goods
    pub quality_levels: QualityLevelDistribution,
    pub supplier_diversity: SupplierDiversity,
    pub supply_volatility: f64,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityLevelDistribution {
    pub poor_supply: u64,
    pub common_supply: u64,
    pub rare_supply: u64,
    pub epic_supply: u64,
    pub legendary_supply: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierDiversity {
    pub total_suppliers: u32,
    pub supplier_concentration: f64, // Herfindahl index
    pub new_suppliers_monthly: u32,
    pub supplier_reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionChain {
    pub chain_id: Uuid,
    pub final_product: Uuid,
    pub production_stages: Vec<ProductionStage>,
    pub bottleneck_stages: Vec<usize>,
    pub total_lead_time: f64,
    pub efficiency_score: f64,
    pub dependency_risks: Vec<DependencyRisk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionStage {
    pub stage_id: Uuid,
    pub stage_name: String,
    pub required_inputs: HashMap<Uuid, u64>, // resource_id -> quantity
    pub output_quantity: u64,
    pub processing_time: f64,
    pub skill_requirements: HashMap<SkillType, f64>,
    pub failure_rate: f64,
    pub capacity_limit: Option<u64>,
    pub current_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyRisk {
    pub risk_type: RiskType,
    pub affected_resource: Uuid,
    pub probability: f64,
    pub impact_severity: f64,
    pub mitigation_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskType {
    SupplierDefault,
    ResourceScarcity,
    TransportDisruption,
    QualityIssue,
    SeasonalUnavailability,
    PoliticalInstability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyForecast {
    pub resource_id: Uuid,
    pub forecast_horizon_days: u32,
    pub predicted_supply_levels: Vec<ForecastPoint>,
    pub confidence_interval: (f64, f64),
    pub forecast_accuracy: f64,
    pub key_assumptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastPoint {
    pub date: SystemTime,
    pub predicted_supply: u64,
    pub confidence_score: f64,
    pub contributing_factors: Vec<String>,
}

/// Predicts and analyzes demand patterns
#[derive(Debug, Default)]
pub struct DemandPredictor {
    pub demand_patterns: HashMap<Uuid, DemandPattern>,
    pub consumer_segments: HashMap<String, ConsumerSegment>,
    pub demand_drivers: Vec<DemandDriver>,
    pub behavioral_models: HashMap<Race, ConsumerBehaviorModel>,
    pub cross_elasticity_matrix: HashMap<(Uuid, Uuid), f64>, // (resource1, resource2) -> elasticity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPattern {
    pub resource_id: Uuid,
    pub base_demand: f64,
    pub current_demand: f64,
    pub trend_direction: TrendDirection,
    pub seasonality: SeasonalityPattern,
    pub demand_volatility: f64,
    pub price_sensitivity: f64,
    pub substitute_sensitivity: f64,
    pub luxury_factor: f64, // 0.0 = necessity, 1.0 = luxury
    pub network_effects: NetworkEffects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrendDirection {
    Declining,
    Stable,
    Growing,
    Exponential,
    Cyclical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalityPattern {
    pub seasonal_multipliers: Vec<f64>, // 12 months
    pub weekly_pattern: Vec<f64>, // 7 days
    pub special_events: HashMap<String, f64>, // event -> multiplier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEffects {
    pub viral_coefficient: f64,
    pub adoption_threshold: f64,
    pub social_influence_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerSegment {
    pub segment_name: String,
    pub segment_size: u64,
    pub purchasing_power: f64,
    pub preferred_qualities: QualityPreference,
    pub price_sensitivity: f64,
    pub brand_loyalty: f64,
    pub innovation_adoption: AdoptionRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityPreference {
    pub quality_threshold: f64,
    pub quality_premium: f64,
    pub quality_variance_tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdoptionRate {
    Innovator,
    EarlyAdopter,
    EarlyMajority,
    LateMajority,
    Laggard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandDriver {
    pub driver_name: String,
    pub driver_type: DriverType,
    pub impact_strength: f64,
    pub affected_resources: Vec<Uuid>,
    pub activation_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DriverType {
    Economic,     // GDP, income levels
    Technological, // New innovations
    Social,       // Cultural trends
    Environmental, // Weather, disasters
    Political,    // Regulations, wars
    Demographic,  // Population changes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsumerBehaviorModel {
    pub race: Race,
    pub decision_making_style: DecisionMakingStyle,
    pub risk_tolerance: f64,
    pub social_influence_susceptibility: f64,
    pub impulse_buying_tendency: f64,
    pub long_term_planning_orientation: f64,
    pub cultural_values: CulturalValues,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionMakingStyle {
    Analytical,
    Intuitive,
    Collaborative,
    Hierarchical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalValues {
    pub collectivism_vs_individualism: f64,
    pub tradition_vs_innovation: f64,
    pub quality_vs_price: f64,
    pub status_importance: f64,
}

/// Calculates dynamic prices based on supply and demand
#[derive(Debug, Default)]
pub struct PriceCalculator {
    pub pricing_models: HashMap<Uuid, PricingModel>,
    pub base_prices: HashMap<Uuid, f64>,
    pub price_adjustments: HashMap<Uuid, PriceAdjustment>,
    pub market_maker_spreads: HashMap<String, f64>, // market_id -> spread
    pub arbitrage_opportunities: Vec<ArbitrageOpportunity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingModel {
    pub resource_id: Uuid,
    pub model_type: PricingModelType,
    pub parameters: HashMap<String, f64>,
    pub accuracy_metrics: AccuracyMetrics,
    pub last_calibration: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PricingModelType {
    SupplyDemandBasic,
    CompetitiveMarket,
    MonopolisticCompetition,
    AuctionBased,
    ReputationWeighted,
    QualityAdjusted,
    NetworkEffectAdjusted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccuracyMetrics {
    pub mean_absolute_error: f64,
    pub prediction_accuracy: f64,
    pub price_stability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceAdjustment {
    pub resource_id: Uuid,
    pub base_adjustment: f64,
    pub quality_premium: f64,
    pub scarcity_multiplier: f64,
    pub urgency_premium: f64,
    pub volume_discount: f64,
    pub reputation_discount: f64,
    pub regional_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub resource_id: Uuid,
    pub buy_market: String,
    pub sell_market: String,
    pub buy_price: f64,
    pub sell_price: f64,
    pub profit_margin: f64,
    pub risk_level: f64,
    pub opportunity_window: f64, // hours
}

/// Analyzes price elasticity of demand
#[derive(Debug, Default)]
pub struct ElasticityAnalyzer {
    pub price_elasticities: HashMap<Uuid, PriceElasticity>,
    pub income_elasticities: HashMap<Uuid, f64>,
    pub cross_elasticities: HashMap<(Uuid, Uuid), f64>,
    pub elasticity_trends: HashMap<Uuid, ElasticityTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceElasticity {
    pub resource_id: Uuid,
    pub elasticity_coefficient: f64,
    pub elasticity_type: ElasticityType,
    pub confidence_interval: (f64, f64),
    pub measurement_period: (SystemTime, SystemTime),
    pub sample_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElasticityType {
    PerfectlyInelastic, // 0
    Inelastic,         // 0 < |e| < 1
    UnitElastic,       // |e| = 1
    Elastic,           // |e| > 1
    PerfectlyElastic,  // infinite
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElasticityTrend {
    pub resource_id: Uuid,
    pub trend_direction: String,
    pub trend_strength: f64,
    pub contributing_factors: Vec<String>,
}

/// Models seasonal patterns and cyclical behavior
#[derive(Debug, Default)]
pub struct SeasonalModeler {
    pub seasonal_patterns: HashMap<Uuid, SeasonalPattern>,
    pub cyclical_trends: HashMap<Uuid, CyclicalTrend>,
    pub weather_impacts: HashMap<Uuid, WeatherImpact>,
    pub cultural_events: Vec<CulturalEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalPattern {
    pub resource_id: Uuid,
    pub seasonal_coefficients: Vec<f64>, // 12 months
    pub peak_seasons: Vec<u8>, // months (1-12)
    pub trough_seasons: Vec<u8>,
    pub seasonal_variance: f64,
    pub predictability_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyclicalTrend {
    pub resource_id: Uuid,
    pub cycle_length_months: f64,
    pub amplitude: f64,
    pub phase_offset: f64,
    pub trend_reliability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherImpact {
    pub resource_id: Uuid,
    pub weather_sensitivities: HashMap<String, f64>, // weather_type -> impact
    pub seasonal_adjustments: HashMap<u8, f64>, // month -> adjustment
    pub extreme_weather_effects: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalEvent {
    pub event_name: String,
    pub event_date: SystemTime,
    pub duration_days: u32,
    pub affected_resources: HashMap<Uuid, f64>, // resource -> impact multiplier
    pub cultural_groups: Vec<Race>,
    pub economic_impact: f64,
}

impl SupplyDemandEngine {
    /// Create a new supply and demand engine
    pub fn new() -> Self {
        Self {
            supply_tracker: Arc::new(RwLock::new(SupplyTracker::default())),
            demand_predictor: Arc::new(RwLock::new(DemandPredictor::default())),
            price_calculator: Arc::new(RwLock::new(PriceCalculator::default())),
            elasticity_analyzer: Arc::new(RwLock::new(ElasticityAnalyzer::default())),
            seasonal_modeler: Arc::new(RwLock::new(SeasonalModeler::default())),
        }
    }

    /// Initialize the supply and demand system
    pub async fn initialize(&self) -> Result<()> {
        info!("📊 Initializing supply and demand engine");

        // Initialize resource tracking
        self.initialize_resource_tracking().await?;
        
        // Setup demand models
        self.setup_demand_models().await?;
        
        // Initialize pricing models
        self.initialize_pricing_models().await?;
        
        // Setup seasonal patterns
        self.setup_seasonal_patterns().await?;

        info!("✅ Supply and demand engine initialized");
        Ok(())
    }

    /// Initialize tracking for common resources
    async fn initialize_resource_tracking(&self) -> Result<()> {
        let mut supply_tracker = self.supply_tracker.write().await;
        
        // Common resource types with different characteristics
        let resources = vec![
            ("Iron Ore", 1000, 50.0, 45.0, None),
            ("Wood", 2000, 100.0, 80.0, Some(0.1)), // Spoilage rate for organic materials
            ("Food", 500, 200.0, 180.0, Some(0.5)), // High spoilage
            ("Magical Crystals", 50, 5.0, 8.0, None),
            ("Rare Gems", 20, 2.0, 3.0, None),
            ("Crafted Tools", 200, 20.0, 25.0, None),
        ];

        for (name, available, production, consumption, spoilage) in resources {
            let resource_id = Uuid::new_v4();
            
            let resource_supply = ResourceSupply {
                resource_id,
                total_available: available,
                regional_distribution: HashMap::new(),
                production_rate: production,
                consumption_rate: consumption,
                spoilage_rate: spoilage,
                quality_levels: QualityLevelDistribution {
                    poor_supply: available / 10,
                    common_supply: available * 6 / 10,
                    rare_supply: available * 2 / 10,
                    epic_supply: available / 20,
                    legendary_supply: available / 100,
                },
                supplier_diversity: SupplierDiversity {
                    total_suppliers: 10,
                    supplier_concentration: 0.3,
                    new_suppliers_monthly: 2,
                    supplier_reliability: 0.85,
                },
                supply_volatility: 0.2,
                last_updated: SystemTime::now(),
            };

            supply_tracker.resource_supplies.insert(resource_id, resource_supply);
            info!("📦 Initialized supply tracking for {}", name);
        }

        Ok(())
    }

    /// Setup demand prediction models
    async fn setup_demand_models(&self) -> Result<()> {
        let mut demand_predictor = self.demand_predictor.write().await;
        
        // Create consumer segments
        let segments = vec![
            ("Casual Players", 1000, 0.5, 0.7),
            ("Hardcore Players", 200, 0.9, 0.3),
            ("Crafting Specialists", 150, 0.8, 0.4),
            ("Traders", 50, 0.95, 0.2),
        ];

        for (segment_name, size, purchasing_power, price_sensitivity) in segments {
            let segment = ConsumerSegment {
                segment_name: segment_name.to_string(),
                segment_size: size,
                purchasing_power,
                preferred_qualities: QualityPreference {
                    quality_threshold: 0.6,
                    quality_premium: 0.2,
                    quality_variance_tolerance: 0.1,
                },
                price_sensitivity,
                brand_loyalty: 0.4,
                innovation_adoption: AdoptionRate::EarlyMajority,
            };

            demand_predictor.consumer_segments.insert(segment_name.to_string(), segment);
        }

        // Setup behavioral models for different races
        let elf_behavior = ConsumerBehaviorModel {
            race: Race::Elf,
            decision_making_style: DecisionMakingStyle::Analytical,
            risk_tolerance: 0.4,
            social_influence_susceptibility: 0.3,
            impulse_buying_tendency: 0.2,
            long_term_planning_orientation: 0.8,
            cultural_values: CulturalValues {
                collectivism_vs_individualism: 0.3,
                tradition_vs_innovation: 0.7,
                quality_vs_price: 0.8,
                status_importance: 0.6,
            },
        };

        demand_predictor.behavioral_models.insert(Race::Elf, elf_behavior);

        Ok(())
    }

    /// Initialize pricing models for resources
    async fn initialize_pricing_models(&self) -> Result<()> {
        let mut price_calculator = self.price_calculator.write().await;
        
        // Set base prices for resources
        let base_prices = vec![
            ("Iron Ore", 10.0),
            ("Wood", 5.0),
            ("Food", 2.0),
            ("Magical Crystals", 100.0),
            ("Rare Gems", 500.0),
            ("Crafted Tools", 50.0),
        ];

        for (name, price) in base_prices {
            let resource_id = Uuid::new_v4();
            price_calculator.base_prices.insert(resource_id, price);
            
            let pricing_model = PricingModel {
                resource_id,
                model_type: PricingModelType::SupplyDemandBasic,
                parameters: {
                    let mut params = HashMap::new();
                    params.insert("base_price".to_string(), price);
                    params.insert("elasticity".to_string(), -0.5);
                    params.insert("volatility".to_string(), 0.2);
                    params
                },
                accuracy_metrics: AccuracyMetrics {
                    mean_absolute_error: 0.1,
                    prediction_accuracy: 0.85,
                    price_stability_score: 0.8,
                },
                last_calibration: SystemTime::now(),
            };

            price_calculator.pricing_models.insert(resource_id, pricing_model);
            info!("💰 Initialized pricing model for {} (base price: {})", name, price);
        }

        Ok(())
    }

    /// Setup seasonal patterns for resources
    async fn setup_seasonal_patterns(&self) -> Result<()> {
        let mut seasonal_modeler = self.seasonal_modeler.write().await;
        
        // Example seasonal pattern for food (higher demand in winter)
        let food_id = Uuid::new_v4();
        let food_pattern = SeasonalPattern {
            resource_id: food_id,
            seasonal_coefficients: vec![1.2, 1.1, 1.0, 0.9, 0.8, 0.8, 0.9, 0.9, 1.0, 1.1, 1.2, 1.3],
            peak_seasons: vec![12, 1, 2], // Winter months
            trough_seasons: vec![6, 7, 8], // Summer months
            seasonal_variance: 0.15,
            predictability_score: 0.9,
        };

        seasonal_modeler.seasonal_patterns.insert(food_id, food_pattern);

        Ok(())
    }

    /// Calculate current price for a resource
    pub async fn calculate_price(&self, resource_id: Uuid, market_id: &str, quantity: u64) -> Result<f64> {
        let supply_tracker = self.supply_tracker.read().await;
        let demand_predictor = self.demand_predictor.read().await;
        let price_calculator = self.price_calculator.read().await;

        // Get base price
        let base_price = price_calculator.base_prices.get(&resource_id).unwrap_or(&100.0);

        // Get supply information
        let supply_ratio = if let Some(supply) = supply_tracker.resource_supplies.get(&resource_id) {
            supply.total_available as f64 / (supply.total_available as f64 + supply.consumption_rate * 24.0)
        } else {
            0.5 // Default neutral ratio
        };

        // Get demand information
        let demand_multiplier = if let Some(demand) = demand_predictor.demand_patterns.get(&resource_id) {
            demand.current_demand / demand.base_demand
        } else {
            1.0 // Default neutral demand
        };

        // Calculate supply-demand adjusted price
        let supply_demand_factor = (2.0 - supply_ratio) * demand_multiplier;
        
        // Apply quantity discounts for bulk orders
        let quantity_factor = if quantity > 100 {
            0.95 // 5% discount for large orders
        } else if quantity > 500 {
            0.9  // 10% discount for very large orders
        } else {
            1.0
        };

        let final_price = base_price * supply_demand_factor * quantity_factor;

        debug!("💲 Calculated price for resource {} in market {}: {} (base: {}, s/d factor: {:.2}, qty factor: {:.2})", 
            resource_id, market_id, final_price, base_price, supply_demand_factor, quantity_factor);

        Ok(final_price)
    }

    /// Update supply levels based on production and consumption
    pub async fn update_supply_levels(&self) -> Result<()> {
        let mut supply_tracker = self.supply_tracker.write().await;
        
        for supply in supply_tracker.resource_supplies.values_mut() {
            // Calculate net change (production - consumption)
            let net_change = supply.production_rate - supply.consumption_rate;
            
            // Apply spoilage if applicable
            let spoilage_loss = if let Some(spoilage_rate) = supply.spoilage_rate {
                supply.total_available as f64 * spoilage_rate / 24.0 // Per hour
            } else {
                0.0
            };

            // Update total available (ensure it doesn't go negative)
            let new_total = ((supply.total_available as f64 + net_change - spoilage_loss).max(0.0)) as u64;
            supply.total_available = new_total;
            supply.last_updated = SystemTime::now();
        }

        Ok(())
    }

    /// Predict demand for next period
    pub async fn predict_demand(&self, resource_id: Uuid, prediction_hours: u64) -> Result<f64> {
        let demand_predictor = self.demand_predictor.read().await;
        let seasonal_modeler = self.seasonal_modeler.read().await;

        // Get base demand
        let base_demand = if let Some(pattern) = demand_predictor.demand_patterns.get(&resource_id) {
            pattern.base_demand
        } else {
            100.0 // Default demand
        };

        // Apply seasonal adjustments
        let seasonal_multiplier = if let Some(seasonal) = seasonal_modeler.seasonal_patterns.get(&resource_id) {
            // Simplified - would use current date to determine seasonal coefficient
            seasonal.seasonal_coefficients[0] // Use first month for now
        } else {
            1.0
        };

        // Apply trend adjustments
        let trend_multiplier = 1.05; // Assume 5% growth trend

        let predicted_demand = base_demand * seasonal_multiplier * trend_multiplier;

        debug!("📈 Predicted demand for resource {} over {} hours: {:.2}", 
            resource_id, prediction_hours, predicted_demand);

        Ok(predicted_demand)
    }

    /// Identify arbitrage opportunities
    pub async fn identify_arbitrage_opportunities(&self) -> Result<Vec<ArbitrageOpportunity>> {
        let price_calculator = self.price_calculator.read().await;
        
        // Simplified arbitrage detection
        // In a real implementation, this would compare prices across all markets
        let opportunities = vec![
            ArbitrageOpportunity {
                resource_id: Uuid::new_v4(),
                buy_market: "mining_outpost".to_string(),
                sell_market: "grand_bazaar".to_string(),
                buy_price: 95.0,
                sell_price: 105.0,
                profit_margin: 0.095, // 9.5%
                risk_level: 0.3,
                opportunity_window: 4.0, // 4 hours
            }
        ];

        Ok(opportunities)
    }

    /// Get supply and demand statistics
    pub async fn get_supply_demand_statistics(&self) -> SupplyDemandStatistics {
        let supply_tracker = self.supply_tracker.read().await;
        let demand_predictor = self.demand_predictor.read().await;
        
        let total_resources_tracked = supply_tracker.resource_supplies.len();
        let total_supply = supply_tracker.resource_supplies.values()
            .map(|supply| supply.total_available)
            .sum::<u64>();
        let consumer_segments = demand_predictor.consumer_segments.len();
        let demand_patterns = demand_predictor.demand_patterns.len();

        SupplyDemandStatistics {
            total_resources_tracked,
            total_supply,
            consumer_segments,
            demand_patterns,
            average_price_volatility: 0.2,
            market_efficiency_score: 0.85,
        }
    }
}

// Additional type definitions

#[derive(Debug, Default)]
pub struct BottleneckAnalysis {
    pub identified_bottlenecks: Vec<String>,
    pub severity_scores: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalInventory {
    pub market_id: String,
    pub inventory_levels: HashMap<Uuid, u64>, // resource_id -> quantity
    pub warehouse_capacity: u64,
    pub utilization_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyDemandStatistics {
    pub total_resources_tracked: usize,
    pub total_supply: u64,
    pub consumer_segments: usize,
    pub demand_patterns: usize,
    pub average_price_volatility: f64,
    pub market_efficiency_score: f64,
}