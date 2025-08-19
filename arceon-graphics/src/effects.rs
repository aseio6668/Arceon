use anyhow::Result;
use std::sync::Arc;
use crate::Renderer;

/// Visual effects manager
pub struct EffectManager {
    _renderer: Arc<Renderer>,
}

impl EffectManager {
    pub async fn new(renderer: Arc<Renderer>) -> Result<Self> {
        Ok(Self { _renderer: renderer })
    }

    pub async fn update(&self, _delta_time: f32) -> Result<()> {
        Ok(())
    }
}