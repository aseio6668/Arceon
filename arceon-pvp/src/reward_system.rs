/*!
# Reward System

Comprehensive reward distribution system for PvP matches and tournaments
with dynamic calculations, seasonal bonuses, and achievement tracking.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;
use anyhow::Result;

use crate::{PlayerId, MatchId, TournamentId, MatchResult, RankTier};

/// Reward distribution coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistributor {
    pub reward_pools: HashMap<String, RewardPool>,
    pub distribution_rules: Vec<DistributionRule>,
    pub seasonal_modifiers: SeasonalModifiers,
    pub achievement_rewards: HashMap<Uuid, AchievementReward>,
    pub player_rewards: HashMap<PlayerId, PlayerRewards>,
    pub distribution_history: Vec<RewardDistribution>,
}

/// Reward pool for different types of activities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPool {
    pub pool_id: String,
    pub pool_type: PoolType,
    pub currency_reserves: HashMap<String, u64>,
    pub item_reserves: HashMap<String, u32>,
    pub replenishment_rate: ReplenishmentRate,
    pub distribution_limits: DistributionLimits,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolType {
    MatchRewards,
    TournamentPrizes,
    SeasonalBonuses,
    AchievementRewards,
    DailyRewards,
    WeeklyRewards,
    SpecialEvents,
    CompensationPool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplenishmentRate {
    pub base_rate: HashMap<String, u64>, // Per hour
    pub scaling_factors: HashMap<String, f64>,
    pub maximum_capacity: HashMap<String, u64>,
    pub emergency_funding: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionLimits {
    pub daily_limit: HashMap<String, u64>,
    pub weekly_limit: HashMap<String, u64>,
    pub per_player_limit: HashMap<String, u64>,
    pub minimum_distribution: HashMap<String, u64>,
}

/// Rules for reward distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionRule {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub applicable_contexts: Vec<RewardContext>,
    pub calculation_method: CalculationMethod,
    pub base_rewards: HashMap<String, u64>,
    pub multipliers: Vec<RewardMultiplier>,
    pub conditions: Vec<RewardCondition>,
    pub priority: u32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RewardContext {
    MatchVictory,
    MatchParticipation,
    TournamentPlacement,
    RankPromotion,
    Achievement,
    DailyObjective,
    WeeklyChallenge,
    SeasonEnd,
    SpecialEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CalculationMethod {
    Fixed,
    PerformanceBased,
    RankingBased,
    ParticipationBased,
    Progressive,
    Tiered,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardMultiplier {
    pub multiplier_type: MultiplierType,
    pub factor: f64,
    pub conditions: Vec<String>,
    pub duration: Option<Duration>,
    pub stack_behavior: StackBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MultiplierType {
    WinStreak,
    RankBonus,
    SeasonalEvent,
    FirstWin,
    PerfectMatch,
    Underdog,
    SkillGap,
    TournamentBonus,
    VIPStatus,
    GroupBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StackBehavior {
    Additive,
    Multiplicative,
    Highest,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCondition {
    pub condition_type: ConditionType,
    pub threshold: f64,
    pub comparison: ComparisonOperator,
    pub reward_modifier: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    PlayerRank,
    MatchDuration,
    PerformanceScore,
    TeamSize,
    OpponentRank,
    WinStreak,
    ParticipationCount,
    TimeOfDay,
    DayOfWeek,
    SeasonProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    GreaterOrEqual,
    LessOrEqual,
    NotEqual,
    Between(f64, f64),
}

/// Seasonal modifiers and events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalModifiers {
    pub current_season: Season,
    pub active_events: Vec<SeasonalEvent>,
    pub global_multipliers: HashMap<String, f64>,
    pub rank_decay_settings: RankDecaySettings,
    pub season_rewards: Vec<SeasonReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub season_id: String,
    pub season_name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub theme: String,
    pub special_rewards: Vec<String>,
    pub progression_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalEvent {
    pub event_id: Uuid,
    pub event_name: String,
    pub event_type: EventType,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub reward_modifiers: HashMap<String, f64>,
    pub special_rewards: Vec<SpecialReward>,
    pub participation_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    DoubleXP,
    BonusGold,
    SpecialTournament,
    ThematicEvent,
    CommunityChallenge,
    HolidayEvent,
    Anniversary,
    BetaTest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecialReward {
    pub reward_id: Uuid,
    pub reward_type: RewardType,
    pub item_name: String,
    pub rarity: RewardRarity,
    pub quantity: u32,
    pub conditions: Vec<String>,
    pub time_limited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    Currency,
    Experience,
    Item,
    Cosmetic,
    Title,
    Badge,
    Emote,
    Avatar,
    Border,
    Effect,
    Mount,
    Companion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Mythic,
    Unique,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankDecaySettings {
    pub enabled: bool,
    pub decay_rate: f64, // Per day
    pub inactivity_threshold: Duration,
    pub minimum_rank: RankTier,
    pub grace_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonReward {
    pub reward_tier: u32,
    pub required_rank: RankTier,
    pub rewards: Vec<SpecialReward>,
    pub retroactive: bool,
}

/// Achievement-based rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    pub achievement_id: Uuid,
    pub reward_package: RewardPackage,
    pub unlock_requirements: Vec<String>,
    pub one_time_only: bool,
    pub prestige_variants: Vec<PrestigeVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPackage {
    pub package_id: Uuid,
    pub package_name: String,
    pub primary_rewards: Vec<RewardItem>,
    pub bonus_rewards: Vec<RewardItem>,
    pub choice_rewards: Vec<Vec<RewardItem>>, // Player can choose from sets
    pub guaranteed_rewards: Vec<RewardItem>,
    pub random_rewards: Vec<RandomRewardPool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardItem {
    pub item_type: RewardType,
    pub item_identifier: String,
    pub quantity: u64,
    pub quality_tier: u32,
    pub bind_type: BindType,
    pub expiration: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindType {
    None,
    OnPickup,
    OnEquip,
    OnUse,
    Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomRewardPool {
    pub pool_name: String,
    pub possible_rewards: Vec<WeightedReward>,
    pub guaranteed_count: u32,
    pub max_count: u32,
    pub duplicate_protection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightedReward {
    pub reward: RewardItem,
    pub weight: f64,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeVariant {
    pub prestige_level: u32,
    pub modified_rewards: Vec<RewardItem>,
    pub additional_rewards: Vec<RewardItem>,
    pub visual_upgrades: Vec<String>,
}

/// Player reward tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRewards {
    pub player_id: PlayerId,
    pub total_earned: HashMap<String, u64>,
    pub seasonal_earned: HashMap<String, u64>,
    pub daily_earned: HashMap<String, u64>,
    pub weekly_earned: HashMap<String, u64>,
    pub streak_bonuses: StreakBonuses,
    pub pending_rewards: Vec<PendingReward>,
    pub reward_preferences: RewardPreferences,
    pub claim_history: Vec<RewardClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreakBonuses {
    pub current_win_streak: u32,
    pub best_win_streak: u32,
    pub daily_login_streak: u32,
    pub match_participation_streak: u32,
    pub streak_multipliers: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReward {
    pub reward_id: Uuid,
    pub source: RewardSource,
    pub reward_items: Vec<RewardItem>,
    pub earned_date: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub claim_requirements: Vec<String>,
    pub notification_sent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardSource {
    MatchCompletion,
    TournamentPlacement,
    Achievement,
    DailyObjective,
    SeasonalReward,
    SpecialEvent,
    Compensation,
    Gift,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardPreferences {
    pub auto_claim: bool,
    pub notification_types: Vec<NotificationType>,
    pub preferred_currencies: Vec<String>,
    pub item_categories: Vec<ItemCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    InGame,
    Email,
    Push,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ItemCategory {
    Cosmetics,
    Consumables,
    Equipment,
    Currency,
    Experience,
    Titles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardClaim {
    pub claim_id: Uuid,
    pub reward_id: Uuid,
    pub claimed_date: DateTime<Utc>,
    pub claimed_items: Vec<RewardItem>,
    pub satisfaction_rating: Option<u32>,
}

/// Reward distribution tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardDistribution {
    pub distribution_id: Uuid,
    pub distribution_type: DistributionType,
    pub source_event: SourceEvent,
    pub recipients: Vec<RewardRecipient>,
    pub total_distributed: HashMap<String, u64>,
    pub distribution_date: DateTime<Utc>,
    pub distribution_rules_applied: Vec<Uuid>,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributionType {
    Individual,
    Group,
    Broadcast,
    Tournament,
    Seasonal,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceEvent {
    Match(MatchId),
    Tournament(TournamentId),
    Achievement(Uuid),
    SeasonEnd(String),
    SpecialEvent(Uuid),
    Administrative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardRecipient {
    pub player_id: PlayerId,
    pub rewards_received: Vec<RewardItem>,
    pub multipliers_applied: Vec<String>,
    pub delivery_status: DeliveryStatus,
    pub delivery_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeliveryStatus {
    Pending,
    Delivered,
    Failed,
    Claimed,
    Expired,
    Rejected,
}

impl RewardDistributor {
    /// Create new reward distributor
    pub fn new() -> Self {
        let mut distributor = Self {
            reward_pools: HashMap::new(),
            distribution_rules: vec![],
            seasonal_modifiers: SeasonalModifiers::default(),
            achievement_rewards: HashMap::new(),
            player_rewards: HashMap::new(),
            distribution_history: vec![],
        };

        distributor.initialize_default_pools();
        distributor.initialize_default_rules();
        distributor
    }

    /// Initialize default reward pools
    fn initialize_default_pools(&mut self) {
        // Match rewards pool
        let match_pool = RewardPool {
            pool_id: "match_rewards".to_string(),
            pool_type: PoolType::MatchRewards,
            currency_reserves: [
                ("gold".to_string(), 1_000_000),
                ("experience".to_string(), 10_000_000),
                ("rating_points".to_string(), 100_000),
            ].into(),
            item_reserves: [
                ("victory_tokens".to_string(), 50_000),
                ("skill_badges".to_string(), 10_000),
            ].into(),
            replenishment_rate: ReplenishmentRate {
                base_rate: [
                    ("gold".to_string(), 1000),
                    ("experience".to_string(), 10000),
                ].into(),
                scaling_factors: [
                    ("player_activity".to_string(), 1.2),
                ].into(),
                maximum_capacity: [
                    ("gold".to_string(), 5_000_000),
                    ("experience".to_string(), 50_000_000),
                ].into(),
                emergency_funding: [
                    ("gold".to_string(), 100_000),
                ].into(),
            },
            distribution_limits: DistributionLimits {
                daily_limit: [("gold".to_string(), 10_000)].into(),
                weekly_limit: [("gold".to_string(), 50_000)].into(),
                per_player_limit: [("gold".to_string(), 1_000)].into(),
                minimum_distribution: [("experience".to_string(), 10)].into(),
            },
            last_updated: Utc::now(),
        };

        // Tournament prize pool
        let tournament_pool = RewardPool {
            pool_id: "tournament_prizes".to_string(),
            pool_type: PoolType::TournamentPrizes,
            currency_reserves: [
                ("gold".to_string(), 500_000),
                ("tournament_points".to_string(), 100_000),
            ].into(),
            item_reserves: [
                ("champion_trophies".to_string(), 100),
                ("exclusive_titles".to_string(), 500),
            ].into(),
            replenishment_rate: ReplenishmentRate {
                base_rate: [("gold".to_string(), 500)].into(),
                scaling_factors: HashMap::new(),
                maximum_capacity: [("gold".to_string(), 2_000_000)].into(),
                emergency_funding: HashMap::new(),
            },
            distribution_limits: DistributionLimits {
                daily_limit: HashMap::new(),
                weekly_limit: [("gold".to_string(), 100_000)].into(),
                per_player_limit: [("gold".to_string(), 50_000)].into(),
                minimum_distribution: HashMap::new(),
            },
            last_updated: Utc::now(),
        };

        self.reward_pools.insert("match_rewards".to_string(), match_pool);
        self.reward_pools.insert("tournament_prizes".to_string(), tournament_pool);
    }

    /// Initialize default distribution rules
    fn initialize_default_rules(&mut self) {
        // Victory reward rule
        let victory_rule = DistributionRule {
            rule_id: Uuid::new_v4(),
            rule_name: "Match Victory Rewards".to_string(),
            applicable_contexts: vec![RewardContext::MatchVictory],
            calculation_method: CalculationMethod::RankingBased,
            base_rewards: [
                ("gold".to_string(), 100),
                ("experience".to_string(), 200),
                ("rating_points".to_string(), 25),
            ].into(),
            multipliers: vec![
                RewardMultiplier {
                    multiplier_type: MultiplierType::WinStreak,
                    factor: 1.1, // +10% per win streak level
                    conditions: vec!["win_streak >= 3".to_string()],
                    duration: None,
                    stack_behavior: StackBehavior::Multiplicative,
                },
                RewardMultiplier {
                    multiplier_type: MultiplierType::RankBonus,
                    factor: 1.5, // Higher rank = more rewards
                    conditions: vec!["player_rank >= Gold".to_string()],
                    duration: None,
                    stack_behavior: StackBehavior::Multiplicative,
                },
            ],
            conditions: vec![
                RewardCondition {
                    condition_type: ConditionType::MatchDuration,
                    threshold: 60.0, // Minimum 1 minute
                    comparison: ComparisonOperator::GreaterThan,
                    reward_modifier: 1.0,
                    description: "Match must last at least 1 minute".to_string(),
                },
            ],
            priority: 1,
            enabled: true,
        };

        // Participation reward rule
        let participation_rule = DistributionRule {
            rule_id: Uuid::new_v4(),
            rule_name: "Match Participation Rewards".to_string(),
            applicable_contexts: vec![RewardContext::MatchParticipation],
            calculation_method: CalculationMethod::Fixed,
            base_rewards: [
                ("experience".to_string(), 50),
                ("participation_points".to_string(), 10),
            ].into(),
            multipliers: vec![
                RewardMultiplier {
                    multiplier_type: MultiplierType::FirstWin,
                    factor: 2.0, // Double rewards for first win of day
                    conditions: vec!["first_win_today".to_string()],
                    duration: Some(Duration::days(1)),
                    stack_behavior: StackBehavior::None,
                },
            ],
            conditions: vec![],
            priority: 2,
            enabled: true,
        };

        self.distribution_rules.push(victory_rule);
        self.distribution_rules.push(participation_rule);
    }

    /// Distribute match rewards
    pub fn distribute_match_rewards(&mut self, match_result: &MatchResult) -> Result<RewardDistribution> {
        let distribution_id = Uuid::new_v4();
        let mut recipients = vec![];
        let mut total_distributed: HashMap<String, u64> = HashMap::new();

        // Process each participant
        let participants = [match_result.opponents.clone(), match_result.teammates.clone()].concat();
        
        for &player_id in &participants {
            let player_rewards = self.calculate_match_rewards(player_id, match_result)?;
            let multipliers_applied = self.get_applicable_multipliers(player_id, match_result);
            
            // Apply multipliers
            let final_rewards = self.apply_multipliers(&player_rewards, &multipliers_applied);
            
            // Update totals
            for reward in &final_rewards {
                *total_distributed.entry(reward.item_identifier.clone()).or_insert(0) += reward.quantity;
            }

            recipients.push(RewardRecipient {
                player_id,
                rewards_received: final_rewards,
                multipliers_applied: multipliers_applied.iter().map(|m| format!("{:?}", m.multiplier_type)).collect(),
                delivery_status: DeliveryStatus::Pending,
                delivery_attempts: 0,
            });
        }

        // Create distribution record
        let distribution = RewardDistribution {
            distribution_id,
            distribution_type: DistributionType::Individual,
            source_event: SourceEvent::Match(match_result.match_id),
            recipients,
            total_distributed,
            distribution_date: Utc::now(),
            distribution_rules_applied: self.distribution_rules.iter().map(|r| r.rule_id).collect(),
            success_rate: 1.0, // Will be updated as deliveries are attempted
        };

        // Execute delivery
        self.execute_reward_delivery(&distribution)?;
        
        // Store distribution history
        self.distribution_history.push(distribution.clone());
        
        Ok(distribution)
    }

    /// Calculate base rewards for a player
    fn calculate_match_rewards(&self, player_id: PlayerId, match_result: &MatchResult) -> Result<Vec<RewardItem>> {
        let mut rewards = vec![];

        // Determine if player won
        let won_match = matches!(match_result.result, crate::MatchOutcome::Victory);
        
        // Apply victory rules
        if won_match {
            for rule in &self.distribution_rules {
                if rule.enabled && rule.applicable_contexts.contains(&RewardContext::MatchVictory) {
                    let rule_rewards = self.apply_distribution_rule(rule, player_id, match_result)?;
                    rewards.extend(rule_rewards);
                }
            }
        }

        // Apply participation rules
        for rule in &self.distribution_rules {
            if rule.enabled && rule.applicable_contexts.contains(&RewardContext::MatchParticipation) {
                let rule_rewards = self.apply_distribution_rule(rule, player_id, match_result)?;
                rewards.extend(rule_rewards);
            }
        }

        Ok(rewards)
    }

    /// Apply a specific distribution rule
    fn apply_distribution_rule(&self, rule: &DistributionRule, 
                              _player_id: PlayerId, match_result: &MatchResult) -> Result<Vec<RewardItem>> {
        let mut rewards = vec![];

        // Check conditions
        for condition in &rule.conditions {
            if !self.check_condition(condition, match_result) {
                return Ok(rewards); // Condition not met
            }
        }

        // Calculate base rewards
        for (reward_type, amount) in &rule.base_rewards {
            rewards.push(RewardItem {
                item_type: self.map_string_to_reward_type(reward_type),
                item_identifier: reward_type.clone(),
                quantity: *amount,
                quality_tier: 1,
                bind_type: BindType::None,
                expiration: None,
                metadata: HashMap::new(),
            });
        }

        Ok(rewards)
    }

    /// Check if a condition is met
    fn check_condition(&self, condition: &RewardCondition, match_result: &MatchResult) -> bool {
        match condition.condition_type {
            ConditionType::MatchDuration => {
                let duration = match_result.duration_seconds as f64;
                match condition.comparison {
                    ComparisonOperator::GreaterThan => duration > condition.threshold,
                    ComparisonOperator::LessThan => duration < condition.threshold,
                    ComparisonOperator::Equal => (duration - condition.threshold).abs() < 1.0,
                    ComparisonOperator::GreaterOrEqual => duration >= condition.threshold,
                    ComparisonOperator::LessOrEqual => duration <= condition.threshold,
                    ComparisonOperator::NotEqual => (duration - condition.threshold).abs() >= 1.0,
                    ComparisonOperator::Between(min, max) => duration >= min && duration <= max,
                }
            },
            ConditionType::PerformanceScore => {
                // In real implementation, would calculate performance score
                true // Simplified
            },
            _ => true, // Simplified - would implement all condition types
        }
    }

    /// Map string to reward type
    fn map_string_to_reward_type(&self, type_string: &str) -> RewardType {
        match type_string {
            "gold" => RewardType::Currency,
            "experience" => RewardType::Experience,
            "rating_points" => RewardType::Currency,
            "participation_points" => RewardType::Currency,
            _ => RewardType::Item,
        }
    }

    /// Get applicable multipliers for a player
    fn get_applicable_multipliers(&self, _player_id: PlayerId, _match_result: &MatchResult) -> Vec<RewardMultiplier> {
        // In real implementation, would check player state, streaks, etc.
        vec![]
    }

    /// Apply multipliers to rewards
    fn apply_multipliers(&self, base_rewards: &[RewardItem], multipliers: &[RewardMultiplier]) -> Vec<RewardItem> {
        let mut final_rewards = base_rewards.to_vec();

        for multiplier in multipliers {
            match multiplier.stack_behavior {
                StackBehavior::Multiplicative => {
                    for reward in &mut final_rewards {
                        reward.quantity = (reward.quantity as f64 * multiplier.factor) as u64;
                    }
                },
                StackBehavior::Additive => {
                    for reward in &mut final_rewards {
                        let bonus = (reward.quantity as f64 * (multiplier.factor - 1.0)) as u64;
                        reward.quantity += bonus;
                    }
                },
                _ => {
                    // Other stacking behaviors would be implemented here
                }
            }
        }

        final_rewards
    }

    /// Execute reward delivery to players
    fn execute_reward_delivery(&mut self, distribution: &RewardDistribution) -> Result<()> {
        for recipient in &distribution.recipients {
            self.deliver_rewards_to_player(recipient.player_id, &recipient.rewards_received)?;
        }
        Ok(())
    }

    /// Deliver rewards to a specific player
    fn deliver_rewards_to_player(&mut self, player_id: PlayerId, rewards: &[RewardItem]) -> Result<()> {
        let player_rewards = self.player_rewards.entry(player_id)
            .or_insert_with(|| PlayerRewards::new(player_id));

        // Add to pending rewards
        for reward in rewards {
            let pending_reward = PendingReward {
                reward_id: Uuid::new_v4(),
                source: RewardSource::MatchCompletion,
                reward_items: vec![reward.clone()],
                earned_date: Utc::now(),
                expiration_date: Some(Utc::now() + Duration::days(30)),
                claim_requirements: vec![],
                notification_sent: false,
            };

            player_rewards.pending_rewards.push(pending_reward);
        }

        // Update totals
        for reward in rewards {
            *player_rewards.total_earned.entry(reward.item_identifier.clone()).or_insert(0) += reward.quantity;
            *player_rewards.seasonal_earned.entry(reward.item_identifier.clone()).or_insert(0) += reward.quantity;
        }

        tracing::info!("Delivered {} rewards to player {}", rewards.len(), player_id);
        Ok(())
    }

    /// Claim pending rewards for a player
    pub fn claim_rewards(&mut self, player_id: PlayerId, reward_ids: Vec<Uuid>) -> Result<Vec<RewardItem>> {
        let player_rewards = self.player_rewards.get_mut(&player_id)
            .ok_or_else(|| anyhow::anyhow!("Player rewards not found"))?;

        let mut claimed_rewards = vec![];

        // Process each reward claim
        for reward_id in reward_ids {
            if let Some(index) = player_rewards.pending_rewards.iter().position(|r| r.reward_id == reward_id) {
                let pending_reward = player_rewards.pending_rewards.remove(index);
                
                // Check expiration
                if let Some(expiry) = pending_reward.expiration_date {
                    if Utc::now() > expiry {
                        continue; // Expired, skip
                    }
                }

                // Check requirements
                if !pending_reward.claim_requirements.is_empty() {
                    // In real implementation, would validate requirements
                }

                // Create claim record
                let claim = RewardClaim {
                    claim_id: Uuid::new_v4(),
                    reward_id: pending_reward.reward_id,
                    claimed_date: Utc::now(),
                    claimed_items: pending_reward.reward_items.clone(),
                    satisfaction_rating: None,
                };

                player_rewards.claim_history.push(claim);
                claimed_rewards.extend(pending_reward.reward_items);
            }
        }

        Ok(claimed_rewards)
    }

    /// Get player's pending rewards
    pub fn get_pending_rewards(&self, player_id: PlayerId) -> Vec<&PendingReward> {
        self.player_rewards.get(&player_id)
            .map(|rewards| rewards.pending_rewards.iter().collect())
            .unwrap_or_default()
    }

    /// Update seasonal modifiers
    pub fn update_seasonal_modifiers(&mut self, season: Season, events: Vec<SeasonalEvent>) -> Result<()> {
        self.seasonal_modifiers.current_season = season;
        self.seasonal_modifiers.active_events = events;
        
        // Recalculate global multipliers
        self.calculate_global_multipliers();
        
        Ok(())
    }

    /// Calculate global multipliers from active events
    fn calculate_global_multipliers(&mut self) {
        let mut multipliers: HashMap<String, f64> = HashMap::new();

        // Base multiplier
        multipliers.insert("experience".to_string(), self.seasonal_modifiers.current_season.progression_multiplier);

        // Event multipliers
        for event in &self.seasonal_modifiers.active_events {
            for (reward_type, multiplier) in &event.reward_modifiers {
                let current = multipliers.get(reward_type).unwrap_or(&1.0);
                multipliers.insert(reward_type.clone(), current * multiplier);
            }
        }

        self.seasonal_modifiers.global_multipliers = multipliers;
    }

    /// Process season end rewards
    pub fn process_season_end_rewards(&mut self, season_id: String) -> Result<RewardDistribution> {
        let distribution_id = Uuid::new_v4();
        let mut recipients = vec![];
        let mut total_distributed: HashMap<String, u64> = HashMap::new();

        // Get all players with seasonal progress
        for (player_id, player_rewards) in &self.player_rewards {
            if let Some(seasonal_earned) = player_rewards.seasonal_earned.get("rating_points") {
                // Determine season reward tier based on earned rating
                let tier = self.calculate_season_tier(*seasonal_earned);
                
                if let Some(season_reward) = self.seasonal_modifiers.season_rewards.iter()
                    .find(|sr| sr.reward_tier == tier) {
                    
                    let rewards: Vec<RewardItem> = season_reward.rewards.iter()
                        .map(|special_reward| RewardItem {
                            item_type: special_reward.reward_type.clone(),
                            item_identifier: special_reward.item_name.clone(),
                            quantity: special_reward.quantity as u64,
                            quality_tier: match special_reward.rarity {
                                RewardRarity::Common => 1,
                                RewardRarity::Uncommon => 2,
                                RewardRarity::Rare => 3,
                                RewardRarity::Epic => 4,
                                RewardRarity::Legendary => 5,
                                _ => 6,
                            },
                            bind_type: BindType::Account,
                            expiration: None,
                            metadata: [("season".to_string(), season_id.clone())].into(),
                        })
                        .collect();

                    // Update totals
                    for reward in &rewards {
                        *total_distributed.entry(reward.item_identifier.clone()).or_insert(0) += reward.quantity;
                    }

                    recipients.push(RewardRecipient {
                        player_id: *player_id,
                        rewards_received: rewards,
                        multipliers_applied: vec!["season_end".to_string()],
                        delivery_status: DeliveryStatus::Pending,
                        delivery_attempts: 0,
                    });
                }
            }
        }

        let distribution = RewardDistribution {
            distribution_id,
            distribution_type: DistributionType::Broadcast,
            source_event: SourceEvent::SeasonEnd(season_id),
            recipients,
            total_distributed,
            distribution_date: Utc::now(),
            distribution_rules_applied: vec![],
            success_rate: 0.0, // Will be updated
        };

        self.execute_reward_delivery(&distribution)?;
        self.distribution_history.push(distribution.clone());

        Ok(distribution)
    }

    /// Calculate season reward tier
    fn calculate_season_tier(&self, rating_points: u64) -> u32 {
        match rating_points {
            0..=999 => 1,
            1000..=2499 => 2,
            2500..=4999 => 3,
            5000..=9999 => 4,
            _ => 5,
        }
    }

    /// Get player reward statistics
    pub fn get_player_reward_stats(&self, player_id: PlayerId) -> Option<&PlayerRewards> {
        self.player_rewards.get(&player_id)
    }

    /// Update player streak bonuses
    pub fn update_streak_bonuses(&mut self, player_id: PlayerId, won_match: bool) -> Result<()> {
        let player_rewards = self.player_rewards.entry(player_id)
            .or_insert_with(|| PlayerRewards::new(player_id));

        if won_match {
            player_rewards.streak_bonuses.current_win_streak += 1;
            if player_rewards.streak_bonuses.current_win_streak > player_rewards.streak_bonuses.best_win_streak {
                player_rewards.streak_bonuses.best_win_streak = player_rewards.streak_bonuses.current_win_streak;
            }
        } else {
            player_rewards.streak_bonuses.current_win_streak = 0;
        }

        // Update streak multipliers
        let win_streak_multiplier = 1.0 + (player_rewards.streak_bonuses.current_win_streak as f64 * 0.05);
        player_rewards.streak_bonuses.streak_multipliers.insert("win_streak".to_string(), win_streak_multiplier);

        Ok(())
    }
}

impl PlayerRewards {
    /// Create new player rewards tracking
    fn new(player_id: PlayerId) -> Self {
        Self {
            player_id,
            total_earned: HashMap::new(),
            seasonal_earned: HashMap::new(),
            daily_earned: HashMap::new(),
            weekly_earned: HashMap::new(),
            streak_bonuses: StreakBonuses {
                current_win_streak: 0,
                best_win_streak: 0,
                daily_login_streak: 0,
                match_participation_streak: 0,
                streak_multipliers: HashMap::new(),
            },
            pending_rewards: vec![],
            reward_preferences: RewardPreferences {
                auto_claim: false,
                notification_types: vec![NotificationType::InGame],
                preferred_currencies: vec!["gold".to_string(), "experience".to_string()],
                item_categories: vec![ItemCategory::Cosmetics, ItemCategory::Currency],
            },
            claim_history: vec![],
        }
    }
}

impl Default for SeasonalModifiers {
    fn default() -> Self {
        Self {
            current_season: Season {
                season_id: "season_1".to_string(),
                season_name: "Dawn of Champions".to_string(),
                start_date: Utc::now() - Duration::days(30),
                end_date: Utc::now() + Duration::days(60),
                theme: "Medieval Fantasy".to_string(),
                special_rewards: vec!["Legendary Sword Skin".to_string()],
                progression_multiplier: 1.0,
            },
            active_events: vec![],
            global_multipliers: [("experience".to_string(), 1.0)].into(),
            rank_decay_settings: RankDecaySettings {
                enabled: true,
                decay_rate: 2.0,
                inactivity_threshold: Duration::days(7),
                minimum_rank: RankTier::Bronze,
                grace_period: Duration::days(3),
            },
            season_rewards: vec![
                SeasonReward {
                    reward_tier: 1,
                    required_rank: RankTier::Bronze,
                    rewards: vec![
                        SpecialReward {
                            reward_id: Uuid::new_v4(),
                            reward_type: RewardType::Title,
                            item_name: "Season Participant".to_string(),
                            rarity: RewardRarity::Common,
                            quantity: 1,
                            conditions: vec![],
                            time_limited: false,
                        }
                    ],
                    retroactive: true,
                },
            ],
        }
    }
}

impl Default for RewardDistributor {
    fn default() -> Self {
        Self::new()
    }
}