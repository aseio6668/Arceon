// Pathfinding AI Module
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathNode {
    pub id: Uuid,
    pub position: (f64, f64),
    pub cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PathfindingAI {
    pub nodes: Vec<PathNode>,
    pub algorithm: String,
}

impl PathfindingAI {
    pub fn new(_config: super::PathfindingConfig) -> Self {
        Self {
            nodes: Vec::new(),
            algorithm: "A*".to_string(),
        }
    }
}

// Type alias for compatibility
pub type PathfindingEngine = PathfindingAI;