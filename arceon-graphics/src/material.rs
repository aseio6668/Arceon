use uuid::Uuid;
use glam::Vec3;
use std::collections::HashMap;

/// PBR material definition
#[derive(Debug, Clone)]
pub struct Material {
    pub id: Uuid,
    pub name: String,
    pub shader: MaterialShader,
    pub properties: MaterialProperties,
    pub textures: HashMap<TextureSlot, Uuid>,
    pub render_queue: RenderQueue,
    pub blend_mode: BlendMode,
    pub cull_mode: CullMode,
    pub depth_test: bool,
    pub depth_write: bool,
}

/// Material shader configuration
#[derive(Debug, Clone)]
pub struct MaterialShader {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub defines: Vec<String>,
    pub features: Vec<ShaderFeature>,
}

/// Material properties for PBR
#[derive(Debug, Clone)]
pub struct MaterialProperties {
    pub albedo: Vec3,
    pub metallic: f32,
    pub roughness: f32,
    pub normal_scale: f32,
    pub occlusion_strength: f32,
    pub emission: Vec3,
    pub emission_intensity: f32,
    pub alpha: f32,
    pub alpha_cutoff: f32,
}

/// Texture slots in material
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum TextureSlot {
    Albedo,
    Normal,
    MetallicRoughness,
    Occlusion,
    Emission,
    Height,
    Detail,
    Mask,
}

/// Render queue priority
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RenderQueue {
    Background = 1000,
    Geometry = 2000,
    AlphaTest = 2450,
    Transparent = 3000,
    Overlay = 4000,
}

/// Blend modes
#[derive(Debug, Clone)]
pub enum BlendMode {
    Opaque,
    Alpha,
    Additive,
    Multiply,
    Screen,
}

/// Cull modes
#[derive(Debug, Clone)]
pub enum CullMode {
    None,
    Front,
    Back,
}

/// Shader features
#[derive(Debug, Clone)]
pub enum ShaderFeature {
    NormalMapping,
    ParallaxMapping,
    AlphaTest,
    Emission,
    DetailTextures,
    VertexColors,
}

impl Material {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            shader: MaterialShader::default(),
            properties: MaterialProperties::default(),
            textures: HashMap::new(),
            render_queue: RenderQueue::Geometry,
            blend_mode: BlendMode::Opaque,
            cull_mode: CullMode::Back,
            depth_test: true,
            depth_write: true,
        }
    }
}

impl Default for MaterialShader {
    fn default() -> Self {
        Self {
            vertex_shader: "geometry.wgsl".to_string(),
            fragment_shader: "geometry.wgsl".to_string(),
            defines: Vec::new(),
            features: Vec::new(),
        }
    }
}

impl Default for MaterialProperties {
    fn default() -> Self {
        Self {
            albedo: Vec3::ONE,
            metallic: 0.0,
            roughness: 0.5,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            emission: Vec3::ZERO,
            emission_intensity: 0.0,
            alpha: 1.0,
            alpha_cutoff: 0.5,
        }
    }
}