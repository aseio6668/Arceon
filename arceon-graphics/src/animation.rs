use anyhow::Result;

/// Animation system for skeletal and vertex animations
pub struct AnimationSystem;

impl AnimationSystem {
    pub async fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn update(&self, _delta_time: f32) -> Result<()> {
        Ok(())
    }
}