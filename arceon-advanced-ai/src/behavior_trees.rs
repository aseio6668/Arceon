/*!
# Behavior Tree System

Advanced behavior trees for NPC AI and decision making including:
- Hierarchical behavior composition
- Conditional logic and state management
- Blackboard system for shared data
- Parallel execution and synchronization
- Dynamic behavior modification
- Behavior tree compilation and optimization
*/

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main behavior tree system
#[derive(Debug)]
pub struct BehaviorTreeSystem {
    pub trees: HashMap<Uuid, BehaviorTree>,
    pub tree_templates: HashMap<String, BehaviorTreeTemplate>,
    pub blackboards: HashMap<Uuid, Arc<RwLock<Blackboard>>>,
    pub execution_contexts: HashMap<Uuid, ExecutionContext>,
    pub config: BehaviorTreeConfig,
    pub performance_metrics: Arc<RwLock<BehaviorTreeMetrics>>,
}

/// Behavior tree configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTreeConfig {
    pub max_tree_depth: usize,
    pub max_nodes_per_tree: usize,
    pub execution_frequency_hz: f64,
    pub enable_debugging: bool,
    pub enable_profiling: bool,
    pub blackboard_size_limit: usize,
    pub parallel_execution_limit: usize,
    pub behavior_cache_size: usize,
    pub timeout_seconds: f64,
}

/// Complete behavior tree structure
#[derive(Debug, Clone)]
pub struct BehaviorTree {
    pub tree_id: Uuid,
    pub name: String,
    pub root_node: Arc<BehaviorNode>,
    pub blackboard_id: Uuid,
    pub metadata: BehaviorTreeMetadata,
    pub execution_state: TreeExecutionState,
    pub created_at: DateTime<Utc>,
    pub last_executed: Option<DateTime<Utc>>,
}

/// Individual behavior tree node
#[derive(Debug, Clone)]
pub struct BehaviorNode {
    pub node_id: Uuid,
    pub node_type: NodeType,
    pub name: String,
    pub description: String,
    pub children: Vec<Arc<BehaviorNode>>,
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub decorators: Vec<Decorator>,
    pub properties: HashMap<String, serde_json::Value>,
    pub state: Arc<RwLock<NodeState>>,
}

/// Types of behavior tree nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    // Composite nodes
    Sequence,
    Selector,
    Parallel,
    Random,
    
    // Decorator nodes
    Inverter,
    Repeater,
    UntilSuccess,
    UntilFailure,
    Timer,
    Cooldown,
    
    // Leaf nodes
    Action,
    Condition,
    Wait,
    
    // Control flow
    SubTree,
    Reference,
}

/// Node execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeState {
    Idle,
    Running,
    Success,
    Failure,
    Aborted,
    Invalid,
}

/// Node execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeResult {
    Success,
    Failure,
    Running,
    Aborted,
    Error(String),
}

/// Condition evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_id: Uuid,
    pub condition_type: ConditionType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub negated: bool,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    // Blackboard conditions
    BlackboardKeyExists,
    BlackboardValueEquals,
    BlackboardValueGreaterThan,
    BlackboardValueLessThan,
    BlackboardValueInRange,
    
    // Game state conditions
    PlayerInRange,
    HealthBelowThreshold,
    EnemyVisible,
    PathExists,
    ItemInInventory,
    SkillOnCooldown,
    
    // Time conditions
    TimeOfDay,
    ElapsedTime,
    RandomChance,
    
    // Custom conditions
    Custom(String),
}

/// Action to execute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_id: Uuid,
    pub action_type: ActionType,
    pub parameters: HashMap<String, serde_json::Value>,
    pub duration: Option<f64>,
    pub interruptible: bool,
    pub priority: ActionPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    // Movement actions
    MoveTo,
    MoveToPlayer,
    Patrol,
    Wander,
    Stop,
    
    // Combat actions
    Attack,
    Defend,
    Retreat,
    UseSkill,
    CastSpell,
    
    // Communication actions
    Say,
    Emote,
    SendMessage,
    PlaySound,
    
    // State actions
    SetBlackboardValue,
    ModifyHealth,
    GiveItem,
    TakeItem,
    
    // Animation actions
    PlayAnimation,
    ChangeState,
    
    // Custom actions
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActionPriority {
    Low = 1,
    Normal = 2,
    High = 3,
    Critical = 4,
    Emergency = 5,
}

/// Node decorators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decorator {
    pub decorator_id: Uuid,
    pub decorator_type: DecoratorType,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecoratorType {
    // Control decorators
    Inverter,
    Repeater { count: Option<usize> },
    RetryUntilSuccess { max_attempts: usize },
    RetryUntilFailure { max_attempts: usize },
    
    // Timing decorators
    Timer { duration_seconds: f64 },
    Cooldown { cooldown_seconds: f64 },
    Timeout { timeout_seconds: f64 },
    
    // Conditional decorators
    ConditionalRunOnce,
    BlackboardDecorator,
    ForceSuccess,
    ForceFailure,
    
    // Probability decorators
    RandomSuccess { probability: f64 },
    RandomFailure { probability: f64 },
}

/// Shared blackboard for data storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Blackboard {
    pub blackboard_id: Uuid,
    pub entries: HashMap<String, BlackboardEntry>,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub access_log: Vec<BlackboardAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackboardEntry {
    pub key: String,
    pub value: serde_json::Value,
    pub data_type: BlackboardDataType,
    pub created_at: DateTime<Utc>,
    pub last_modified: DateTime<Utc>,
    pub access_count: u64,
    pub ttl: Option<DateTime<Utc>>, // Time to live
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlackboardDataType {
    Boolean,
    Integer,
    Float,
    String,
    Vector2,
    Vector3,
    Object,
    Array,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackboardAccess {
    pub timestamp: DateTime<Utc>,
    pub access_type: BlackboardAccessType,
    pub key: String,
    pub node_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlackboardAccessType {
    Read,
    Write,
    Delete,
    Check,
}

/// Behavior tree template for reuse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTreeTemplate {
    pub template_id: Uuid,
    pub name: String,
    pub description: String,
    pub category: String,
    pub template_data: serde_json::Value,
    pub parameters: Vec<TemplateParameter>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    pub name: String,
    pub data_type: BlackboardDataType,
    pub default_value: Option<serde_json::Value>,
    pub required: bool,
    pub description: String,
}

/// Tree execution context
#[derive(Debug)]
pub struct ExecutionContext {
    pub context_id: Uuid,
    pub tree_id: Uuid,
    pub owner_entity_id: Option<Uuid>,
    pub execution_stack: Vec<NodeExecution>,
    pub current_node: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub total_execution_time: f64,
    pub execution_count: u64,
    pub debug_info: Option<DebugInfo>,
}

#[derive(Debug)]
pub struct NodeExecution {
    pub node_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub result: Option<NodeResult>,
    pub execution_count: u32,
    pub children_executed: Vec<Uuid>,
}

#[derive(Debug)]
pub struct DebugInfo {
    pub execution_trace: Vec<ExecutionStep>,
    pub blackboard_snapshots: Vec<BlackboardSnapshot>,
    pub performance_data: Vec<PerformanceDataPoint>,
    pub error_log: Vec<ErrorEntry>,
}

#[derive(Debug)]
pub struct ExecutionStep {
    pub step_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub node_id: Uuid,
    pub step_type: ExecutionStepType,
    pub details: String,
}

#[derive(Debug)]
pub enum ExecutionStepType {
    NodeEntered,
    NodeExited,
    ConditionEvaluated,
    ActionExecuted,
    BlackboardAccessed,
    Error,
}

#[derive(Debug)]
pub struct BlackboardSnapshot {
    pub timestamp: DateTime<Utc>,
    pub entries: HashMap<String, serde_json::Value>,
}

#[derive(Debug)]
pub struct PerformanceDataPoint {
    pub timestamp: DateTime<Utc>,
    pub node_id: Uuid,
    pub execution_time_ms: f64,
    pub memory_usage_bytes: u64,
}

#[derive(Debug)]
pub struct ErrorEntry {
    pub timestamp: DateTime<Utc>,
    pub error_type: String,
    pub message: String,
    pub node_id: Option<Uuid>,
    pub stack_trace: Option<String>,
}

/// Behavior tree metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTreeMetadata {
    pub author: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub complexity_score: f64,
    pub estimated_performance: PerformanceEstimate,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimate {
    pub avg_execution_time_ms: f64,
    pub max_execution_time_ms: f64,
    pub memory_usage_estimate_bytes: u64,
    pub cpu_intensity: CpuIntensity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CpuIntensity {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Tree execution state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeExecutionState {
    pub state: TreeState,
    pub current_node_id: Option<Uuid>,
    pub execution_count: u64,
    pub success_count: u64,
    pub failure_count: u64,
    pub average_execution_time_ms: f64,
    pub last_result: Option<NodeResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeState {
    Idle,
    Running,
    Paused,
    Completed,
    Error,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTreeMetrics {
    pub tree_performance: HashMap<Uuid, TreePerformanceMetrics>,
    pub node_performance: HashMap<Uuid, NodePerformanceMetrics>,
    pub blackboard_performance: HashMap<Uuid, BlackboardPerformanceMetrics>,
    pub system_performance: SystemPerformanceMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreePerformanceMetrics {
    pub executions_per_second: f64,
    pub average_execution_time_ms: f64,
    pub success_rate: f64,
    pub failure_rate: f64,
    pub memory_usage_bytes: u64,
    pub cpu_utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    pub execution_count: u64,
    pub average_execution_time_ms: f64,
    pub success_rate: f64,
    pub memory_impact_bytes: u64,
    pub cache_hit_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackboardPerformanceMetrics {
    pub read_operations_per_second: f64,
    pub write_operations_per_second: f64,
    pub cache_hit_rate: f64,
    pub average_entry_count: f64,
    pub memory_usage_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemPerformanceMetrics {
    pub total_trees_active: usize,
    pub total_nodes_executed_per_second: f64,
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub garbage_collection_frequency: f64,
}

impl BehaviorTreeSystem {
    pub fn new(config: BehaviorTreeConfig) -> Self {
        Self {
            trees: HashMap::new(),
            tree_templates: HashMap::new(),
            blackboards: HashMap::new(),
            execution_contexts: HashMap::new(),
            config,
            performance_metrics: Arc::new(RwLock::new(BehaviorTreeMetrics::default())),
        }
    }

    /// Create a new behavior tree
    pub async fn create_tree(&mut self, name: String, root_node: BehaviorNode) -> Result<Uuid> {
        let tree_id = Uuid::new_v4();
        let blackboard_id = Uuid::new_v4();
        
        // Create blackboard
        let blackboard = Blackboard::new(blackboard_id);
        self.blackboards.insert(blackboard_id, Arc::new(RwLock::new(blackboard)));
        
        // Create tree
        let tree = BehaviorTree {
            tree_id,
            name,
            root_node: Arc::new(root_node),
            blackboard_id,
            metadata: BehaviorTreeMetadata::default(),
            execution_state: TreeExecutionState::new(),
            created_at: Utc::now(),
            last_executed: None,
        };
        
        self.trees.insert(tree_id, tree);
        
        tracing::info!("Created behavior tree: {}", tree_id);
        Ok(tree_id)
    }

    /// Execute a behavior tree
    pub async fn execute_tree(&mut self, tree_id: Uuid) -> Result<NodeResult> {
        // Get tree data first
        let (blackboard_id, root_node) = {
            let tree = self.trees.get(&tree_id)
                .ok_or_else(|| anyhow::anyhow!("Tree not found: {}", tree_id))?;
            (tree.blackboard_id, tree.root_node.clone())
        };
        
        let blackboard = self.blackboards.get(&blackboard_id)
            .ok_or_else(|| anyhow::anyhow!("Blackboard not found: {}", blackboard_id))?
            .clone();
        
        // Create execution context
        let context = ExecutionContext::new(tree_id, blackboard_id);
        let context_id = context.context_id;
        self.execution_contexts.insert(context_id, context);
        
        // Execute root node
        let result = self.execute_node(&root_node, blackboard, context_id).await?;
        
        // Update tree state
        let tree = self.trees.get_mut(&tree_id)
            .ok_or_else(|| anyhow::anyhow!("Tree not found: {}", tree_id))?;
        tree.execution_state.execution_count += 1;
        tree.execution_state.last_result = Some(result.clone());
        tree.last_executed = Some(Utc::now());
        
        // Clean up context
        self.execution_contexts.remove(&context_id);
        
        Ok(result)
    }

    /// Execute a single node
    async fn execute_node(
        &self,
        node: &BehaviorNode,
        blackboard: Arc<RwLock<Blackboard>>,
        context_id: Uuid,
    ) -> Result<NodeResult> {
        let mut node_state = node.state.write().await;
        *node_state = NodeState::Running;
        drop(node_state);
        
        // Evaluate conditions
        for condition in &node.conditions {
            if !self.evaluate_condition(condition, &blackboard).await? {
                let mut node_state = node.state.write().await;
                *node_state = NodeState::Failure;
                return Ok(NodeResult::Failure);
            }
        }
        
        // Execute based on node type
        let result = match node.node_type {
            NodeType::Sequence => self.execute_sequence(node, blackboard, context_id).await?,
            NodeType::Selector => self.execute_selector(node, blackboard, context_id).await?,
            NodeType::Parallel => self.execute_parallel(node, blackboard, context_id).await?,
            NodeType::Action => self.execute_actions(&node.actions, &blackboard).await?,
            NodeType::Condition => {
                if node.conditions.is_empty() {
                    NodeResult::Success
                } else {
                    NodeResult::Success // Conditions already evaluated above
                }
            },
            _ => NodeResult::Success, // Placeholder for other node types
        };
        
        // Update node state
        let mut node_state = node.state.write().await;
        *node_state = match result {
            NodeResult::Success => NodeState::Success,
            NodeResult::Failure => NodeState::Failure,
            NodeResult::Running => NodeState::Running,
            NodeResult::Aborted => NodeState::Aborted,
            NodeResult::Error(_) => NodeState::Invalid,
        };
        
        Ok(result)
    }

    /// Execute sequence node (all children must succeed)
    async fn execute_sequence(
        &self,
        node: &BehaviorNode,
        blackboard: Arc<RwLock<Blackboard>>,
        context_id: Uuid,
    ) -> Result<NodeResult> {
        for child in &node.children {
            let result = Box::pin(self.execute_node(child, blackboard.clone(), context_id)).await?;
            match result {
                NodeResult::Success => continue,
                NodeResult::Running => return Ok(NodeResult::Running),
                NodeResult::Failure => return Ok(NodeResult::Failure),
                NodeResult::Aborted => return Ok(NodeResult::Aborted),
                NodeResult::Error(e) => return Ok(NodeResult::Error(e)),
            }
        }
        Ok(NodeResult::Success)
    }

    /// Execute selector node (first success wins)
    async fn execute_selector(
        &self,
        node: &BehaviorNode,
        blackboard: Arc<RwLock<Blackboard>>,
        context_id: Uuid,
    ) -> Result<NodeResult> {
        for child in &node.children {
            let result = Box::pin(self.execute_node(child, blackboard.clone(), context_id)).await?;
            match result {
                NodeResult::Success => return Ok(NodeResult::Success),
                NodeResult::Running => return Ok(NodeResult::Running),
                NodeResult::Failure => continue,
                NodeResult::Aborted => return Ok(NodeResult::Aborted),
                NodeResult::Error(e) => return Ok(NodeResult::Error(e)),
            }
        }
        Ok(NodeResult::Failure)
    }

    /// Execute parallel node (all children execute simultaneously)
    async fn execute_parallel(
        &self,
        node: &BehaviorNode,
        blackboard: Arc<RwLock<Blackboard>>,
        context_id: Uuid,
    ) -> Result<NodeResult> {
        let mut handles = Vec::new();
        
        for child in &node.children {
            let child_clone = Arc::clone(child);
            let blackboard_clone = Arc::clone(&blackboard);
            let context_clone = context_id;
            
            // Note: In a real implementation, you'd want to spawn actual async tasks
            // For now, we'll execute sequentially as a placeholder
            let result = Box::pin(self.execute_node(&child_clone, blackboard_clone, context_clone)).await?;
            handles.push(result);
        }
        
        // Determine overall result based on parallel policy
        let success_count = handles.iter()
            .filter(|r| matches!(r, NodeResult::Success))
            .count();
        
        if success_count == handles.len() {
            Ok(NodeResult::Success)
        } else if success_count > 0 {
            Ok(NodeResult::Running) // Partial success
        } else {
            Ok(NodeResult::Failure)
        }
    }

    /// Evaluate a condition
    async fn evaluate_condition(
        &self,
        condition: &Condition,
        blackboard: &Arc<RwLock<Blackboard>>,
    ) -> Result<bool> {
        let result = match condition.condition_type {
            ConditionType::BlackboardKeyExists => {
                let bb = blackboard.read().await;
                let key = condition.parameters.get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing key parameter"))?;
                bb.entries.contains_key(key)
            },
            ConditionType::BlackboardValueEquals => {
                let bb = blackboard.read().await;
                let key = condition.parameters.get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Missing key parameter"))?;
                let expected_value = condition.parameters.get("value")
                    .ok_or_else(|| anyhow::anyhow!("Missing value parameter"))?;
                
                bb.entries.get(key)
                    .map(|entry| &entry.value == expected_value)
                    .unwrap_or(false)
            },
            ConditionType::RandomChance => {
                let chance = condition.parameters.get("chance")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.5);
                rand::random::<f64>() < chance
            },
            _ => true, // Placeholder for other condition types
        };
        
        Ok(if condition.negated { !result } else { result })
    }

    /// Execute actions
    async fn execute_actions(
        &self,
        actions: &[Action],
        blackboard: &Arc<RwLock<Blackboard>>,
    ) -> Result<NodeResult> {
        for action in actions {
            match action.action_type {
                ActionType::SetBlackboardValue => {
                    let mut bb = blackboard.write().await;
                    let key = action.parameters.get("key")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| anyhow::anyhow!("Missing key parameter"))?;
                    let value = action.parameters.get("value")
                        .ok_or_else(|| anyhow::anyhow!("Missing value parameter"))?;
                    
                    bb.set_value(key.to_string(), value.clone()).await?;
                },
                _ => {
                    // Placeholder for other action types
                    tracing::debug!("Executing action: {:?}", action.action_type);
                }
            }
        }
        Ok(NodeResult::Success)
    }

    /// Get performance metrics
    pub async fn get_performance_metrics(&self) -> BehaviorTreeMetrics {
        self.performance_metrics.read().await.clone()
    }
}

impl ExecutionContext {
    fn new(tree_id: Uuid, _blackboard_id: Uuid) -> Self {
        Self {
            context_id: Uuid::new_v4(),
            tree_id,
            owner_entity_id: None,
            execution_stack: Vec::new(),
            current_node: None,
            start_time: Utc::now(),
            total_execution_time: 0.0,
            execution_count: 0,
            debug_info: None,
        }
    }
}

impl Blackboard {
    fn new(blackboard_id: Uuid) -> Self {
        Self {
            blackboard_id,
            entries: HashMap::new(),
            created_at: Utc::now(),
            last_modified: Utc::now(),
            access_log: Vec::new(),
        }
    }

    pub async fn set_value(&mut self, key: String, value: serde_json::Value) -> Result<()> {
        let entry = BlackboardEntry {
            key: key.clone(),
            value,
            data_type: BlackboardDataType::Object, // Could be inferred
            created_at: Utc::now(),
            last_modified: Utc::now(),
            access_count: 0,
            ttl: None,
        };
        
        self.entries.insert(key, entry);
        self.last_modified = Utc::now();
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.entries.get(key).map(|entry| &entry.value)
    }
}

impl TreeExecutionState {
    fn new() -> Self {
        Self {
            state: TreeState::Idle,
            current_node_id: None,
            execution_count: 0,
            success_count: 0,
            failure_count: 0,
            average_execution_time_ms: 0.0,
            last_result: None,
        }
    }
}

impl Default for BehaviorTreeConfig {
    fn default() -> Self {
        Self {
            max_tree_depth: 20,
            max_nodes_per_tree: 1000,
            execution_frequency_hz: 60.0,
            enable_debugging: false,
            enable_profiling: true,
            blackboard_size_limit: 10000,
            parallel_execution_limit: 10,
            behavior_cache_size: 100,
            timeout_seconds: 5.0,
        }
    }
}

impl Default for BehaviorTreeMetadata {
    fn default() -> Self {
        Self {
            author: "System".to_string(),
            version: "1.0.0".to_string(),
            description: "Generated behavior tree".to_string(),
            tags: Vec::new(),
            complexity_score: 1.0,
            estimated_performance: PerformanceEstimate {
                avg_execution_time_ms: 1.0,
                max_execution_time_ms: 10.0,
                memory_usage_estimate_bytes: 1024,
                cpu_intensity: CpuIntensity::Low,
            },
            dependencies: Vec::new(),
        }
    }
}

impl Default for BehaviorTreeMetrics {
    fn default() -> Self {
        Self {
            tree_performance: HashMap::new(),
            node_performance: HashMap::new(),
            blackboard_performance: HashMap::new(),
            system_performance: SystemPerformanceMetrics {
                total_trees_active: 0,
                total_nodes_executed_per_second: 0.0,
                memory_usage_mb: 0.0,
                cpu_usage_percent: 0.0,
                garbage_collection_frequency: 0.0,
            },
        }
    }
}