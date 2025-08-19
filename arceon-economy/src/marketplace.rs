use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, debug};
use arceon_core::entities::Race;
use arceon_ai::{AdaptiveBehaviorSystem, NeuralNetworkManager};

/// Decentralized marketplace system with AI-driven dynamics
pub struct MarketplaceSystem {
    pub market_manager: Arc<RwLock<MarketManager>>,
    pub order_book: Arc<RwLock<OrderBook>>,
    pub transaction_processor: Arc<RwLock<TransactionProcessor>>,
    pub market_analytics: Arc<RwLock<MarketAnalytics>>,
    pub reputation_system: Arc<RwLock<ReputationSystem>>,
    pub ai_behavior_system: Arc<AdaptiveBehaviorSystem>,
    pub neural_network_manager: Arc<NeuralNetworkManager>,
}

/// Manages multiple regional markets and their interactions
#[derive(Debug, Default)]
pub struct MarketManager {
    pub regional_markets: HashMap<String, RegionalMarket>,
    pub global_commodities: HashMap<Uuid, GlobalCommodity>,
    pub market_connections: HashMap<String, Vec<MarketConnection>>,
    pub economic_events: Vec<EconomicEvent>,
    pub market_regulations: HashMap<String, MarketRegulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalMarket {
    pub market_id: String,
    pub location: MarketLocation,
    pub specializations: Vec<MarketSpecialization>,
    pub local_resources: HashMap<Uuid, ResourceAvailability>,
    pub cultural_preferences: HashMap<Race, CulturalPreference>,
    pub trading_hours: TradingSchedule,
    pub market_size: MarketSize,
    pub economic_stability: f64,
    pub trade_volume_24h: f64,
    pub dominant_guilds: Vec<Uuid>,
    pub tax_rates: TaxStructure,
    pub infrastructure_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketLocation {
    pub region_name: String,
    pub coordinates: (f64, f64),
    pub climate_zone: ClimateZone,
    pub accessibility: AccessibilityRating,
    pub strategic_importance: f64,
    pub connected_regions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum MarketSpecialization {
    RareResources,
    CraftedGoods,
    FoodAndProvisions,
    MagicalComponents,
    TechnologicalDevices,
    CulturalArtifacts,
    ServicesAndSkills,
    InformationBrokerage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAvailability {
    pub resource_id: Uuid,
    pub current_supply: u64,
    pub production_rate: f64,
    pub seasonal_variation: f64,
    pub quality_distribution: QualityDistribution,
    pub local_demand_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityDistribution {
    pub poor_percentage: f64,
    pub common_percentage: f64,
    pub rare_percentage: f64,
    pub epic_percentage: f64,
    pub legendary_percentage: f64,
}

/// Advanced order book with multiple order types and AI optimization
#[derive(Debug, Default)]
pub struct OrderBook {
    pub buy_orders: HashMap<Uuid, Vec<BuyOrder>>,    // resource_id -> orders
    pub sell_orders: HashMap<Uuid, Vec<SellOrder>>,  // resource_id -> orders
    pub auction_orders: HashMap<Uuid, AuctionOrder>,
    pub bulk_orders: HashMap<Uuid, BulkOrder>,
    pub conditional_orders: HashMap<Uuid, ConditionalOrder>,
    pub subscription_orders: HashMap<Uuid, SubscriptionOrder>,
    pub matching_engine: OrderMatchingEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyOrder {
    pub order_id: Uuid,
    pub buyer_id: Uuid,
    pub resource_id: Uuid,
    pub quantity: u64,
    pub max_price: f64,
    pub quality_requirements: QualityRequirements,
    pub deadline: Option<SystemTime>,
    pub order_type: OrderType,
    pub market_region: String,
    pub payment_method: PaymentMethod,
    pub reputation_requirement: f64,
    pub special_conditions: Vec<SpecialCondition>,
    pub created_at: SystemTime,
    pub priority_level: PriorityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellOrder {
    pub order_id: Uuid,
    pub seller_id: Uuid,
    pub resource_id: Uuid,
    pub quantity: u64,
    pub min_price: f64,
    pub item_quality: f64,
    pub condition_description: String,
    pub order_type: OrderType,
    pub market_region: String,
    pub shipping_options: Vec<ShippingOption>,
    pub bulk_discounts: Vec<BulkDiscount>,
    pub expiration_time: Option<SystemTime>,
    pub created_at: SystemTime,
    pub seller_reputation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Market,        // Execute immediately at best available price
    Limit,         // Execute only at specified price or better
    Stop,          // Trigger when price reaches threshold
    StopLimit,     // Combination of stop and limit
    Iceberg,       // Large order executed in smaller chunks
    ReserveOrder,  // Futures contract for future delivery
    Auction,       // Competitive bidding
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionOrder {
    pub auction_id: Uuid,
    pub seller_id: Uuid,
    pub item_id: Uuid,
    pub starting_price: f64,
    pub reserve_price: Option<f64>,
    pub current_highest_bid: f64,
    pub highest_bidder: Option<Uuid>,
    pub bidding_history: Vec<BidRecord>,
    pub auction_type: AuctionType,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub bid_increment: f64,
    pub buyout_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuctionType {
    English,      // Open ascending price
    Dutch,        // Descending price
    Sealed,       // Hidden bids
    Vickrey,      // Second-price sealed
    Reserve,      // With minimum price
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidRecord {
    pub bidder_id: Uuid,
    pub bid_amount: f64,
    pub timestamp: SystemTime,
    pub bid_type: BidType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BidType {
    Regular,
    Proxy,
    Snipe,
    Counter,
}

/// Processes and validates all marketplace transactions
#[derive(Debug, Default)]
pub struct TransactionProcessor {
    pub pending_transactions: HashMap<Uuid, PendingTransaction>,
    pub completed_transactions: Vec<CompletedTransaction>,
    pub failed_transactions: Vec<FailedTransaction>,
    pub escrow_accounts: HashMap<Uuid, EscrowAccount>,
    pub payment_gateways: Vec<PaymentGateway>,
    pub fraud_detection: FraudDetectionSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingTransaction {
    pub transaction_id: Uuid,
    pub transaction_type: TransactionType,
    pub buyer_id: Uuid,
    pub seller_id: Uuid,
    pub resource_id: Uuid,
    pub quantity: u64,
    pub agreed_price: f64,
    pub total_amount: f64,
    pub fees: TransactionFees,
    pub escrow_required: bool,
    pub verification_steps: Vec<VerificationStep>,
    pub estimated_completion: SystemTime,
    pub timeout_deadline: SystemTime,
    pub blockchain_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    DirectPurchase,
    AuctionWin,
    BulkTrade,
    SubscriptionFulfillment,
    BarterExchange,
    ServiceContract,
    ResourceLease,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionFees {
    pub marketplace_fee: f64,
    pub processing_fee: f64,
    pub shipping_cost: f64,
    pub insurance_cost: f64,
    pub tax_amount: f64,
    pub total_fees: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowAccount {
    pub escrow_id: Uuid,
    pub transaction_id: Uuid,
    pub held_amount: f64,
    pub release_conditions: Vec<ReleaseCondition>,
    pub arbitrator: Option<Uuid>,
    pub created_at: SystemTime,
    pub auto_release_time: SystemTime,
}

/// Analytics and insights for market behavior
#[derive(Debug, Default)]
pub struct MarketAnalytics {
    pub price_history: HashMap<Uuid, PriceHistory>,
    pub volume_analytics: VolumeAnalytics,
    pub trend_analysis: TrendAnalysis,
    pub market_sentiment: MarketSentiment,
    pub predictive_models: Vec<PredictiveModel>,
    pub anomaly_detection: AnomalyDetection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceHistory {
    pub resource_id: Uuid,
    pub price_points: Vec<PricePoint>,
    pub moving_averages: MovingAverages,
    pub volatility_metrics: VolatilityMetrics,
    pub support_resistance_levels: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub timestamp: SystemTime,
    pub price: f64,
    pub volume: u64,
    pub market_region: String,
    pub transaction_count: u32,
}

/// Reputation system for traders and quality assurance
#[derive(Debug, Default)]
pub struct ReputationSystem {
    pub trader_reputations: HashMap<Uuid, TraderReputation>,
    pub reputation_history: HashMap<Uuid, Vec<ReputationEvent>>,
    pub verification_badges: HashMap<Uuid, Vec<VerificationBadge>>,
    pub dispute_resolution: DisputeResolution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderReputation {
    pub trader_id: Uuid,
    pub overall_score: f64,
    pub transaction_count: u64,
    pub successful_transactions: u64,
    pub dispute_rate: f64,
    pub average_delivery_time: f64,
    pub quality_rating: f64,
    pub communication_score: f64,
    pub specialization_scores: HashMap<MarketSpecialization, f64>,
    pub trust_network: Vec<TrustConnection>,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub event_id: Uuid,
    pub event_type: ReputationEventType,
    pub impact_score: f64,
    pub related_transaction: Option<Uuid>,
    pub timestamp: SystemTime,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReputationEventType {
    SuccessfulTrade,
    FailedDelivery,
    QualityIssue,
    ExceptionalService,
    DisputeResolution,
    CommunityContribution,
    VerificationEarned,
    PenaltyApplied,
}

impl MarketplaceSystem {
    /// Create a new marketplace system
    pub fn new(
        ai_behavior_system: Arc<AdaptiveBehaviorSystem>,
        neural_network_manager: Arc<NeuralNetworkManager>
    ) -> Self {
        Self {
            market_manager: Arc::new(RwLock::new(MarketManager::default())),
            order_book: Arc::new(RwLock::new(OrderBook::default())),
            transaction_processor: Arc::new(RwLock::new(TransactionProcessor::default())),
            market_analytics: Arc::new(RwLock::new(MarketAnalytics::default())),
            reputation_system: Arc::new(RwLock::new(ReputationSystem::default())),
            ai_behavior_system,
            neural_network_manager,
        }
    }

    /// Initialize the marketplace system with regional markets
    pub async fn initialize(&self) -> Result<()> {
        info!("🏪 Initializing dynamic marketplace system");

        // Create regional markets
        self.create_regional_markets().await?;
        
        // Initialize AI traders
        self.initialize_ai_traders().await?;
        
        // Setup market analytics
        self.setup_market_analytics().await?;
        
        // Initialize reputation system
        self.initialize_reputation_system().await?;

        info!("✅ Dynamic marketplace system initialized");
        Ok(())
    }

    /// Create regional markets with unique characteristics
    async fn create_regional_markets(&self) -> Result<()> {
        let mut manager = self.market_manager.write().await;
        
        // Create the Grand Bazaar - Central trading hub
        let grand_bazaar = RegionalMarket {
            market_id: "grand_bazaar".to_string(),
            location: MarketLocation {
                region_name: "Central Plains".to_string(),
                coordinates: (0.0, 0.0),
                climate_zone: ClimateZone::Temperate,
                accessibility: AccessibilityRating::Excellent,
                strategic_importance: 1.0,
                connected_regions: vec!["northern_mountains".to_string(), "southern_forests".to_string(), "eastern_coastlands".to_string()],
            },
            specializations: vec![
                MarketSpecialization::CraftedGoods,
                MarketSpecialization::CulturalArtifacts,
                MarketSpecialization::ServicesAndSkills,
            ],
            local_resources: HashMap::new(),
            cultural_preferences: HashMap::new(),
            trading_hours: TradingSchedule {
                open_hours: 18,
                market_days: vec![true; 7],
                seasonal_adjustments: HashMap::new(),
            },
            market_size: MarketSize::Metropolis,
            economic_stability: 0.9,
            trade_volume_24h: 1000000.0,
            dominant_guilds: vec![],
            tax_rates: TaxStructure {
                transaction_tax: 0.02,
                luxury_tax: 0.05,
                foreign_trader_tax: 0.03,
            },
            infrastructure_level: 0.95,
        };

        manager.regional_markets.insert("grand_bazaar".to_string(), grand_bazaar);

        // Create Mining Outpost - Resource specialization
        let mining_outpost = RegionalMarket {
            market_id: "mining_outpost".to_string(),
            location: MarketLocation {
                region_name: "Northern Mountains".to_string(),
                coordinates: (-100.0, 200.0),
                climate_zone: ClimateZone::Alpine,
                accessibility: AccessibilityRating::Moderate,
                strategic_importance: 0.7,
                connected_regions: vec!["grand_bazaar".to_string()],
            },
            specializations: vec![
                MarketSpecialization::RareResources,
                MarketSpecialization::TechnologicalDevices,
            ],
            local_resources: HashMap::new(),
            cultural_preferences: HashMap::new(),
            trading_hours: TradingSchedule {
                open_hours: 16,
                market_days: vec![true, true, true, true, true, true, false],
                seasonal_adjustments: HashMap::new(),
            },
            market_size: MarketSize::Town,
            economic_stability: 0.7,
            trade_volume_24h: 150000.0,
            dominant_guilds: vec![],
            tax_rates: TaxStructure {
                transaction_tax: 0.015,
                luxury_tax: 0.03,
                foreign_trader_tax: 0.025,
            },
            infrastructure_level: 0.6,
        };

        manager.regional_markets.insert("mining_outpost".to_string(), mining_outpost);

        // Create Coastal Harbor - Trade specialization
        let coastal_harbor = RegionalMarket {
            market_id: "coastal_harbor".to_string(),
            location: MarketLocation {
                region_name: "Eastern Coastlands".to_string(),
                coordinates: (200.0, 0.0),
                climate_zone: ClimateZone::Coastal,
                accessibility: AccessibilityRating::Excellent,
                strategic_importance: 0.8,
                connected_regions: vec!["grand_bazaar".to_string(), "southern_forests".to_string()],
            },
            specializations: vec![
                MarketSpecialization::FoodAndProvisions,
                MarketSpecialization::MagicalComponents,
                MarketSpecialization::InformationBrokerage,
            ],
            local_resources: HashMap::new(),
            cultural_preferences: HashMap::new(),
            trading_hours: TradingSchedule {
                open_hours: 20,
                market_days: vec![true; 7],
                seasonal_adjustments: HashMap::new(),
            },
            market_size: MarketSize::City,
            economic_stability: 0.8,
            trade_volume_24h: 500000.0,
            dominant_guilds: vec![],
            tax_rates: TaxStructure {
                transaction_tax: 0.018,
                luxury_tax: 0.04,
                foreign_trader_tax: 0.01, // Lower foreign tax to encourage international trade
            },
            infrastructure_level: 0.85,
        };

        manager.regional_markets.insert("coastal_harbor".to_string(), coastal_harbor);

        Ok(())
    }

    /// Initialize AI traders with different trading strategies
    async fn initialize_ai_traders(&self) -> Result<()> {
        info!("🤖 Initializing AI traders with diverse strategies");
        
        // Create different types of AI traders
        let trader_archetypes = vec![
            ("conservative_trader", "Focuses on stable, low-risk trades"),
            ("aggressive_speculator", "High-risk, high-reward trading strategy"),
            ("arbitrage_specialist", "Exploits price differences between markets"),
            ("bulk_trader", "Specializes in large volume transactions"),
            ("quality_curator", "Focuses on rare and high-quality items"),
            ("service_broker", "Trades in skills and services"),
        ];

        for (archetype, description) in trader_archetypes {
            let trader_id = Uuid::new_v4();
            
            // Initialize AI behavior for this trader
            self.ai_behavior_system.initialize_npc_behavior(
                trader_id,
                self.generate_trader_personality(archetype),
                archetype.to_string()
            ).await?;

            info!("✅ Initialized AI trader: {} ({})", archetype, description);
        }

        Ok(())
    }

    /// Generate personality for different trader archetypes
    fn generate_trader_personality(&self, archetype: &str) -> arceon_ai::PersonalityVector {
        use arceon_ai::PersonalityVector;
        
        match archetype {
            "conservative_trader" => PersonalityVector {
                openness: 0.3,
                conscientiousness: 0.9,
                extraversion: 0.4,
                agreeableness: 0.7,
                neuroticism: 0.2,
                curiosity: 0.5,
                creativity: 0.3,
                leadership: 0.4,
            },
            "aggressive_speculator" => PersonalityVector {
                openness: 0.8,
                conscientiousness: 0.4,
                extraversion: 0.8,
                agreeableness: 0.3,
                neuroticism: 0.6,
                curiosity: 0.9,
                creativity: 0.8,
                leadership: 0.7,
            },
            "arbitrage_specialist" => PersonalityVector {
                openness: 0.7,
                conscientiousness: 0.8,
                extraversion: 0.5,
                agreeableness: 0.5,
                neuroticism: 0.3,
                curiosity: 0.8,
                creativity: 0.6,
                leadership: 0.5,
            },
            "bulk_trader" => PersonalityVector {
                openness: 0.5,
                conscientiousness: 0.7,
                extraversion: 0.6,
                agreeableness: 0.8,
                neuroticism: 0.4,
                curiosity: 0.4,
                creativity: 0.4,
                leadership: 0.6,
            },
            "quality_curator" => PersonalityVector {
                openness: 0.9,
                conscientiousness: 0.8,
                extraversion: 0.4,
                agreeableness: 0.6,
                neuroticism: 0.3,
                curiosity: 0.9,
                creativity: 0.8,
                leadership: 0.5,
            },
            "service_broker" => PersonalityVector {
                openness: 0.6,
                conscientiousness: 0.6,
                extraversion: 0.9,
                agreeableness: 0.8,
                neuroticism: 0.4,
                curiosity: 0.7,
                creativity: 0.7,
                leadership: 0.8,
            },
            _ => PersonalityVector::default(),
        }
    }

    /// Setup market analytics and monitoring
    async fn setup_market_analytics(&self) -> Result<()> {
        let mut analytics = self.market_analytics.write().await;
        
        // Initialize price tracking for common resources
        let tracked_resources = vec![
            "iron_ore", "wood", "food", "magical_crystals", "rare_gems"
        ];

        for resource_name in tracked_resources {
            let resource_id = Uuid::new_v4();
            analytics.price_history.insert(resource_id, PriceHistory {
                resource_id,
                price_points: Vec::new(),
                moving_averages: MovingAverages {
                    ma_5: 0.0,
                    ma_20: 0.0,
                    ma_50: 0.0,
                    ema_12: 0.0,
                    ema_26: 0.0,
                },
                volatility_metrics: VolatilityMetrics {
                    daily_volatility: 0.0,
                    weekly_volatility: 0.0,
                    monthly_volatility: 0.0,
                    beta_coefficient: 1.0,
                },
                support_resistance_levels: Vec::new(),
            });
        }

        Ok(())
    }

    /// Initialize reputation system
    async fn initialize_reputation_system(&self) -> Result<()> {
        let mut reputation = self.reputation_system.write().await;
        
        // Setup dispute resolution system
        reputation.dispute_resolution = DisputeResolution {
            active_disputes: HashMap::new(),
            arbitrators: vec![],
            resolution_procedures: vec![],
            appeal_process: AppealProcess::default(),
        };

        Ok(())
    }

    /// Place a buy order in the marketplace
    pub async fn place_buy_order(&self, order: BuyOrder) -> Result<Uuid> {
        info!("📋 Placing buy order for {} units of resource {} at max price {}", 
            order.quantity, order.resource_id, order.max_price);

        let mut order_book = self.order_book.write().await;
        let order_id = order.order_id;
        let resource_id = order.resource_id;
        
        // Add to order book
        order_book.buy_orders
            .entry(resource_id)
            .or_insert_with(Vec::new)
            .push(order);

        // Try to match with existing sell orders
        self.attempt_order_matching(order_id, resource_id).await?;

        Ok(order_id)
    }

    /// Place a sell order in the marketplace
    pub async fn place_sell_order(&self, order: SellOrder) -> Result<Uuid> {
        info!("📋 Placing sell order for {} units of resource {} at min price {}", 
            order.quantity, order.resource_id, order.min_price);

        let mut order_book = self.order_book.write().await;
        let order_id = order.order_id;
        let resource_id = order.resource_id;
        
        // Add to order book
        order_book.sell_orders
            .entry(resource_id)
            .or_insert_with(Vec::new)
            .push(order);

        // Try to match with existing buy orders
        self.attempt_order_matching(order_id, resource_id).await?;

        Ok(order_id)
    }

    /// Attempt to match orders
    async fn attempt_order_matching(&self, _order_id: Uuid, _resource_id: Uuid) -> Result<()> {
        // Simplified matching logic - would be more sophisticated in practice
        // This would involve checking compatible orders and creating transactions
        
        debug!("🔄 Attempting order matching for resource");
        
        // The actual matching logic would:
        // 1. Find compatible buy/sell orders
        // 2. Check price compatibility
        // 3. Verify quality requirements
        // 4. Create pending transactions
        // 5. Initiate escrow if needed
        
        Ok(())
    }

    /// Get marketplace statistics
    pub async fn get_marketplace_statistics(&self) -> MarketplaceStatistics {
        let manager = self.market_manager.read().await;
        let order_book = self.order_book.read().await;
        let transaction_processor = self.transaction_processor.read().await;
        
        let total_markets = manager.regional_markets.len();
        let active_orders = order_book.buy_orders.values().map(|orders| orders.len()).sum::<usize>() +
                           order_book.sell_orders.values().map(|orders| orders.len()).sum::<usize>();
        let pending_transactions = transaction_processor.pending_transactions.len();
        let completed_transactions = transaction_processor.completed_transactions.len();
        
        let total_volume = manager.regional_markets.values()
            .map(|market| market.trade_volume_24h)
            .sum();

        MarketplaceStatistics {
            total_markets,
            active_orders,
            pending_transactions,
            completed_transactions,
            total_volume_24h: total_volume,
            average_transaction_time: 300.0, // 5 minutes average
        }
    }
}

// Supporting type definitions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClimateZone {
    Arctic,
    Alpine,
    Temperate,
    Coastal,
    Desert,
    Tropical,
    Underground,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityRating {
    Poor,
    Moderate,
    Good,
    Excellent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketSize {
    Village,
    Town,
    City,
    Metropolis,
    MegaHub,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSchedule {
    pub open_hours: u8,
    pub market_days: Vec<bool>, // Days of the week market is open
    pub seasonal_adjustments: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxStructure {
    pub transaction_tax: f64,
    pub luxury_tax: f64,
    pub foreign_trader_tax: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalPreference {
    pub race: Race,
    pub preferred_goods: Vec<Uuid>,
    pub price_sensitivity: f64,
    pub quality_importance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalCommodity {
    pub commodity_id: Uuid,
    pub name: String,
    pub category: String,
    pub base_value: f64,
    pub volatility_factor: f64,
    pub seasonal_patterns: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketConnection {
    pub connected_market: String,
    pub connection_strength: f64,
    pub transport_cost: f64,
    pub transport_time_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicEvent {
    pub event_id: Uuid,
    pub event_type: String,
    pub affected_markets: Vec<String>,
    pub impact_magnitude: f64,
    pub duration_hours: u64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketRegulation {
    pub regulation_name: String,
    pub description: String,
    pub enforcement_level: f64,
    pub penalties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub minimum_quality: f64,
    pub preferred_quality: f64,
    pub quality_premium: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentMethod {
    Gold,
    ResourceBarter,
    ServiceExchange,
    CreditNote,
    Cryptocurrency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PriorityLevel {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialCondition {
    pub condition_type: String,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingOption {
    pub method: String,
    pub cost: f64,
    pub delivery_time_hours: u64,
    pub insurance_included: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkDiscount {
    pub minimum_quantity: u64,
    pub discount_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkOrder {
    pub order_id: Uuid,
    pub resource_requirements: HashMap<Uuid, u64>,
    pub total_budget: f64,
    pub deadline: SystemTime,
    pub delivery_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOrder {
    pub order_id: Uuid,
    pub base_order: OrderType,
    pub trigger_conditions: Vec<String>,
    pub execution_parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionOrder {
    pub subscription_id: Uuid,
    pub resource_id: Uuid,
    pub quantity_per_period: u64,
    pub delivery_frequency: String,
    pub subscription_length: u64,
}

#[derive(Debug, Default)]
pub struct OrderMatchingEngine {
    pub matching_algorithm: String,
    pub optimization_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTransaction {
    pub transaction_id: Uuid,
    pub completion_time: SystemTime,
    pub final_price: f64,
    pub buyer_satisfaction: f64,
    pub seller_satisfaction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedTransaction {
    pub transaction_id: Uuid,
    pub failure_reason: String,
    pub failure_time: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationStep {
    pub step_name: String,
    pub completed: bool,
    pub verification_data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGateway {
    pub gateway_name: String,
    pub supported_currencies: Vec<String>,
    pub transaction_fee: f64,
}

#[derive(Debug, Default)]
pub struct FraudDetectionSystem {
    pub suspicious_patterns: Vec<String>,
    pub risk_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseCondition {
    pub condition_type: String,
    pub parameters: HashMap<String, String>,
    pub satisfied: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovingAverages {
    pub ma_5: f64,
    pub ma_20: f64,
    pub ma_50: f64,
    pub ema_12: f64,
    pub ema_26: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolatilityMetrics {
    pub daily_volatility: f64,
    pub weekly_volatility: f64,
    pub monthly_volatility: f64,
    pub beta_coefficient: f64,
}

#[derive(Debug, Default)]
pub struct VolumeAnalytics {
    pub daily_volume: f64,
    pub volume_trend: String,
}

#[derive(Debug, Default)]
pub struct TrendAnalysis {
    pub short_term_trend: String,
    pub long_term_trend: String,
}

#[derive(Debug, Default)]
pub struct MarketSentiment {
    pub overall_sentiment: f64,
    pub sentiment_indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveModel {
    pub model_name: String,
    pub accuracy: f64,
    pub prediction_horizon: u64,
}

#[derive(Debug, Default)]
pub struct AnomalyDetection {
    pub detected_anomalies: Vec<String>,
    pub risk_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationBadge {
    pub badge_type: String,
    pub issued_by: Uuid,
    pub issued_at: SystemTime,
    pub verification_criteria: Vec<String>,
}

#[derive(Debug, Default)]
pub struct DisputeResolution {
    pub active_disputes: HashMap<Uuid, String>,
    pub arbitrators: Vec<Uuid>,
    pub resolution_procedures: Vec<String>,
    pub appeal_process: AppealProcess,
}

#[derive(Debug, Default)]
pub struct AppealProcess {
    pub appeal_window_hours: u64,
    pub appeal_fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustConnection {
    pub connected_trader: Uuid,
    pub trust_score: f64,
    pub relationship_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStatistics {
    pub total_markets: usize,
    pub active_orders: usize,
    pub pending_transactions: usize,
    pub completed_transactions: usize,
    pub total_volume_24h: f64,
    pub average_transaction_time: f64,
}