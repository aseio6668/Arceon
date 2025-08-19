/*!
# Talent System

Advanced talent trees with specializations, synergies, and dynamic unlocks.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

use crate::{TalentTreeId, TalentId};

/// Talent system management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentSystem {
    pub talent_trees: HashMap<TalentTreeId, TalentTree>,
    pub talents: HashMap<TalentId, Talent>,
    pub talent_categories: Vec<TalentCategory>,
    pub synergy_combinations: Vec<TalentSynergy>,
}

/// Talent tree definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentTree {
    pub tree_id: TalentTreeId,
    pub name: String,
    pub description: String,
    pub category: TalentCategory,
    pub unlock_requirements: Vec<String>,
    pub talent_tiers: Vec<TalentTier>,
    pub tree_bonuses: Vec<TreeMasteryBonus>,
}

/// Talent categories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TalentCategory {
    Combat,
    Magic,
    Crafting,
    Social,
    Leadership,
    Survival,
    Utility,
}

/// Talent tier within a tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentTier {
    pub tier_level: u32,
    pub talents: Vec<TalentId>,
    pub unlock_requirement: u32, // Points needed in previous tiers
}

/// Individual talent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Talent {
    pub talent_id: TalentId,
    pub name: String,
    pub description: String,
    pub tier: u32,
    pub max_rank: u32,
    pub talent_type: TalentType,
    pub effects: Vec<TalentEffect>,
    pub prerequisites: Vec<TalentId>,
    pub conflicts: Vec<TalentId>, // Mutually exclusive talents
}

/// Types of talents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TalentType {
    Passive,
    Active,
    Toggle,
    Threshold,
    Capstone,
}

/// Talent effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentEffect {
    pub effect_name: String,
    pub magnitude_per_rank: f64,
    pub effect_type: TalentEffectType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TalentEffectType {
    StatBonus,
    SkillEnhancement,
    AbilityUnlock,
    ResourceModification,
    BehaviorChange,
}

/// Talent synergies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentSynergy {
    pub synergy_id: Uuid,
    pub name: String,
    pub required_talents: Vec<(TalentId, u32)>, // Talent and required rank
    pub synergy_bonus: String,
}

/// Tree mastery bonuses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeMasteryBonus {
    pub points_required: u32,
    pub bonus_description: String,
    pub bonus_effects: Vec<TalentEffect>,
}

impl TalentSystem {
    /// Create new talent system
    pub fn new() -> Self {
        let mut system = Self {
            talent_trees: HashMap::new(),
            talents: HashMap::new(),
            talent_categories: vec![
                TalentCategory::Combat,
                TalentCategory::Magic,
                TalentCategory::Crafting,
                TalentCategory::Social,
                TalentCategory::Leadership,
                TalentCategory::Survival,
                TalentCategory::Utility,
            ],
            synergy_combinations: vec![],
        };

        system.initialize_default_trees();
        system
    }

    /// Initialize default talent trees
    fn initialize_default_trees(&mut self) {
        self.create_combat_talent_tree();
        self.create_magic_talent_tree();
    }

    /// Create combat talent tree
    fn create_combat_talent_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        
        // Create talents
        let berserker_rage = self.create_talent(
            "Berserker Rage".to_string(),
            "Increased damage when health is low".to_string(),
            1, 5, TalentType::Passive,
            vec![TalentEffect {
                effect_name: "Damage Bonus".to_string(),
                magnitude_per_rank: 5.0,
                effect_type: TalentEffectType::StatBonus,
            }]
        );

        let weapon_master = self.create_talent(
            "Weapon Master".to_string(),
            "Expertise with all weapon types".to_string(),
            2, 3, TalentType::Passive,
            vec![TalentEffect {
                effect_name: "Weapon Proficiency".to_string(),
                magnitude_per_rank: 10.0,
                effect_type: TalentEffectType::SkillEnhancement,
            }]
        );

        let combat_tree = TalentTree {
            tree_id,
            name: "Warrior".to_string(),
            description: "Combat focused talents".to_string(),
            category: TalentCategory::Combat,
            unlock_requirements: vec!["Level 5".to_string()],
            talent_tiers: vec![
                TalentTier {
                    tier_level: 1,
                    talents: vec![berserker_rage],
                    unlock_requirement: 0,
                },
                TalentTier {
                    tier_level: 2,
                    talents: vec![weapon_master],
                    unlock_requirement: 5,
                },
            ],
            tree_bonuses: vec![
                TreeMasteryBonus {
                    points_required: 10,
                    bonus_description: "Combat Mastery".to_string(),
                    bonus_effects: vec![TalentEffect {
                        effect_name: "All Combat Bonuses".to_string(),
                        magnitude_per_rank: 15.0,
                        effect_type: TalentEffectType::StatBonus,
                    }],
                }
            ],
        };

        self.talent_trees.insert(tree_id, combat_tree);
    }

    /// Create magic talent tree
    fn create_magic_talent_tree(&mut self) {
        let tree_id = Uuid::new_v4();
        
        let spell_power = self.create_talent(
            "Spell Power".to_string(),
            "Increased magical damage".to_string(),
            1, 5, TalentType::Passive,
            vec![TalentEffect {
                effect_name: "Magic Damage".to_string(),
                magnitude_per_rank: 8.0,
                effect_type: TalentEffectType::StatBonus,
            }]
        );

        let magic_tree = TalentTree {
            tree_id,
            name: "Arcane Scholar".to_string(),
            description: "Magic focused talents".to_string(),
            category: TalentCategory::Magic,
            unlock_requirements: vec!["Level 5".to_string()],
            talent_tiers: vec![
                TalentTier {
                    tier_level: 1,
                    talents: vec![spell_power],
                    unlock_requirement: 0,
                },
            ],
            tree_bonuses: vec![],
        };

        self.talent_trees.insert(tree_id, magic_tree);
    }

    /// Helper to create talents
    fn create_talent(&mut self, name: String, description: String, tier: u32, max_rank: u32, talent_type: TalentType, effects: Vec<TalentEffect>) -> TalentId {
        let talent_id = Uuid::new_v4();
        let talent = Talent {
            talent_id,
            name,
            description,
            tier,
            max_rank,
            talent_type,
            effects,
            prerequisites: vec![],
            conflicts: vec![],
        };

        self.talents.insert(talent_id, talent);
        talent_id
    }

    /// Check if talent can be selected
    pub fn can_select_talent(&self, _character_id: Uuid, _talent_tree_id: TalentTreeId, talent_id: TalentId, current_selections: &HashMap<TalentTreeId, Vec<TalentId>>) -> Result<bool> {
        let talent = self.talents.get(&talent_id)
            .ok_or_else(|| anyhow::anyhow!("Talent not found"))?;

        // Check prerequisites
        for prereq in &talent.prerequisites {
            let has_prereq = current_selections.values()
                .any(|selections| selections.contains(prereq));
            if !has_prereq {
                return Ok(false);
            }
        }

        // Check conflicts
        for conflict in &talent.conflicts {
            let has_conflict = current_selections.values()
                .any(|selections| selections.contains(conflict));
            if has_conflict {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Get talent details
    pub fn get_talent(&self, talent_id: TalentId) -> Option<&Talent> {
        self.talents.get(&talent_id)
    }

    /// Get all talent trees
    pub fn get_talent_trees(&self) -> &HashMap<TalentTreeId, TalentTree> {
        &self.talent_trees
    }
}

impl Default for TalentSystem {
    fn default() -> Self {
        Self::new()
    }
}