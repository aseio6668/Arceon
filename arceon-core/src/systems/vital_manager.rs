use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::entities::being::{Being, Vital, PassiveEffect};

/// Manages vital systems for all beings - handles regeneration, skill influences, and vital unlocking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalManager {
    pub vital_definitions: HashMap<String, VitalDefinition>,
    pub vital_unlocks: Vec<VitalUnlock>,
    pub regeneration_modifiers: HashMap<Uuid, HashMap<String, f64>>, // being_id -> vital_name -> modifier
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalDefinition {
    pub name: String,
    pub description: String,
    pub is_essential: bool,  // Health, Energy, Mana are essential
    pub base_maximum: f64,
    pub base_regeneration: f64,
    pub color_theme: String, // For UI representation
    pub depletion_effects: Vec<DepletionEffect>,
    pub overflow_behavior: OverflowBehavior,
    pub skill_influences: Vec<String>, // Skills that can affect this vital
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DepletionEffect {
    Death,                    // Health reaching 0 = death
    Unconsciousness,         // Energy reaching 0 = unconscious
    SilenceMagic,           // Mana reaching 0 = can't cast spells
    LoseWrath,              // Wrath drains over time when not in combat
    BerserkEnd,             // Fury ending cancels berserker effects
    Custom(String),         // Custom effect description
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverflowBehavior {
    Cap,                    // Cannot exceed maximum
    Overflow(f64),          // Can exceed up to percentage (e.g., 120%)
    Unlimited,              // No upper limit (not recommended for most vitals)
    Convert(String, f64),   // Convert overflow to another vital (vital_name, ratio)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalUnlock {
    pub vital_name: String,
    pub unlocked_by: Uuid,
    pub unlock_method: UnlockMethod,
    pub unlock_timestamp: DateTime<Utc>,
    pub requirements_met: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockMethod {
    SkillMastery(String, f64),        // Skill name, level required
    CombinedSkills(Vec<(String, f64)>), // Multiple skills needed
    Combat(String),                   // Unlocked through specific combat
    Discovery(String),                // Discovered through exploration
    Ritual(String),                   // Unlocked through magical ritual
    Training(String),                 // Unlocked through focused training
    Evolution(String),                // Natural evolution of being
}

impl VitalManager {
    pub fn new() -> Self {
        let mut manager = Self {
            vital_definitions: HashMap::new(),
            vital_unlocks: Vec::new(),
            regeneration_modifiers: HashMap::new(),
        };

        // Initialize essential vitals
        manager.initialize_essential_vitals();
        
        // Initialize optional vitals
        manager.initialize_optional_vitals();

        manager
    }

    fn initialize_essential_vitals(&mut self) {
        // Health - Essential for all beings
        self.vital_definitions.insert("Health".to_string(), VitalDefinition {
            name: "Health".to_string(),
            description: "Life force - reaching 0 causes death".to_string(),
            is_essential: true,
            base_maximum: 100.0,
            base_regeneration: 1.0,
            color_theme: "#FF0000".to_string(), // Red
            depletion_effects: vec![DepletionEffect::Death],
            overflow_behavior: OverflowBehavior::Overflow(50.0), // Can have up to 150% health
            skill_influences: vec!["Health".to_string(), "Defense".to_string()],
        });

        // Energy - Essential for physical actions
        self.vital_definitions.insert("Energy".to_string(), VitalDefinition {
            name: "Energy".to_string(),
            description: "Physical stamina for actions like running, climbing, fighting".to_string(),
            is_essential: true,
            base_maximum: 100.0,
            base_regeneration: 2.0,
            color_theme: "#00FF00".to_string(), // Green
            depletion_effects: vec![DepletionEffect::Unconsciousness],
            overflow_behavior: OverflowBehavior::Cap, // Cannot exceed 100%
            skill_influences: vec!["Athletics".to_string(), "Endurance".to_string()],
        });

        // Mana - Essential for magical abilities
        self.vital_definitions.insert("Mana".to_string(), VitalDefinition {
            name: "Mana".to_string(),
            description: "Magical energy for casting spells and channeling arcane power".to_string(),
            is_essential: true,
            base_maximum: 50.0,
            base_regeneration: 1.5,
            color_theme: "#0000FF".to_string(), // Blue
            depletion_effects: vec![DepletionEffect::SilenceMagic],
            overflow_behavior: OverflowBehavior::Convert("Energy".to_string(), 0.5), // Overflow mana converts to energy
            skill_influences: vec!["Mana".to_string(), "Arcane".to_string(), "Divine".to_string()],
        });
    }

    fn initialize_optional_vitals(&mut self) {
        // Wrath - Fighter/Warrior vital
        self.vital_definitions.insert("Wrath".to_string(), VitalDefinition {
            name: "Wrath".to_string(),
            description: "Righteous anger that fuels combat techniques".to_string(),
            is_essential: false,
            base_maximum: 100.0,
            base_regeneration: 0.0, // Only builds in combat
            color_theme: "#800000".to_string(), // Dark red
            depletion_effects: vec![DepletionEffect::LoseWrath],
            overflow_behavior: OverflowBehavior::Cap,
            skill_influences: vec!["Wrath".to_string(), "Intimidation".to_string()],
        });

        // Fury - Berserker vital
        self.vital_definitions.insert("Fury".to_string(), VitalDefinition {
            name: "Fury".to_string(),
            description: "Berserker rage that enhances combat at the cost of control".to_string(),
            is_essential: false,
            base_maximum: 100.0,
            base_regeneration: -1.0, // Drains over time
            color_theme: "#FF4500".to_string(), // Orange red
            depletion_effects: vec![DepletionEffect::BerserkEnd],
            overflow_behavior: OverflowBehavior::Unlimited, // Fury can build beyond normal limits
            skill_influences: vec!["Fury".to_string(), "Berserking".to_string()],
        });

        // Spirit - Divine/Spiritual vital
        self.vital_definitions.insert("Spirit".to_string(), VitalDefinition {
            name: "Spirit".to_string(),
            description: "Connection to divine forces and spiritual energy".to_string(),
            is_essential: false,
            base_maximum: 75.0,
            base_regeneration: 0.5,
            color_theme: "#FFD700".to_string(), // Gold
            depletion_effects: vec![DepletionEffect::Custom("Lose divine connection".to_string())],
            overflow_behavior: OverflowBehavior::Overflow(100.0), // Can double
            skill_influences: vec!["Divine".to_string(), "Spirit".to_string(), "Faith".to_string()],
        });

        // Focus - Mental/Concentration vital
        self.vital_definitions.insert("Focus".to_string(), VitalDefinition {
            name: "Focus".to_string(),
            description: "Mental concentration for complex tasks and spellcasting".to_string(),
            is_essential: false,
            base_maximum: 60.0,
            base_regeneration: 1.0,
            color_theme: "#800080".to_string(), // Purple
            depletion_effects: vec![DepletionEffect::Custom("Cannot focus on complex tasks".to_string())],
            overflow_behavior: OverflowBehavior::Cap,
            skill_influences: vec!["Focus".to_string(), "Intellect".to_string(), "Wisdom".to_string()],
        });

        // Chi - Martial arts vital
        self.vital_definitions.insert("Chi".to_string(), VitalDefinition {
            name: "Chi".to_string(),
            description: "Internal energy for martial arts and special techniques".to_string(),
            is_essential: false,
            base_maximum: 80.0,
            base_regeneration: 1.0,
            color_theme: "#008080".to_string(), // Teal
            depletion_effects: vec![DepletionEffect::Custom("Cannot use martial techniques".to_string())],
            overflow_behavior: OverflowBehavior::Cap,
            skill_influences: vec!["Chi".to_string(), "Martial Arts".to_string(), "Meditation".to_string()],
        });

        // Shadow - Rogue/Assassin vital
        self.vital_definitions.insert("Shadow".to_string(), VitalDefinition {
            name: "Shadow".to_string(),
            description: "Ability to blend with shadows and use stealth techniques".to_string(),
            is_essential: false,
            base_maximum: 50.0,
            base_regeneration: 2.0, // Regenerates quickly in darkness
            color_theme: "#2F2F2F".to_string(), // Dark gray
            depletion_effects: vec![DepletionEffect::Custom("Cannot use stealth abilities".to_string())],
            overflow_behavior: OverflowBehavior::Cap,
            skill_influences: vec!["Stealth".to_string(), "Shadow Magic".to_string()],
        });
    }

    /// Update vital regeneration for a being
    pub fn update_vitals(&mut self, being: &mut Being, delta_time: f64) {
        // Update essential vitals - need to avoid borrowing conflicts
        {
            let health_regen = being.vitals.health.regeneration_rate;
            let energy_regen = being.vitals.energy.regeneration_rate;
            let mana_regen = being.vitals.mana.regeneration_rate;
            
            // Apply regeneration directly
            being.vitals.health.current += health_regen * delta_time;
            being.vitals.energy.current += energy_regen * delta_time;
            being.vitals.mana.current += mana_regen * delta_time;
            
            // Cap at maximum
            being.vitals.health.current = being.vitals.health.current.min(being.vitals.health.maximum);
            being.vitals.energy.current = being.vitals.energy.current.min(being.vitals.energy.maximum);
            being.vitals.mana.current = being.vitals.mana.current.min(being.vitals.mana.maximum);
        }

        // Update optional vitals
        let optional_vitals: Vec<String> = being.vitals.optional_vitals.keys().cloned().collect();
        for vital_name in optional_vitals {
            if let Some(vital) = being.vitals.optional_vitals.get_mut(&vital_name) {
                vital.current += vital.regeneration_rate * delta_time;
                vital.current = vital.current.min(vital.maximum).max(0.0);
            }
        }

        // Apply skill influences to vitals
        self.apply_skill_influences(being);
    }

    fn _update_vital(&self, vital: &mut Vital, delta_time: f64, being: &Being) {
        // Calculate regeneration with modifiers
        let base_regen = vital.regeneration_rate;
        let skill_modifier = self._calculate_skill_modifier(vital, being);
        let total_regen = base_regen * skill_modifier;

        // Apply regeneration
        vital.current += total_regen * delta_time;

        // Handle overflow behavior
        if let Some(definition) = self.vital_definitions.get(&vital.name) {
            match &definition.overflow_behavior {
                OverflowBehavior::Cap => {
                    vital.current = vital.current.min(vital.maximum);
                },
                OverflowBehavior::Overflow(percent) => {
                    let max_overflow = vital.maximum * (1.0 + percent / 100.0);
                    vital.current = vital.current.min(max_overflow);
                },
                OverflowBehavior::Unlimited => {
                    // No cap
                },
                OverflowBehavior::Convert(_target_vital, _ratio) => {
                    if vital.current > vital.maximum {
                        let _overflow = vital.current - vital.maximum;
                        vital.current = vital.maximum;
                        // Would need to add overflow to target vital
                        // This requires access to being's other vitals
                    }
                }
            }
        }

        // Ensure vital doesn't go below 0
        vital.current = vital.current.max(0.0);
    }

    /// Calculate skill modifier for vital regeneration
    fn _calculate_skill_modifier(&self, vital: &Vital, being: &Being) -> f64 {
        let mut modifier = 1.0;

        // Check if being has the related skill
        if let Some(skill) = being.skills.skills.get(&vital.skill_influence) {
            // Apply passive effects from skill
            for effect in &skill.passive_trait.effects {
                match effect {
                    PassiveEffect::IncreaseRegeneration(vital_name, multiplier) => {
                        if vital_name == &vital.name {
                            modifier += multiplier * skill.level / 100.0;
                        }
                    },
                    _ => {},
                }
            }
        }

        modifier
    }

    /// Apply skill influences to vital maximums
    fn apply_skill_influences(&self, being: &mut Being) {
        // Reset maximums to base
        being.vitals.health.maximum = being.vitals.health.base_maximum;
        being.vitals.energy.maximum = being.vitals.energy.base_maximum;
        being.vitals.mana.maximum = being.vitals.mana.base_maximum;

        // Apply skill bonuses
        for (_skill_name, skill) in &being.skills.skills {
            for effect in &skill.passive_trait.effects {
                match effect {
                    PassiveEffect::IncreaseMaxHealth(bonus) => {
                        being.vitals.health.maximum += bonus * skill.level;
                    },
                    PassiveEffect::IncreaseMaxMana(bonus) => {
                        being.vitals.mana.maximum += bonus * skill.level;
                    },
                    PassiveEffect::IncreaseMaxEnergy(bonus) => {
                        being.vitals.energy.maximum += bonus * skill.level;
                    },
                    _ => {},
                }
            }
        }

        // Apply to optional vitals
        for (vital_name, vital) in being.vitals.optional_vitals.iter_mut() {
            vital.maximum = vital.base_maximum;
            
            // Apply skill bonuses to optional vitals
            for (_skill_name, skill) in &being.skills.skills {
                // Check if skill influences this vital
                if skill.passive_trait.effects.iter().any(|e| matches!(e, 
                    PassiveEffect::IncreaseRegeneration(vn, _) if vn == vital_name)) {
                    // Apply bonus based on skill level
                    vital.maximum += 5.0 * skill.level; // 5 points per skill level
                }
            }
        }
    }

    /// Attempt to unlock a new vital for a being
    pub fn try_unlock_vital(&mut self, being: &mut Being, vital_name: &str) -> Result<(), String> {
        // Check if vital already unlocked
        if being.vitals.optional_vitals.contains_key(vital_name) {
            return Err("Vital already unlocked".to_string());
        }

        // Check if being can have more optional vitals
        if being.vitals.active_optionals.len() >= 3 {
            return Err("Maximum optional vitals reached (3)".to_string());
        }

        // Check if vital exists
        let vital_def = self.vital_definitions.get(vital_name)
            .ok_or("Vital not found")?;

        // Check unlock requirements
        if !self.check_unlock_requirements(being, vital_name) {
            return Err("Requirements not met".to_string());
        }

        // Create the vital
        let vital = Vital {
            name: vital_name.to_string(),
            current: vital_def.base_maximum,
            maximum: vital_def.base_maximum,
            base_maximum: vital_def.base_maximum,
            regeneration_rate: vital_def.base_regeneration,
            skill_influence: vital_name.to_string(), // Same as vital name by default
            is_essential: false,
            unlock_requirements: Vec::new(),
        };

        // Add to being
        being.vitals.optional_vitals.insert(vital_name.to_string(), vital);
        being.vitals.active_optionals.push(vital_name.to_string());

        // Record unlock
        self.vital_unlocks.push(VitalUnlock {
            vital_name: vital_name.to_string(),
            unlocked_by: being.id,
            unlock_method: UnlockMethod::SkillMastery("Unknown".to_string(), 0.0), // Would determine actual method
            unlock_timestamp: Utc::now(),
            requirements_met: Vec::new(),
        });

        Ok(())
    }

    /// Check if being meets requirements to unlock a vital
    fn check_unlock_requirements(&self, being: &Being, vital_name: &str) -> bool {
        match vital_name {
            "Wrath" => {
                // Requires Defense and Taunt skills
                being.skills.skills.get("Defense").map_or(false, |s| s.level >= 20.0) &&
                being.skills.skills.get("Taunt").map_or(false, |s| s.level >= 15.0)
            },
            "Fury" => {
                // Requires Wrath vital and high Wrath skill
                being.vitals.optional_vitals.contains_key("Wrath") &&
                being.skills.skills.get("Wrath").map_or(false, |s| s.level >= 30.0)
            },
            "Spirit" => {
                // Requires some divine or spiritual skill
                being.skills.skills.iter().any(|(name, skill)| 
                    (name.contains("Divine") || name.contains("Spirit")) && skill.level >= 25.0)
            },
            "Focus" => {
                // Requires intellectual skills
                being.skills.skills.get("Intellect").map_or(false, |s| s.level >= 20.0) ||
                being.skills.skills.get("Mana").map_or(false, |s| s.level >= 30.0)
            },
            "Chi" => {
                // Requires martial or unarmed combat skills
                being.skills.skills.iter().any(|(name, skill)| 
                    (name.contains("Martial") || name.contains("Unarmed")) && skill.level >= 25.0)
            },
            "Shadow" => {
                // Requires stealth skills
                being.skills.skills.get("Stealth").map_or(false, |s| s.level >= 20.0)
            },
            _ => false, // Unknown vitals cannot be unlocked
        }
    }

    /// Get vital status for display/debugging
    pub fn get_vital_status(&self, being: &Being) -> VitalStatus {
        VitalStatus {
            being_id: being.id,
            essential_vitals: vec![
                (being.vitals.health.name.clone(), being.vitals.health.current, being.vitals.health.maximum),
                (being.vitals.energy.name.clone(), being.vitals.energy.current, being.vitals.energy.maximum),
                (being.vitals.mana.name.clone(), being.vitals.mana.current, being.vitals.mana.maximum),
            ],
            optional_vitals: being.vitals.optional_vitals.iter()
                .map(|(name, vital)| (name.clone(), vital.current, vital.maximum))
                .collect(),
            available_vitals: self.vital_definitions.keys()
                .filter(|name| !self.vital_definitions[*name].is_essential)
                .filter(|name| !being.vitals.optional_vitals.contains_key(*name))
                .cloned()
                .collect(),
            can_unlock_more: being.vitals.active_optionals.len() < 3,
        }
    }

    /// Consume vital for skill activation
    pub fn consume_vital(&mut self, being: &mut Being, vital_name: &str, amount: f64) -> Result<(), String> {
        match vital_name {
            "Health" => {
                if being.vitals.health.current >= amount {
                    being.vitals.health.current -= amount;
                    Ok(())
                } else {
                    Err("Insufficient health".to_string())
                }
            },
            "Energy" => {
                if being.vitals.energy.current >= amount {
                    being.vitals.energy.current -= amount;
                    Ok(())
                } else {
                    Err("Insufficient energy".to_string())
                }
            },
            "Mana" => {
                if being.vitals.mana.current >= amount {
                    being.vitals.mana.current -= amount;
                    Ok(())
                } else {
                    Err("Insufficient mana".to_string())
                }
            },
            _ => {
                // Optional vital
                if let Some(vital) = being.vitals.optional_vitals.get_mut(vital_name) {
                    if vital.current >= amount {
                        vital.current -= amount;
                        Ok(())
                    } else {
                        Err(format!("Insufficient {}", vital_name))
                    }
                } else {
                    Err(format!("{} vital not available", vital_name))
                }
            }
        }
    }

    /// Build vital (add amount to current)
    pub fn build_vital(&mut self, being: &mut Being, vital_name: &str, amount: f64) {
        match vital_name {
            "Health" => {
                being.vitals.health.current = (being.vitals.health.current + amount).min(being.vitals.health.maximum);
            },
            "Energy" => {
                being.vitals.energy.current = (being.vitals.energy.current + amount).min(being.vitals.energy.maximum);
            },
            "Mana" => {
                being.vitals.mana.current = (being.vitals.mana.current + amount).min(being.vitals.mana.maximum);
            },
            _ => {
                if let Some(vital) = being.vitals.optional_vitals.get_mut(vital_name) {
                    vital.current = (vital.current + amount).min(vital.maximum);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalStatus {
    pub being_id: Uuid,
    pub essential_vitals: Vec<(String, f64, f64)>, // name, current, maximum
    pub optional_vitals: Vec<(String, f64, f64)>,
    pub available_vitals: Vec<String>, // Vitals that can be unlocked
    pub can_unlock_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::being::{Being, BeingType, Race};

    #[test]
    fn test_vital_regeneration() {
        let mut manager = VitalManager::new();
        let mut being = Being::new("Test".to_string(), BeingType::Npc, Race::Human, false);
        
        // Damage health
        being.vitals.health.current = 50.0;
        
        // Update vitals (1 second)
        manager.update_vitals(&mut being, 1.0);
        
        // Health should regenerate
        assert!(being.vitals.health.current > 50.0);
    }

    #[test]
    fn test_optional_vital_unlock() {
        let mut manager = VitalManager::new();
        let mut being = Being::new("Test".to_string(), BeingType::Npc, Race::Human, false);
        
        // Give being required skills for Wrath
        being.gain_skill_experience("Defense", 1000.0, crate::entities::being::ExperienceSource::CombatVictory);
        being.gain_skill_experience("Taunt", 800.0, crate::entities::being::ExperienceSource::CombatVictory);
        
        // Try to unlock Wrath
        assert!(manager.try_unlock_vital(&mut being, "Wrath").is_ok());
        assert!(being.vitals.optional_vitals.contains_key("Wrath"));
    }

    #[test]
    fn test_vital_consumption() {
        let mut manager = VitalManager::new();
        let mut being = Being::new("Test".to_string(), BeingType::Npc, Race::Human, false);
        
        let initial_energy = being.vitals.energy.current;
        
        // Consume energy
        assert!(manager.consume_vital(&mut being, "Energy", 20.0).is_ok());
        assert_eq!(being.vitals.energy.current, initial_energy - 20.0);
        
        // Try to consume more than available
        assert!(manager.consume_vital(&mut being, "Energy", 1000.0).is_err());
    }
}
