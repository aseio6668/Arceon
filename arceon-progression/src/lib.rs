/*!
# Arceon Progression System

This module provides advanced character progression mechanics including:
- Skill trees and talent systems
- Experience and leveling mechanics  
- Character customization and builds
- Achievement and milestone tracking
- Dynamic stat progression
- Multi-path character development

The progression system integrates with:
- Combat systems for skill-based gameplay
- Economy for character investment
- World systems for environmental progression
- AI systems for adaptive difficulty
*/

pub mod skill_trees;
pub mod experience_system;
pub mod character_builds;
pub mod achievements;
pub mod talent_system;
pub mod progression_analytics;
pub mod customization_system;
pub mod archetype_system;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;
use arceon_core::entities::being::{SkillType, SkillCategory};

pub use skill_trees::*;
pub use experience_system::*;
pub use character_builds::*;
pub use achievements::*;
pub use talent_system::*;
pub use progression_analytics::*;
pub use customization_system::*;
pub use archetype_system::*;

/// Main progression system coordinator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionSystem {
    pub skill_tree_manager: SkillTreeManager,
    pub experience_manager: ExperienceManager,
    pub achievement_system: AchievementSystem,
    pub talent_system: TalentSystem,
    pub customization_system: CustomizationSystem,
    pub progression_analytics: ProgressionAnalytics,
    pub archetype_system: ArchetypeSystem,
    pub active_progressions: HashMap<Uuid, CharacterProgression>,
}

/// Comprehensive character progression state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterProgression {
    pub character_id: Uuid,
    pub level: u32,
    pub total_experience: u64,
    pub skill_points_available: u32,
    pub talent_points_available: u32,
    pub unlocked_skills: Vec<SkillNodeId>,
    pub talent_selections: HashMap<TalentTreeId, Vec<TalentId>>,
    pub achievements_earned: Vec<AchievementId>,
    pub build_configuration: CharacterBuild,
    pub progression_history: Vec<ProgressionEvent>,
    pub customization_choices: CustomizationProfile,
    pub prestige_level: u32,
    pub mastery_points: HashMap<SkillCategory, u32>,
    pub current_archetypes: Option<PlayerArchetypes>,
}

/// Events in character progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressionEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: ProgressionEventType,
    pub experience_gained: u64,
    pub skills_unlocked: Vec<SkillNodeId>,
    pub achievements_earned: Vec<AchievementId>,
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgressionEventType {
    LevelUp { old_level: u32, new_level: u32 },
    SkillUnlocked { skill_id: SkillNodeId },
    TalentSelected { talent_id: TalentId },
    AchievementEarned { achievement_id: AchievementId },
    MasteryGained { category: SkillCategory, points: u32 },
    PrestigeGained { new_prestige: u32 },
    BuildChanged { old_build: String, new_build: String },
    CustomizationUpdated { category: String },
}

// Type aliases for cleaner code
pub type SkillNodeId = Uuid;
pub type TalentTreeId = Uuid;  
pub type TalentId = Uuid;
pub type AchievementId = Uuid;

impl ProgressionSystem {
    /// Create a new progression system
    pub fn new() -> Self {
        Self {
            skill_tree_manager: SkillTreeManager::new(),
            experience_manager: ExperienceManager::new(),
            achievement_system: AchievementSystem::new(),
            talent_system: TalentSystem::new(),
            customization_system: CustomizationSystem::new(),
            progression_analytics: ProgressionAnalytics::new(),
            archetype_system: ArchetypeSystem::new(),
            active_progressions: HashMap::new(),
        }
    }

    /// Initialize progression for a new character
    pub fn initialize_character_progression(&mut self, character_id: Uuid) -> Result<CharacterProgression> {
        let progression = CharacterProgression {
            character_id,
            level: 1,
            total_experience: 0,
            skill_points_available: 5, // Starting skill points
            talent_points_available: 0,
            unlocked_skills: vec![],
            talent_selections: HashMap::new(),
            achievements_earned: vec![],
            build_configuration: CharacterBuild::new_default(),
            progression_history: vec![],
            customization_choices: CustomizationProfile::new_default(),
            prestige_level: 0,
            mastery_points: HashMap::new(),
            current_archetypes: None,
        };

        self.active_progressions.insert(character_id, progression.clone());
        tracing::info!("Initialized progression for character {}", character_id);
        
        Ok(progression)
    }

    /// Award experience and handle leveling
    pub fn award_experience(&mut self, character_id: Uuid, experience: u64, source: &str) -> Result<Vec<ProgressionEvent>> {
        let progression = self.active_progressions.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character progression not found"))?;

        let old_level = progression.level;
        progression.total_experience += experience;

        let mut events = vec![];

        // Check for level up
        let new_level = self.experience_manager.calculate_level(progression.total_experience);
        if new_level > old_level {
            progression.level = new_level;
            progression.skill_points_available += (new_level - old_level) * 2; // 2 skill points per level
            
            if new_level % 5 == 0 {
                progression.talent_points_available += 1; // Talent point every 5 levels
            }

            events.push(ProgressionEvent {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event_type: ProgressionEventType::LevelUp { old_level, new_level },
                experience_gained: experience,
                skills_unlocked: vec![],
                achievements_earned: vec![],
                context: [("source".to_string(), source.to_string())].into(),
            });

            tracing::info!("Character {} leveled up from {} to {}", character_id, old_level, new_level);
        }

        // Check for achievements
        let new_achievements = self.achievement_system.check_achievements(character_id, progression)?;
        for achievement_id in new_achievements {
            progression.achievements_earned.push(achievement_id);
            events.push(ProgressionEvent {
                event_id: Uuid::new_v4(),
                timestamp: Utc::now(),
                event_type: ProgressionEventType::AchievementEarned { achievement_id },
                experience_gained: 0,
                skills_unlocked: vec![],
                achievements_earned: vec![achievement_id],
                context: HashMap::new(),
            });
        }

        // Update analytics
        self.progression_analytics.record_experience_gain(character_id, experience, source)?;

        Ok(events)
    }

    /// Unlock a skill in the skill tree
    pub fn unlock_skill(&mut self, character_id: Uuid, skill_id: SkillNodeId) -> Result<()> {
        let progression = self.active_progressions.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character progression not found"))?;

        // Validate skill can be unlocked
        if !self.skill_tree_manager.can_unlock_skill(character_id, skill_id, &progression.unlocked_skills)? {
            return Err(anyhow::anyhow!("Skill cannot be unlocked - prerequisites not met"));
        }

        let skill_cost = self.skill_tree_manager.get_skill_cost(skill_id)?;
        if progression.skill_points_available < skill_cost {
            return Err(anyhow::anyhow!("Insufficient skill points"));
        }

        // Unlock the skill
        progression.skill_points_available -= skill_cost;
        progression.unlocked_skills.push(skill_id);

        // Record progression event
        let event = ProgressionEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: ProgressionEventType::SkillUnlocked { skill_id },
            experience_gained: 0,
            skills_unlocked: vec![skill_id],
            achievements_earned: vec![],
            context: HashMap::new(),
        };
        progression.progression_history.push(event);

        tracing::info!("Character {} unlocked skill {}", character_id, skill_id);
        Ok(())
    }

    /// Select a talent from a talent tree
    pub fn select_talent(&mut self, character_id: Uuid, talent_tree_id: TalentTreeId, talent_id: TalentId) -> Result<()> {
        let progression = self.active_progressions.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character progression not found"))?;

        if progression.talent_points_available == 0 {
            return Err(anyhow::anyhow!("No talent points available"));
        }

        // Validate talent selection
        if !self.talent_system.can_select_talent(character_id, talent_tree_id, talent_id, &progression.talent_selections)? {
            return Err(anyhow::anyhow!("Talent cannot be selected"));
        }

        // Select the talent
        progression.talent_points_available -= 1;
        progression.talent_selections
            .entry(talent_tree_id)
            .or_insert_with(Vec::new)
            .push(talent_id);

        // Record progression event
        let event = ProgressionEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: ProgressionEventType::TalentSelected { talent_id },
            experience_gained: 0,
            skills_unlocked: vec![],
            achievements_earned: vec![],
            context: HashMap::new(),
        };
        progression.progression_history.push(event);

        tracing::info!("Character {} selected talent {} from tree {}", character_id, talent_id, talent_tree_id);
        Ok(())
    }

    /// Get character progression summary
    pub fn get_character_progression(&self, character_id: Uuid) -> Option<&CharacterProgression> {
        self.active_progressions.get(&character_id)
    }

    /// Update character build configuration
    pub fn update_character_build(&mut self, character_id: Uuid, new_build: CharacterBuild) -> Result<()> {
        let progression = self.active_progressions.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character progression not found"))?;

        let old_build_name = progression.build_configuration.build_name.clone();
        progression.build_configuration = new_build.clone();

        // Record progression event
        let event = ProgressionEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: ProgressionEventType::BuildChanged { 
                old_build: old_build_name, 
                new_build: new_build.build_name 
            },
            experience_gained: 0,
            skills_unlocked: vec![],
            achievements_earned: vec![],
            context: HashMap::new(),
        };
        progression.progression_history.push(event);

        tracing::info!("Character {} updated build configuration", character_id);
        Ok(())
    }

    /// Calculate character's total power level
    pub fn calculate_power_level(&self, character_id: Uuid) -> Result<f64> {
        let progression = self.active_progressions.get(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character progression not found"))?;

        let base_power = progression.level as f64 * 10.0;
        let skill_power = progression.unlocked_skills.len() as f64 * 5.0;
        let talent_power = progression.talent_selections.values()
            .map(|talents| talents.len() as f64 * 3.0)
            .sum::<f64>();
        let prestige_power = progression.prestige_level as f64 * 50.0;
        let mastery_power = progression.mastery_points.values().sum::<u32>() as f64 * 2.0;

        Ok(base_power + skill_power + talent_power + prestige_power + mastery_power)
    }

    /// Recalculate and update character archetypes based on current skills
    pub fn update_archetypes(&mut self, character_id: Uuid, skills: &HashMap<SkillType, SkillLevel>) -> Result<PlayerArchetypes> {
        let archetypes = self.archetype_system.calculate_archetypes(character_id, skills)?;
        
        // Update progression record
        if let Some(progression) = self.active_progressions.get_mut(&character_id) {
            progression.current_archetypes = Some(archetypes.clone());
        }

        tracing::info!("Updated archetypes for character {}: {} total archetype points", 
                      character_id, archetypes.total_archetype_points);

        Ok(archetypes)
    }

    /// Get character's current archetype levels
    pub fn get_character_archetypes(&self, character_id: Uuid) -> Option<&PlayerArchetypes> {
        self.archetype_system.get_player_archetypes(character_id)
    }

    /// Get archetype bonuses applicable to character
    pub fn get_archetype_bonuses(&self, character_id: Uuid) -> Vec<&ArchetypeBonus> {
        self.archetype_system.get_applicable_bonuses(character_id)
    }

    /// Get archetype progression history
    pub fn get_archetype_history(&self, character_id: Uuid) -> Option<&Vec<ArchetypeSnapshot>> {
        self.archetype_system.get_progression_history(character_id)
    }

    /// Get archetype display information
    pub fn get_archetype_info(&self, archetype_type: &ArchetypeType) -> Option<&ArchetypeDefinition> {
        self.archetype_system.get_archetype_display(archetype_type)
    }

    /// Award experience and update archetypes if skills changed
    pub fn award_experience_with_archetype_update(&mut self, character_id: Uuid, experience: u64, source: &str, skills: &HashMap<SkillType, SkillLevel>) -> Result<(Vec<ProgressionEvent>, PlayerArchetypes)> {
        // First award experience normally
        let progression_events = self.award_experience(character_id, experience, source)?;
        
        // Then update archetypes based on current skills
        let updated_archetypes = self.update_archetypes(character_id, skills)?;

        Ok((progression_events, updated_archetypes))
    }
}

impl Default for ProgressionSystem {
    fn default() -> Self {
        Self::new()
    }
}