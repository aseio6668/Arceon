/*!
# Customization System

Advanced character customization with appearance, cosmetics, titles,
and personalization options.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

/// Character customization system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationSystem {
    pub customization_categories: HashMap<String, CustomizationCategory>,
    pub available_options: HashMap<Uuid, CustomizationOption>,
    pub character_profiles: HashMap<Uuid, CustomizationProfile>,
    pub unlockable_content: Vec<UnlockableCustomization>,
}

/// Customization category (appearance, cosmetics, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationCategory {
    pub category_id: Uuid,
    pub name: String,
    pub description: String,
    pub category_type: CustomizationType,
    pub unlock_requirements: Vec<String>,
    pub available_options: Vec<Uuid>,
    pub is_premium: bool,
}

/// Types of customization
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CustomizationType {
    Appearance,
    Cosmetic,
    Title,
    Emote,
    VoiceLine,
    Effect,
    Mount,
    Companion,
    Housing,
    Badge,
}

/// Individual customization option
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationOption {
    pub option_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: CustomizationType,
    pub rarity: CustomizationRarity,
    pub unlock_method: UnlockMethod,
    pub cost: Option<CustomizationCost>,
    pub preview_data: PreviewData,
    pub effects: Vec<CustomizationEffect>,
}

/// Rarity levels for customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomizationRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Unique,
}

/// How customization is unlocked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockMethod {
    Default,
    Achievement(String),
    Level(u32),
    Purchase,
    Event,
    Quest,
    Crafting,
    Drop,
    Special,
}

/// Cost for customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationCost {
    pub currency_type: String,
    pub amount: u32,
    pub alternative_costs: Vec<AlternativeCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeCost {
    pub cost_type: String,
    pub amount: u32,
    pub description: String,
}

/// Preview data for customization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewData {
    pub preview_image: Option<String>,
    pub preview_animation: Option<String>,
    pub color_variants: Vec<ColorVariant>,
    pub size_variants: Vec<SizeVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorVariant {
    pub variant_name: String,
    pub primary_color: String,
    pub secondary_color: Option<String>,
    pub accent_color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SizeVariant {
    pub variant_name: String,
    pub scale_factor: f64,
    pub description: String,
}

/// Effects of customization options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationEffect {
    pub effect_type: EffectType,
    pub magnitude: f64,
    pub duration: Option<f64>,
    pub visual_enhancement: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Visual,
    Audio,
    Particle,
    Animation,
    Aura,
    Trail,
    Glow,
    None,
}

/// Character's customization profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationProfile {
    pub character_id: Uuid,
    pub active_customizations: HashMap<CustomizationType, Uuid>,
    pub unlocked_options: Vec<Uuid>,
    pub customization_presets: Vec<CustomizationPreset>,
    pub favorite_options: Vec<Uuid>,
    pub purchase_history: Vec<PurchaseRecord>,
}

/// Saved customization presets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizationPreset {
    pub preset_id: Uuid,
    pub name: String,
    pub description: String,
    pub customizations: HashMap<CustomizationType, Uuid>,
    pub created_date: chrono::DateTime<chrono::Utc>,
    pub usage_count: u32,
}

/// Purchase history tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseRecord {
    pub purchase_id: Uuid,
    pub option_id: Uuid,
    pub purchase_date: chrono::DateTime<chrono::Utc>,
    pub cost_paid: CustomizationCost,
    pub purchase_method: String,
}

/// Unlockable customization content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockableCustomization {
    pub unlock_id: Uuid,
    pub customization_id: Uuid,
    pub unlock_condition: UnlockCondition,
    pub unlock_description: String,
    pub reward_tier: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnlockCondition {
    LevelReached(u32),
    AchievementEarned(String),
    SkillUnlocked(String),
    QuestCompleted(String),
    EventParticipation(String),
    TimePlayedHours(u32),
    CommunityContribution(u32),
    SpecialAction(String),
}

impl CustomizationSystem {
    /// Create new customization system
    pub fn new() -> Self {
        let mut system = Self {
            customization_categories: HashMap::new(),
            available_options: HashMap::new(),
            character_profiles: HashMap::new(),
            unlockable_content: vec![],
        };

        system.initialize_default_customizations();
        system
    }

    /// Initialize default customization options
    fn initialize_default_customizations(&mut self) {
        self.create_appearance_options();
        self.create_title_options();
        self.create_emote_options();
        self.create_effect_options();
    }

    /// Create appearance customization options
    fn create_appearance_options(&mut self) {
        let category = CustomizationCategory {
            category_id: Uuid::new_v4(),
            name: "Appearance".to_string(),
            description: "Character appearance modifications".to_string(),
            category_type: CustomizationType::Appearance,
            unlock_requirements: vec![],
            available_options: vec![],
            is_premium: false,
        };

        // Hair styles
        let hair_style_1 = CustomizationOption {
            option_id: Uuid::new_v4(),
            name: "Classic Style".to_string(),
            description: "Traditional neat appearance".to_string(),
            category: CustomizationType::Appearance,
            rarity: CustomizationRarity::Common,
            unlock_method: UnlockMethod::Default,
            cost: None,
            preview_data: PreviewData {
                preview_image: Some("hair_classic.png".to_string()),
                preview_animation: None,
                color_variants: vec![
                    ColorVariant {
                        variant_name: "Black".to_string(),
                        primary_color: "#000000".to_string(),
                        secondary_color: None,
                        accent_color: None,
                    },
                    ColorVariant {
                        variant_name: "Brown".to_string(),
                        primary_color: "#8B4513".to_string(),
                        secondary_color: None,
                        accent_color: None,
                    },
                ],
                size_variants: vec![],
            },
            effects: vec![],
        };

        self.available_options.insert(hair_style_1.option_id, hair_style_1);
        self.customization_categories.insert("appearance".to_string(), category);
    }

    /// Create title options
    fn create_title_options(&mut self) {
        let category = CustomizationCategory {
            category_id: Uuid::new_v4(),
            name: "Titles".to_string(),
            description: "Earned titles and honors".to_string(),
            category_type: CustomizationType::Title,
            unlock_requirements: vec![],
            available_options: vec![],
            is_premium: false,
        };

        // Basic titles
        let novice_title = CustomizationOption {
            option_id: Uuid::new_v4(),
            name: "Novice".to_string(),
            description: "New to the world of Arceon".to_string(),
            category: CustomizationType::Title,
            rarity: CustomizationRarity::Common,
            unlock_method: UnlockMethod::Default,
            cost: None,
            preview_data: PreviewData {
                preview_image: None,
                preview_animation: None,
                color_variants: vec![],
                size_variants: vec![],
            },
            effects: vec![],
        };

        let warrior_title = CustomizationOption {
            option_id: Uuid::new_v4(),
            name: "Warrior".to_string(),
            description: "Proven in combat".to_string(),
            category: CustomizationType::Title,
            rarity: CustomizationRarity::Uncommon,
            unlock_method: UnlockMethod::Achievement("First Victory".to_string()),
            cost: None,
            preview_data: PreviewData {
                preview_image: None,
                preview_animation: None,
                color_variants: vec![],
                size_variants: vec![],
            },
            effects: vec![
                CustomizationEffect {
                    effect_type: EffectType::Aura,
                    magnitude: 0.1,
                    duration: None,
                    visual_enhancement: true,
                }
            ],
        };

        self.available_options.insert(novice_title.option_id, novice_title);
        self.available_options.insert(warrior_title.option_id, warrior_title);
        self.customization_categories.insert("titles".to_string(), category);
    }

    /// Create emote options
    fn create_emote_options(&mut self) {
        let category = CustomizationCategory {
            category_id: Uuid::new_v4(),
            name: "Emotes".to_string(),
            description: "Character expressions and gestures".to_string(),
            category_type: CustomizationType::Emote,
            unlock_requirements: vec![],
            available_options: vec![],
            is_premium: false,
        };

        let wave_emote = CustomizationOption {
            option_id: Uuid::new_v4(),
            name: "Wave".to_string(),
            description: "Friendly greeting gesture".to_string(),
            category: CustomizationType::Emote,
            rarity: CustomizationRarity::Common,
            unlock_method: UnlockMethod::Default,
            cost: None,
            preview_data: PreviewData {
                preview_image: None,
                preview_animation: Some("wave_animation.gif".to_string()),
                color_variants: vec![],
                size_variants: vec![],
            },
            effects: vec![
                CustomizationEffect {
                    effect_type: EffectType::Animation,
                    magnitude: 1.0,
                    duration: Some(2.0),
                    visual_enhancement: false,
                }
            ],
        };

        self.available_options.insert(wave_emote.option_id, wave_emote);
        self.customization_categories.insert("emotes".to_string(), category);
    }

    /// Create effect options
    fn create_effect_options(&mut self) {
        let category = CustomizationCategory {
            category_id: Uuid::new_v4(),
            name: "Effects".to_string(),
            description: "Visual effects and enhancements".to_string(),
            category_type: CustomizationType::Effect,
            unlock_requirements: vec!["Level 10".to_string()],
            available_options: vec![],
            is_premium: true,
        };

        let fire_aura = CustomizationOption {
            option_id: Uuid::new_v4(),
            name: "Fire Aura".to_string(),
            description: "Surround yourself with flames".to_string(),
            category: CustomizationType::Effect,
            rarity: CustomizationRarity::Rare,
            unlock_method: UnlockMethod::Purchase,
            cost: Some(CustomizationCost {
                currency_type: "premium_coins".to_string(),
                amount: 500,
                alternative_costs: vec![
                    AlternativeCost {
                        cost_type: "achievement_points".to_string(),
                        amount: 1000,
                        description: "Achievement point alternative".to_string(),
                    }
                ],
            }),
            preview_data: PreviewData {
                preview_image: Some("fire_aura_preview.png".to_string()),
                preview_animation: Some("fire_aura_loop.gif".to_string()),
                color_variants: vec![
                    ColorVariant {
                        variant_name: "Red Fire".to_string(),
                        primary_color: "#FF4500".to_string(),
                        secondary_color: Some("#FFD700".to_string()),
                        accent_color: Some("#FF0000".to_string()),
                    },
                    ColorVariant {
                        variant_name: "Blue Fire".to_string(),
                        primary_color: "#1E90FF".to_string(),
                        secondary_color: Some("#00BFFF".to_string()),
                        accent_color: Some("#0000FF".to_string()),
                    },
                ],
                size_variants: vec![
                    SizeVariant {
                        variant_name: "Subtle".to_string(),
                        scale_factor: 0.7,
                        description: "Smaller, more subtle effect".to_string(),
                    },
                    SizeVariant {
                        variant_name: "Intense".to_string(),
                        scale_factor: 1.3,
                        description: "Larger, more dramatic effect".to_string(),
                    },
                ],
            },
            effects: vec![
                CustomizationEffect {
                    effect_type: EffectType::Particle,
                    magnitude: 1.0,
                    duration: None,
                    visual_enhancement: true,
                },
                CustomizationEffect {
                    effect_type: EffectType::Glow,
                    magnitude: 0.8,
                    duration: None,
                    visual_enhancement: true,
                },
            ],
        };

        self.available_options.insert(fire_aura.option_id, fire_aura);
        self.customization_categories.insert("effects".to_string(), category);
    }

    /// Apply customization to character
    pub fn apply_customization(&mut self, character_id: Uuid, customization_type: CustomizationType, option_id: Uuid) -> Result<()> {
        // Get or create character profile
        let profile = self.character_profiles.entry(character_id)
            .or_insert_with(|| CustomizationProfile::new(character_id));

        // Check if option is unlocked
        if !profile.unlocked_options.contains(&option_id) {
            return Err(anyhow::anyhow!("Customization option not unlocked"));
        }

        // Apply the customization
        profile.active_customizations.insert(customization_type, option_id);

        Ok(())
    }

    /// Unlock customization option for character
    pub fn unlock_customization(&mut self, character_id: Uuid, option_id: Uuid) -> Result<()> {
        let option = self.available_options.get(&option_id)
            .ok_or_else(|| anyhow::anyhow!("Customization option not found"))?;

        // Check unlock method requirements (simplified)
        match &option.unlock_method {
            UnlockMethod::Default => {
                // Always available
            },
            UnlockMethod::Level(required_level) => {
                // Would check character level
                if *required_level > 1 {
                    return Err(anyhow::anyhow!("Level requirement not met"));
                }
            },
            UnlockMethod::Purchase => {
                // Would check payment
                return Err(anyhow::anyhow!("Purchase required"));
            },
            _ => {
                // Other requirements would be checked here
            },
        }

        // Add to unlocked options
        let profile = self.character_profiles.entry(character_id)
            .or_insert_with(|| CustomizationProfile::new(character_id));
        
        if !profile.unlocked_options.contains(&option_id) {
            profile.unlocked_options.push(option_id);
        }

        Ok(())
    }

    /// Save customization preset
    pub fn save_preset(&mut self, character_id: Uuid, preset_name: String, description: String) -> Result<Uuid> {
        let profile = self.character_profiles.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character profile not found"))?;

        let preset = CustomizationPreset {
            preset_id: Uuid::new_v4(),
            name: preset_name,
            description,
            customizations: profile.active_customizations.clone(),
            created_date: chrono::Utc::now(),
            usage_count: 0,
        };

        let preset_id = preset.preset_id;
        profile.customization_presets.push(preset);

        Ok(preset_id)
    }

    /// Load customization preset
    pub fn load_preset(&mut self, character_id: Uuid, preset_id: Uuid) -> Result<()> {
        let profile = self.character_profiles.get_mut(&character_id)
            .ok_or_else(|| anyhow::anyhow!("Character profile not found"))?;

        let preset = profile.customization_presets.iter_mut()
            .find(|p| p.preset_id == preset_id)
            .ok_or_else(|| anyhow::anyhow!("Preset not found"))?;

        // Apply all customizations from preset
        profile.active_customizations = preset.customizations.clone();
        preset.usage_count += 1;

        Ok(())
    }

    /// Get available customizations for character
    pub fn get_available_customizations(&self, character_id: Uuid, category: &CustomizationType) -> Vec<&CustomizationOption> {
        if let Some(profile) = self.character_profiles.get(&character_id) {
            self.available_options.values()
                .filter(|option| &option.category == category && profile.unlocked_options.contains(&option.option_id))
                .collect()
        } else {
            // Return default unlocked options
            self.available_options.values()
                .filter(|option| &option.category == category && matches!(option.unlock_method, UnlockMethod::Default))
                .collect()
        }
    }

    /// Get character's current customizations
    pub fn get_character_customizations(&self, character_id: Uuid) -> Option<&HashMap<CustomizationType, Uuid>> {
        self.character_profiles.get(&character_id)
            .map(|profile| &profile.active_customizations)
    }

    /// Check if customization is unlocked
    pub fn is_customization_unlocked(&self, character_id: Uuid, option_id: Uuid) -> bool {
        self.character_profiles.get(&character_id)
            .map(|profile| profile.unlocked_options.contains(&option_id))
            .unwrap_or(false)
    }
}

impl CustomizationProfile {
    /// Create new customization profile
    pub fn new_default() -> Self {
        Self {
            character_id: Uuid::new_v4(),
            active_customizations: HashMap::new(),
            unlocked_options: vec![],
            customization_presets: vec![],
            favorite_options: vec![],
            purchase_history: vec![],
        }
    }

    /// Create new customization profile for character
    pub fn new(character_id: Uuid) -> Self {
        Self {
            character_id,
            active_customizations: HashMap::new(),
            unlocked_options: vec![],
            customization_presets: vec![],
            favorite_options: vec![],
            purchase_history: vec![],
        }
    }
}

impl Default for CustomizationSystem {
    fn default() -> Self {
        Self::new()
    }
}