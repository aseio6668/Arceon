/*!
# Achievement System

Cross-platform achievement synchronization and management.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Achievement information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub unlocked: bool,
    pub unlock_time: Option<chrono::DateTime<chrono::Utc>>,
    pub progress: Option<AchievementProgress>,
    pub category: AchievementCategory,
    pub rarity: AchievementRarity,
    pub platform_specific_data: HashMap<String, serde_json::Value>,
}

/// Achievement progress for progressive achievements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    pub current: u32,
    pub target: u32,
    pub unit: String, // e.g., "kills", "points", "minutes"
}

/// Achievement categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementCategory {
    Combat,
    Exploration,
    Social,
    Crafting,
    Story,
    Collection,
    PvP,
    PvE,
    Skill,
    Miscellaneous,
}

/// Achievement rarity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AchievementRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Secret,
}

impl Achievement {
    /// Create a new achievement
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            icon_url: None,
            unlocked: false,
            unlock_time: None,
            progress: None,
            category: AchievementCategory::Miscellaneous,
            rarity: AchievementRarity::Common,
            platform_specific_data: HashMap::new(),
        }
    }
    
    /// Mark achievement as unlocked
    pub fn unlock(&mut self) {
        self.unlocked = true;
        self.unlock_time = Some(chrono::Utc::now());
        
        // Set progress to complete if it exists
        if let Some(ref mut progress) = self.progress {
            progress.current = progress.target;
        }
    }
    
    /// Update achievement progress
    pub fn update_progress(&mut self, current: u32) -> bool {
        if let Some(ref mut progress) = self.progress {
            progress.current = current.min(progress.target);
            
            // Auto-unlock if target reached
            if progress.current >= progress.target && !self.unlocked {
                self.unlock();
                return true;
            }
        }
        false
    }
    
    /// Get progress percentage (0.0 to 1.0)
    pub fn progress_percentage(&self) -> f32 {
        if let Some(progress) = &self.progress {
            if progress.target > 0 {
                progress.current as f32 / progress.target as f32
            } else {
                1.0
            }
        } else if self.unlocked {
            1.0
        } else {
            0.0
        }
    }
}