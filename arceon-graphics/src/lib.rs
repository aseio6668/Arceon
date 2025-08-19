pub mod renderer;
pub mod mesh;
pub mod material;
pub mod texture;
pub mod lighting;
pub mod camera;
pub mod scene;
pub mod effects;
pub mod animation;
pub mod terrain;
pub mod water;
pub mod sky;
pub mod ui;
pub mod particles;
pub mod shadows;
pub mod postprocessing;

pub use renderer::*;
pub use mesh::*;
pub use material::*;
pub use texture::*;
pub use lighting::*;
pub use scene::*;
pub use effects::*;
pub use animation::*;
pub use terrain::*;
pub use water::*;
pub use sky::*;
pub use ui::*;
pub use particles::*;
pub use postprocessing::*;

use anyhow::Result;
use std::sync::Arc;
use winit::window::Window;

/// Main graphics engine for Arceon MMORPG
/// Provides high-performance 3D rendering with modern visual effects
pub struct GraphicsEngine {
    pub renderer: Arc<Renderer>,
    pub scene_manager: Arc<SceneManager>,
    pub resource_manager: Arc<ResourceManager>,
    pub effect_manager: Arc<EffectManager>,
    pub animation_system: Arc<AnimationSystem>,
    pub lighting_system: Arc<LightingSystem>,
    pub post_processor: Arc<PostProcessor>,
}

impl GraphicsEngine {
    /// Initialize the graphics engine
    pub async fn new(window: &Window) -> Result<Self> {
        let renderer = Arc::new(Renderer::new(window).await?);
        let scene_manager = Arc::new(SceneManager::new(renderer.clone()).await?);
        let resource_manager = Arc::new(ResourceManager::new(renderer.clone()).await?);
        let effect_manager = Arc::new(EffectManager::new(renderer.clone()).await?);
        let animation_system = Arc::new(AnimationSystem::new().await?);
        let lighting_system = Arc::new(LightingSystem::new(renderer.clone()).await?);
        let post_processor = Arc::new(PostProcessor::new(renderer.clone()).await?);

        Ok(Self {
            renderer,
            scene_manager,
            resource_manager,
            effect_manager,
            animation_system,
            lighting_system,
            post_processor,
        })
    }

    /// Render a frame
    pub async fn render_frame(&self, delta_time: f32) -> Result<()> {
        // Update animations
        self.animation_system.update(delta_time).await?;

        // Update effects
        self.effect_manager.update(delta_time).await?;

        // Update lighting
        self.lighting_system.update(delta_time).await?;

        // Begin rendering
        let frame = self.renderer.begin_frame().await?;

        // Render scene
        self.scene_manager.render(&frame, &self.renderer).await?;

        // Apply post-processing
        self.post_processor.process(&frame, &self.renderer).await?;

        // Present frame
        self.renderer.end_frame(frame).await?;

        Ok(())
    }

    /// Resize the graphics context
    pub async fn resize(&self, new_size: (u32, u32)) -> Result<()> {
        self.renderer.resize(new_size).await?;
        self.post_processor.resize(new_size).await?;
        Ok(())
    }
}