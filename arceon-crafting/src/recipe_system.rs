use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use uuid::Uuid;
use petgraph::{Graph, Direction};
use petgraph::graph::NodeIndex;

/// Advanced recipe management with dynamic unlocking and relationships
pub struct RecipeSystem {
    pub recipe_tree: RecipeTree,
    pub unlock_conditions: HashMap<Uuid, UnlockConditions>,
    pub recipe_variations: HashMap<Uuid, Vec<RecipeVariation>>,
    pub mastery_progression: HashMap<Uuid, MasteryProgression>,
}

/// Complex recipe dependency tree using graph structures
#[derive(Debug, Default)]
pub struct RecipeTree {
    pub dependency_graph: Graph<RecipeNode, RecipeRelationship>,
    pub recipe_nodes: HashMap<Uuid, NodeIndex>,
    pub root_recipes: Vec<Uuid>,
    pub leaf_recipes: Vec<Uuid>,
    pub recipe_chains: Vec<RecipeChain>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeNode {
    pub recipe_id: Uuid,
    pub tier_level: u32,
    pub complexity_score: f64,
    pub prerequisite_count: u32,
    pub unlocks_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeRelationship {
    pub relationship_type: RelationshipType,
    pub strength: f64,
    pub required_mastery: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    DirectPrerequisite,   // Must complete to unlock
    SoftPrerequisite,     // Recommended but not required
    AlternativeUnlock,    // One of several paths to unlock
    SkillGated,          // Requires specific skill level
    CulturalGated,       // Cultural affinity required
    MasteryGated,        // Mastery of prerequisite required
    ChainLink,           // Part of sequential chain
    Synergistic,         // Benefits from combining
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeChain {
    pub chain_id: Uuid,
    pub chain_name: String,
    pub chain_theme: String,
    pub recipes_in_order: Vec<Uuid>,
    pub chain_bonuses: Vec<ChainBonus>,
    pub completion_rewards: ChainRewards,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainBonus {
    pub bonus_type: ChainBonusType,
    pub bonus_value: f64,
    pub applies_from_step: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChainBonusType {
    QualityBonus,
    TimeReduction,
    MaterialEfficiency,
    ExperienceMultiplier,
    UnlockSpeed,
    InnovationChance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainRewards {
    pub completion_title: String,
    pub mastery_points: f64,
    pub exclusive_recipes: Vec<Uuid>,
    pub special_abilities: Vec<String>,
}

/// Conditions that must be met to unlock recipes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockConditions {
    pub recipe_id: Uuid,
    pub skill_requirements: HashMap<String, f64>,
    pub prerequisite_recipes: Vec<PrerequisiteRecipe>,
    pub character_requirements: CharacterRequirements,
    pub social_requirements: SocialRequirements,
    pub environmental_requirements: Vec<EnvironmentalUnlock>,
    pub discovery_requirements: DiscoveryRequirements,
    pub temporal_requirements: Option<TemporalRequirements>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrerequisiteRecipe {
    pub recipe_id: Uuid,
    pub completion_required: bool,
    pub mastery_level_required: Option<f64>,
    pub quality_threshold: Option<f64>,
    pub completion_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterRequirements {
    pub minimum_level: u32,
    pub race_requirements: Vec<String>,
    pub class_requirements: Vec<String>,
    pub reputation_requirements: HashMap<String, f64>,
    pub achievement_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialRequirements {
    pub guild_membership: Option<String>,
    pub mentor_required: bool,
    pub collaboration_count: Option<u32>,
    pub teaching_experience: Option<f64>,
    pub cultural_standing: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalUnlock {
    pub location_type: String,
    pub specific_location: Option<String>,
    pub environmental_conditions: HashMap<String, f64>,
    pub seasonal_restrictions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryRequirements {
    pub discovery_method: String,
    pub research_points_required: f64,
    pub experimentation_attempts: Option<u32>,
    pub knowledge_sources: Vec<String>,
    pub innovation_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalRequirements {
    pub time_of_day: Option<String>,
    pub lunar_phase: Option<String>,
    pub seasonal_timing: Option<String>,
    pub duration_window: Option<u64>,
}

/// Recipe variations discovered through experimentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeVariation {
    pub variation_id: Uuid,
    pub base_recipe_id: Uuid,
    pub variation_name: String,
    pub variation_type: VariationType,
    pub ingredient_changes: Vec<IngredientChange>,
    pub process_changes: Vec<ProcessChange>,
    pub outcome_changes: OutcomeChanges,
    pub discovery_conditions: VariationDiscovery,
    pub rarity_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariationType {
    QualityImprovement,
    QuantityIncrease,
    TimeOptimization,
    MaterialSubstitution,
    ProcessInnovation,
    SerendipitousDiscovery,
    CulturalAdaptation,
    EnvironmentalAdaptation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngredientChange {
    pub change_type: IngredientChangeType,
    pub ingredient_id: Uuid,
    pub new_ingredient_id: Option<Uuid>,
    pub quantity_modifier: f64,
    pub quality_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IngredientChangeType {
    Substitute,
    AddOptional,
    RemoveOptional,
    QuantityAdjust,
    QualityUpgrade,
    PreparationChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessChange {
    pub change_type: ProcessChangeType,
    pub parameter_adjustments: HashMap<String, f64>,
    pub new_steps: Vec<String>,
    pub removed_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessChangeType {
    TemperatureModification,
    TimingAdjustment,
    ToolUpgrade,
    TechniqueRefinement,
    QualityControlEnhancement,
    AutomationIntegration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutcomeChanges {
    pub quality_range_shift: (f64, f64),
    pub quantity_multiplier: f64,
    pub new_properties: Vec<String>,
    pub enhanced_properties: Vec<String>,
    pub experience_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationDiscovery {
    pub discovery_method: String,
    pub required_attempts: u32,
    pub skill_thresholds: HashMap<String, f64>,
    pub environmental_triggers: Vec<String>,
    pub chance_factors: Vec<ChanceFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChanceFactor {
    pub factor_name: String,
    pub base_chance: f64,
    pub modifier_conditions: HashMap<String, f64>,
}

/// Tracks mastery progression for individual recipes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryProgression {
    pub recipe_id: Uuid,
    pub crafter_id: Uuid,
    pub current_mastery_level: f64,
    pub total_attempts: u32,
    pub successful_crafts: u32,
    pub perfect_crafts: u32,
    pub innovation_discoveries: u32,
    pub mastery_milestones: Vec<MasteryMilestone>,
    pub efficiency_improvements: EfficiencyTracker,
    pub quality_consistency: QualityConsistency,
    pub teaching_contributions: TeachingContributions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasteryMilestone {
    pub milestone_name: String,
    pub achieved_at_level: f64,
    pub achievement_date: SystemTime,
    pub benefits_unlocked: Vec<String>,
    pub recognition_earned: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EfficiencyTracker {
    pub time_reduction_percentage: f64,
    pub material_efficiency_bonus: f64,
    pub failure_rate_reduction: f64,
    pub quality_improvement_rate: f64,
    pub innovation_frequency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConsistency {
    pub average_quality: f64,
    pub quality_variance: f64,
    pub minimum_quality_achieved: f64,
    pub maximum_quality_achieved: f64,
    pub quality_trend: QualityTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QualityTrend {
    Improving,
    Stable,
    Declining,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeachingContributions {
    pub students_mentored: u32,
    pub knowledge_shared: f64,
    pub teaching_effectiveness: f64,
    pub cultural_preservation_score: f64,
}

impl RecipeSystem {
    /// Create a new recipe system
    pub fn new() -> Self {
        Self {
            recipe_tree: RecipeTree::default(),
            unlock_conditions: HashMap::new(),
            recipe_variations: HashMap::new(),
            mastery_progression: HashMap::new(),
        }
    }

    /// Build the recipe dependency tree
    pub fn build_dependency_tree(&mut self, recipes: &HashMap<Uuid, super::CraftingRecipe>) -> Result<()> {
        // Add all recipes as nodes
        for (recipe_id, recipe) in recipes {
            let node = RecipeNode {
                recipe_id: *recipe_id,
                tier_level: self.calculate_tier_level(recipe),
                complexity_score: recipe.difficulty_rating,
                prerequisite_count: 0, // Will be calculated
                unlocks_count: 0,      // Will be calculated
            };

            let node_index = self.recipe_tree.dependency_graph.add_node(node);
            self.recipe_tree.recipe_nodes.insert(*recipe_id, node_index);
        }

        // Add relationships based on recipe requirements
        for (recipe_id, recipe) in recipes {
            if let Some(current_node) = self.recipe_tree.recipe_nodes.get(recipe_id) {
                // Add skill-based prerequisites
                for (skill_type, required_level) in &recipe.skill_requirements {
                    // Find recipes that teach this skill at a lower level
                    for (other_recipe_id, other_recipe) in recipes {
                        if other_recipe_id != recipe_id {
                            if let Some(other_skill_level) = other_recipe.skill_requirements.get(skill_type) {
                                if *other_skill_level < *required_level && other_recipe.difficulty_rating < recipe.difficulty_rating {
                                    if let Some(other_node) = self.recipe_tree.recipe_nodes.get(other_recipe_id) {
                                        let relationship = RecipeRelationship {
                                            relationship_type: RelationshipType::SkillGated,
                                            strength: (required_level - other_skill_level) / required_level,
                                            required_mastery: Some(0.5),
                                        };
                                        
                                        self.recipe_tree.dependency_graph.add_edge(*other_node, *current_node, relationship);
                                    }
                                }
                            }
                        }
                    }
                }

                // Add cultural prerequisites
                if let Some(cultural_affinity) = &recipe.cultural_affinity {
                    for (other_recipe_id, other_recipe) in recipes {
                        if other_recipe_id != recipe_id {
                            if let Some(other_affinity) = &other_recipe.cultural_affinity {
                                if other_affinity == cultural_affinity && other_recipe.difficulty_rating < recipe.difficulty_rating {
                                    if let Some(other_node) = self.recipe_tree.recipe_nodes.get(other_recipe_id) {
                                        let relationship = RecipeRelationship {
                                            relationship_type: RelationshipType::CulturalGated,
                                            strength: 0.7,
                                            required_mastery: Some(0.3),
                                        };
                                        
                                        self.recipe_tree.dependency_graph.add_edge(*other_node, *current_node, relationship);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Update prerequisite and unlock counts
        self.update_node_counts();

        // Identify root and leaf recipes
        self.identify_root_and_leaf_recipes();

        // Build recipe chains
        self.build_recipe_chains(recipes)?;

        Ok(())
    }

    /// Calculate the tier level of a recipe based on complexity
    fn calculate_tier_level(&self, recipe: &super::CraftingRecipe) -> u32 {
        let base_tier = (recipe.difficulty_rating * 10.0) as u32;
        let skill_tier = recipe.skill_requirements.values()
            .map(|level| (*level / 25.0) as u32)
            .max()
            .unwrap_or(0);
        let level_tier = recipe.minimum_character_level / 10;

        (base_tier + skill_tier + level_tier).min(10) // Cap at tier 10
    }

    /// Update prerequisite and unlock counts for all nodes
    fn update_node_counts(&mut self) {
        let mut prerequisite_counts = HashMap::new();
        let mut unlock_counts = HashMap::new();

        // Count incoming and outgoing edges for each node
        for node_index in self.recipe_tree.dependency_graph.node_indices() {
            let incoming = self.recipe_tree.dependency_graph
                .neighbors_directed(node_index, Direction::Incoming)
                .count() as u32;
            let outgoing = self.recipe_tree.dependency_graph
                .neighbors_directed(node_index, Direction::Outgoing)
                .count() as u32;

            if let Some(node) = self.recipe_tree.dependency_graph.node_weight(node_index) {
                prerequisite_counts.insert(node.recipe_id, incoming);
                unlock_counts.insert(node.recipe_id, outgoing);
            }
        }

        // Update node weights
        for node_index in self.recipe_tree.dependency_graph.node_indices() {
            if let Some(node) = self.recipe_tree.dependency_graph.node_weight_mut(node_index) {
                node.prerequisite_count = *prerequisite_counts.get(&node.recipe_id).unwrap_or(&0);
                node.unlocks_count = *unlock_counts.get(&node.recipe_id).unwrap_or(&0);
            }
        }
    }

    /// Identify root recipes (no prerequisites) and leaf recipes (unlock nothing)
    fn identify_root_and_leaf_recipes(&mut self) {
        self.recipe_tree.root_recipes.clear();
        self.recipe_tree.leaf_recipes.clear();

        for node_index in self.recipe_tree.dependency_graph.node_indices() {
            if let Some(node) = self.recipe_tree.dependency_graph.node_weight(node_index) {
                if node.prerequisite_count == 0 {
                    self.recipe_tree.root_recipes.push(node.recipe_id);
                }
                if node.unlocks_count == 0 {
                    self.recipe_tree.leaf_recipes.push(node.recipe_id);
                }
            }
        }
    }

    /// Build logical recipe chains for structured progression
    fn build_recipe_chains(&mut self, recipes: &HashMap<Uuid, super::CraftingRecipe>) -> Result<()> {
        // Group recipes by category and create chains
        let mut category_groups: HashMap<String, Vec<Uuid>> = HashMap::new();
        
        for (recipe_id, recipe) in recipes {
            let category_name = format!("{:?}", recipe.category);
            category_groups.entry(category_name).or_insert_with(Vec::new).push(*recipe_id);
        }

        for (category, recipe_ids) in category_groups {
            if recipe_ids.len() >= 3 { // Only create chains for categories with multiple recipes
                let chain = self.create_category_chain(&category, recipe_ids, recipes)?;
                self.recipe_tree.recipe_chains.push(chain);
            }
        }

        Ok(())
    }

    /// Create a recipe chain for a specific category
    fn create_category_chain(&self, category: &str, mut recipe_ids: Vec<Uuid>, recipes: &HashMap<Uuid, super::CraftingRecipe>) -> Result<RecipeChain> {
        // Sort recipes by difficulty within the category
        recipe_ids.sort_by(|a, b| {
            let recipe_a = recipes.get(a).unwrap();
            let recipe_b = recipes.get(b).unwrap();
            recipe_a.difficulty_rating.partial_cmp(&recipe_b.difficulty_rating).unwrap()
        });

        let chain_id = Uuid::new_v4();
        let chain_name = format!("{} Mastery Path", category);
        
        let chain_bonuses = vec![
            ChainBonus {
                bonus_type: ChainBonusType::QualityBonus,
                bonus_value: 0.05,
                applies_from_step: 2,
            },
            ChainBonus {
                bonus_type: ChainBonusType::TimeReduction,
                bonus_value: 0.1,
                applies_from_step: 3,
            },
            ChainBonus {
                bonus_type: ChainBonusType::InnovationChance,
                bonus_value: 0.15,
                applies_from_step: 4,
            },
        ];

        let completion_rewards = ChainRewards {
            completion_title: format!("{} Master", category),
            mastery_points: 100.0,
            exclusive_recipes: vec![], // Would be populated with advanced recipes
            special_abilities: vec![
                format!("Enhanced {} Efficiency", category),
                format!("{} Innovation Mastery", category),
            ],
        };

        Ok(RecipeChain {
            chain_id,
            chain_name,
            chain_theme: category.to_string(),
            recipes_in_order: recipe_ids,
            chain_bonuses,
            completion_rewards,
        })
    }

    /// Check if a recipe can be unlocked for a specific crafter
    pub fn can_unlock_recipe(&self, recipe_id: Uuid, crafter_id: Uuid, crafter_data: &CrafterData) -> bool {
        if let Some(conditions) = self.unlock_conditions.get(&recipe_id) {
            // Check skill requirements
            for (skill_name, required_level) in &conditions.skill_requirements {
                if let Some(crafter_level) = crafter_data.skills.get(skill_name) {
                    if *crafter_level < *required_level {
                        return false;
                    }
                } else {
                    return false; // Skill not found
                }
            }

            // Check character requirements
            if crafter_data.level < conditions.character_requirements.minimum_level {
                return false;
            }

            // Check prerequisite recipes
            for prereq in &conditions.prerequisite_recipes {
                if !self.is_prerequisite_satisfied(prereq, crafter_id, crafter_data) {
                    return false;
                }
            }

            // Check social requirements
            if !self.are_social_requirements_met(&conditions.social_requirements, crafter_data) {
                return false;
            }

            true
        } else {
            true // No specific unlock conditions
        }
    }

    /// Check if a prerequisite recipe requirement is satisfied
    fn is_prerequisite_satisfied(&self, prereq: &PrerequisiteRecipe, crafter_id: Uuid, crafter_data: &CrafterData) -> bool {
        if let Some(mastery) = self.mastery_progression.get(&prereq.recipe_id) {
            if mastery.crafter_id == crafter_id {
                if prereq.completion_required && mastery.successful_crafts == 0 {
                    return false;
                }
                
                if let Some(required_mastery) = prereq.mastery_level_required {
                    if mastery.current_mastery_level < required_mastery {
                        return false;
                    }
                }

                if let Some(required_count) = prereq.completion_count {
                    if mastery.successful_crafts < required_count {
                        return false;
                    }
                }

                true
            } else {
                false // No mastery record for this crafter
            }
        } else {
            false // Recipe never attempted
        }
    }

    /// Check if social requirements are met
    fn are_social_requirements_met(&self, social_reqs: &SocialRequirements, crafter_data: &CrafterData) -> bool {
        if let Some(required_guild) = &social_reqs.guild_membership {
            if crafter_data.guild_membership.as_ref() != Some(required_guild) {
                return false;
            }
        }

        if social_reqs.mentor_required && !crafter_data.has_mentor {
            return false;
        }

        if let Some(required_collaborations) = social_reqs.collaboration_count {
            if crafter_data.collaboration_count < required_collaborations {
                return false;
            }
        }

        true
    }

    /// Get recommended next recipes for a crafter
    pub fn get_recommended_recipes(&self, crafter_id: Uuid, crafter_data: &CrafterData, limit: usize) -> Vec<RecommendedRecipe> {
        let mut recommendations = Vec::new();

        // Find recipes that can be unlocked
        for (recipe_id, _conditions) in &self.unlock_conditions {
            if self.can_unlock_recipe(*recipe_id, crafter_id, crafter_data) {
                // Calculate recommendation score
                let score = self.calculate_recommendation_score(*recipe_id, crafter_data);
                
                recommendations.push(RecommendedRecipe {
                    recipe_id: *recipe_id,
                    recommendation_score: score,
                    reason: self.generate_recommendation_reason(*recipe_id, crafter_data),
                    difficulty_match: self.calculate_difficulty_match(*recipe_id, crafter_data),
                    skill_growth_potential: self.calculate_skill_growth_potential(*recipe_id, crafter_data),
                });
            }
        }

        // Sort by recommendation score and return top results
        recommendations.sort_by(|a, b| b.recommendation_score.partial_cmp(&a.recommendation_score).unwrap());
        recommendations.truncate(limit);

        recommendations
    }

    /// Calculate recommendation score for a recipe
    fn calculate_recommendation_score(&self, recipe_id: Uuid, crafter_data: &CrafterData) -> f64 {
        let mut score = 0.0;

        // Base score from skill alignment
        if let Some(conditions) = self.unlock_conditions.get(&recipe_id) {
            for (skill_name, required_level) in &conditions.skill_requirements {
                if let Some(crafter_level) = crafter_data.skills.get(skill_name) {
                    // Prefer recipes slightly above current skill level
                    let level_diff = required_level - crafter_level;
                    if level_diff > 0.0 && level_diff <= 10.0 {
                        score += 1.0 - (level_diff / 20.0);
                    }
                }
            }
        }

        // Bonus for chain progression
        for chain in &self.recipe_tree.recipe_chains {
            if let Some(position) = chain.recipes_in_order.iter().position(|&id| id == recipe_id) {
                if position > 0 {
                    let prev_recipe = chain.recipes_in_order[position - 1];
                    if let Some(mastery) = self.mastery_progression.get(&prev_recipe) {
                        if mastery.crafter_id == crafter_data.character_id && mastery.current_mastery_level > 0.5 {
                            score += 0.5; // Bonus for following chain progression
                        }
                    }
                }
            }
        }

        score
    }

    /// Generate a human-readable recommendation reason
    fn generate_recommendation_reason(&self, recipe_id: Uuid, crafter_data: &CrafterData) -> String {
        // This would generate contextual reasons based on crafter's progress
        format!("Good skill match and progression opportunity for recipe {}", recipe_id)
    }

    /// Calculate how well the recipe difficulty matches crafter's skill
    fn calculate_difficulty_match(&self, _recipe_id: Uuid, _crafter_data: &CrafterData) -> f64 {
        // Simplified calculation - would be more sophisticated in practice
        0.7
    }

    /// Calculate potential for skill growth from this recipe
    fn calculate_skill_growth_potential(&self, _recipe_id: Uuid, _crafter_data: &CrafterData) -> f64 {
        // Simplified calculation - would analyze skill gaps and growth opportunities
        0.6
    }
}

// Supporting data structures

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrafterData {
    pub character_id: Uuid,
    pub level: u32,
    pub skills: HashMap<String, f64>,
    pub guild_membership: Option<String>,
    pub has_mentor: bool,
    pub collaboration_count: u32,
    pub completed_recipes: Vec<Uuid>,
    pub mastery_levels: HashMap<Uuid, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedRecipe {
    pub recipe_id: Uuid,
    pub recommendation_score: f64,
    pub reason: String,
    pub difficulty_match: f64,
    pub skill_growth_potential: f64,
}