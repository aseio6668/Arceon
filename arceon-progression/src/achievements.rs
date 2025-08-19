/*!
# Achievement System

Comprehensive achievement tracking with categories, progress monitoring,
rewards, and social features.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;

use crate::{CharacterProgression, AchievementId};

/// Achievement system management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementSystem {
    pub achievements: HashMap<AchievementId, Achievement>,
    pub achievement_categories: HashMap<String, Vec<AchievementId>>,
    pub character_progress: HashMap<Uuid, CharacterAchievements>,
    pub global_statistics: AchievementStatistics,
}

/// Individual achievement definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub achievement_id: AchievementId,
    pub name: String,
    pub description: String,
    pub category: AchievementCategory,
    pub difficulty: AchievementDifficulty,
    pub requirements: Vec<AchievementRequirement>,
    pub rewards: Vec<AchievementReward>,
    pub hidden: bool,
    pub points: u32,
    pub unlock_conditions: Vec<String>,
    pub completion_percentage: f64,
}

/// Achievement categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Combat,
    Exploration,
    Crafting,
    Social,
    Economic,
    Progression,
    Seasonal,
    Special,
    Meta,
}

/// Achievement difficulty levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementDifficulty {
    Trivial,
    Easy,
    Medium,
    Hard,
    Extreme,
    Legendary,
}

/// Character's achievement progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterAchievements {
    pub character_id: Uuid,
    pub unlocked_achievements: Vec<AchievementId>,
    pub achievement_progress: HashMap<AchievementId, AchievementProgress>,
    pub total_points: u32,
    pub completion_timestamps: HashMap<AchievementId, DateTime<Utc>>,
}

/// Progress tracking for an achievement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    pub achievement_id: AchievementId,
    pub current_progress: HashMap<String, f64>,
    pub progress_percentage: f64,
    pub milestones_reached: Vec<String>,
}

/// Achievement requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementRequirement {
    pub requirement_type: RequirementType,
    pub target_value: f64,
    pub current_value: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequirementType {
    LevelReached,
    ExperienceGained,
    SkillsUnlocked,
    ItemsCrafted,
    MonstersDefeated,
    AreasExplored,
    QuestsCompleted,
    ResourcesGathered,
    TradesCompleted,
    AchievementsEarned,
    TimeSpent,
    Custom(String),
}

/// Achievement rewards
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementReward {
    pub reward_type: RewardType,
    pub amount: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RewardType {
    Experience,
    SkillPoints,
    TalentPoints,
    Currency,
    Items,
    Titles,
    Cosmetics,
    Abilities,
    AccessUnlocks,
}

/// Global achievement statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementStatistics {
    pub total_achievements: u32,
    pub completion_rates: HashMap<AchievementId, f64>,
    pub average_completion_time: HashMap<AchievementId, f64>,
    pub rarest_achievements: Vec<AchievementId>,
}

impl AchievementSystem {
    /// Create new achievement system
    pub fn new() -> Self {
        let mut system = Self {
            achievements: HashMap::new(),
            achievement_categories: HashMap::new(),
            character_progress: HashMap::new(),
            global_statistics: AchievementStatistics::default(),
        };
        
        system.initialize_default_achievements();
        system
    }

    /// Initialize default achievements
    fn initialize_default_achievements(&mut self) {
        // Combat achievements
        self.add_achievement(Achievement {
            achievement_id: Uuid::new_v4(),
            name: "First Blood".to_string(),
            description: "Defeat your first enemy".to_string(),
            category: AchievementCategory::Combat,
            difficulty: AchievementDifficulty::Trivial,
            requirements: vec![
                AchievementRequirement {
                    requirement_type: RequirementType::MonstersDefeated,
                    target_value: 1.0,
                    current_value: 0.0,
                    description: "Defeat 1 enemy".to_string(),
                }
            ],
            rewards: vec![
                AchievementReward {
                    reward_type: RewardType::Experience,
                    amount: 100,
                    description: "Bonus experience".to_string(),
                }
            ],
            hidden: false,
            points: 5,
            unlock_conditions: vec![],
            completion_percentage: 0.0,
        });

        // Progression achievements
        self.add_achievement(Achievement {
            achievement_id: Uuid::new_v4(),
            name: "Level Up!".to_string(),
            description: "Reach level 10".to_string(),
            category: AchievementCategory::Progression,
            difficulty: AchievementDifficulty::Easy,
            requirements: vec![
                AchievementRequirement {
                    requirement_type: RequirementType::LevelReached,
                    target_value: 10.0,
                    current_value: 0.0,
                    description: "Reach level 10".to_string(),
                }
            ],
            rewards: vec![
                AchievementReward {
                    reward_type: RewardType::SkillPoints,
                    amount: 3,
                    description: "Bonus skill points".to_string(),
                }
            ],
            hidden: false,
            points: 10,
            unlock_conditions: vec![],
            completion_percentage: 0.0,
        });
    }

    /// Add new achievement
    pub fn add_achievement(&mut self, achievement: Achievement) {
        let category_name = format!("{:?}", achievement.category);
        let achievement_id = achievement.achievement_id;
        
        self.achievements.insert(achievement_id, achievement);
        self.achievement_categories
            .entry(category_name)
            .or_insert_with(Vec::new)
            .push(achievement_id);
    }

    /// Check achievements for a character
    pub fn check_achievements(&mut self, character_id: Uuid, progression: &CharacterProgression) -> Result<Vec<AchievementId>> {
        let mut newly_earned = vec![];
        
        // Create achievements to check (avoiding multiple borrows)
        let achievements_to_check: Vec<(AchievementId, Achievement)> = {
            // Get or create character achievement progress
            let character_achievements = self.character_progress
                .entry(character_id)
                .or_insert_with(|| CharacterAchievements::new(character_id));

            // Collect achievements that need checking
            self.achievements.iter()
                .filter(|(achievement_id, _)| !character_achievements.unlocked_achievements.contains(achievement_id))
                .map(|(id, achievement)| (*id, achievement.clone()))
                .collect()
        };

        // Check each achievement
        for (achievement_id, achievement) in achievements_to_check {
            let character_achievements = self.character_progress.get(&character_id).unwrap();
            
            // Check requirements  
            if self.check_achievement_requirements(&achievement, progression, character_achievements)? {
                let character_achievements = self.character_progress.get_mut(&character_id).unwrap();
                character_achievements.unlocked_achievements.push(achievement_id);
                character_achievements.completion_timestamps.insert(achievement_id, Utc::now());
                character_achievements.total_points += achievement.points;
                newly_earned.push(achievement_id);
            }
        }

        Ok(newly_earned)
    }

    /// Check if achievement requirements are met
    fn check_achievement_requirements(
        &self, 
        achievement: &Achievement, 
        progression: &CharacterProgression,
        _character_achievements: &CharacterAchievements
    ) -> Result<bool> {
        for requirement in &achievement.requirements {
            let current_value = match requirement.requirement_type {
                RequirementType::LevelReached => progression.level as f64,
                RequirementType::ExperienceGained => progression.total_experience as f64,
                RequirementType::SkillsUnlocked => progression.unlocked_skills.len() as f64,
                RequirementType::AchievementsEarned => progression.achievements_earned.len() as f64,
                _ => 0.0, // Simplified for other types
            };

            if current_value < requirement.target_value {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get character achievements
    pub fn get_character_achievements(&self, character_id: Uuid) -> Option<&CharacterAchievements> {
        self.character_progress.get(&character_id)
    }

    /// Get achievement by id
    pub fn get_achievement(&self, achievement_id: AchievementId) -> Option<&Achievement> {
        self.achievements.get(&achievement_id)
    }
}

impl CharacterAchievements {
    /// Create new character achievement tracking
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            unlocked_achievements: vec![],
            achievement_progress: HashMap::new(),
            total_points: 0,
            completion_timestamps: HashMap::new(),
        }
    }
}

impl Default for AchievementStatistics {
    fn default() -> Self {
        Self {
            total_achievements: 0,
            completion_rates: HashMap::new(),
            average_completion_time: HashMap::new(),
            rarest_achievements: vec![],
        }
    }
}

impl Default for AchievementSystem {
    fn default() -> Self {
        Self::new()
    }
}