use anyhow::Result;
use std::sync::Arc;
use crate::{Renderer, Frame};

/// Post-processing pipeline
pub struct PostProcessor {
    _renderer: Arc<Renderer>,
}

impl PostProcessor {
    pub async fn new(renderer: Arc<Renderer>) -> Result<Self> {
        Ok(Self { _renderer: renderer })
    }

    pub async fn process(&self, _frame: &Frame, _renderer: &Renderer) -> Result<()> {
        Ok(())
    }

    pub async fn resize(&self, _new_size: (u32, u32)) -> Result<()> {
        Ok(())
    }
}