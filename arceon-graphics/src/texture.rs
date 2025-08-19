use uuid::Uuid;

/// Texture resource
#[derive(Debug)]
pub struct Texture {
    pub id: Uuid,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub mip_levels: u32,
    pub format: TextureFormat,
    pub texture_type: TextureType,
    pub filter_mode: FilterMode,
    pub wrap_mode: WrapMode,
    pub data: Option<Vec<u8>>,
    pub gpu_texture: Option<wgpu::Texture>,
}

/// Texture formats
#[derive(Debug, Clone)]
pub enum TextureFormat {
    RGBA8,
    RGBA16F,
    RGBA32F,
    R8,
    RG8,
    RGB8,
    SRGBA8,
    Depth24Stencil8,
    Depth32F,
    BC1,
    BC3,
    BC7,
}

/// Texture types
#[derive(Debug, Clone)]
pub enum TextureType {
    Texture2D,
    Texture3D,
    TextureCube,
    Texture2DArray,
    TextureCubeArray,
}

/// Texture filtering
#[derive(Debug, Clone)]
pub enum FilterMode {
    Point,
    Linear,
    Trilinear,
    Anisotropic(u32),
}

/// Texture wrapping
#[derive(Debug, Clone)]
pub enum WrapMode {
    Repeat,
    Clamp,
    Mirror,
    Border,
}

impl Texture {
    pub fn new(name: String, width: u32, height: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            width,
            height,
            depth: 1,
            mip_levels: 1,
            format: TextureFormat::RGBA8,
            texture_type: TextureType::Texture2D,
            filter_mode: FilterMode::Linear,
            wrap_mode: WrapMode::Repeat,
            data: None,
            gpu_texture: None,
        }
    }
}