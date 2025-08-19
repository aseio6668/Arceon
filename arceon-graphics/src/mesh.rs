use uuid::Uuid;
use glam::{Vec3, Vec2};

/// 3D mesh data
#[derive(Debug)]
pub struct Mesh {
    pub id: Uuid,
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub submeshes: Vec<SubMesh>,
    pub bounding_box: BoundingBox,
    pub vertex_buffer: Option<wgpu::Buffer>,
    pub index_buffer: Option<wgpu::Buffer>,
}

/// Vertex data structure
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub uv: Vec2,
    pub color: [f32; 4],
}

/// Submesh for multi-material meshes
#[derive(Debug, Clone)]
pub struct SubMesh {
    pub material_index: u32,
    pub index_start: u32,
    pub index_count: u32,
    pub vertex_start: u32,
    pub vertex_count: u32,
}

/// Bounding box
#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

impl Mesh {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            vertices: Vec::new(),
            indices: Vec::new(),
            submeshes: Vec::new(),
            bounding_box: BoundingBox {
                min: Vec3::ZERO,
                max: Vec3::ZERO,
            },
            vertex_buffer: None,
            index_buffer: None,
        }
    }
}