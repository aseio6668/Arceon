use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Work Classification System for determining valuable network contributions
/// Replaces traditional mining with productive work that benefits the entire network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkClassificationSystem {
    pub work_categories: HashMap<WorkCategory, WorkCategoryConfig>,
    pub active_work_assignments: HashMap<Uuid, WorkAssignment>,
    pub completed_work_history: Vec<CompletedWork>,
    pub work_demand_metrics: WorkDemandMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkCategory {
    // Core Network Operations
    WorldStateComputation,
    ConsensusValidation,
    PeerNetworkRelay,
    DataStorageAndRetrieval,
    
    // Game World Services
    NPCBehaviorProcessing,
    PhysicsSimulation,
    QuestLogicExecution,
    EconomicModelingAndAnalysis,
    
    // Network Infrastructure
    NetworkOptimization,
    SecurityMonitoring,
    NodeBootstrapAssistance,
    FailureRecoveryOperations,
    
    // Community Contributions
    ContentModeration,
    BugReporting,
    PerformanceTesting,
    DocumentationMaintenance,
    
    // Advanced Computations
    AIModelTraining,
    DataAnalytics,
    CryptographicOperations,
    DistributedComputing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCategoryConfig {
    pub category: WorkCategory,
    pub base_reward_rate: f64,
    pub difficulty_scaling: DifficultyScaling,
    pub resource_requirements: ResourceRequirements,
    pub verification_method: WorkVerification,
    pub priority_multiplier: f64,
    pub max_concurrent_workers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyScaling {
    pub base_difficulty: f64,
    pub scaling_factor: f64,
    pub max_difficulty: f64,
    pub adjustment_period: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    pub cpu_cores_required: u32,
    pub memory_mb_required: u64,
    pub storage_mb_required: u64,
    pub network_bandwidth_required: u32,
    pub estimated_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkVerification {
    /// Work verified by consensus of multiple nodes
    ConsensusVerification { min_validators: u32 },
    /// Work verified by cryptographic proof
    CryptographicProof { proof_type: String },
    /// Work verified by deterministic computation check
    DeterministicVerification,
    /// Work verified by network performance metrics
    PerformanceMetrics,
    /// Work verified by master node inspection
    MasterNodeReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkAssignment {
    pub assignment_id: Uuid,
    pub worker_node_id: Uuid,
    pub work_category: WorkCategory,
    pub work_description: String,
    pub assigned_at: SystemTime,
    pub expected_completion: SystemTime,
    pub work_parameters: WorkParameters,
    pub reward_pool: f64,
    pub status: WorkStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkParameters {
    pub input_data: Vec<u8>,
    pub computation_target: ComputationTarget,
    pub quality_requirements: QualityRequirements,
    pub deadline: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComputationTarget {
    WorldStateUpdate { 
        area_ids: Vec<String>,
        update_type: String,
    },
    NPCProcessing {
        npc_ids: Vec<Uuid>,
        behavior_cycles: u32,
    },
    PhysicsCalculation {
        objects: Vec<Uuid>,
        time_steps: u32,
    },
    NetworkOptimization {
        target_nodes: Vec<Uuid>,
        optimization_type: String,
    },
    SecurityAnalysis {
        scan_targets: Vec<String>,
        analysis_depth: u32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityRequirements {
    pub accuracy_threshold: f64,
    pub performance_threshold: Duration,
    pub resource_efficiency_threshold: f64,
    pub error_tolerance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkStatus {
    Assigned,
    InProgress,
    Completed,
    UnderVerification,
    Verified,
    Rejected { reason: String },
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedWork {
    pub work_id: Uuid,
    pub worker_node_id: Uuid,
    pub work_category: WorkCategory,
    pub completion_time: SystemTime,
    pub computation_duration: Duration,
    pub quality_score: f64,
    pub reward_earned: f64,
    pub verification_result: VerificationResult,
    pub network_benefit_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub verified: bool,
    pub verification_method: WorkVerification,
    pub verification_score: f64,
    pub verifier_nodes: Vec<Uuid>,
    pub verification_time: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkDemandMetrics {
    pub category_demand: HashMap<WorkCategory, f64>,
    pub total_pending_work: u32,
    pub average_completion_time: HashMap<WorkCategory, Duration>,
    pub worker_availability: HashMap<WorkCategory, u32>,
    pub network_priority_adjustments: HashMap<WorkCategory, f64>,
}

impl WorkClassificationSystem {
    pub fn new() -> Self {
        let mut work_categories = HashMap::new();
        
        // Initialize core network operation categories
        work_categories.insert(WorkCategory::WorldStateComputation, WorkCategoryConfig {
            category: WorkCategory::WorldStateComputation,
            base_reward_rate: 10.0,
            difficulty_scaling: DifficultyScaling {
                base_difficulty: 1.0,
                scaling_factor: 1.2,
                max_difficulty: 5.0,
                adjustment_period: Duration::from_secs(3600),
            },
            resource_requirements: ResourceRequirements {
                cpu_cores_required: 4,
                memory_mb_required: 2048,
                storage_mb_required: 100,
                network_bandwidth_required: 10,
                estimated_duration: Duration::from_secs(300),
            },
            verification_method: WorkVerification::ConsensusVerification { min_validators: 3 },
            priority_multiplier: 2.0,
            max_concurrent_workers: 50,
        });

        work_categories.insert(WorkCategory::ConsensusValidation, WorkCategoryConfig {
            category: WorkCategory::ConsensusValidation,
            base_reward_rate: 15.0,
            difficulty_scaling: DifficultyScaling {
                base_difficulty: 1.5,
                scaling_factor: 1.1,
                max_difficulty: 3.0,
                adjustment_period: Duration::from_secs(1800),
            },
            resource_requirements: ResourceRequirements {
                cpu_cores_required: 2,
                memory_mb_required: 1024,
                storage_mb_required: 50,
                network_bandwidth_required: 20,
                estimated_duration: Duration::from_secs(60),
            },
            verification_method: WorkVerification::CryptographicProof { 
                proof_type: "consensus_signature".to_string() 
            },
            priority_multiplier: 3.0,
            max_concurrent_workers: 100,
        });

        work_categories.insert(WorkCategory::NPCBehaviorProcessing, WorkCategoryConfig {
            category: WorkCategory::NPCBehaviorProcessing,
            base_reward_rate: 8.0,
            difficulty_scaling: DifficultyScaling {
                base_difficulty: 0.8,
                scaling_factor: 1.3,
                max_difficulty: 4.0,
                adjustment_period: Duration::from_secs(7200),
            },
            resource_requirements: ResourceRequirements {
                cpu_cores_required: 2,
                memory_mb_required: 1024,
                storage_mb_required: 20,
                network_bandwidth_required: 5,
                estimated_duration: Duration::from_secs(180),
            },
            verification_method: WorkVerification::DeterministicVerification,
            priority_multiplier: 1.5,
            max_concurrent_workers: 200,
        });

        Self {
            work_categories,
            active_work_assignments: HashMap::new(),
            completed_work_history: Vec::new(),
            work_demand_metrics: WorkDemandMetrics {
                category_demand: HashMap::new(),
                total_pending_work: 0,
                average_completion_time: HashMap::new(),
                worker_availability: HashMap::new(),
                network_priority_adjustments: HashMap::new(),
            },
        }
    }

    /// Assign work to a capable node based on network needs and node capabilities
    pub fn assign_work(&mut self, node_id: Uuid, node_capabilities: &crate::master_node::NodeCapabilities) -> Result<Option<WorkAssignment>> {
        // Find the most suitable work for this node
        let suitable_work = self.find_suitable_work(node_capabilities)?;
        
        if let Some(work_category) = suitable_work {
            let assignment_id = Uuid::new_v4();
            let config = self.work_categories.get(&work_category).unwrap();
            
            let assignment = WorkAssignment {
                assignment_id,
                worker_node_id: node_id,
                work_category: work_category.clone(),
                work_description: self.generate_work_description(&work_category)?,
                assigned_at: SystemTime::now(),
                expected_completion: SystemTime::now() + config.resource_requirements.estimated_duration,
                work_parameters: self.generate_work_parameters(&work_category)?,
                reward_pool: self.calculate_work_reward(&work_category)?,
                status: WorkStatus::Assigned,
            };
            
            self.active_work_assignments.insert(assignment_id, assignment.clone());
            Ok(Some(assignment))
        } else {
            Ok(None)
        }
    }

    /// Verify completed work and distribute rewards
    pub fn verify_and_reward_work(&mut self, assignment_id: Uuid, work_result: WorkResult) -> Result<f64> {
        if let Some(mut assignment) = self.active_work_assignments.remove(&assignment_id) {
            assignment.status = WorkStatus::UnderVerification;
            
            // Perform verification based on work category
            let verification_result = self.verify_work_result(&assignment, &work_result)?;
            
            if verification_result.verified {
                assignment.status = WorkStatus::Verified;
                
                // Calculate final reward based on quality
                let quality_multiplier = verification_result.verification_score;
                let final_reward = assignment.reward_pool * quality_multiplier;
                
                // Record completed work
                let completed_work = CompletedWork {
                    work_id: assignment.assignment_id,
                    worker_node_id: assignment.worker_node_id,
                    work_category: assignment.work_category.clone(),
                    completion_time: SystemTime::now(),
                    computation_duration: SystemTime::now().duration_since(assignment.assigned_at).unwrap_or_default(),
                    quality_score: verification_result.verification_score,
                    reward_earned: final_reward,
                    verification_result,
                    network_benefit_score: self.calculate_network_benefit(&assignment.work_category)?,
                };
                
                self.completed_work_history.push(completed_work);
                self.update_work_demand_metrics(&assignment.work_category)?;
                
                Ok(final_reward)
            } else {
                assignment.status = WorkStatus::Rejected { 
                    reason: "Verification failed".to_string() 
                };
                Ok(0.0)
            }
        } else {
            Err(anyhow::anyhow!("Work assignment not found"))
        }
    }

    /// Calculate dynamic work rewards based on network demand and difficulty
    pub fn calculate_work_reward(&self, category: &WorkCategory) -> Result<f64> {
        let config = self.work_categories.get(category).ok_or_else(|| {
            anyhow::anyhow!("Work category not found: {:?}", category)
        })?;

        let base_reward = config.base_reward_rate;
        let difficulty_multiplier = config.difficulty_scaling.base_difficulty;
        let priority_multiplier = config.priority_multiplier;
        
        // Apply demand-based scaling
        let demand_multiplier = self.work_demand_metrics.category_demand
            .get(category)
            .copied()
            .unwrap_or(1.0);

        let final_reward = base_reward * difficulty_multiplier * priority_multiplier * demand_multiplier;
        Ok(final_reward)
    }

    fn find_suitable_work(&self, capabilities: &crate::master_node::NodeCapabilities) -> Result<Option<WorkCategory>> {
        // Find work categories that match node capabilities
        for (category, config) in &self.work_categories {
            if capabilities.cpu_cores >= config.resource_requirements.cpu_cores_required &&
               capabilities.memory_gb * 1024 >= config.resource_requirements.memory_mb_required &&
               capabilities.storage_gb * 1024 >= config.resource_requirements.storage_mb_required &&
               capabilities.bandwidth_mbps >= config.resource_requirements.network_bandwidth_required {
                
                // Check if there's demand for this work
                let current_workers = self.active_work_assignments.values()
                    .filter(|a| a.work_category == *category)
                    .count() as u32;
                
                if current_workers < config.max_concurrent_workers {
                    return Ok(Some(category.clone()));
                }
            }
        }
        Ok(None)
    }

    fn generate_work_description(&self, category: &WorkCategory) -> Result<String> {
        let description = match category {
            WorkCategory::WorldStateComputation => "Compute world state updates for active game areas",
            WorkCategory::ConsensusValidation => "Validate network consensus and transactions",
            WorkCategory::NPCBehaviorProcessing => "Process NPC behavior and decision making",
            WorkCategory::PhysicsSimulation => "Calculate physics interactions and collisions",
            WorkCategory::PeerNetworkRelay => "Relay network connections between peers",
            WorkCategory::DataStorageAndRetrieval => "Store and retrieve distributed world data",
            WorkCategory::NetworkOptimization => "Optimize network routing and performance",
            WorkCategory::SecurityMonitoring => "Monitor network for security threats",
            WorkCategory::NodeBootstrapAssistance => "Assist new nodes joining the network",
            WorkCategory::FailureRecoveryOperations => "Help recover from node failures",
            _ => "Perform specialized network computation",
        };
        Ok(description.to_string())
    }

    fn generate_work_parameters(&self, category: &WorkCategory) -> Result<WorkParameters> {
        Ok(WorkParameters {
            input_data: vec![], // Would contain actual work data
            computation_target: match category {
                WorkCategory::WorldStateComputation => ComputationTarget::WorldStateUpdate {
                    area_ids: vec!["area_1".to_string(), "area_2".to_string()],
                    update_type: "simulation_tick".to_string(),
                },
                WorkCategory::NPCBehaviorProcessing => ComputationTarget::NPCProcessing {
                    npc_ids: vec![Uuid::new_v4()],
                    behavior_cycles: 10,
                },
                _ => ComputationTarget::WorldStateUpdate {
                    area_ids: vec!["default".to_string()],
                    update_type: "general".to_string(),
                },
            },
            quality_requirements: QualityRequirements {
                accuracy_threshold: 0.95,
                performance_threshold: Duration::from_secs(300),
                resource_efficiency_threshold: 0.8,
                error_tolerance: 0.05,
            },
            deadline: SystemTime::now() + Duration::from_secs(3600),
        })
    }

    fn verify_work_result(&self, assignment: &WorkAssignment, _work_result: &WorkResult) -> Result<VerificationResult> {
        let config = self.work_categories.get(&assignment.work_category).unwrap();
        
        // Perform verification based on the verification method
        let verified = match &config.verification_method {
            WorkVerification::ConsensusVerification { min_validators: _ } => {
                // Would implement consensus verification
                true
            },
            WorkVerification::CryptographicProof { proof_type: _ } => {
                // Would implement cryptographic verification
                true
            },
            WorkVerification::DeterministicVerification => {
                // Would implement deterministic verification
                true
            },
            WorkVerification::PerformanceMetrics => {
                // Would implement performance-based verification
                true
            },
            WorkVerification::MasterNodeReview => {
                // Would implement master node review
                true
            },
        };

        Ok(VerificationResult {
            verified,
            verification_method: config.verification_method.clone(),
            verification_score: if verified { 1.0 } else { 0.0 },
            verifier_nodes: vec![Uuid::new_v4()], // Mock verifiers
            verification_time: SystemTime::now(),
        })
    }

    fn calculate_network_benefit(&self, category: &WorkCategory) -> Result<f64> {
        // Calculate how much this work benefits the entire network
        match category {
            WorkCategory::WorldStateComputation => Ok(0.9),
            WorkCategory::ConsensusValidation => Ok(1.0),
            WorkCategory::NPCBehaviorProcessing => Ok(0.7),
            WorkCategory::NetworkOptimization => Ok(0.8),
            WorkCategory::SecurityMonitoring => Ok(0.95),
            _ => Ok(0.6),
        }
    }

    fn update_work_demand_metrics(&mut self, _category: &WorkCategory) -> Result<()> {
        // Update demand metrics based on completed work
        Ok(())
    }

    pub fn get_network_work_statistics(&self) -> NetworkWorkStatistics {
        NetworkWorkStatistics {
            total_work_completed: self.completed_work_history.len() as u64,
            active_work_assignments: self.active_work_assignments.len() as u32,
            total_rewards_distributed: self.completed_work_history.iter()
                .map(|w| w.reward_earned)
                .sum(),
            average_work_quality: self.completed_work_history.iter()
                .map(|w| w.quality_score)
                .sum::<f64>() / self.completed_work_history.len() as f64,
            category_statistics: self.work_categories.keys()
                .map(|cat| {
                    let completed_count = self.completed_work_history.iter()
                        .filter(|w| w.work_category == *cat)
                        .count() as u64;
                    (cat.clone(), completed_count)
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkResult {
    pub assignment_id: Uuid,
    pub result_data: Vec<u8>,
    pub computation_proof: Vec<u8>,
    pub performance_metrics: WorkPerformanceMetrics,
    pub worker_signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkPerformanceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub execution_time: Duration,
    pub network_usage: u64,
    pub energy_efficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkWorkStatistics {
    pub total_work_completed: u64,
    pub active_work_assignments: u32,
    pub total_rewards_distributed: f64,
    pub average_work_quality: f64,
    pub category_statistics: HashMap<WorkCategory, u64>,
}