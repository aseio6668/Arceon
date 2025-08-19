/*!
# Quest Generation System

Dynamic quest creation based on player behavior, world state,
and narrative requirements.
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use rand::{Rng, thread_rng};
use anyhow::Result;

use crate::{
    Quest, QuestId, PlayerId, NPCId, LocationId, QuestType, QuestObjective,
    ObjectiveType, ObjectiveTarget, QuestRewards, QuestRequirements, NarrativeContext,
    QuestStatus, QuestMetadata, QuestCreator, QuestDifficulty, NarrativeMood,
    GenerationContext, QuestTrigger
};

/// Dynamic quest generation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestGenerator {
    pub active_quests: HashMap<QuestId, Quest>,
    pub player_quests: HashMap<PlayerId, Vec<QuestId>>,
    pub quest_templates: Vec<QuestTemplate>,
    pub generation_rules: GenerationRules,
    pub player_profiles: HashMap<PlayerId, PlayerProfile>,
    pub world_state: WorldState,
}

/// Template for generating similar quests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub quest_type: QuestType,
    pub objective_patterns: Vec<ObjectivePattern>,
    pub reward_formulas: RewardFormulas,
    pub requirement_rules: RequirementRules,
    pub narrative_themes: Vec<String>,
    pub generation_weight: f64,
    pub conditions: Vec<GenerationCondition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectivePattern {
    pub pattern_type: ObjectiveType,
    pub target_type: TargetType,
    pub progress_range: (u32, u32),
    pub variations: Vec<String>,
    pub difficulty_scaling: DifficultyScaling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TargetType {
    Dynamic,        // Generated based on context
    Fixed(String),  // Specific target
    Category(String), // Type of target (e.g., "undead", "goblin")
    Random,         // Completely random
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyScaling {
    pub base_value: u32,
    pub level_multiplier: f64,
    pub variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardFormulas {
    pub experience_base: u64,
    pub experience_multiplier: f64,
    pub gold_base: u64,
    pub gold_multiplier: f64,
    pub item_chance: f64,
    pub item_quality_weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequirementRules {
    pub level_requirements: Option<LevelRequirement>,
    pub prerequisite_patterns: Vec<String>,
    pub skill_requirements: HashMap<String, u32>,
    pub item_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelRequirement {
    pub minimum: u32,
    pub optimal: u32,
    pub scaling_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenerationCondition {
    PlayerLevel(u32, u32),
    Location(LocationId),
    Time(TimeCondition),
    WorldState(String),
    PlayerChoice(String),
    QuestCompletion(QuestId),
    Random(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeCondition {
    Hour(u32, u32),
    DayOfWeek(u32),
    Season(String),
    Event(String),
}

/// Rules governing quest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRules {
    pub max_active_per_player: u32,
    pub daily_generation_limit: u32,
    pub difficulty_curve: DifficultyCurve,
    pub variety_requirements: VarietyRequirements,
    pub narrative_consistency: NarrativeConsistency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyCurve {
    pub base_difficulty: f64,
    pub level_scaling: f64,
    pub challenge_increase_rate: f64,
    pub player_skill_factor: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarietyRequirements {
    pub min_quest_types: u32,
    pub max_same_type_ratio: f64,
    pub objective_diversity: f64,
    pub location_spread: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeConsistency {
    pub maintain_mood: bool,
    pub character_continuity: bool,
    pub theme_coherence: f64,
    pub branching_factor: u32,
}

/// Player behavior and preference profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub player_id: PlayerId,
    pub level: u32,
    pub preferred_quest_types: HashMap<QuestType, f64>,
    pub completion_rate: f64,
    pub average_duration: u32,
    pub skill_focuses: Vec<String>,
    pub narrative_choices: HashMap<String, String>,
    pub play_style: PlayStyle,
    pub activity_patterns: ActivityPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayStyle {
    Casual,
    Explorer,
    Achiever,
    Socializer,
    Competitor,
    Storyteller,
    Optimizer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityPatterns {
    pub preferred_play_times: Vec<u32>, // Hours of day
    pub session_length: u32, // Average minutes
    pub quest_completion_speed: f64,
    pub exploration_tendency: f64,
    pub social_interaction_level: f64,
}

/// Current world state affecting quest generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub current_events: Vec<WorldEvent>,
    pub faction_standings: HashMap<String, i32>,
    pub resource_availability: HashMap<String, f64>,
    pub npc_states: HashMap<NPCId, NPCState>,
    pub location_conditions: HashMap<LocationId, LocationCondition>,
    pub seasonal_modifiers: SeasonalModifiers,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub event_id: Uuid,
    pub name: String,
    pub event_type: String,
    pub active: bool,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub affected_regions: Vec<String>,
    pub quest_modifiers: Vec<QuestModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestModifier {
    pub modifier_type: ModifierType,
    pub value: f64,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModifierType {
    ExperienceMultiplier,
    GoldMultiplier,
    DifficultyIncrease,
    SpawnRate,
    RequirementChange,
    RewardBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPCState {
    pub npc_id: NPCId,
    pub mood: String,
    pub availability: bool,
    pub relationship_levels: HashMap<PlayerId, i32>,
    pub current_quests: Vec<QuestId>,
    pub dialogue_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationCondition {
    pub location_id: LocationId,
    pub accessibility: f64,
    pub danger_level: u32,
    pub active_events: Vec<String>,
    pub resource_richness: HashMap<String, f64>,
    pub population_density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonalModifiers {
    pub current_season: String,
    pub weather_effects: HashMap<String, f64>,
    pub holiday_events: Vec<String>,
    pub migration_patterns: HashMap<String, String>,
    pub resource_cycles: HashMap<String, f64>,
}

impl QuestGenerator {
    /// Create new quest generator
    pub fn new() -> Self {
        let mut generator = Self {
            active_quests: HashMap::new(),
            player_quests: HashMap::new(),
            quest_templates: vec![],
            generation_rules: GenerationRules::default(),
            player_profiles: HashMap::new(),
            world_state: WorldState::default(),
        };
        
        generator.initialize_default_templates();
        generator
    }

    /// Initialize default quest templates
    fn initialize_default_templates(&mut self) {
        // Kill Quest Template
        let kill_template = QuestTemplate {
            template_id: Uuid::new_v4(),
            name: "Monster Slaying".to_string(),
            quest_type: QuestType::Combat,
            objective_patterns: vec![
                ObjectivePattern {
                    pattern_type: ObjectiveType::Kill,
                    target_type: TargetType::Category("monsters".to_string()),
                    progress_range: (5, 25),
                    variations: vec![
                        "Eliminate the {} threat".to_string(),
                        "Hunt down dangerous {}".to_string(),
                        "Clear the area of {}".to_string(),
                    ],
                    difficulty_scaling: DifficultyScaling {
                        base_value: 10,
                        level_multiplier: 1.2,
                        variance: 0.3,
                    },
                },
            ],
            reward_formulas: RewardFormulas {
                experience_base: 100,
                experience_multiplier: 1.5,
                gold_base: 50,
                gold_multiplier: 1.2,
                item_chance: 0.3,
                item_quality_weights: [
                    ("common".to_string(), 0.6),
                    ("uncommon".to_string(), 0.3),
                    ("rare".to_string(), 0.1),
                ].iter().cloned().collect(),
            },
            requirement_rules: RequirementRules {
                level_requirements: Some(LevelRequirement {
                    minimum: 1,
                    optimal: 5,
                    scaling_factor: 1.0,
                }),
                prerequisite_patterns: vec![],
                skill_requirements: HashMap::new(),
                item_requirements: vec![],
            },
            narrative_themes: vec!["heroism".to_string(), "protection".to_string()],
            generation_weight: 1.0,
            conditions: vec![],
        };

        // Collection Quest Template
        let collection_template = QuestTemplate {
            template_id: Uuid::new_v4(),
            name: "Resource Gathering".to_string(),
            quest_type: QuestType::Crafting,
            objective_patterns: vec![
                ObjectivePattern {
                    pattern_type: ObjectiveType::Collect,
                    target_type: TargetType::Category("resources".to_string()),
                    progress_range: (10, 50),
                    variations: vec![
                        "Gather {} for the village".to_string(),
                        "Collect precious {}".to_string(),
                        "Find rare {}".to_string(),
                    ],
                    difficulty_scaling: DifficultyScaling {
                        base_value: 15,
                        level_multiplier: 1.1,
                        variance: 0.2,
                    },
                },
            ],
            reward_formulas: RewardFormulas {
                experience_base: 80,
                experience_multiplier: 1.3,
                gold_base: 75,
                gold_multiplier: 1.4,
                item_chance: 0.4,
                item_quality_weights: [
                    ("common".to_string(), 0.5),
                    ("uncommon".to_string(), 0.4),
                    ("rare".to_string(), 0.1),
                ].iter().cloned().collect(),
            },
            requirement_rules: RequirementRules {
                level_requirements: Some(LevelRequirement {
                    minimum: 1,
                    optimal: 3,
                    scaling_factor: 0.8,
                }),
                prerequisite_patterns: vec![],
                skill_requirements: HashMap::new(),
                item_requirements: vec![],
            },
            narrative_themes: vec!["exploration".to_string(), "resourcefulness".to_string()],
            generation_weight: 0.8,
            conditions: vec![],
        };

        // Delivery Quest Template
        let delivery_template = QuestTemplate {
            template_id: Uuid::new_v4(),
            name: "Message Delivery".to_string(),
            quest_type: QuestType::Social,
            objective_patterns: vec![
                ObjectivePattern {
                    pattern_type: ObjectiveType::Deliver,
                    target_type: TargetType::Dynamic,
                    progress_range: (1, 3),
                    variations: vec![
                        "Deliver this {} to {}".to_string(),
                        "Take this message to {}".to_string(),
                        "Bring {} to the {}".to_string(),
                    ],
                    difficulty_scaling: DifficultyScaling {
                        base_value: 1,
                        level_multiplier: 1.0,
                        variance: 0.1,
                    },
                },
            ],
            reward_formulas: RewardFormulas {
                experience_base: 60,
                experience_multiplier: 1.1,
                gold_base: 100,
                gold_multiplier: 1.3,
                item_chance: 0.2,
                item_quality_weights: [
                    ("common".to_string(), 0.8),
                    ("uncommon".to_string(), 0.2),
                ].iter().cloned().collect(),
            },
            requirement_rules: RequirementRules {
                level_requirements: Some(LevelRequirement {
                    minimum: 1,
                    optimal: 2,
                    scaling_factor: 0.5,
                }),
                prerequisite_patterns: vec![],
                skill_requirements: HashMap::new(),
                item_requirements: vec![],
            },
            narrative_themes: vec!["communication".to_string(), "trust".to_string()],
            generation_weight: 0.6,
            conditions: vec![],
        };

        self.quest_templates.extend(vec![kill_template, collection_template, delivery_template]);
    }

    /// Generate a new quest for a player
    pub fn generate_quest(&mut self, player_id: PlayerId, context: GenerationContext) -> Result<Quest> {
        // Get or create player profile
        let profile = self.get_or_create_profile(player_id);
        
        // Select appropriate template
        let template = self.select_template(&context, &profile)?;
        
        // Generate quest from template
        let quest = self.create_quest_from_template(template, player_id, &context, &profile)?;
        
        // Store quest
        self.active_quests.insert(quest.quest_id, quest.clone());
        self.player_quests.entry(player_id).or_default().push(quest.quest_id);
        
        tracing::info!("Generated quest '{}' for player {}", quest.title, player_id);
        Ok(quest)
    }

    /// Select appropriate template based on context and player profile
    fn select_template(&self, context: &GenerationContext, profile: &PlayerProfile) -> Result<&QuestTemplate> {
        let mut suitable_templates: Vec<&QuestTemplate> = self.quest_templates.iter()
            .filter(|template| self.template_matches_context(template, context))
            .filter(|template| self.template_suitable_for_player(template, profile))
            .collect();

        if suitable_templates.is_empty() {
            return Err(anyhow::anyhow!("No suitable quest templates found"));
        }

        // Weight templates by player preferences and generation rules
        suitable_templates.sort_by(|a, b| {
            let a_weight = self.calculate_template_weight(a, profile);
            let b_weight = self.calculate_template_weight(b, profile);
            b_weight.partial_cmp(&a_weight).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(suitable_templates[0])
    }

    /// Check if template matches generation context
    fn template_matches_context(&self, template: &QuestTemplate, context: &GenerationContext) -> bool {
        // Check generation conditions
        for condition in &template.conditions {
            if !self.evaluate_condition(condition, context) {
                return false;
            }
        }

        // Check trigger compatibility
        match &context.trigger {
            QuestTrigger::PlayerInteraction(_) => true,
            QuestTrigger::LocationDiscovery(_) => matches!(template.quest_type, QuestType::Discovery | QuestType::Side),
            QuestTrigger::ItemFound(_) => matches!(template.quest_type, QuestType::Crafting | QuestType::Side),
            QuestTrigger::LevelUp => true,
            QuestTrigger::TimeEvent => matches!(template.quest_type, QuestType::Daily | QuestType::Weekly),
            QuestTrigger::WorldEvent(_) => matches!(template.quest_type, QuestType::Event),
            QuestTrigger::NarrativeBranch(_) => matches!(template.quest_type, QuestType::Main | QuestType::Chain),
            _ => true,
        }
    }

    /// Check if template is suitable for player
    fn template_suitable_for_player(&self, template: &QuestTemplate, profile: &PlayerProfile) -> bool {
        // Check level requirements
        if let Some(level_req) = &template.requirement_rules.level_requirements {
            if profile.level < level_req.minimum {
                return false;
            }
        }

        // Check if player has too many quests of this type
        let type_ratio = profile.preferred_quest_types.get(&template.quest_type).unwrap_or(&0.0);
        if *type_ratio > self.generation_rules.variety_requirements.max_same_type_ratio {
            return false;
        }

        true
    }

    /// Calculate template weight for selection
    fn calculate_template_weight(&self, template: &QuestTemplate, profile: &PlayerProfile) -> f64 {
        let mut weight = template.generation_weight;

        // Factor in player preferences
        if let Some(preference) = profile.preferred_quest_types.get(&template.quest_type) {
            weight *= 1.0 + preference;
        }

        // Factor in play style
        weight *= match (&profile.play_style, &template.quest_type) {
            (PlayStyle::Explorer, QuestType::Discovery) => 2.0,
            (PlayStyle::Achiever, QuestType::Combat) => 1.8,
            (PlayStyle::Socializer, QuestType::Social) => 1.9,
            (PlayStyle::Storyteller, QuestType::Main) => 2.1,
            (PlayStyle::Casual, QuestType::Daily) => 1.5,
            _ => 1.0,
        };

        weight
    }

    /// Evaluate generation condition
    fn evaluate_condition(&self, condition: &GenerationCondition, context: &GenerationContext) -> bool {
        match condition {
            GenerationCondition::PlayerLevel(min, max) => {
                if let Some(level) = context.player_level {
                    level >= *min && level <= *max
                } else {
                    true // Can't evaluate, assume true
                }
            },
            GenerationCondition::Location(location_id) => {
                context.location == Some(*location_id)
            },
            GenerationCondition::Random(chance) => {
                thread_rng().gen::<f64>() < *chance
            },
            _ => true, // Other conditions not implemented yet
        }
    }

    /// Create quest from template
    fn create_quest_from_template(
        &self,
        template: &QuestTemplate,
        _player_id: PlayerId,
        context: &GenerationContext,
        profile: &PlayerProfile,
    ) -> Result<Quest> {
        let quest_id = Uuid::new_v4();
        
        // Generate objectives
        let objectives = self.generate_objectives(template, profile)?;
        
        // Generate rewards
        let rewards = self.generate_rewards(template, profile)?;
        
        // Generate requirements
        let requirements = self.generate_requirements(template, profile)?;
        
        // Generate narrative context
        let narrative_context = self.generate_narrative_context(template, context)?;
        
        // Generate title and description
        let (title, description) = self.generate_quest_text(template, &objectives)?;

        Ok(Quest {
            quest_id,
            title,
            description,
            quest_type: template.quest_type.clone(),
            objectives,
            rewards,
            requirements,
            narrative_context,
            status: QuestStatus::Available,
            metadata: QuestMetadata {
                created_at: Utc::now(),
                creator: QuestCreator::System,
                difficulty: self.calculate_difficulty(template, profile),
                estimated_duration: self.estimate_duration(template, profile),
                tags: template.narrative_themes.clone(),
                region: None,
                seasonal: false,
                repeatable: matches!(template.quest_type, QuestType::Daily | QuestType::Weekly),
            },
        })
    }

    /// Generate quest objectives from template
    fn generate_objectives(&self, template: &QuestTemplate, profile: &PlayerProfile) -> Result<Vec<QuestObjective>> {
        let mut objectives = vec![];

        for pattern in &template.objective_patterns {
            let required_progress = self.scale_objective_difficulty(pattern, profile);
            
            let objective = QuestObjective {
                objective_id: Uuid::new_v4(),
                description: self.generate_objective_description(pattern)?,
                objective_type: pattern.pattern_type.clone(),
                target: self.generate_objective_target(&pattern.target_type)?,
                current_progress: 0,
                required_progress,
                optional: false,
                completion_order: None,
            };

            objectives.push(objective);
        }

        Ok(objectives)
    }

    /// Scale objective difficulty based on player profile
    fn scale_objective_difficulty(&self, pattern: &ObjectivePattern, profile: &PlayerProfile) -> u32 {
        let base = pattern.difficulty_scaling.base_value;
        let level_factor = pattern.difficulty_scaling.level_multiplier * profile.level as f64;
        let variance = thread_rng().gen::<f64>() * pattern.difficulty_scaling.variance;
        
        let scaled = base as f64 * level_factor * (1.0 + variance);
        scaled.max(pattern.progress_range.0 as f64).min(pattern.progress_range.1 as f64) as u32
    }

    /// Generate objective description from pattern
    fn generate_objective_description(&self, pattern: &ObjectivePattern) -> Result<String> {
        if pattern.variations.is_empty() {
            return Ok(format!("{:?} objective", pattern.pattern_type));
        }

        let variation = &pattern.variations[thread_rng().gen_range(0..pattern.variations.len())];
        Ok(self.format_objective_text(variation, &pattern.target_type))
    }

    /// Format objective text with target information
    fn format_objective_text(&self, text: &str, target_type: &TargetType) -> String {
        match target_type {
            TargetType::Category(category) => text.replace("{}", category),
            TargetType::Fixed(target) => text.replace("{}", target),
            _ => text.replace("{}", "targets"),
        }
    }

    /// Generate objective target from pattern
    fn generate_objective_target(&self, target_type: &TargetType) -> Result<ObjectiveTarget> {
        match target_type {
            TargetType::Category(category) => Ok(ObjectiveTarget::Monster(category.clone())),
            TargetType::Fixed(_target) => Ok(ObjectiveTarget::Item(Uuid::new_v4())), // Placeholder
            TargetType::Dynamic => Ok(ObjectiveTarget::Location(Uuid::new_v4())), // Placeholder
            TargetType::Random => Ok(ObjectiveTarget::Area("random_area".to_string())),
        }
    }

    /// Generate quest rewards from template
    fn generate_rewards(&self, template: &QuestTemplate, profile: &PlayerProfile) -> Result<QuestRewards> {
        let formulas = &template.reward_formulas;
        
        let experience = (formulas.experience_base as f64 * 
                         formulas.experience_multiplier * 
                         profile.level as f64) as u64;
        
        let gold = (formulas.gold_base as f64 * 
                   formulas.gold_multiplier * 
                   profile.level as f64) as u64;

        let mut items = vec![];
        if thread_rng().gen::<f64>() < formulas.item_chance {
            // Generate random item reward
            items.push(crate::ItemReward {
                item_id: Uuid::new_v4(),
                quantity: 1,
                quality: Some(self.select_item_quality(&formulas.item_quality_weights)),
                bind_type: crate::BindType::OnPickup,
            });
        }

        Ok(QuestRewards {
            experience,
            gold,
            items,
            reputation: HashMap::new(),
            titles: vec![],
            abilities: vec![],
            narrative_unlocks: vec![],
        })
    }

    /// Select item quality based on weights
    fn select_item_quality(&self, weights: &HashMap<String, f64>) -> String {
        let roll = thread_rng().gen::<f64>();
        let mut cumulative = 0.0;

        for (quality, weight) in weights {
            cumulative += weight;
            if roll <= cumulative {
                return quality.clone();
            }
        }

        "common".to_string()
    }

    /// Generate quest requirements from template
    fn generate_requirements(&self, template: &QuestTemplate, _profile: &PlayerProfile) -> Result<QuestRequirements> {
        let level = template.requirement_rules.level_requirements.as_ref()
            .map(|req| req.minimum);

        Ok(QuestRequirements {
            level,
            completed_quests: vec![],
            items: vec![],
            reputation: HashMap::new(),
            skills: template.requirement_rules.skill_requirements.clone(),
            location: None,
            time_restrictions: None,
        })
    }

    /// Generate narrative context for quest
    fn generate_narrative_context(&self, template: &QuestTemplate, context: &GenerationContext) -> Result<NarrativeContext> {
        let story_arc = context.story_context.clone()
            .unwrap_or_else(|| "standalone".to_string());

        let mood = match template.quest_type {
            QuestType::Main => NarrativeMood::Heroic,
            QuestType::Combat => NarrativeMood::Tense,
            QuestType::Discovery => NarrativeMood::Mysterious,
            QuestType::Social => NarrativeMood::Peaceful,
            _ => NarrativeMood::Uplifting,
        };

        Ok(NarrativeContext {
            story_arc,
            mood,
            characters: vec![],
            themes: template.narrative_themes.clone(),
            branching_paths: HashMap::new(),
            player_choices: vec![],
        })
    }

    /// Generate quest title and description
    fn generate_quest_text(&self, template: &QuestTemplate, objectives: &[QuestObjective]) -> Result<(String, String)> {
        let title = match template.quest_type {
            QuestType::Combat => format!("Dangerous Encounter"),
            QuestType::Crafting => format!("Resource Request"),
            QuestType::Social => format!("Important Message"),
            QuestType::Discovery => format!("Unknown Territory"),
            _ => format!("{} Task", template.name),
        };

        let description = if objectives.is_empty() {
            "A simple task awaits completion.".to_string()
        } else {
            format!("{}. Complete the objectives to earn your reward.", 
                   objectives[0].description)
        };

        Ok((title, description))
    }

    /// Calculate quest difficulty
    fn calculate_difficulty(&self, template: &QuestTemplate, profile: &PlayerProfile) -> QuestDifficulty {
        let base_difficulty = match template.quest_type {
            QuestType::Main => 3,
            QuestType::Combat => 4,
            QuestType::Discovery => 2,
            _ => 1,
        };

        let adjusted = base_difficulty + (profile.level / 10) as i32;

        match adjusted {
            0..=1 => QuestDifficulty::Trivial,
            2 => QuestDifficulty::Easy,
            3 => QuestDifficulty::Medium,
            4 => QuestDifficulty::Hard,
            5 => QuestDifficulty::Elite,
            _ => QuestDifficulty::Legendary,
        }
    }

    /// Estimate quest completion duration
    fn estimate_duration(&self, template: &QuestTemplate, profile: &PlayerProfile) -> u32 {
        let base_minutes = match template.quest_type {
            QuestType::Daily => 15,
            QuestType::Main => 60,
            QuestType::Combat => 30,
            QuestType::Crafting => 20,
            QuestType::Social => 10,
            _ => 25,
        };

        // Adjust for player completion speed
        let adjusted = base_minutes as f64 / profile.activity_patterns.quest_completion_speed;
        adjusted as u32
    }

    /// Get or create player profile
    fn get_or_create_profile(&mut self, player_id: PlayerId) -> PlayerProfile {
        self.player_profiles.entry(player_id).or_insert_with(|| {
            PlayerProfile {
                player_id,
                level: 1,
                preferred_quest_types: HashMap::new(),
                completion_rate: 0.8,
                average_duration: 30,
                skill_focuses: vec![],
                narrative_choices: HashMap::new(),
                play_style: PlayStyle::Casual,
                activity_patterns: ActivityPatterns {
                    preferred_play_times: vec![18, 19, 20, 21], // Evening hours
                    session_length: 90,
                    quest_completion_speed: 1.0,
                    exploration_tendency: 0.5,
                    social_interaction_level: 0.6,
                },
            }
        }).clone()
    }

    /// Validate quest acceptance
    pub fn validate_quest_acceptance(&self, player_id: PlayerId, quest_id: QuestId) -> Result<()> {
        let quest = self.active_quests.get(&quest_id)
            .ok_or_else(|| anyhow::anyhow!("Quest not found"))?;

        if !matches!(quest.status, QuestStatus::Available) {
            return Err(anyhow::anyhow!("Quest not available"));
        }

        // Check if player already has this quest
        if let Some(player_quests) = self.player_quests.get(&player_id) {
            if player_quests.contains(&quest_id) {
                return Err(anyhow::anyhow!("Player already has this quest"));
            }
        }

        // Check quest limits
        let active_count = self.get_player_quests(player_id, QuestStatus::Active).len() as u32;
        if active_count >= self.generation_rules.max_active_per_player {
            return Err(anyhow::anyhow!("Player has too many active quests"));
        }

        Ok(())
    }

    /// Update objective progress
    pub fn update_objective_progress(&mut self, quest_id: QuestId, objective_id: Uuid, progress: u32) -> Result<bool> {
        let quest = self.active_quests.get_mut(&quest_id)
            .ok_or_else(|| anyhow::anyhow!("Quest not found"))?;

        let objective = quest.objectives.iter_mut()
            .find(|obj| obj.objective_id == objective_id)
            .ok_or_else(|| anyhow::anyhow!("Objective not found"))?;

        objective.current_progress = progress.min(objective.required_progress);
        Ok(objective.current_progress >= objective.required_progress)
    }

    /// Check if quest is complete
    pub fn is_quest_complete(&self, quest_id: QuestId) -> Result<bool> {
        let quest = self.active_quests.get(&quest_id)
            .ok_or_else(|| anyhow::anyhow!("Quest not found"))?;

        let required_complete = quest.objectives.iter()
            .filter(|obj| !obj.optional)
            .all(|obj| obj.current_progress >= obj.required_progress);

        Ok(required_complete)
    }

    /// Complete quest and return rewards
    pub fn complete_quest(&mut self, quest_id: QuestId) -> Result<QuestRewards> {
        let quest = self.active_quests.get_mut(&quest_id)
            .ok_or_else(|| anyhow::anyhow!("Quest not found"))?;

        quest.status = QuestStatus::Completed;
        Ok(quest.rewards.clone())
    }

    /// Get player quests by status
    pub fn get_player_quests(&self, player_id: PlayerId, status: QuestStatus) -> Vec<&Quest> {
        if let Some(quest_ids) = self.player_quests.get(&player_id) {
            quest_ids.iter()
                .filter_map(|id| self.active_quests.get(id))
                .filter(|quest| quest.status == status)
                .collect()
        } else {
            vec![]
        }
    }

    /// Get available quests for player at location
    pub fn get_available_quests(&self, _player_id: PlayerId, _location_id: Option<LocationId>) -> Result<Vec<Quest>> {
        // For now, return some sample available quests
        // In a real implementation, this would query available quests based on location, NPCs, etc.
        Ok(vec![])
    }
}

impl Default for GenerationRules {
    fn default() -> Self {
        Self {
            max_active_per_player: 10,
            daily_generation_limit: 50,
            difficulty_curve: DifficultyCurve {
                base_difficulty: 1.0,
                level_scaling: 0.1,
                challenge_increase_rate: 0.05,
                player_skill_factor: 0.2,
            },
            variety_requirements: VarietyRequirements {
                min_quest_types: 3,
                max_same_type_ratio: 0.6,
                objective_diversity: 0.7,
                location_spread: 0.5,
            },
            narrative_consistency: NarrativeConsistency {
                maintain_mood: true,
                character_continuity: true,
                theme_coherence: 0.8,
                branching_factor: 3,
            },
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self {
            current_events: vec![],
            faction_standings: HashMap::new(),
            resource_availability: HashMap::new(),
            npc_states: HashMap::new(),
            location_conditions: HashMap::new(),
            seasonal_modifiers: SeasonalModifiers {
                current_season: "spring".to_string(),
                weather_effects: HashMap::new(),
                holiday_events: vec![],
                migration_patterns: HashMap::new(),
                resource_cycles: HashMap::new(),
            },
        }
    }
}

impl Default for QuestGenerator {
    fn default() -> Self {
        Self::new()
    }
}