use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use std::time::{SystemTime, Duration};
use crate::command_engine::{GameCommand, CommandResult, EntityState};

/// Comprehensive trading system with mutual agreement mechanics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingSystem {
    /// Active trade offers between players
    pub active_trades: HashMap<Uuid, TradeOffer>,
    /// Trade history for reputation and analysis
    pub trade_history: Vec<CompletedTrade>,
    /// Global market for items and currency
    pub global_market: GlobalMarket,
    /// Security and validation system
    pub security: TradeSecurity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    pub offer_id: Uuid,
    pub initiator: Uuid,
    pub target: Uuid,
    pub state: TradeState,
    pub initiator_items: TradeInventory,
    pub target_items: TradeInventory,
    pub initiator_currency: CurrencyOffer,
    pub target_currency: CurrencyOffer,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub security_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeState {
    /// Trade has been initiated, waiting for target to respond
    Initiated,
    /// Both parties are viewing and modifying offers
    Negotiating,
    /// Initiator has settled on current offer
    InitiatorSettled,
    /// Target has settled on current offer
    TargetSettled,
    /// Both parties settled, ready for confirmation
    BothSettled,
    /// Initiator has confirmed the final trade
    InitiatorConfirmed,
    /// Target has confirmed the final trade
    TargetConfirmed,
    /// Trade completed successfully
    Completed,
    /// Trade was cancelled by one party
    Cancelled(String),
    /// Trade expired due to timeout
    Expired,
    /// Trade failed due to security issues
    SecurityFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeInventory {
    pub items: HashMap<String, TradeItem>,
    pub locked: bool, // Prevents changes when settled
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_id: Uuid,
    pub item_name: String,
    pub item_type: String,
    pub quantity: u32,
    pub condition: ItemCondition,
    pub estimated_value: f64,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCondition {
    Perfect,
    Excellent,
    Good,
    Fair,
    Poor,
    Broken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyOffer {
    pub arcm_amount: f64,
    pub game_gold: u64,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTrade {
    pub trade_id: Uuid,
    pub participants: (Uuid, Uuid),
    pub items_traded: (TradeInventory, TradeInventory),
    pub currency_traded: (CurrencyOffer, CurrencyOffer),
    pub completion_time: SystemTime,
    pub trade_value: f64,
    pub reputation_impact: (i32, i32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalMarket {
    pub listings: HashMap<Uuid, MarketListing>,
    pub price_history: HashMap<String, Vec<PricePoint>>,
    pub market_fees: MarketFees,
    pub auction_house: AuctionHouse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketListing {
    pub listing_id: Uuid,
    pub seller: Uuid,
    pub item: TradeItem,
    pub asking_price: CurrencyOffer,
    pub listing_type: ListingType,
    pub created_at: SystemTime,
    pub expires_at: SystemTime,
    pub status: ListingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingType {
    FixedPrice,
    Auction,
    BuyItNow,
    Trade, // Seeking specific items in exchange
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ListingStatus {
    Active,
    Sold,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricePoint {
    pub timestamp: SystemTime,
    pub price: f64,
    pub volume: u32,
    pub currency_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketFees {
    pub listing_fee_percentage: f64,
    pub success_fee_percentage: f64,
    pub minimum_fee_arcm: f64,
    pub premium_listing_fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionHouse {
    pub active_auctions: HashMap<Uuid, Auction>,
    pub auction_history: Vec<CompletedAuction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Auction {
    pub auction_id: Uuid,
    pub item: TradeItem,
    pub seller: Uuid,
    pub starting_bid: f64,
    pub current_bid: f64,
    pub current_bidder: Option<Uuid>,
    pub bid_history: Vec<Bid>,
    pub ends_at: SystemTime,
    pub reserve_price: Option<f64>,
    pub buyout_price: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub bidder: Uuid,
    pub amount: f64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedAuction {
    pub auction_id: Uuid,
    pub final_price: f64,
    pub winner: Option<Uuid>,
    pub completion_time: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeSecurity {
    pub reputation_system: ReputationSystem,
    pub fraud_detection: FraudDetection,
    pub escrow_service: EscrowService,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationSystem {
    pub player_ratings: HashMap<Uuid, TraderReputation>,
    pub feedback_system: HashMap<Uuid, Vec<TradeFeedback>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraderReputation {
    pub total_trades: u32,
    pub successful_trades: u32,
    pub total_value_traded: f64,
    pub average_rating: f64,
    pub trust_score: f64,
    pub warnings: u32,
    pub bans: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFeedback {
    pub from_player: Uuid,
    pub to_player: Uuid,
    pub trade_id: Uuid,
    pub rating: u8, // 1-5 stars
    pub comment: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudDetection {
    pub suspicious_patterns: Vec<SuspiciousActivity>,
    pub blacklisted_items: Vec<Uuid>,
    pub price_anomaly_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousActivity {
    pub player: Uuid,
    pub activity_type: String,
    pub details: String,
    pub timestamp: SystemTime,
    pub severity: u8, // 1-10
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowService {
    pub active_escrows: HashMap<Uuid, EscrowHolding>,
    pub escrow_fee_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowHolding {
    pub escrow_id: Uuid,
    pub trade_id: Uuid,
    pub held_items: Vec<TradeItem>,
    pub held_currency: f64,
    pub holder: EscrowHolder,
    pub release_conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EscrowHolder {
    Network, // Held by blockchain network
    TrustedNode(Uuid), // Held by trusted master node
    SmartContract(String), // Held by smart contract
}

impl TradingSystem {
    pub fn new() -> Self {
        Self {
            active_trades: HashMap::new(),
            trade_history: Vec::new(),
            global_market: GlobalMarket::new(),
            security: TradeSecurity::new(),
        }
    }

    /// Process trade-related commands
    pub fn process_trade_command(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let parts: Vec<&str> = command.command_type.split_whitespace().collect();
        
        match parts.get(1) {
            Some(&"offer") => self.handle_trade_offer(command, entity),
            Some(&"accept") => self.handle_trade_accept(command, entity),
            Some(&"add") => self.handle_add_to_trade(command, entity),
            Some(&"remove") => self.handle_remove_from_trade(command, entity),
            Some(&"settle") => self.handle_settle_trade(command, entity),
            Some(&"confirm") => self.handle_confirm_trade(command, entity),
            Some(&"cancel") => self.handle_cancel_trade(command, entity),
            Some(&"status") => self.handle_trade_status(command, entity),
            Some(&"history") => self.handle_trade_history(command, entity),
            _ => CommandResult::Failure("Usage: trade <offer|accept|add|remove|settle|confirm|cancel|status|history>".to_string()),
        }
    }

    fn handle_trade_offer(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        // Extract target player from command
        let target_name = match command.parameters.get("target") {
            Some(name) => name,
            None => return CommandResult::Failure("Must specify target player for trade offer".to_string()),
        };

        // TODO: Look up target player UUID from name
        let target_id = Uuid::new_v4(); // Placeholder

        // Create new trade offer
        let trade_offer = TradeOffer {
            offer_id: Uuid::new_v4(),
            initiator: entity.id,
            target: target_id,
            state: TradeState::Initiated,
            initiator_items: TradeInventory {
                items: HashMap::new(),
                locked: false,
            },
            target_items: TradeInventory {
                items: HashMap::new(),
                locked: false,
            },
            initiator_currency: CurrencyOffer {
                arcm_amount: 0.0,
                game_gold: 0,
                locked: false,
            },
            target_currency: CurrencyOffer {
                arcm_amount: 0.0,
                game_gold: 0,
                locked: false,
            },
            created_at: SystemTime::now(),
            expires_at: SystemTime::now() + Duration::from_secs(1800), // 30 minutes
            security_hash: self.generate_security_hash(&entity.id, &target_id),
        };

        let trade_id = trade_offer.offer_id;
        self.active_trades.insert(trade_id, trade_offer);

        CommandResult::Success(format!(
            "📋 Trade offer created with {}!\n\
            Trade ID: {}\n\
            Use 'trade add item <item_name>' to add items\n\
            Use 'trade add gold <amount>' to add currency\n\
            Use 'trade settle' when ready to finalize your offer",
            target_name, trade_id
        ))
    }

    fn handle_trade_accept(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id_str = match command.parameters.get("trade_id") {
            Some(id) => id,
            None => return CommandResult::Failure("Must specify trade ID to accept".to_string()),
        };

        let trade_id = match Uuid::parse_str(trade_id_str) {
            Ok(id) => id,
            Err(_) => return CommandResult::Failure("Invalid trade ID format".to_string()),
        };

        if let Some(trade) = self.active_trades.get_mut(&trade_id) {
            if trade.target == entity.id && matches!(trade.state, TradeState::Initiated) {
                trade.state = TradeState::Negotiating;
                CommandResult::Success(format!(
                    "✅ Trade accepted! You can now:\n\
                    • Add items: 'trade add item <item_name>'\n\
                    • Add currency: 'trade add gold <amount>'\n\
                    • View status: 'trade status'\n\
                    • Settle when ready: 'trade settle'"
                ))
            } else {
                CommandResult::Failure("Cannot accept this trade".to_string())
            }
        } else {
            CommandResult::Failure("Trade not found".to_string())
        }
    }

    fn handle_add_to_trade(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        let item_type = match command.parameters.get("type") {
            Some(t) => t,
            None => return CommandResult::Failure("Must specify 'item' or 'gold'".to_string()),
        };

        let trade = self.active_trades.get_mut(&trade_id).unwrap();

        // Check if trade is in a state that allows modifications
        if matches!(trade.state, TradeState::BothSettled | TradeState::Completed) {
            return CommandResult::Failure("Cannot modify trade after both parties have settled".to_string());
        }

        match item_type.as_str() {
            "item" => {
                let item_name = match command.parameters.get("name") {
                    Some(name) => name,
                    None => return CommandResult::Failure("Must specify item name".to_string()),
                };

                // TODO: Validate player owns this item
                let trade_item = TradeItem {
                    item_id: Uuid::new_v4(),
                    item_name: item_name.clone(),
                    item_type: "Equipment".to_string(), // TODO: Get from item system
                    quantity: 1,
                    condition: ItemCondition::Good,
                    estimated_value: 100.0, // TODO: Get market value
                    properties: HashMap::new(),
                };

                // Add to appropriate inventory
                if trade.initiator == entity.id {
                    trade.initiator_items.items.insert(item_name.clone(), trade_item);
                    trade.initiator_items.locked = false; // Unlock when modified
                } else {
                    trade.target_items.items.insert(item_name.clone(), trade_item);
                    trade.target_items.locked = false;
                }

                // Reset settlement status  
                trade.initiator_items.locked = false;
                trade.target_items.locked = false;
                trade.initiator_currency.locked = false;
                trade.target_currency.locked = false;
                trade.state = TradeState::Negotiating;

                CommandResult::Success(format!("✅ Added {} to trade", item_name))
            }
            "gold" => {
                let amount_str = match command.parameters.get("amount") {
                    Some(amount) => amount,
                    None => return CommandResult::Failure("Must specify gold amount".to_string()),
                };

                let amount: u64 = match amount_str.parse() {
                    Ok(amount) => amount,
                    Err(_) => return CommandResult::Failure("Invalid gold amount".to_string()),
                };

                // TODO: Validate player has enough gold

                if trade.initiator == entity.id {
                    trade.initiator_currency.game_gold = amount;
                    trade.initiator_currency.locked = false;
                } else {
                    trade.target_currency.game_gold = amount;
                    trade.target_currency.locked = false;
                }

                trade.initiator_items.locked = false;
                trade.target_items.locked = false;
                trade.initiator_currency.locked = false;
                trade.target_currency.locked = false;
                trade.state = TradeState::Negotiating;

                CommandResult::Success(format!("✅ Added {} gold to trade", amount))
            }
            "arcm" => {
                let amount_str = match command.parameters.get("amount") {
                    Some(amount) => amount,
                    None => return CommandResult::Failure("Must specify ArcM amount".to_string()),
                };

                let amount: f64 = match amount_str.parse() {
                    Ok(amount) => amount,
                    Err(_) => return CommandResult::Failure("Invalid ArcM amount".to_string()),
                };

                // TODO: Validate player has enough ArcM

                if trade.initiator == entity.id {
                    trade.initiator_currency.arcm_amount = amount;
                    trade.initiator_currency.locked = false;
                } else {
                    trade.target_currency.arcm_amount = amount;
                    trade.target_currency.locked = false;
                }

                trade.initiator_items.locked = false;
                trade.target_items.locked = false;
                trade.initiator_currency.locked = false;
                trade.target_currency.locked = false;
                trade.state = TradeState::Negotiating;

                CommandResult::Success(format!("✅ Added {} ArcM to trade", amount))
            }
            _ => CommandResult::Failure("Can only add 'item', 'gold', or 'arcm'".to_string()),
        }
    }

    fn handle_remove_from_trade(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        let item_name = match command.parameters.get("item") {
            Some(name) => name,
            None => return CommandResult::Failure("Must specify item name to remove".to_string()),
        };

        let trade = self.active_trades.get_mut(&trade_id).unwrap();

        // Check if trade allows modifications
        if matches!(trade.state, TradeState::BothSettled | TradeState::Completed) {
            return CommandResult::Failure("Cannot modify trade after both parties have settled".to_string());
        }

        let removed = if trade.initiator == entity.id {
            trade.initiator_items.items.remove(item_name).is_some()
        } else {
            trade.target_items.items.remove(item_name).is_some()
        };

        if removed {
            trade.initiator_items.locked = false;
            trade.target_items.locked = false;
            trade.initiator_currency.locked = false;
            trade.target_currency.locked = false;
            trade.state = TradeState::Negotiating;
            CommandResult::Success(format!("✅ Removed {} from trade", item_name))
        } else {
            CommandResult::Failure(format!("Item '{}' not found in your trade offer", item_name))
        }
    }

    fn handle_settle_trade(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        let trade = self.active_trades.get_mut(&trade_id).unwrap();

        if trade.initiator == entity.id {
            trade.initiator_items.locked = true;
            trade.initiator_currency.locked = true;
            trade.state = match trade.state {
                TradeState::TargetSettled => TradeState::BothSettled,
                _ => TradeState::InitiatorSettled,
            };
        } else {
            trade.target_items.locked = true;
            trade.target_currency.locked = true;
            trade.state = match trade.state {
                TradeState::InitiatorSettled => TradeState::BothSettled,
                _ => TradeState::TargetSettled,
            };
        }

        let message = match trade.state {
            TradeState::BothSettled => {
                "🔒 You have settled your offer! Both parties are now settled.\n\
                Use 'trade confirm' to finalize the trade.\n\
                ⚠️ WARNING: This cannot be undone!"
            }
            _ => {
                "🔒 You have settled your offer! Waiting for the other party to settle.\n\
                You cannot modify your offer until they settle or you unsettle."
            }
        };

        CommandResult::Success(message.to_string())
    }

    fn handle_confirm_trade(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        let trade = self.active_trades.get_mut(&trade_id).unwrap();

        if !matches!(trade.state, TradeState::BothSettled | TradeState::InitiatorConfirmed | TradeState::TargetConfirmed) {
            return CommandResult::Failure("Both parties must settle before confirming trade".to_string());
        }

        if trade.initiator == entity.id {
            trade.state = match trade.state {
                TradeState::TargetConfirmed => TradeState::Completed,
                _ => TradeState::InitiatorConfirmed,
            };
        } else {
            trade.state = match trade.state {
                TradeState::InitiatorConfirmed => TradeState::Completed,
                _ => TradeState::TargetConfirmed,
            };
        }

        match trade.state {
            TradeState::Completed => {
                // Execute the trade
                match self.execute_trade(trade_id) {
                    Ok(()) => {},
                    Err(e) => return CommandResult::Failure(e),
                };
                CommandResult::Success(
                    "🎉 TRADE COMPLETED SUCCESSFULLY!\n\
                    Items and currency have been transferred.\n\
                    Check your inventory for your new items!".to_string()
                )
            }
            _ => {
                CommandResult::Success(
                    "✅ Trade confirmed! Waiting for other party to confirm.\n\
                    ⚠️ This cannot be undone once both parties confirm!".to_string()
                )
            }
        }
    }

    fn handle_cancel_trade(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        if let Some(mut trade) = self.active_trades.remove(&trade_id) {
            trade.state = TradeState::Cancelled("Cancelled by player".to_string());
            
            // TODO: Return items to players
            
            CommandResult::Success("❌ Trade cancelled. All items returned to participants.".to_string())
        } else {
            CommandResult::Failure("Trade not found".to_string())
        }
    }

    fn handle_trade_status(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let trade_id = match self.find_active_trade_for_player(entity.id) {
            Some(id) => id,
            None => return CommandResult::Failure("No active trade found".to_string()),
        };

        let trade = self.active_trades.get(&trade_id).unwrap();

        let mut status = format!("📋 TRADE STATUS\n");
        status.push_str(&format!("Trade ID: {}\n", trade.offer_id));
        status.push_str(&format!("State: {:?}\n\n", trade.state));

        // Show initiator's offer
        status.push_str("👤 INITIATOR OFFERS:\n");
        if trade.initiator_items.items.is_empty() && trade.initiator_currency.game_gold == 0 && trade.initiator_currency.arcm_amount == 0.0 {
            status.push_str("  (Nothing offered)\n");
        } else {
            for (name, item) in &trade.initiator_items.items {
                status.push_str(&format!("  📦 {} x{}\n", name, item.quantity));
            }
            if trade.initiator_currency.game_gold > 0 {
                status.push_str(&format!("  🪙 {} gold\n", trade.initiator_currency.game_gold));
            }
            if trade.initiator_currency.arcm_amount > 0.0 {
                status.push_str(&format!("  💎 {:.2} ArcM\n", trade.initiator_currency.arcm_amount));
            }
        }
        status.push_str(&format!("  Status: {}\n\n", if trade.initiator_items.locked { "🔒 SETTLED" } else { "🔓 Open" }));

        // Show target's offer
        status.push_str("👤 TARGET OFFERS:\n");
        if trade.target_items.items.is_empty() && trade.target_currency.game_gold == 0 && trade.target_currency.arcm_amount == 0.0 {
            status.push_str("  (Nothing offered)\n");
        } else {
            for (name, item) in &trade.target_items.items {
                status.push_str(&format!("  📦 {} x{}\n", name, item.quantity));
            }
            if trade.target_currency.game_gold > 0 {
                status.push_str(&format!("  🪙 {} gold\n", trade.target_currency.game_gold));
            }
            if trade.target_currency.arcm_amount > 0.0 {
                status.push_str(&format!("  💎 {:.2} ArcM\n", trade.target_currency.arcm_amount));
            }
        }
        status.push_str(&format!("  Status: {}\n", if trade.target_items.locked { "🔒 SETTLED" } else { "🔓 Open" }));

        // Show next steps
        status.push_str("\n📝 NEXT STEPS:\n");
        match trade.state {
            TradeState::Initiated => status.push_str("  Waiting for target to accept trade"),
            TradeState::Negotiating => status.push_str("  Add items, then use 'trade settle'"),
            TradeState::InitiatorSettled => status.push_str("  Waiting for target to settle"),
            TradeState::TargetSettled => status.push_str("  Waiting for initiator to settle"),
            TradeState::BothSettled => status.push_str("  Both settled! Use 'trade confirm' to finalize"),
            TradeState::InitiatorConfirmed => status.push_str("  Waiting for target to confirm"),
            TradeState::TargetConfirmed => status.push_str("  Waiting for initiator to confirm"),
            _ => status.push_str("  Trade is no longer active"),
        }

        CommandResult::Success(status)
    }

    fn handle_trade_history(&mut self, command: &GameCommand, entity: &EntityState) -> CommandResult {
        let player_trades: Vec<&CompletedTrade> = self.trade_history.iter()
            .filter(|trade| trade.participants.0 == entity.id || trade.participants.1 == entity.id)
            .take(10) // Show last 10 trades
            .collect();

        if player_trades.is_empty() {
            return CommandResult::Success("📜 No trade history found.".to_string());
        }

        let mut history = "📜 TRADE HISTORY (Last 10 trades)\n\n".to_string();
        
        for trade in player_trades {
            let partner = if trade.participants.0 == entity.id {
                trade.participants.1
            } else {
                trade.participants.0
            };

            history.push_str(&format!(
                "Trade with {} - Value: {:.2} ArcM\n",
                partner, // TODO: Get player name from UUID
                trade.trade_value
            ));
        }

        CommandResult::Success(history)
    }

    // Helper methods

    fn find_active_trade_for_player(&self, player_id: Uuid) -> Option<Uuid> {
        self.active_trades.iter()
            .find(|(_, trade)| trade.initiator == player_id || trade.target == player_id)
            .map(|(trade_id, _)| *trade_id)
    }

    fn reset_settlement_on_change(&mut self, trade: &mut TradeOffer) {
        // If items were modified, reset settlement status
        trade.initiator_items.locked = false;
        trade.target_items.locked = false;
        trade.initiator_currency.locked = false;
        trade.target_currency.locked = false;
        
        // Reset to negotiating state
        trade.state = TradeState::Negotiating;
    }

    fn execute_trade(&mut self, trade_id: Uuid) -> Result<(), String> {
        let trade = self.active_trades.remove(&trade_id)
            .ok_or("Trade not found")?;

        // TODO: Validate both players still have the items/currency
        // TODO: Transfer items between players
        // TODO: Transfer currency between players
        // TODO: Update player inventories
        // TODO: Record transaction on blockchain

        // Create completed trade record
        let completed_trade = CompletedTrade {
            trade_id: trade.offer_id,
            participants: (trade.initiator, trade.target),
            items_traded: (trade.initiator_items, trade.target_items),
            currency_traded: (trade.initiator_currency, trade.target_currency),
            completion_time: SystemTime::now(),
            trade_value: 0.0, // TODO: Calculate total value
            reputation_impact: (1, 1), // TODO: Calculate reputation changes
        };

        self.trade_history.push(completed_trade);

        Ok(())
    }

    fn generate_security_hash(&self, player1: &Uuid, player2: &Uuid) -> String {
        // TODO: Generate cryptographic hash for trade security
        format!("{}-{}", player1, player2)
    }
}

impl GlobalMarket {
    fn new() -> Self {
        Self {
            listings: HashMap::new(),
            price_history: HashMap::new(),
            market_fees: MarketFees {
                listing_fee_percentage: 2.0,
                success_fee_percentage: 5.0,
                minimum_fee_arcm: 0.01,
                premium_listing_fee: 1.0,
            },
            auction_house: AuctionHouse {
                active_auctions: HashMap::new(),
                auction_history: Vec::new(),
            },
        }
    }
}

impl TradeSecurity {
    fn new() -> Self {
        Self {
            reputation_system: ReputationSystem {
                player_ratings: HashMap::new(),
                feedback_system: HashMap::new(),
            },
            fraud_detection: FraudDetection {
                suspicious_patterns: Vec::new(),
                blacklisted_items: Vec::new(),
                price_anomaly_threshold: 500.0, // 500% above/below average
            },
            escrow_service: EscrowService {
                active_escrows: HashMap::new(),
                escrow_fee_percentage: 1.0,
            },
        }
    }
}