// Emotion Simulation Module
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionState {
    pub emotion_type: String,
    pub intensity: f64,
    pub duration: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EmotionSimulation {
    pub entity_id: Uuid,
    pub current_emotions: Vec<EmotionState>,
}

impl EmotionSimulation {
    pub fn new(_config: super::EmotionConfig) -> Self {
        Self {
            entity_id: Uuid::new_v4(),
            current_emotions: Vec::new(),
        }
    }
}

// Type alias for compatibility
pub type EmotionSimulator = EmotionSimulation;