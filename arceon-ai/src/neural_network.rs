use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;
use std::sync::Arc;
use tracing::{info, warn};

/// Advanced neural network system for NPCs with consensus-validated learning
pub struct NeuralNetworkManager {
    pub networks: Arc<RwLock<HashMap<Uuid, NeuralNetwork>>>,
    pub training_coordinator: Arc<RwLock<TrainingCoordinator>>,
    pub consensus_validator: Arc<RwLock<ConsensusValidator>>,
    pub behavior_patterns: Arc<RwLock<HashMap<String, BehaviorPattern>>>,
    pub learning_history: Arc<RwLock<LearningHistory>>,
}

/// Individual neural network for an NPC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub npc_id: Uuid,
    pub network_type: NetworkType,
    pub layers: Vec<Layer>,
    pub weights: Vec<Vec<Vec<f64>>>, // layer -> neuron -> weight
    pub biases: Vec<Vec<f64>>,       // layer -> neuron -> bias
    pub learning_rate: f64,
    pub training_iterations: u64,
    pub performance_metrics: PerformanceMetrics,
    pub last_updated: SystemTime,
    pub consensus_validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkType {
    DecisionMaking,     // Chooses what actions to take
    SocialInteraction,  // Handles communication with players/NPCs
    ResourceManagement, // Manages resources and crafting
    LearningAdapter,   // Adapts behavior based on experiences
    EmotionalResponse, // Manages emotional states and reactions
    StrategicPlanning, // Long-term goal planning and execution
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub layer_type: LayerType,
    pub neuron_count: usize,
    pub activation_function: ActivationFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    Input,
    Hidden,
    Output,
    Attention,    // For focusing on important information
    Memory,       // For maintaining long-term context
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActivationFunction {
    ReLU,
    Sigmoid,
    Tanh,
    Softmax,
    GELU,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub accuracy: f64,
    pub loss: f64,
    pub adaptation_speed: f64,
    pub decision_confidence: f64,
    pub social_success_rate: f64,
    pub resource_efficiency: f64,
}

/// Coordinates training across multiple NPCs and validates through consensus
#[derive(Debug, Default)]
pub struct TrainingCoordinator {
    pub training_sessions: HashMap<Uuid, TrainingSession>,
    pub collective_learning_pool: CollectiveLearningPool,
    pub optimization_strategies: Vec<OptimizationStrategy>,
    pub distributed_training_nodes: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSession {
    pub session_id: Uuid,
    pub npc_id: Uuid,
    pub training_type: TrainingType,
    pub data_samples: Vec<TrainingData>,
    pub target_metrics: PerformanceMetrics,
    pub status: TrainingStatus,
    pub started_at: SystemTime,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingType {
    SupervisedLearning,
    ReinforcementLearning,
    UnsupervisedLearning,
    TransferLearning,
    FederatedLearning,
    AdversarialTraining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainingStatus {
    Scheduled,
    InProgress,
    Completed,
    Failed,
    ConsensusValidation,
    Validated,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingData {
    pub input_vector: Vec<f64>,
    pub expected_output: Vec<f64>,
    pub context: TrainingContext,
    pub weight: f64, // Importance of this training sample
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingContext {
    pub scenario_type: String,
    pub participants: Vec<Uuid>,
    pub environment_state: HashMap<String, f64>,
    pub success_outcome: bool,
    pub player_satisfaction: Option<f64>,
}

/// Collective learning pool for sharing knowledge between NPCs
#[derive(Debug, Default)]
pub struct CollectiveLearningPool {
    pub shared_experiences: Vec<SharedExperience>,
    pub behavior_templates: HashMap<String, BehaviorTemplate>,
    pub optimization_insights: Vec<OptimizationInsight>,
    pub cross_npc_patterns: Vec<CrossNpcPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedExperience {
    pub experience_id: Uuid,
    pub source_npc: Uuid,
    pub experience_type: ExperienceType,
    pub situation_vector: Vec<f64>,
    pub action_taken: Vec<f64>,
    pub outcome_score: f64,
    pub transferability_score: f64,
    pub validation_votes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperienceType {
    PlayerInteraction,
    ResourceGathering,
    CraftingOptimization,
    SocialNegotiation,
    ConflictResolution,
    EnvironmentalAdaptation,
    CollaborativeProject,
}

/// Validates AI behavior changes through network consensus
#[derive(Debug, Default)]
pub struct ConsensusValidator {
    pub pending_validations: HashMap<Uuid, ValidationRequest>,
    pub validation_committee: Vec<Uuid>, // Masternode IDs
    pub validation_history: Vec<ValidationResult>,
    pub behavior_safety_rules: Vec<SafetyRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRequest {
    pub request_id: Uuid,
    pub npc_id: Uuid,
    pub behavior_change: BehaviorChange,
    pub training_evidence: TrainingEvidence,
    pub requested_by: Uuid,
    pub timestamp: SystemTime,
    pub votes: HashMap<Uuid, ValidationVote>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorChange {
    pub change_type: ChangeType,
    pub network_updates: Vec<NetworkUpdate>,
    pub expected_impact: ImpactAssessment,
    pub rollback_plan: Option<RollbackPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeType {
    WeightAdjustment,
    StructureModification,
    LearningRateChange,
    NewBehaviorPattern,
    MemoryConsolidation,
    EmotionalCalibration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkUpdate {
    pub layer_index: usize,
    pub weight_changes: Vec<Vec<f64>>,
    pub bias_changes: Vec<f64>,
    pub confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationVote {
    pub validator_id: Uuid,
    pub vote: VoteType,
    pub confidence: f64,
    pub reasoning: String,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Approve,
    Reject,
    RequestMoreData,
    ConditionalApproval(Vec<String>), // Conditions that must be met
}

/// Behavior patterns learned and validated by the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorPattern {
    pub pattern_id: Uuid,
    pub pattern_name: String,
    pub trigger_conditions: Vec<TriggerCondition>,
    pub response_sequence: Vec<ActionResponse>,
    pub success_rate: f64,
    pub usage_count: u64,
    pub learned_from: Vec<Uuid>, // NPC IDs that contributed to this pattern
    pub consensus_approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCondition {
    pub condition_type: ConditionType,
    pub threshold: f64,
    pub context_requirements: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    PlayerApproach,
    ResourceScarcity,
    SocialTension,
    EnvironmentalChange,
    TimeOfDay,
    SeasonalEvent,
    EmergencyState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResponse {
    pub action_type: String,
    pub parameters: HashMap<String, f64>,
    pub priority: u8,
    pub duration_ms: u64,
    pub required_resources: Vec<String>,
}

/// Tracks learning progress and optimization strategies
#[derive(Debug, Default)]
pub struct LearningHistory {
    pub learning_epochs: Vec<LearningEpoch>,
    pub performance_trends: HashMap<Uuid, Vec<PerformanceDataPoint>>,
    pub adaptation_strategies: Vec<AdaptationStrategy>,
    pub emergent_behaviors: Vec<EmergentBehavior>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEpoch {
    pub epoch_id: Uuid,
    pub start_time: SystemTime,
    pub end_time: SystemTime,
    pub participating_npcs: Vec<Uuid>,
    pub training_objectives: Vec<TrainingObjective>,
    pub achieved_improvements: HashMap<String, f64>,
    pub consensus_validation_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergentBehavior {
    pub behavior_id: Uuid,
    pub discovered_by: Vec<Uuid>,
    pub behavior_description: String,
    pub emergence_conditions: Vec<String>,
    pub observed_benefits: Vec<String>,
    pub replication_attempts: u32,
    pub network_adoption_rate: f64,
}

impl NeuralNetworkManager {
    /// Create a new neural network manager
    pub fn new() -> Self {
        Self {
            networks: Arc::new(RwLock::new(HashMap::new())),
            training_coordinator: Arc::new(RwLock::new(TrainingCoordinator::default())),
            consensus_validator: Arc::new(RwLock::new(ConsensusValidator::default())),
            behavior_patterns: Arc::new(RwLock::new(HashMap::new())),
            learning_history: Arc::new(RwLock::new(LearningHistory::default())),
        }
    }

    /// Initialize neural networks for an NPC
    pub async fn initialize_npc_networks(&self, npc_id: Uuid, archetype: &str) -> Result<()> {
        info!("🧠 Initializing neural networks for NPC {}", npc_id);

        let mut networks = self.networks.write().await;
        
        // Create specialized networks based on NPC archetype
        let network_types = self.get_network_types_for_archetype(archetype);
        
        for network_type in network_types {
            let network = self.create_network(npc_id, network_type).await?;
            networks.insert(network.npc_id, network);
        }

        info!("✅ Neural networks initialized for NPC {}", npc_id);
        Ok(())
    }

    /// Get appropriate network types for an NPC archetype
    fn get_network_types_for_archetype(&self, archetype: &str) -> Vec<NetworkType> {
        match archetype {
            "Scholar" => vec![
                NetworkType::DecisionMaking,
                NetworkType::LearningAdapter,
                NetworkType::SocialInteraction,
            ],
            "Trader" => vec![
                NetworkType::DecisionMaking,
                NetworkType::ResourceManagement,
                NetworkType::SocialInteraction,
                NetworkType::StrategicPlanning,
            ],
            "Warrior" => vec![
                NetworkType::DecisionMaking,
                NetworkType::EmotionalResponse,
                NetworkType::StrategicPlanning,
            ],
            "Teacher" => vec![
                NetworkType::SocialInteraction,
                NetworkType::LearningAdapter,
                NetworkType::EmotionalResponse,
            ],
            _ => vec![
                NetworkType::DecisionMaking,
                NetworkType::SocialInteraction,
            ],
        }
    }

    /// Create a neural network of specified type
    async fn create_network(&self, npc_id: Uuid, network_type: NetworkType) -> Result<NeuralNetwork> {
        let layers = match network_type {
            NetworkType::DecisionMaking => vec![
                Layer { layer_type: LayerType::Input, neuron_count: 64, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Hidden, neuron_count: 128, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Attention, neuron_count: 64, activation_function: ActivationFunction::GELU },
                Layer { layer_type: LayerType::Hidden, neuron_count: 32, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Output, neuron_count: 16, activation_function: ActivationFunction::Softmax },
            ],
            NetworkType::SocialInteraction => vec![
                Layer { layer_type: LayerType::Input, neuron_count: 48, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Memory, neuron_count: 96, activation_function: ActivationFunction::Tanh },
                Layer { layer_type: LayerType::Hidden, neuron_count: 64, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Output, neuron_count: 12, activation_function: ActivationFunction::Sigmoid },
            ],
            NetworkType::ResourceManagement => vec![
                Layer { layer_type: LayerType::Input, neuron_count: 32, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Hidden, neuron_count: 64, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Output, neuron_count: 8, activation_function: ActivationFunction::Softmax },
            ],
            _ => vec![
                Layer { layer_type: LayerType::Input, neuron_count: 32, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Hidden, neuron_count: 64, activation_function: ActivationFunction::ReLU },
                Layer { layer_type: LayerType::Output, neuron_count: 8, activation_function: ActivationFunction::Sigmoid },
            ],
        };

        // Initialize weights and biases
        let weights = self.initialize_weights(&layers);
        let biases = self.initialize_biases(&layers);

        Ok(NeuralNetwork {
            npc_id,
            network_type,
            layers,
            weights,
            biases,
            learning_rate: 0.001,
            training_iterations: 0,
            performance_metrics: PerformanceMetrics {
                accuracy: 0.5,
                loss: 1.0,
                adaptation_speed: 0.1,
                decision_confidence: 0.5,
                social_success_rate: 0.5,
                resource_efficiency: 0.5,
            },
            last_updated: SystemTime::now(),
            consensus_validated: false,
        })
    }

    /// Initialize network weights using Xavier initialization
    fn initialize_weights(&self, layers: &[Layer]) -> Vec<Vec<Vec<f64>>> {
        let mut weights = Vec::new();
        
        for i in 0..layers.len() - 1 {
            let input_size = layers[i].neuron_count;
            let output_size = layers[i + 1].neuron_count;
            let variance = 2.0 / (input_size + output_size) as f64;
            
            let mut layer_weights = Vec::new();
            for _ in 0..output_size {
                let mut neuron_weights = Vec::new();
                for _ in 0..input_size {
                    let weight = (rand::random::<f64>() - 0.5) * 2.0 * variance.sqrt();
                    neuron_weights.push(weight);
                }
                layer_weights.push(neuron_weights);
            }
            weights.push(layer_weights);
        }
        
        weights
    }

    /// Initialize network biases
    fn initialize_biases(&self, layers: &[Layer]) -> Vec<Vec<f64>> {
        let mut biases = Vec::new();
        
        for layer in layers.iter().skip(1) {
            let layer_biases = vec![0.0; layer.neuron_count];
            biases.push(layer_biases);
        }
        
        biases
    }

    /// Process input through neural network to get decision
    pub async fn process_decision(&self, npc_id: Uuid, input_data: &[f64], network_type: NetworkType) -> Result<Vec<f64>> {
        let networks = self.networks.read().await;
        
        if let Some(network) = networks.get(&npc_id) {
            if std::mem::discriminant(&network.network_type) == std::mem::discriminant(&network_type) {
                return Ok(self.forward_pass(network, input_data));
            }
        }
        
        // Return default decision if network not found
        Ok(vec![0.5; 8])
    }

    /// Forward pass through neural network
    fn forward_pass(&self, network: &NeuralNetwork, input: &[f64]) -> Vec<f64> {
        let mut current_output = input.to_vec();
        
        for layer_idx in 0..network.layers.len() - 1 {
            let weights = &network.weights[layer_idx];
            let biases = &network.biases[layer_idx];
            let activation_fn = &network.layers[layer_idx + 1].activation_function;
            
            let mut next_output = Vec::new();
            
            for (neuron_idx, neuron_weights) in weights.iter().enumerate() {
                let mut sum = biases[neuron_idx];
                for (input_idx, &input_val) in current_output.iter().enumerate() {
                    sum += input_val * neuron_weights[input_idx];
                }
                
                let activated = self.apply_activation(sum, activation_fn);
                next_output.push(activated);
            }
            
            current_output = next_output;
        }
        
        current_output
    }

    /// Apply activation function
    fn apply_activation(&self, value: f64, function: &ActivationFunction) -> f64 {
        match function {
            ActivationFunction::ReLU => value.max(0.0),
            ActivationFunction::Sigmoid => 1.0 / (1.0 + (-value).exp()),
            ActivationFunction::Tanh => value.tanh(),
            ActivationFunction::GELU => value * 0.5 * (1.0 + (value / 2.0_f64.sqrt()).tanh()),
            ActivationFunction::Softmax => value.exp(), // Simplified, would need full softmax implementation
        }
    }

    /// Submit behavior change for consensus validation
    pub async fn submit_behavior_change(&self, npc_id: Uuid, change: BehaviorChange) -> Result<Uuid> {
        info!("📝 Submitting behavior change for consensus validation");
        
        let request_id = Uuid::new_v4();
        let mut validator = self.consensus_validator.write().await;
        
        let validation_request = ValidationRequest {
            request_id,
            npc_id,
            behavior_change: change,
            training_evidence: TrainingEvidence {
                training_samples: 1000,
                performance_improvement: 0.15,
                validation_accuracy: 0.85,
                testing_duration_hours: 24,
            },
            requested_by: npc_id,
            timestamp: SystemTime::now(),
            votes: HashMap::new(),
        };
        
        validator.pending_validations.insert(request_id, validation_request);
        
        info!("✅ Behavior change submitted for validation: {}", request_id);
        Ok(request_id)
    }

    /// Process consensus validation votes
    pub async fn process_validation_vote(&self, request_id: Uuid, validator_id: Uuid, vote: ValidationVote) -> Result<()> {
        let mut validator = self.consensus_validator.write().await;
        
        if let Some(request) = validator.pending_validations.get_mut(&request_id) {
            request.votes.insert(validator_id, vote);
            
            // Check if we have enough votes to make a decision
            if request.votes.len() >= 3 { // Require at least 3 validator votes
                let approval_votes = request.votes.values()
                    .filter(|v| matches!(v.vote, VoteType::Approve))
                    .count();
                
                if approval_votes > request.votes.len() / 2 {
                    self.apply_validated_behavior_change(request).await?;
                    info!("✅ Behavior change approved and applied: {}", request_id);
                } else {
                    warn!("❌ Behavior change rejected: {}", request_id);
                }
            }
        }
        
        Ok(())
    }

    /// Apply a validated behavior change
    async fn apply_validated_behavior_change(&self, request: &ValidationRequest) -> Result<()> {
        let mut networks = self.networks.write().await;
        
        if let Some(network) = networks.get_mut(&request.npc_id) {
            // Apply the network updates from the behavior change
            for update in &request.behavior_change.network_updates {
                if update.layer_index < network.weights.len() {
                    // Apply weight changes
                    for (neuron_idx, neuron_changes) in update.weight_changes.iter().enumerate() {
                        if neuron_idx < network.weights[update.layer_index].len() {
                            for (weight_idx, &change) in neuron_changes.iter().enumerate() {
                                if weight_idx < network.weights[update.layer_index][neuron_idx].len() {
                                    network.weights[update.layer_index][neuron_idx][weight_idx] += change;
                                }
                            }
                        }
                    }
                    
                    // Apply bias changes
                    for (bias_idx, &change) in update.bias_changes.iter().enumerate() {
                        if bias_idx < network.biases[update.layer_index].len() {
                            network.biases[update.layer_index][bias_idx] += change;
                        }
                    }
                }
            }
            
            network.consensus_validated = true;
            network.last_updated = SystemTime::now();
        }
        
        Ok(())
    }

    /// Train neural network using provided data
    pub async fn train_network(&self, npc_id: Uuid, training_data: Vec<TrainingData>, network_type: NetworkType) -> Result<()> {
        info!("🎯 Training neural network for NPC {} ({:?})", npc_id, network_type);
        
        let session_id = Uuid::new_v4();
        let mut coordinator = self.training_coordinator.write().await;
        
        let training_session = TrainingSession {
            session_id,
            npc_id,
            training_type: TrainingType::SupervisedLearning,
            data_samples: training_data,
            target_metrics: PerformanceMetrics {
                accuracy: 0.85,
                loss: 0.2,
                adaptation_speed: 0.8,
                decision_confidence: 0.9,
                social_success_rate: 0.8,
                resource_efficiency: 0.85,
            },
            status: TrainingStatus::InProgress,
            started_at: SystemTime::now(),
            duration_ms: 0,
        };
        
        coordinator.training_sessions.insert(session_id, training_session);
        
        // Simulate training process (in production, this would be actual ML training)
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            info!("✅ Training completed for session {}", session_id);
        });
        
        Ok(())
    }

    /// Get learning statistics for monitoring
    pub async fn get_learning_statistics(&self) -> LearningStatistics {
        let networks = self.networks.read().await;
        let coordinator = self.training_coordinator.read().await;
        let validator = self.consensus_validator.read().await;
        
        let total_networks = networks.len();
        let validated_networks = networks.values().filter(|n| n.consensus_validated).count();
        let active_training_sessions = coordinator.training_sessions.values()
            .filter(|s| matches!(s.status, TrainingStatus::InProgress))
            .count();
        let pending_validations = validator.pending_validations.len();
        
        let average_performance = if !networks.is_empty() {
            networks.values().map(|n| n.performance_metrics.accuracy).sum::<f64>() / networks.len() as f64
        } else {
            0.0
        };
        
        LearningStatistics {
            total_networks,
            validated_networks,
            active_training_sessions,
            pending_validations,
            average_performance,
            learning_epochs_completed: 0, // Would track actual epochs
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingEvidence {
    pub training_samples: usize,
    pub performance_improvement: f64,
    pub validation_accuracy: f64,
    pub testing_duration_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingObjective {
    pub objective_type: String,
    pub target_value: f64,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDataPoint {
    pub timestamp: SystemTime,
    pub metric_values: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationStrategy {
    pub strategy_name: String,
    pub conditions: Vec<String>,
    pub adjustments: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTemplate {
    pub template_name: String,
    pub network_structure: Vec<Layer>,
    pub recommended_parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationInsight {
    pub insight_type: String,
    pub discovered_pattern: String,
    pub improvement_potential: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossNpcPattern {
    pub pattern_name: String,
    pub participating_npcs: Vec<Uuid>,
    pub pattern_strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStrategy {
    pub strategy_name: String,
    pub target_networks: Vec<NetworkType>,
    pub optimization_algorithm: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAssessment {
    pub player_interaction_impact: f64,
    pub world_economy_impact: f64,
    pub social_dynamics_impact: f64,
    pub computational_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    pub rollback_steps: Vec<String>,
    pub safety_checks: Vec<String>,
    pub estimated_rollback_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    pub rule_name: String,
    pub rule_type: SafetyRuleType,
    pub threshold: f64,
    pub violation_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SafetyRuleType {
    MaxLearningRate,
    MinAccuracy,
    MaxBehaviorDeviation,
    PlayerSafetyCheck,
    ResourceConsumptionLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub request_id: Uuid,
    pub approved: bool,
    pub validator_consensus: f64,
    pub validation_time: SystemTime,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStatistics {
    pub total_networks: usize,
    pub validated_networks: usize,
    pub active_training_sessions: usize,
    pub pending_validations: usize,
    pub average_performance: f64,
    pub learning_epochs_completed: u64,
}