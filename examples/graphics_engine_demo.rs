use anyhow::Result;
use arceon_graphics::*;
use glam::{Vec3, Vec2, Quat};
use std::sync::Arc;
use tracing::{info, warn};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use std::time::Instant;

/// Comprehensive demo of the Advanced Graphics Engine
/// Showcases 3D rendering, lighting, materials, and visual effects
#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::init();

    info!("🎨 Starting Advanced Graphics Engine Demo");

    // Create event loop and window
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Arceon Advanced Graphics Engine Demo")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
        .build(&event_loop)?;

    // Initialize graphics engine
    let graphics_engine = Arc::new(GraphicsEngine::new(&window).await?);

    info!("✅ Graphics engine initialized successfully");

    // Setup demo scene
    setup_demo_scene(&graphics_engine).await?;

    // Run render loop
    let mut last_frame_time = Instant::now();
    let mut frame_count = 0u64;
    let mut fps_timer = Instant::now();

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("👋 Closing graphics demo");
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    info!("🔄 Resizing window to: {}x{}", size.width, size.height);
                    let graphics_engine = graphics_engine.clone();
                    pollster::block_on(async {
                        if let Err(e) = graphics_engine.resize((size.width, size.height)).await {
                            warn!("Failed to resize graphics context: {}", e);
                        }
                    });
                }
                WindowEvent::RedrawRequested => {
                    let current_time = Instant::now();
                    let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
                    last_frame_time = current_time;

                    // Render frame
                    let graphics_engine = graphics_engine.clone();
                    pollster::block_on(async {
                        if let Err(e) = render_frame(&graphics_engine, delta_time).await {
                            warn!("Render error: {}", e);
                        }
                    });

                    // Update FPS counter
                    frame_count += 1;
                    if fps_timer.elapsed().as_secs() >= 1 {
                        let fps = frame_count as f64 / fps_timer.elapsed().as_secs_f64();
                        info!("📊 FPS: {:.1} | Frame Time: {:.1}ms", fps, delta_time * 1000.0);
                        frame_count = 0;
                        fps_timer = Instant::now();
                    }
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

/// Setup the demo scene with various objects and lighting
async fn setup_demo_scene(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🎬 Setting up demo scene");

    // Create sun light (directional)
    let sun_id = graphics_engine.lighting_system.create_directional_light(
        Vec3::new(0.3, -0.7, 0.2).normalize(),
        Vec3::new(1.0, 0.95, 0.8), // Warm sunlight
        3.0,
        true, // Cast shadows
    ).await?;

    info!("☀️ Created sun light: {}", sun_id);

    // Create some point lights
    let torch_positions = [
        Vec3::new(-5.0, 2.0, -5.0),
        Vec3::new(5.0, 2.0, -5.0),
        Vec3::new(-5.0, 2.0, 5.0),
        Vec3::new(5.0, 2.0, 5.0),
    ];

    for (i, &position) in torch_positions.iter().enumerate() {
        let torch_id = graphics_engine.lighting_system.create_point_light(
            position,
            Vec3::new(1.0, 0.6, 0.2), // Warm torch light
            5.0,
            8.0,
            true, // Cast shadows
        ).await?;

        info!("🔥 Created torch light {}: {}", i + 1, torch_id);
    }

    // Create a spot light (like a flashlight)
    let spotlight_id = graphics_engine.lighting_system.create_spot_light(
        Vec3::new(0.0, 5.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.8, 0.9, 1.0), // Cool spotlight
        10.0,
        15.0,
        45.0_f32.to_radians(), // Outer angle
        30.0_f32.to_radians(), // Inner angle
        true,
    ).await?;

    info!("🔦 Created spotlight: {}", spotlight_id);

    // Create area light (like a window)
    let area_light_id = graphics_engine.lighting_system.create_area_light(
        Vec3::new(-8.0, 3.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.9, 0.95, 1.0), // Cool daylight
        2.0,
        Vec3::new(0.1, 3.0, 2.0), // Window size
        AreaLightShape::Rectangle,
    ).await?;

    info!("🪟 Created area light: {}", area_light_id);

    // Create some demo meshes and materials
    create_demo_objects(graphics_engine).await?;

    // Set initial time of day (noon)
    graphics_engine.lighting_system.set_time_of_day(12.0).await?;

    info!("✅ Demo scene setup completed");
    Ok(())
}

/// Create demo objects with different materials
async fn create_demo_objects(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🎯 Creating demo objects");

    // Create materials for different object types
    create_demo_materials(graphics_engine).await?;

    // Create various geometric objects
    create_geometric_objects(graphics_engine).await?;

    // Create terrain
    create_demo_terrain(graphics_engine).await?;

    // Create water surface
    create_demo_water(graphics_engine).await?;

    // Create vegetation
    create_demo_vegetation(graphics_engine).await?;

    Ok(())
}

/// Create various PBR materials for demo
async fn create_demo_materials(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🎨 Creating PBR materials");

    // Metal material (sword, armor)
    let metal_material = Material {
        id: uuid::Uuid::new_v4(),
        name: "Steel".to_string(),
        shader: MaterialShader::default(),
        properties: MaterialProperties {
            albedo: Vec3::new(0.7, 0.7, 0.8),
            metallic: 0.9,
            roughness: 0.1,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            emission: Vec3::ZERO,
            emission_intensity: 0.0,
            alpha: 1.0,
            alpha_cutoff: 0.5,
        },
        textures: std::collections::HashMap::new(),
        render_queue: RenderQueue::Geometry,
        blend_mode: BlendMode::Opaque,
        cull_mode: CullMode::Back,
        depth_test: true,
        depth_write: true,
    };

    // Stone material
    let stone_material = Material {
        id: uuid::Uuid::new_v4(),
        name: "Stone".to_string(),
        shader: MaterialShader::default(),
        properties: MaterialProperties {
            albedo: Vec3::new(0.5, 0.5, 0.4),
            metallic: 0.0,
            roughness: 0.8,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            emission: Vec3::ZERO,
            emission_intensity: 0.0,
            alpha: 1.0,
            alpha_cutoff: 0.5,
        },
        textures: std::collections::HashMap::new(),
        render_queue: RenderQueue::Geometry,
        blend_mode: BlendMode::Opaque,
        cull_mode: CullMode::Back,
        depth_test: true,
        depth_write: true,
    };

    // Wood material
    let wood_material = Material {
        id: uuid::Uuid::new_v4(),
        name: "Wood".to_string(),
        shader: MaterialShader::default(),
        properties: MaterialProperties {
            albedo: Vec3::new(0.6, 0.4, 0.2),
            metallic: 0.0,
            roughness: 0.7,
            normal_scale: 1.0,
            occlusion_strength: 1.0,
            emission: Vec3::ZERO,
            emission_intensity: 0.0,
            alpha: 1.0,
            alpha_cutoff: 0.5,
        },
        textures: std::collections::HashMap::new(),
        render_queue: RenderQueue::Geometry,
        blend_mode: BlendMode::Opaque,
        cull_mode: CullMode::Back,
        depth_test: true,
        depth_write: true,
    };

    // Glowing crystal material
    let crystal_material = Material {
        id: uuid::Uuid::new_v4(),
        name: "Magic Crystal".to_string(),
        shader: MaterialShader::default(),
        properties: MaterialProperties {
            albedo: Vec3::new(0.3, 0.6, 1.0),
            metallic: 0.0,
            roughness: 0.0,
            normal_scale: 1.0,
            occlusion_strength: 0.0,
            emission: Vec3::new(0.2, 0.4, 0.8),
            emission_intensity: 2.0,
            alpha: 0.8,
            alpha_cutoff: 0.1,
        },
        textures: std::collections::HashMap::new(),
        render_queue: RenderQueue::Transparent,
        blend_mode: BlendMode::Alpha,
        cull_mode: CullMode::None,
        depth_test: true,
        depth_write: false,
    };

    info!("✅ Created {} demo materials", 4);
    Ok(())
}

/// Create various geometric objects for the demo
async fn create_geometric_objects(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("📐 Creating geometric objects");

    // Create a central structure (like a temple or building)
    let building_transform = Transform {
        position: Vec3::new(0.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(2.0, 3.0, 2.0),
        matrix: glam::Mat4::IDENTITY,
        is_dirty: true,
    };

    let building_components = vec![
        Component::MeshRenderer {
            mesh: Arc::new(create_cube_mesh()),
            materials: vec![Arc::new(create_stone_material())],
            cast_shadows: true,
            receive_shadows: true,
            motion_vectors: false,
        }
    ];

    let building_id = graphics_engine.scene_manager.add_object(
        None, // Root object
        building_transform,
        building_components,
    ).await?;

    info!("🏛️ Created building: {}", building_id);

    // Create pillars around the building
    for i in 0..8 {
        let angle = i as f32 * std::f32::consts::PI * 2.0 / 8.0;
        let pillar_pos = Vec3::new(angle.cos() * 6.0, 0.0, angle.sin() * 6.0);

        let pillar_transform = Transform {
            position: pillar_pos,
            rotation: Quat::IDENTITY,
            scale: Vec3::new(0.5, 4.0, 0.5),
            matrix: glam::Mat4::IDENTITY,
            is_dirty: true,
        };

        let pillar_components = vec![
            Component::MeshRenderer {
                mesh: Arc::new(create_cylinder_mesh()),
                materials: vec![Arc::new(create_stone_material())],
                cast_shadows: true,
                receive_shadows: true,
                motion_vectors: false,
            }
        ];

        let pillar_id = graphics_engine.scene_manager.add_object(
            Some(building_id),
            pillar_transform,
            pillar_components,
        ).await?;

        info!("🏛️ Created pillar {}: {}", i + 1, pillar_id);
    }

    // Create floating crystals with emissive materials
    for i in 0..4 {
        let angle = i as f32 * std::f32::consts::PI * 0.5;
        let crystal_pos = Vec3::new(
            angle.cos() * 3.0,
            4.0 + (angle * 2.0).sin() * 0.5,
            angle.sin() * 3.0,
        );

        let crystal_transform = Transform {
            position: crystal_pos,
            rotation: Quat::from_rotation_y(angle),
            scale: Vec3::splat(0.3),
            matrix: glam::Mat4::IDENTITY,
            is_dirty: true,
        };

        let crystal_components = vec![
            Component::MeshRenderer {
                mesh: Arc::new(create_crystal_mesh()),
                materials: vec![Arc::new(create_crystal_material())],
                cast_shadows: false,
                receive_shadows: false,
                motion_vectors: true, // They rotate
            }
        ];

        let crystal_id = graphics_engine.scene_manager.add_object(
            None,
            crystal_transform,
            crystal_components,
        ).await?;

        info!("💎 Created floating crystal {}: {}", i + 1, crystal_id);
    }

    Ok(())
}

/// Create demo terrain
async fn create_demo_terrain(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🏔️ Creating terrain");

    // Create a simple terrain heightmap
    let heightmap = create_heightmap_texture(256, 256);
    let terrain_texture = create_grass_texture();
    let detail_textures = vec![
        Arc::new(create_rock_texture()),
        Arc::new(create_dirt_texture()),
    ];

    let terrain_transform = Transform {
        position: Vec3::new(0.0, -2.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(50.0, 5.0, 50.0),
        matrix: glam::Mat4::IDENTITY,
        is_dirty: true,
    };

    let terrain_components = vec![
        Component::Terrain {
            heightmap: Arc::new(heightmap),
            detail_maps: detail_textures,
            splat_map: Arc::new(create_splat_map()),
            size: (256, 256),
        }
    ];

    let terrain_id = graphics_engine.scene_manager.add_object(
        None,
        terrain_transform,
        terrain_components,
    ).await?;

    info!("🏔️ Created terrain: {}", terrain_id);
    Ok(())
}

/// Create demo water surface
async fn create_demo_water(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🌊 Creating water surface");

    let water_transform = Transform {
        position: Vec3::new(15.0, -1.0, 0.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(10.0, 1.0, 10.0),
        matrix: glam::Mat4::IDENTITY,
        is_dirty: true,
    };

    let water_components = vec![
        Component::Water {
            surface_level: -1.0,
            flow_map: Some(Arc::new(create_flow_map())),
            normal_map: Arc::new(create_water_normal_map()),
            reflection_probe: None,
        }
    ];

    let water_id = graphics_engine.scene_manager.add_object(
        None,
        water_transform,
        water_components,
    ).await?;

    info!("🌊 Created water surface: {}", water_id);
    Ok(())
}

/// Create demo vegetation
async fn create_demo_vegetation(graphics_engine: &GraphicsEngine) -> Result<()> {
    info!("🌱 Creating vegetation");

    // Create grass patches
    for i in 0..5 {
        let grass_pos = Vec3::new(
            (i as f32 - 2.0) * 4.0,
            -1.8,
            8.0,
        );

        let grass_transform = Transform {
            position: grass_pos,
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(2.0),
            matrix: glam::Mat4::IDENTITY,
            is_dirty: true,
        };

        let grass_components = vec![
            Component::Vegetation {
                grass_texture: Arc::new(create_grass_texture()),
                density_map: Arc::new(create_density_map()),
                wind_strength: 0.5,
                lod_distances: vec![5.0, 15.0, 30.0],
            }
        ];

        let grass_id = graphics_engine.scene_manager.add_object(
            None,
            grass_transform,
            grass_components,
        ).await?;

        info!("🌱 Created grass patch {}: {}", i + 1, grass_id);
    }

    Ok(())
}

/// Main render function
async fn render_frame(graphics_engine: &GraphicsEngine, delta_time: f32) -> Result<()> {
    // Update time of day for dynamic lighting
    static mut TIME_OF_DAY: f32 = 12.0;
    unsafe {
        TIME_OF_DAY += delta_time * 0.1; // Slow time progression
        if TIME_OF_DAY > 24.0 {
            TIME_OF_DAY = 0.0;
        }
        graphics_engine.lighting_system.set_time_of_day(TIME_OF_DAY).await?;
    }

    // Render the frame
    graphics_engine.render_frame(delta_time).await?;

    Ok(())
}

// Helper functions to create demo content
fn create_cube_mesh() -> Mesh {
    let mut mesh = Mesh::new("Cube".to_string());
    
    // Add vertices for a simple cube (simplified)
    mesh.vertices = vec![
        // Front face
        Vertex { position: Vec3::new(-1.0, -1.0,  1.0), normal: Vec3::Z, tangent: Vec3::X, uv: Vec2::new(0.0, 0.0), color: [1.0; 4] },
        Vertex { position: Vec3::new( 1.0, -1.0,  1.0), normal: Vec3::Z, tangent: Vec3::X, uv: Vec2::new(1.0, 0.0), color: [1.0; 4] },
        Vertex { position: Vec3::new( 1.0,  1.0,  1.0), normal: Vec3::Z, tangent: Vec3::X, uv: Vec2::new(1.0, 1.0), color: [1.0; 4] },
        Vertex { position: Vec3::new(-1.0,  1.0,  1.0), normal: Vec3::Z, tangent: Vec3::X, uv: Vec2::new(0.0, 1.0), color: [1.0; 4] },
    ];
    
    // Add indices for two triangles (simplified)
    mesh.indices = vec![0, 1, 2, 2, 3, 0];
    
    mesh
}

fn create_cylinder_mesh() -> Mesh {
    Mesh::new("Cylinder".to_string()) // Placeholder
}

fn create_crystal_mesh() -> Mesh {
    Mesh::new("Crystal".to_string()) // Placeholder
}

fn create_stone_material() -> Material {
    Material::new("Stone".to_string())
}

fn create_crystal_material() -> Material {
    Material::new("Crystal".to_string())
}

fn create_heightmap_texture(width: u32, height: u32) -> Texture {
    Texture::new("Heightmap".to_string(), width, height)
}

fn create_grass_texture() -> Texture {
    Texture::new("Grass".to_string(), 512, 512)
}

fn create_rock_texture() -> Texture {
    Texture::new("Rock".to_string(), 512, 512)
}

fn create_dirt_texture() -> Texture {
    Texture::new("Dirt".to_string(), 512, 512)
}

fn create_splat_map() -> Texture {
    Texture::new("Splat Map".to_string(), 256, 256)
}

fn create_flow_map() -> Texture {
    Texture::new("Flow Map".to_string(), 256, 256)
}

fn create_water_normal_map() -> Texture {
    Texture::new("Water Normal".to_string(), 512, 512)
}

fn create_density_map() -> Texture {
    Texture::new("Density Map".to_string(), 128, 128)
}

// Import required types from scene module
use crate::scene::*;