use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, debug};
use arceon_core::entities::{SkillType, Race};
use crate::neural_network::{NeuralNetworkManager, NetworkType, TrainingData, TrainingContext};

/// Advanced behavioral adaptation system that learns from player interactions
pub struct AdaptiveBehaviorSystem {
    pub behavior_manager: Arc<RwLock<BehaviorManager>>,
    pub interaction_analyzer: Arc<RwLock<InteractionAnalyzer>>,
    pub personality_engine: Arc<RwLock<PersonalityEngine>>,
    pub social_dynamics: Arc<RwLock<SocialDynamicsTracker>>,
    pub neural_network_manager: Arc<NeuralNetworkManager>,
}

/// Manages and evolves NPC behaviors based on learning
#[derive(Debug, Default)]
pub struct BehaviorManager {
    pub npc_behaviors: HashMap<Uuid, AdaptiveBehaviorProfile>,
    pub behavior_templates: HashMap<String, BehaviorTemplate>,
    pub evolution_history: Vec<BehaviorEvolution>,
    pub emergent_patterns: Vec<EmergentPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveBehaviorProfile {
    pub npc_id: Uuid,
    pub base_personality: PersonalityVector,
    pub learned_preferences: HashMap<String, f64>,
    pub behavioral_adaptations: Vec<BehavioralAdaptation>,
    pub interaction_patterns: HashMap<Uuid, InteractionPattern>, // player_id -> pattern
    pub success_metrics: BehaviorSuccessMetrics,
    pub adaptation_speed: f64,
    pub learning_confidence: f64,
    pub last_updated: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityVector {
    pub openness: f64,           // Willingness to try new things
    pub conscientiousness: f64,  // Organization and reliability
    pub extraversion: f64,       // Social energy and assertiveness
    pub agreeableness: f64,      // Cooperation and empathy
    pub neuroticism: f64,        // Emotional stability
    pub curiosity: f64,          // Learning drive
    pub creativity: f64,         // Innovation and artistic expression
    pub leadership: f64,         // Taking initiative and guiding others
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralAdaptation {
    pub adaptation_id: Uuid,
    pub adaptation_type: AdaptationType,
    pub trigger_conditions: Vec<AdaptationTrigger>,
    pub behavioral_changes: Vec<BehaviorChange>,
    pub success_rate: f64,
    pub usage_count: u64,
    pub learned_from: LearningSource,
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdaptationType {
    CommunicationStyle,
    TaskApproach,
    SocialInteraction,
    ResourceManagement,
    ConflictResolution,
    LearningStrategy,
    CreativeExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationTrigger {
    pub trigger_type: TriggerType,
    pub condition_parameters: HashMap<String, f64>,
    pub activation_threshold: f64,
    pub confidence_required: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerType {
    PlayerBehaviorPattern,
    EnvironmentalChange,
    SocialContext,
    TaskComplexity,
    EmotionalState,
    ResourceAvailability,
    TimeOfDay,
    GroupDynamics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorChange {
    pub change_type: ChangeType,
    pub parameter_adjustments: HashMap<String, f64>,
    pub neural_network_updates: Vec<NetworkUpdate>,
    pub priority_shifts: HashMap<String, i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    ResponseTiming,
    CommunicationTone,
    TaskPrioritization,
    SocialDistance,
    RiskTolerance,
    CreativityLevel,
    CooperationWillingness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUpdate {
    pub network_type: NetworkType,
    pub weight_adjustments: Vec<WeightAdjustment>,
    pub learning_rate_modifier: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightAdjustment {
    pub layer_index: usize,
    pub neuron_index: usize,
    pub weight_index: usize,
    pub adjustment_value: f64,
}

/// Analyzes player-NPC interactions for learning opportunities
#[derive(Debug, Default)]
pub struct InteractionAnalyzer {
    pub recent_interactions: Vec<InteractionRecord>,
    pub interaction_patterns: HashMap<Uuid, PlayerInteractionProfile>,
    pub success_predictors: Vec<SuccessPredictor>,
    pub sentiment_analyzer: SentimentAnalyzer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionRecord {
    pub interaction_id: Uuid,
    pub npc_id: Uuid,
    pub player_id: Uuid,
    pub interaction_type: InteractionType,
    pub duration_seconds: u64,
    pub context: InteractionContext,
    pub npc_actions: Vec<NpcActionRecord>,
    pub player_responses: Vec<PlayerResponse>,
    pub outcome_rating: f64, // -1.0 to 1.0
    pub player_satisfaction: Option<f64>,
    pub learning_value: f64,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum InteractionType {
    Conversation,
    Quest,
    Trade,
    Teaching,
    Collaboration,
    Conflict,
    SocialEvent,
    EmergencyResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionContext {
    pub location: String,
    pub time_of_day: String,
    pub weather: String,
    pub other_npcs_present: Vec<Uuid>,
    pub other_players_present: Vec<Uuid>,
    pub recent_events: Vec<String>,
    pub emotional_atmosphere: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcActionRecord {
    pub action_type: String,
    pub parameters: HashMap<String, f64>,
    pub timing_ms: u64,
    pub neural_network_confidence: f64,
    pub player_reaction_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerResponse {
    pub response_type: String,
    pub response_timing_ms: u64,
    pub sentiment_score: f64,
    pub engagement_level: f64,
    pub follow_up_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInteractionProfile {
    pub player_id: Uuid,
    pub preferred_interaction_styles: HashMap<InteractionType, f64>,
    pub communication_preferences: CommunicationPreferences,
    pub skill_interests: HashMap<SkillType, f64>,
    pub social_comfort_level: f64,
    pub challenge_preference: f64,
    pub cooperation_tendency: f64,
    pub personality_indicators: PersonalityVector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPreferences {
    pub formality_level: f64,
    pub response_speed: f64,
    pub detail_level: f64,
    pub emotional_expression: f64,
    pub humor_appreciation: f64,
    pub directness: f64,
}

/// Manages dynamic personality evolution
#[derive(Debug, Default)]
pub struct PersonalityEngine {
    pub personality_evolution_rules: Vec<EvolutionRule>,
    pub trait_interactions: HashMap<String, TraitInteraction>,
    pub cultural_influences: HashMap<Race, CulturalInfluence>,
    pub experience_modifiers: Vec<ExperienceModifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionRule {
    pub rule_name: String,
    pub trigger_conditions: Vec<PersonalityTrigger>,
    pub trait_modifications: HashMap<String, f64>,
    pub activation_threshold: f64,
    pub max_change_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrigger {
    pub trigger_type: PersonalityTriggerType,
    pub value_threshold: f64,
    pub time_window_hours: u64,
    pub confidence_required: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersonalityTriggerType {
    RepeatedSuccess,
    RepeatedFailure,
    PositiveSocialFeedback,
    NegativeSocialFeedback,
    NewLearningOpportunity,
    StressfulSituation,
    CreativeAchievement,
    LeadershipOpportunity,
}

/// Tracks social dynamics and group behavior
#[derive(Debug, Default)]
pub struct SocialDynamicsTracker {
    pub group_dynamics: HashMap<String, GroupDynamic>,
    pub relationship_networks: HashMap<Uuid, RelationshipNetwork>,
    pub social_hierarchies: Vec<SocialHierarchy>,
    pub cultural_exchanges: Vec<CulturalExchange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupDynamic {
    pub group_id: String,
    pub participants: Vec<Uuid>,
    pub group_cohesion: f64,
    pub leadership_distribution: HashMap<Uuid, f64>,
    pub communication_patterns: Vec<CommunicationPattern>,
    pub collaborative_efficiency: f64,
    pub conflict_resolution_style: ConflictStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipNetwork {
    pub central_npc: Uuid,
    pub connections: HashMap<Uuid, RelationshipStrength>,
    pub influence_scores: HashMap<Uuid, f64>,
    pub trust_levels: HashMap<Uuid, f64>,
    pub interaction_frequencies: HashMap<Uuid, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipStrength {
    VeryWeak(f64),
    Weak(f64),
    Moderate(f64),
    Strong(f64),
    VeryStrong(f64),
}

impl AdaptiveBehaviorSystem {
    /// Create a new adaptive behavior system
    pub fn new(neural_network_manager: Arc<NeuralNetworkManager>) -> Self {
        Self {
            behavior_manager: Arc::new(RwLock::new(BehaviorManager::default())),
            interaction_analyzer: Arc::new(RwLock::new(InteractionAnalyzer::default())),
            personality_engine: Arc::new(RwLock::new(PersonalityEngine::default())),
            social_dynamics: Arc::new(RwLock::new(SocialDynamicsTracker::default())),
            neural_network_manager,
        }
    }

    /// Initialize adaptive behavior for an NPC
    pub async fn initialize_npc_behavior(&self, npc_id: Uuid, base_personality: PersonalityVector, archetype: String) -> Result<()> {
        info!("🎭 Initializing adaptive behavior for NPC {}", npc_id);

        let mut behavior_manager = self.behavior_manager.write().await;
        
        let behavior_profile = AdaptiveBehaviorProfile {
            npc_id,
            base_personality: base_personality.clone(),
            learned_preferences: HashMap::new(),
            behavioral_adaptations: Vec::new(),
            interaction_patterns: HashMap::new(),
            success_metrics: BehaviorSuccessMetrics::default(),
            adaptation_speed: self.calculate_adaptation_speed(&base_personality),
            learning_confidence: 0.5,
            last_updated: SystemTime::now(),
        };

        behavior_manager.npc_behaviors.insert(npc_id, behavior_profile);

        // Initialize neural networks for this NPC
        self.neural_network_manager.initialize_npc_networks(npc_id, &archetype).await?;

        info!("✅ Adaptive behavior initialized for NPC {}", npc_id);
        Ok(())
    }

    /// Calculate adaptation speed based on personality
    fn calculate_adaptation_speed(&self, personality: &PersonalityVector) -> f64 {
        // NPCs with high openness and curiosity adapt faster
        let base_speed = 0.1;
        let openness_factor = personality.openness * 0.3;
        let curiosity_factor = personality.curiosity * 0.2;
        let conscientiousness_penalty = personality.conscientiousness * -0.1; // More cautious adaptation
        
        (base_speed + openness_factor + curiosity_factor + conscientiousness_penalty).clamp(0.01, 0.5)
    }

    /// Record a player-NPC interaction for learning
    pub async fn record_interaction(&self, interaction: InteractionRecord) -> Result<()> {
        debug!("📝 Recording interaction {} between NPC {} and player {}", 
            interaction.interaction_id, interaction.npc_id, interaction.player_id);

        let mut analyzer = self.interaction_analyzer.write().await;
        
        // Add to recent interactions
        analyzer.recent_interactions.push(interaction.clone());
        
        // Update player interaction profile
        self.update_player_profile(&interaction).await?;
        
        // Generate training data from this interaction
        let training_data = self.generate_training_data_from_interaction(&interaction).await?;
        
        // Train the NPC's neural networks
        self.neural_network_manager.train_network(
            interaction.npc_id,
            training_data,
            NetworkType::SocialInteraction
        ).await?;

        // Analyze for potential behavioral adaptations
        self.analyze_for_adaptations(&interaction).await?;

        Ok(())
    }

    /// Update player interaction profile based on recent interaction
    async fn update_player_profile(&self, interaction: &InteractionRecord) -> Result<()> {
        let mut analyzer = self.interaction_analyzer.write().await;
        
        let profile = analyzer.interaction_patterns
            .entry(interaction.player_id)
            .or_insert_with(|| PlayerInteractionProfile {
                player_id: interaction.player_id,
                preferred_interaction_styles: HashMap::new(),
                communication_preferences: CommunicationPreferences {
                    formality_level: 0.5,
                    response_speed: 0.5,
                    detail_level: 0.5,
                    emotional_expression: 0.5,
                    humor_appreciation: 0.5,
                    directness: 0.5,
                },
                skill_interests: HashMap::new(),
                social_comfort_level: 0.5,
                challenge_preference: 0.5,
                cooperation_tendency: 0.5,
                personality_indicators: PersonalityVector::default(),
            });

        // Update interaction style preferences
        let current_preference = profile.preferred_interaction_styles
            .entry(interaction.interaction_type.clone())
            .or_insert(0.5);
        
        // Adjust preference based on outcome
        let adjustment = interaction.outcome_rating * 0.1;
        *current_preference = (*current_preference + adjustment).clamp(0.0, 1.0);

        // Update communication preferences based on player responses
        self.update_communication_preferences(profile, interaction).await;

        Ok(())
    }

    /// Update communication preferences based on player behavior
    async fn update_communication_preferences(&self, profile: &mut PlayerInteractionProfile, interaction: &InteractionRecord) {
        for response in &interaction.player_responses {
            // Analyze response timing to infer speed preference
            if response.response_timing_ms < 5000 { // Fast response
                profile.communication_preferences.response_speed += 0.05;
            } else if response.response_timing_ms > 15000 { // Slow response
                profile.communication_preferences.response_speed -= 0.05;
            }

            // Analyze sentiment to infer emotional expression preference
            if response.sentiment_score > 0.7 {
                profile.communication_preferences.emotional_expression += 0.03;
            } else if response.sentiment_score < 0.3 {
                profile.communication_preferences.emotional_expression -= 0.03;
            }

            // Clamp all values
            profile.communication_preferences.response_speed = profile.communication_preferences.response_speed.clamp(0.0, 1.0);
            profile.communication_preferences.emotional_expression = profile.communication_preferences.emotional_expression.clamp(0.0, 1.0);
        }
    }

    /// Generate training data from an interaction
    async fn generate_training_data_from_interaction(&self, interaction: &InteractionRecord) -> Result<Vec<TrainingData>> {
        let mut training_data = Vec::new();

        for (i, npc_action) in interaction.npc_actions.iter().enumerate() {
            // Create input vector from context and previous actions
            let mut input_vector = Vec::new();
            
            // Add context features
            input_vector.push(interaction.context.emotional_atmosphere);
            input_vector.push(interaction.context.other_npcs_present.len() as f64 / 10.0); // Normalize
            input_vector.push(interaction.context.other_players_present.len() as f64 / 10.0);
            
            // Add timing features
            input_vector.push(npc_action.timing_ms as f64 / 30000.0); // Normalize to 30 seconds
            
            // Add previous action success
            if i > 0 {
                input_vector.push(interaction.npc_actions[i-1].player_reaction_score);
            } else {
                input_vector.push(0.5); // Neutral baseline
            }

            // Pad or truncate to expected input size
            while input_vector.len() < 48 {
                input_vector.push(0.0);
            }
            input_vector.truncate(48);

            // Create expected output based on player reaction
            let mut expected_output = vec![0.0; 12];
            let reaction_index = (npc_action.player_reaction_score * 11.0).clamp(0.0, 11.0) as usize;
            expected_output[reaction_index] = 1.0;

            let training_context = TrainingContext {
                scenario_type: format!("{:?}", interaction.interaction_type),
                participants: vec![interaction.npc_id, interaction.player_id],
                environment_state: HashMap::new(),
                success_outcome: npc_action.player_reaction_score > 0.6,
                player_satisfaction: interaction.player_satisfaction,
            };

            training_data.push(TrainingData {
                input_vector,
                expected_output,
                context: training_context,
                weight: interaction.learning_value,
            });
        }

        Ok(training_data)
    }

    /// Analyze interaction for potential behavioral adaptations
    async fn analyze_for_adaptations(&self, interaction: &InteractionRecord) -> Result<()> {
        let mut behavior_manager = self.behavior_manager.write().await;
        
        if let Some(behavior_profile) = behavior_manager.npc_behaviors.get_mut(&interaction.npc_id) {
            // Look for patterns that suggest adaptation opportunities
            if interaction.outcome_rating < 0.3 && interaction.player_satisfaction.unwrap_or(0.5) < 0.4 {
                // Poor interaction suggests need for adaptation
                let adaptation = self.generate_behavioral_adaptation(interaction, behavior_profile).await?;
                behavior_profile.behavioral_adaptations.push(adaptation);
                
                info!("🔄 Generated behavioral adaptation for NPC {} based on poor interaction", interaction.npc_id);
            }
            
            // Update success metrics
            behavior_profile.success_metrics.update_from_interaction(interaction);
            behavior_profile.last_updated = SystemTime::now();
        }

        Ok(())
    }

    /// Generate a behavioral adaptation based on interaction analysis
    async fn generate_behavioral_adaptation(&self, interaction: &InteractionRecord, profile: &AdaptiveBehaviorProfile) -> Result<BehavioralAdaptation> {
        let adaptation_id = Uuid::new_v4();
        
        // Analyze what went wrong in the interaction
        let adaptation_type = if interaction.player_responses.iter().any(|r| r.sentiment_score < 0.3) {
            AdaptationType::CommunicationStyle
        } else if interaction.duration_seconds < 30 {
            AdaptationType::SocialInteraction
        } else {
            AdaptationType::TaskApproach
        };

        // Create triggers based on similar conditions
        let triggers = vec![
            AdaptationTrigger {
                trigger_type: TriggerType::PlayerBehaviorPattern,
                condition_parameters: {
                    let mut params = HashMap::new();
                    params.insert("interaction_type".to_string(), match interaction.interaction_type {
                        InteractionType::Conversation => 1.0,
                        InteractionType::Trade => 2.0,
                        InteractionType::Teaching => 3.0,
                        _ => 0.0,
                    });
                    params
                },
                activation_threshold: 0.7,
                confidence_required: 0.6,
            }
        ];

        // Create behavior changes
        let changes = vec![
            BehaviorChange {
                change_type: ChangeType::CommunicationTone,
                parameter_adjustments: {
                    let mut adjustments = HashMap::new();
                    adjustments.insert("formality".to_string(), -0.1); // Less formal
                    adjustments.insert("warmth".to_string(), 0.2); // More warm
                    adjustments
                },
                neural_network_updates: vec![
                    NetworkUpdate {
                        network_type: NetworkType::SocialInteraction,
                        weight_adjustments: vec![], // Would calculate specific adjustments
                        learning_rate_modifier: 1.2, // Increase learning rate temporarily
                    }
                ],
                priority_shifts: HashMap::new(),
            }
        ];

        Ok(BehavioralAdaptation {
            adaptation_id,
            adaptation_type,
            trigger_conditions: triggers,
            behavioral_changes: changes,
            success_rate: 0.0, // Will be updated as it's used
            usage_count: 0,
            learned_from: LearningSource::PlayerInteraction(interaction.player_id),
            validation_status: ValidationStatus::Pending,
        })
    }

    /// Apply personality evolution based on experiences
    pub async fn evolve_personality(&self, npc_id: Uuid) -> Result<()> {
        let mut behavior_manager = self.behavior_manager.write().await;
        let personality_engine = self.personality_engine.read().await;
        
        if let Some(behavior_profile) = behavior_manager.npc_behaviors.get_mut(&npc_id) {
            // Apply evolution rules
            for rule in &personality_engine.personality_evolution_rules {
                if self.check_evolution_triggers(rule, behavior_profile).await {
                    self.apply_personality_changes(rule, &mut behavior_profile.base_personality).await;
                }
            }
            
            behavior_profile.last_updated = SystemTime::now();
        }

        Ok(())
    }

    /// Check if evolution rule triggers are met
    async fn check_evolution_triggers(&self, rule: &EvolutionRule, profile: &AdaptiveBehaviorProfile) -> bool {
        // Simplified trigger checking - would be more sophisticated in practice
        profile.success_metrics.overall_success_rate > rule.activation_threshold
    }

    /// Apply personality changes from evolution rule
    async fn apply_personality_changes(&self, rule: &EvolutionRule, personality: &mut PersonalityVector) {
        for (trait_name, change) in &rule.trait_modifications {
            let clamped_change = change.clamp(-rule.max_change_rate, rule.max_change_rate);
            
            match trait_name.as_str() {
                "openness" => personality.openness = (personality.openness + clamped_change).clamp(0.0, 1.0),
                "conscientiousness" => personality.conscientiousness = (personality.conscientiousness + clamped_change).clamp(0.0, 1.0),
                "extraversion" => personality.extraversion = (personality.extraversion + clamped_change).clamp(0.0, 1.0),
                "agreeableness" => personality.agreeableness = (personality.agreeableness + clamped_change).clamp(0.0, 1.0),
                "neuroticism" => personality.neuroticism = (personality.neuroticism + clamped_change).clamp(0.0, 1.0),
                "curiosity" => personality.curiosity = (personality.curiosity + clamped_change).clamp(0.0, 1.0),
                "creativity" => personality.creativity = (personality.creativity + clamped_change).clamp(0.0, 1.0),
                "leadership" => personality.leadership = (personality.leadership + clamped_change).clamp(0.0, 1.0),
                _ => {}
            }
        }
    }

    /// Get behavioral adaptation statistics
    pub async fn get_adaptation_statistics(&self) -> AdaptationStatistics {
        let behavior_manager = self.behavior_manager.read().await;
        let analyzer = self.interaction_analyzer.read().await;
        
        let total_npcs = behavior_manager.npc_behaviors.len();
        let total_adaptations: usize = behavior_manager.npc_behaviors.values()
            .map(|profile| profile.behavioral_adaptations.len())
            .sum();
        let recent_interactions = analyzer.recent_interactions.len();
        
        let average_adaptation_speed = if total_npcs > 0 {
            behavior_manager.npc_behaviors.values()
                .map(|profile| profile.adaptation_speed)
                .sum::<f64>() / total_npcs as f64
        } else {
            0.0
        };

        AdaptationStatistics {
            total_npcs,
            total_adaptations,
            recent_interactions,
            average_adaptation_speed,
            successful_adaptations: 0, // Would calculate from validation status
        }
    }
}

// Additional type definitions

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTemplate {
    pub template_name: String,
    pub archetype_compatibility: Vec<String>,
    pub personality_requirements: HashMap<String, (f64, f64)>, // trait -> (min, max)
    pub behavioral_patterns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorEvolution {
    pub evolution_id: Uuid,
    pub npc_id: Uuid,
    pub evolution_type: String,
    pub changes_made: Vec<String>,
    pub trigger_events: Vec<String>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentPattern {
    pub pattern_id: Uuid,
    pub pattern_description: String,
    pub observed_in_npcs: Vec<Uuid>,
    pub emergence_conditions: Vec<String>,
    pub pattern_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BehaviorSuccessMetrics {
    pub total_interactions: u64,
    pub positive_interactions: u64,
    pub overall_success_rate: f64,
    pub player_satisfaction_average: f64,
    pub adaptation_effectiveness: f64,
}

impl BehaviorSuccessMetrics {
    fn update_from_interaction(&mut self, interaction: &InteractionRecord) {
        self.total_interactions += 1;
        if interaction.outcome_rating > 0.6 {
            self.positive_interactions += 1;
        }
        
        self.overall_success_rate = self.positive_interactions as f64 / self.total_interactions as f64;
        
        if let Some(satisfaction) = interaction.player_satisfaction {
            // Running average of player satisfaction
            let weight = 0.1; // How much the new value affects the average
            self.player_satisfaction_average = self.player_satisfaction_average * (1.0 - weight) + satisfaction * weight;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningSource {
    PlayerInteraction(Uuid),
    NPCObservation(Uuid),
    EnvironmentalFeedback,
    SystemOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationStatus {
    Pending,
    Approved,
    Rejected,
    UnderReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub success_rate: f64,
    pub preferred_contexts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessPredictor {
    pub predictor_name: String,
    pub input_features: Vec<String>,
    pub accuracy: f64,
}

#[derive(Debug, Default)]
pub struct SentimentAnalyzer {
    // Placeholder for sentiment analysis functionality
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitInteraction {
    pub primary_trait: String,
    pub interacting_trait: String,
    pub interaction_strength: f64,
    pub effect_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalInfluence {
    pub race: Race,
    pub trait_modifiers: HashMap<String, f64>,
    pub behavioral_tendencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceModifier {
    pub experience_type: String,
    pub trait_impacts: HashMap<String, f64>,
    pub duration_effect: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationPattern {
    pub pattern_type: String,
    pub frequency: f64,
    pub effectiveness: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictStyle {
    Collaborative,
    Competitive,
    Accommodating,
    Avoiding,
    Compromising,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialHierarchy {
    pub hierarchy_id: Uuid,
    pub context: String,
    pub rankings: Vec<(Uuid, f64)>, // NPC ID, influence score
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalExchange {
    pub exchange_id: Uuid,
    pub participants: Vec<Uuid>,
    pub cultural_elements_shared: Vec<String>,
    pub mutual_understanding_gain: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStatistics {
    pub total_npcs: usize,
    pub total_adaptations: usize,
    pub recent_interactions: usize,
    pub average_adaptation_speed: f64,
    pub successful_adaptations: usize,
}

impl Default for PersonalityVector {
    fn default() -> Self {
        Self {
            openness: 0.5,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
            curiosity: 0.5,
            creativity: 0.5,
            leadership: 0.5,
        }
    }
}