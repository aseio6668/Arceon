pub mod marketplace;
pub mod supply_demand;
pub mod ai_traders;
pub mod economic_simulator;
pub mod price_engine;
pub mod trade_routes;
pub mod resource_economy;

// Re-export main types with module prefixes to avoid conflicts
pub use marketplace::{
    MarketplaceSystem, MarketManager, RegionalMarket, OrderBook,
    BuyOrder, SellOrder, OrderType, TransactionProcessor,
    PricePoint as MarketplacePricePoint,
};

pub use supply_demand::{
    SupplyDemandEngine, SupplyTracker, ResourceSupply, DemandPredictor,
    PriceCalculator, PricingModel, ElasticityAnalyzer, SeasonalModeler,
};

pub use ai_traders::{
    AiTraderSystem, TraderManager, AiTrader, TradingStrategy,
    StrategyEngine, RiskManager, TradingLearningSystem,
    TradingSchedule as AiTradingSchedule,
};

pub use economic_simulator::{
    EconomicSimulator, SimulationScenario, MarketModel, SimulationResult,
};

pub use price_engine::{
    PriceEngine, PricingAlgorithm, PricingContext,
    PricePoint as EnginePricePoint,
};

pub use trade_routes::{
    TradeRouteManager, TradeRoute,
};

pub use resource_economy::{
    ResourceEconomyManager, ResourceEconomy, ProductionChain,
};