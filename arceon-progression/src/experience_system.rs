/*!
# Experience System

Advanced experience and leveling mechanics with multiple experience types,
dynamic scaling, and milestone rewards.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

/// Manages experience gain, levels, and progression curves
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceManager {
    pub experience_curves: HashMap<ExperienceType, ExperienceCurve>,
    pub level_rewards: HashMap<u32, Vec<LevelReward>>,
    pub experience_modifiers: Vec<ExperienceModifier>,
    pub milestone_rewards: HashMap<u32, MilestoneReward>,
    pub prestige_system: PrestigeSystem,
}

/// Different types of experience that can be gained
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum ExperienceType {
    Combat,
    Magic,
    Crafting,
    Social,
    Exploration,
    Trading,
    Leadership,
    Survival,
    General, // Overall character level
}

/// Experience curve configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceCurve {
    pub experience_type: ExperienceType,
    pub base_experience: u64,
    pub scaling_factor: f64,
    pub curve_type: CurveType,
    pub level_cap: Option<u32>,
    pub prestige_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurveType {
    Linear { per_level: u64 },
    Exponential { base: f64, exponent: f64 },
    Logarithmic { base: f64, scale: f64 },
    Polynomial { coefficients: Vec<f64> },
    Custom { formula: String },
}

/// Rewards given when leveling up
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelReward {
    pub reward_id: Uuid,
    pub level: u32,
    pub reward_type: LevelRewardType,
    pub amount: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LevelRewardType {
    SkillPoints,
    TalentPoints,
    AttributePoints,
    HealthIncrease,
    ManaIncrease,
    SpecialAbility,
    ItemReward,
    CurrencyReward,
    CustomizationUnlock,
    Title,
}

/// Modifiers that affect experience gain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceModifier {
    pub modifier_id: Uuid,
    pub name: String,
    pub description: String,
    pub multiplier: f64,
    pub applicable_types: Vec<ExperienceType>,
    pub duration: Option<DateTime<Utc>>,
    pub conditions: Vec<ModifierCondition>,
    pub source: ModifierSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierCondition {
    pub condition_type: String,
    pub target_value: f64,
    pub current_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifierSource {
    Item,
    Buff,
    Achievement,
    GroupBonus,
    EventBonus,
    Premium,
    Guild,
    Environmental,
}

/// Major milestone rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MilestoneReward {
    pub milestone_id: Uuid,
    pub level: u32,
    pub name: String,
    pub description: String,
    pub rewards: Vec<LevelReward>,
    pub permanent_bonuses: Vec<PermanentBonus>,
    pub unlocks: Vec<UnlockType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentBonus {
    pub bonus_type: String,
    pub magnitude: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockType {
    SkillTree,
    TalentTree,
    CustomizationCategory,
    Feature,
    Area,
    Activity,
}

/// Prestige system for advanced progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeSystem {
    pub prestige_levels: HashMap<u32, PrestigeLevel>,
    pub prestige_requirements: Vec<PrestigeRequirement>,
    pub prestige_bonuses: Vec<PrestigeBonus>,
    pub max_prestige_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeLevel {
    pub level: u32,
    pub name: String,
    pub description: String,
    pub requirements: Vec<PrestigeRequirement>,
    pub rewards: Vec<PrestigeReward>,
    pub experience_multiplier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeRequirement {
    pub requirement_type: PrestigeRequirementType,
    pub target_value: u64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrestigeRequirementType {
    MaxLevel,
    TotalExperience,
    SkillsUnlocked,
    AchievementsEarned,
    TimePlayedHours,
    QuestsCompleted,
    MilestonesReached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeReward {
    pub reward_type: PrestigeRewardType,
    pub amount: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrestigeRewardType {
    ExperienceMultiplier,
    SkillPointBonus,
    TalentPointBonus,
    UniqueAbility,
    CosmeticUnlock,
    Title,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeBonus {
    pub bonus_id: Uuid,
    pub name: String,
    pub description: String,
    pub prestige_level_required: u32,
    pub bonus_effects: Vec<PrestigeBonusEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrestigeBonusEffect {
    pub effect_type: String,
    pub magnitude: f64,
    pub stacks_with_levels: bool,
}

/// Experience tracking for a character
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterExperience {
    pub character_id: Uuid,
    pub experience_pools: HashMap<ExperienceType, ExperiencePool>,
    pub active_modifiers: Vec<ExperienceModifier>,
    pub milestones_reached: Vec<u32>,
    pub prestige_level: u32,
    pub total_lifetime_experience: u64,
    pub experience_history: Vec<ExperienceGainEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperiencePool {
    pub experience_type: ExperienceType,
    pub current_experience: u64,
    pub level: u32,
    pub experience_to_next_level: u64,
    pub total_experience_earned: u64,
    pub highest_level_reached: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceGainEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub experience_type: ExperienceType,
    pub base_experience: u64,
    pub modified_experience: u64,
    pub modifiers_applied: Vec<String>,
    pub source: String,
    pub level_before: u32,
    pub level_after: u32,
}

impl ExperienceManager {
    /// Create a new experience manager with default curves
    pub fn new() -> Self {
        let mut manager = Self {
            experience_curves: HashMap::new(),
            level_rewards: HashMap::new(),
            experience_modifiers: vec![],
            milestone_rewards: HashMap::new(),
            prestige_system: PrestigeSystem::new(),
        };

        manager.initialize_default_curves();
        manager.setup_level_rewards();
        manager.setup_milestone_rewards();
        manager
    }

    /// Initialize default experience curves for each type
    fn initialize_default_curves(&mut self) {
        // General leveling curve - exponential with moderate scaling
        self.experience_curves.insert(
            ExperienceType::General,
            ExperienceCurve {
                experience_type: ExperienceType::General,
                base_experience: 100,
                scaling_factor: 1.15,
                curve_type: CurveType::Exponential { base: 1.15, exponent: 1.8 },
                level_cap: Some(100),
                prestige_multiplier: 1.0,
            },
        );

        // Combat experience - slightly faster than general
        self.experience_curves.insert(
            ExperienceType::Combat,
            ExperienceCurve {
                experience_type: ExperienceType::Combat,
                base_experience: 80,
                scaling_factor: 1.12,
                curve_type: CurveType::Exponential { base: 1.12, exponent: 1.6 },
                level_cap: Some(150),
                prestige_multiplier: 1.1,
            },
        );

        // Magic experience - slower but with high prestige bonus
        self.experience_curves.insert(
            ExperienceType::Magic,
            ExperienceCurve {
                experience_type: ExperienceType::Magic,
                base_experience: 120,
                scaling_factor: 1.18,
                curve_type: CurveType::Exponential { base: 1.18, exponent: 2.0 },
                level_cap: Some(120),
                prestige_multiplier: 1.3,
            },
        );

        // Crafting experience - linear progression with cap
        self.experience_curves.insert(
            ExperienceType::Crafting,
            ExperienceCurve {
                experience_type: ExperienceType::Crafting,
                base_experience: 90,
                scaling_factor: 1.0,
                curve_type: CurveType::Linear { per_level: 150 },
                level_cap: Some(200),
                prestige_multiplier: 1.2,
            },
        );

        // Social experience - moderate curve
        self.experience_curves.insert(
            ExperienceType::Social,
            ExperienceCurve {
                experience_type: ExperienceType::Social,
                base_experience: 110,
                scaling_factor: 1.14,
                curve_type: CurveType::Exponential { base: 1.14, exponent: 1.7 },
                level_cap: Some(100),
                prestige_multiplier: 1.15,
            },
        );

        // Exploration experience - logarithmic for diminishing returns
        self.experience_curves.insert(
            ExperienceType::Exploration,
            ExperienceCurve {
                experience_type: ExperienceType::Exploration,
                base_experience: 95,
                scaling_factor: 1.0,
                curve_type: CurveType::Logarithmic { base: 2.0, scale: 200.0 },
                level_cap: Some(75),
                prestige_multiplier: 1.25,
            },
        );
    }

    /// Setup rewards given at various levels
    fn setup_level_rewards(&mut self) {
        // Level 5 rewards
        self.level_rewards.insert(5, vec![
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 5,
                reward_type: LevelRewardType::TalentPoints,
                amount: 1,
                description: "First talent point".to_string(),
            },
        ]);

        // Level 10 rewards
        self.level_rewards.insert(10, vec![
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 10,
                reward_type: LevelRewardType::SkillPoints,
                amount: 3,
                description: "Bonus skill points".to_string(),
            },
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 10,
                reward_type: LevelRewardType::HealthIncrease,
                amount: 50,
                description: "Health bonus".to_string(),
            },
        ]);

        // Level 25 rewards
        self.level_rewards.insert(25, vec![
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 25,
                reward_type: LevelRewardType::SpecialAbility,
                amount: 1,
                description: "Unlock special ability slot".to_string(),
            },
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 25,
                reward_type: LevelRewardType::CustomizationUnlock,
                amount: 1,
                description: "Advanced customization options".to_string(),
            },
        ]);

        // Level 50 rewards
        self.level_rewards.insert(50, vec![
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 50,
                reward_type: LevelRewardType::Title,
                amount: 1,
                description: "Veteran title".to_string(),
            },
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 50,
                reward_type: LevelRewardType::AttributePoints,
                amount: 5,
                description: "Major attribute bonus".to_string(),
            },
        ]);

        // Level 100 rewards (Max level for general)
        self.level_rewards.insert(100, vec![
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 100,
                reward_type: LevelRewardType::Title,
                amount: 1,
                description: "Master title".to_string(),
            },
            LevelReward {
                reward_id: Uuid::new_v4(),
                level: 100,
                reward_type: LevelRewardType::SpecialAbility,
                amount: 1,
                description: "Ultimate ability unlock".to_string(),
            },
        ]);
    }

    /// Setup major milestone rewards
    fn setup_milestone_rewards(&mut self) {
        // Level 25 milestone
        self.milestone_rewards.insert(25, MilestoneReward {
            milestone_id: Uuid::new_v4(),
            level: 25,
            name: "Journeyman".to_string(),
            description: "You have proven yourself as a capable adventurer".to_string(),
            rewards: vec![
                LevelReward {
                    reward_id: Uuid::new_v4(),
                    level: 25,
                    reward_type: LevelRewardType::SkillPoints,
                    amount: 5,
                    description: "Milestone skill points".to_string(),
                },
            ],
            permanent_bonuses: vec![
                PermanentBonus {
                    bonus_type: "experience_rate".to_string(),
                    magnitude: 0.05,
                    description: "5% increased experience gain".to_string(),
                },
            ],
            unlocks: vec![UnlockType::SkillTree],
        });

        // Level 50 milestone
        self.milestone_rewards.insert(50, MilestoneReward {
            milestone_id: Uuid::new_v4(),
            level: 50,
            name: "Veteran".to_string(),
            description: "Your experience and skill are widely recognized".to_string(),
            rewards: vec![
                LevelReward {
                    reward_id: Uuid::new_v4(),
                    level: 50,
                    reward_type: LevelRewardType::TalentPoints,
                    amount: 3,
                    description: "Milestone talent points".to_string(),
                },
            ],
            permanent_bonuses: vec![
                PermanentBonus {
                    bonus_type: "skill_effectiveness".to_string(),
                    magnitude: 0.1,
                    description: "10% increased skill effectiveness".to_string(),
                },
            ],
            unlocks: vec![UnlockType::TalentTree, UnlockType::CustomizationCategory],
        });

        // Level 100 milestone
        self.milestone_rewards.insert(100, MilestoneReward {
            milestone_id: Uuid::new_v4(),
            level: 100,
            name: "Master".to_string(),
            description: "You have achieved true mastery of your chosen path".to_string(),
            rewards: vec![
                LevelReward {
                    reward_id: Uuid::new_v4(),
                    level: 100,
                    reward_type: LevelRewardType::SpecialAbility,
                    amount: 1,
                    description: "Legendary ability".to_string(),
                },
            ],
            permanent_bonuses: vec![
                PermanentBonus {
                    bonus_type: "prestige_eligibility".to_string(),
                    magnitude: 1.0,
                    description: "Eligible for prestige advancement".to_string(),
                },
            ],
            unlocks: vec![UnlockType::Feature], // Unlocks prestige system
        });
    }

    /// Calculate level from total experience
    pub fn calculate_level(&self, total_experience: u64) -> u32 {
        let _curve = self.experience_curves.get(&ExperienceType::General)
            .expect("General experience curve should exist");

        self.calculate_level_for_type(total_experience, &ExperienceType::General)
    }

    /// Calculate level for a specific experience type
    pub fn calculate_level_for_type(&self, total_experience: u64, experience_type: &ExperienceType) -> u32 {
        let curve = self.experience_curves.get(experience_type)
            .expect("Experience curve should exist");

        let mut level = 1;
        let mut experience_required = curve.base_experience;

        while total_experience >= experience_required {
            level += 1;
            
            if let Some(cap) = curve.level_cap {
                if level > cap {
                    return cap;
                }
            }

            experience_required += self.calculate_experience_for_level(level, curve);
        }

        level - 1
    }

    /// Calculate experience required for a specific level
    fn calculate_experience_for_level(&self, level: u32, curve: &ExperienceCurve) -> u64 {
        match &curve.curve_type {
            CurveType::Linear { per_level } => *per_level,
            CurveType::Exponential { base, exponent } => {
                (curve.base_experience as f64 * base.powf(level as f64 * exponent)) as u64
            },
            CurveType::Logarithmic { base, scale } => {
                (scale * base.ln() * (level as f64).ln()) as u64
            },
            CurveType::Polynomial { coefficients } => {
                let mut result = 0.0;
                for (i, coeff) in coefficients.iter().enumerate() {
                    result += coeff * (level as f64).powi(i as i32);
                }
                result.max(curve.base_experience as f64) as u64
            },
            CurveType::Custom { formula: _ } => {
                // For now, fall back to exponential
                (curve.base_experience as f64 * curve.scaling_factor.powf(level as f64)) as u64
            },
        }
    }

    /// Calculate total experience needed to reach a level
    pub fn calculate_total_experience_for_level(&self, target_level: u32, experience_type: &ExperienceType) -> u64 {
        if target_level <= 1 {
            return 0;
        }

        let curve = self.experience_curves.get(experience_type)
            .expect("Experience curve should exist");

        let mut total_experience = 0u64;
        for level in 1..target_level {
            total_experience += self.calculate_experience_for_level(level + 1, curve);
        }

        total_experience
    }

    /// Calculate experience needed for next level
    pub fn calculate_experience_to_next_level(&self, current_level: u32, experience_type: &ExperienceType) -> u64 {
        let curve = self.experience_curves.get(experience_type)
            .expect("Experience curve should exist");

        if let Some(cap) = curve.level_cap {
            if current_level >= cap {
                return 0; // Already at max level
            }
        }

        self.calculate_experience_for_level(current_level + 1, curve)
    }

    /// Apply experience modifiers to base experience
    pub fn apply_experience_modifiers(&self, base_experience: u64, experience_type: &ExperienceType, active_modifiers: &[ExperienceModifier]) -> u64 {
        let mut modified_experience = base_experience as f64;

        for modifier in active_modifiers {
            if modifier.applicable_types.contains(experience_type) || modifier.applicable_types.is_empty() {
                // Check if modifier is still valid (not expired)
                if let Some(expiry) = &modifier.duration {
                    if Utc::now() > *expiry {
                        continue;
                    }
                }

                // Check conditions (simplified for now)
                let conditions_met = modifier.conditions.iter()
                    .all(|condition| self.evaluate_modifier_condition(condition));

                if conditions_met {
                    modified_experience *= modifier.multiplier;
                }
            }
        }

        modified_experience as u64
    }

    /// Evaluate if a modifier condition is met
    fn evaluate_modifier_condition(&self, _condition: &ModifierCondition) -> bool {
        // Simplified implementation - in a real system this would check actual game state
        true
    }

    /// Get rewards for reaching a specific level
    pub fn get_level_rewards(&self, level: u32) -> Vec<&LevelReward> {
        self.level_rewards.get(&level)
            .map(|rewards| rewards.iter().collect())
            .unwrap_or_default()
    }

    /// Get milestone reward for a level (if any)
    pub fn get_milestone_reward(&self, level: u32) -> Option<&MilestoneReward> {
        self.milestone_rewards.get(&level)
    }

    /// Check if a character is eligible for prestige
    pub fn is_eligible_for_prestige(&self, character_experience: &CharacterExperience) -> bool {
        self.prestige_system.check_prestige_eligibility(character_experience)
    }

    /// Get available experience modifiers
    pub fn get_available_modifiers(&self) -> &[ExperienceModifier] {
        &self.experience_modifiers
    }

    /// Add a temporary experience modifier
    pub fn add_experience_modifier(&mut self, modifier: ExperienceModifier) {
        self.experience_modifiers.push(modifier);
    }

    /// Remove expired modifiers
    pub fn cleanup_expired_modifiers(&mut self) {
        let now = Utc::now();
        self.experience_modifiers.retain(|modifier| {
            modifier.duration.map(|expiry| expiry > now).unwrap_or(true)
        });
    }
}

impl PrestigeSystem {
    /// Create a new prestige system
    pub fn new() -> Self {
        let mut system = Self {
            prestige_levels: HashMap::new(),
            prestige_requirements: vec![],
            prestige_bonuses: vec![],
            max_prestige_level: 10,
        };

        system.setup_prestige_levels();
        system
    }

    /// Setup prestige levels and requirements
    fn setup_prestige_levels(&mut self) {
        for prestige_level in 1..=self.max_prestige_level {
            let level = PrestigeLevel {
                level: prestige_level,
                name: format!("Prestige {}", prestige_level),
                description: format!("Advanced mastery level {}", prestige_level),
                requirements: vec![
                    PrestigeRequirement {
                        requirement_type: PrestigeRequirementType::MaxLevel,
                        target_value: 100,
                        description: "Reach maximum level".to_string(),
                    },
                    PrestigeRequirement {
                        requirement_type: PrestigeRequirementType::TotalExperience,
                        target_value: 1_000_000 * prestige_level as u64,
                        description: format!("Earn {} total experience", 1_000_000 * prestige_level),
                    },
                ],
                rewards: vec![
                    PrestigeReward {
                        reward_type: PrestigeRewardType::ExperienceMultiplier,
                        amount: 10, // 10% per prestige level
                        description: "Increased experience gain".to_string(),
                    },
                    PrestigeReward {
                        reward_type: PrestigeRewardType::SkillPointBonus,
                        amount: 5,
                        description: "Bonus skill points".to_string(),
                    },
                ],
                experience_multiplier: 1.0 + (prestige_level as f64 * 0.1),
            };

            self.prestige_levels.insert(prestige_level, level);
        }
    }

    /// Check if character meets prestige requirements
    pub fn check_prestige_eligibility(&self, character_experience: &CharacterExperience) -> bool {
        let next_prestige = character_experience.prestige_level + 1;
        
        if let Some(prestige_level) = self.prestige_levels.get(&next_prestige) {
            prestige_level.requirements.iter().all(|req| {
                self.check_prestige_requirement(req, character_experience)
            })
        } else {
            false // No more prestige levels available
        }
    }

    /// Check if a specific prestige requirement is met
    fn check_prestige_requirement(&self, requirement: &PrestigeRequirement, character_experience: &CharacterExperience) -> bool {
        match requirement.requirement_type {
            PrestigeRequirementType::MaxLevel => {
                if let Some(general_pool) = character_experience.experience_pools.get(&ExperienceType::General) {
                    general_pool.level >= requirement.target_value as u32
                } else {
                    false
                }
            },
            PrestigeRequirementType::TotalExperience => {
                character_experience.total_lifetime_experience >= requirement.target_value
            },
            PrestigeRequirementType::SkillsUnlocked => {
                // Would check against character progression data
                true // Simplified for now
            },
            PrestigeRequirementType::AchievementsEarned => {
                // Would check against achievement system
                true // Simplified for now
            },
            PrestigeRequirementType::TimePlayedHours => {
                // Would check against playtime tracking
                true // Simplified for now
            },
            PrestigeRequirementType::QuestsCompleted => {
                // Would check against quest system
                true // Simplified for now
            },
            PrestigeRequirementType::MilestonesReached => {
                character_experience.milestones_reached.len() >= requirement.target_value as usize
            },
        }
    }

    /// Get prestige level details
    pub fn get_prestige_level(&self, level: u32) -> Option<&PrestigeLevel> {
        self.prestige_levels.get(&level)
    }

    /// Get all prestige bonuses for a character
    pub fn get_prestige_bonuses(&self, prestige_level: u32) -> Vec<&PrestigeBonus> {
        self.prestige_bonuses.iter()
            .filter(|bonus| bonus.prestige_level_required <= prestige_level)
            .collect()
    }
}

impl CharacterExperience {
    /// Create new character experience tracking
    pub fn new(character_id: Uuid) -> Self {
        let mut experience_pools = HashMap::new();
        
        // Initialize all experience types
        for exp_type in [
            ExperienceType::General,
            ExperienceType::Combat,
            ExperienceType::Magic,
            ExperienceType::Crafting,
            ExperienceType::Social,
            ExperienceType::Exploration,
            ExperienceType::Trading,
            ExperienceType::Leadership,
            ExperienceType::Survival,
        ] {
            experience_pools.insert(exp_type.clone(), ExperiencePool {
                experience_type: exp_type,
                current_experience: 0,
                level: 1,
                experience_to_next_level: 100, // Default starting value
                total_experience_earned: 0,
                highest_level_reached: 1,
            });
        }

        Self {
            character_id,
            experience_pools,
            active_modifiers: vec![],
            milestones_reached: vec![],
            prestige_level: 0,
            total_lifetime_experience: 0,
            experience_history: vec![],
        }
    }

    /// Award experience to a specific pool
    pub fn award_experience(&mut self, experience_type: ExperienceType, amount: u64, source: String, modifiers: &[ExperienceModifier]) -> Result<ExperienceGainEvent> {
        // Apply modifiers first (no borrow of self needed)
        let modified_amount = self.apply_modifiers(amount, &experience_type, modifiers);
        
        let pool = self.experience_pools.get_mut(&experience_type)
            .ok_or_else(|| anyhow::anyhow!("Experience pool not found"))?;

        let level_before = pool.level;

        // Add to pools
        pool.current_experience += modified_amount;
        pool.total_experience_earned += modified_amount;
        self.total_lifetime_experience += modified_amount;

        // Check for level up (simplified)
        if pool.current_experience >= pool.experience_to_next_level {
            pool.level += 1;
            pool.current_experience = 0; // Reset for next level
            pool.experience_to_next_level = (pool.experience_to_next_level as f64 * 1.15) as u64; // Increase requirement
            
            if pool.level > pool.highest_level_reached {
                pool.highest_level_reached = pool.level;
            }
        }

        // Create experience gain event
        let event = ExperienceGainEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            experience_type,
            base_experience: amount,
            modified_experience: modified_amount,
            modifiers_applied: modifiers.iter().map(|m| m.name.clone()).collect(),
            source,
            level_before,
            level_after: pool.level,
        };

        self.experience_history.push(event.clone());

        Ok(event)
    }

    /// Apply experience modifiers
    fn apply_modifiers(&self, base_amount: u64, experience_type: &ExperienceType, modifiers: &[ExperienceModifier]) -> u64 {
        let mut modified = base_amount as f64;

        for modifier in modifiers {
            if modifier.applicable_types.contains(experience_type) || modifier.applicable_types.is_empty() {
                modified *= modifier.multiplier;
            }
        }

        // Apply prestige multiplier
        let prestige_multiplier = 1.0 + (self.prestige_level as f64 * 0.1);
        modified *= prestige_multiplier;

        modified as u64
    }

    /// Get experience pool for a type
    pub fn get_experience_pool(&self, experience_type: &ExperienceType) -> Option<&ExperiencePool> {
        self.experience_pools.get(experience_type)
    }

    /// Get total level across all experience types
    pub fn get_total_level(&self) -> u32 {
        self.experience_pools.values()
            .map(|pool| pool.level)
            .sum()
    }

    /// Get highest single level
    pub fn get_highest_level(&self) -> u32 {
        self.experience_pools.values()
            .map(|pool| pool.level)
            .max()
            .unwrap_or(1)
    }
}

impl Default for ExperienceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PrestigeSystem {
    fn default() -> Self {
        Self::new()
    }
}