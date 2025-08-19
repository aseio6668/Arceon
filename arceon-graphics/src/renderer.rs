use anyhow::Result;
use wgpu::*;
use winit::window::Window;
use parking_lot::RwLock;
use tracing::info;

/// High-performance renderer supporting modern graphics features
pub struct Renderer {
    pub device: Device,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_config: RwLock<SurfaceConfiguration>,
    pub adapter: Adapter,
    pub render_targets: RwLock<RenderTargets>,
    pub render_passes: RwLock<RenderPasses>,
    pub pipelines: RwLock<RenderPipelines>,
    pub debug_info: RwLock<DebugInfo>,
}

/// Render targets for various rendering stages
#[derive(Debug)]
pub struct RenderTargets {
    pub main_color: TextureView,
    pub main_depth: TextureView,
    pub shadow_maps: Vec<TextureView>,
    pub reflection_probes: Vec<TextureView>,
    pub hdr_buffer: TextureView,
    pub bloom_buffers: Vec<TextureView>,
    pub ssao_buffer: Option<TextureView>,
    pub temporal_buffers: [TextureView; 2],
    pub current_temporal_index: usize,
}

/// Render pass configuration (without the actual descriptors that have lifetime issues)
#[derive(Debug)]
pub struct RenderPasses {
    // We'll create the actual RenderPassDescriptor instances when needed
    pub initialized: bool,
}

/// Compiled render pipelines
#[derive(Debug)]
pub struct RenderPipelines {
    pub geometry_pipeline: RenderPipeline,
    pub lighting_pipeline: RenderPipeline,
    pub shadow_pipeline: RenderPipeline,
    pub water_pipeline: RenderPipeline,
    pub terrain_pipeline: RenderPipeline,
    pub particle_pipeline: RenderPipeline,
    pub ui_pipeline: RenderPipeline,
    pub post_process_pipelines: Vec<RenderPipeline>,
}

/// Debug and performance information
#[derive(Debug, Default, Clone)]
pub struct DebugInfo {
    pub frame_count: u64,
    pub draw_calls: u32,
    pub vertices_rendered: u64,
    pub triangles_rendered: u64,
    pub texture_memory_mb: f32,
    pub buffer_memory_mb: f32,
    pub gpu_time_ms: f32,
    pub cpu_time_ms: f32,
    pub features_enabled: Vec<String>,
}

/// Frame data for rendering
pub struct Frame {
    pub surface_texture: SurfaceTexture,
    pub view: TextureView,
    pub encoder: CommandEncoder,
    pub timestamp: std::time::Instant,
}

impl Renderer {
    /// Create a new renderer  
    pub async fn new<'w>(window: &'w Window) -> Result<Self> {
        info!("🎨 Initializing Advanced Graphics Renderer");

        // Create instance with all backends
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // Create surface - use unsafe to extend lifetime to 'static
        // SAFETY: The caller must ensure the window lives for the entire duration of the renderer
        let surface = unsafe {
            let target = wgpu::SurfaceTargetUnsafe::from_window(&*window)?;
            instance.create_surface_unsafe(target)?
        };

        // Request adapter with high performance preference
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to find suitable adapter"))?;

        info!("🔧 Selected adapter: {}", adapter.get_info().name);

        // Get required features
        let mut required_features = Features::empty();
        required_features |= Features::MULTI_DRAW_INDIRECT;
        required_features |= Features::INDIRECT_FIRST_INSTANCE;
        required_features |= Features::TIMESTAMP_QUERY;
        required_features |= Features::PIPELINE_STATISTICS_QUERY;
        required_features |= Features::TEXTURE_COMPRESSION_BC;
        required_features |= Features::TEXTURE_COMPRESSION_ETC2;
        required_features |= Features::TEXTURE_COMPRESSION_ASTC;
        
        // Optional advanced features
        let adapter_features = adapter.features();
        if adapter_features.contains(Features::RAY_TRACING_ACCELERATION_STRUCTURE) {
            required_features |= Features::RAY_TRACING_ACCELERATION_STRUCTURE;
        }
        // Note: SHADER_FLOAT64 feature may not be available in current WGPU version
        // Commented out for compatibility
        // if adapter_features.contains(Features::SHADER_FLOAT64) {
        //     required_features |= Features::SHADER_FLOAT64;
        // }

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Arceon Graphics Device"),
                    required_features,
                    required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
                    // memory_hints: MemoryHints::Performance, // Field not available in current WGPU version
                },
                None,
            )
            .await?;

        // Configure surface
        let size = window.inner_size();
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: PresentMode::Fifo, // VSync enabled by default
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &surface_config);

        // Create render targets
        let render_targets = Self::create_render_targets(&device, &surface_config)?;

        // Create render passes
        let render_passes = Self::create_render_passes(&render_targets)?;

        // Create pipelines
        let pipelines = Self::create_pipelines(&device, &surface_config).await?;

        let mut debug_info = DebugInfo::default();
        debug_info.features_enabled = Self::get_enabled_features(&device);

        info!("✅ Graphics renderer initialized successfully");
        info!("   Surface format: {:?}", surface_format);
        info!("   Resolution: {}x{}", surface_config.width, surface_config.height);
        info!("   Features: {}", debug_info.features_enabled.join(", "));

        Ok(Self {
            device,
            queue,
            surface,
            surface_config: RwLock::new(surface_config),
            adapter,
            render_targets: RwLock::new(render_targets),
            render_passes: RwLock::new(render_passes),
            pipelines: RwLock::new(pipelines),
            debug_info: RwLock::new(debug_info),
        })
    }

    /// Begin rendering a frame
    pub async fn begin_frame(&self) -> Result<Frame> {
        let surface_texture = self
            .surface
            .get_current_texture()
            .map_err(|e| anyhow::anyhow!("Failed to get surface texture: {:?}", e))?;

        let view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Frame Encoder"),
            });

        let mut debug_info = self.debug_info.write();
        debug_info.frame_count += 1;
        debug_info.draw_calls = 0;
        debug_info.vertices_rendered = 0;
        debug_info.triangles_rendered = 0;

        Ok(Frame {
            surface_texture,
            view,
            encoder,
            timestamp: std::time::Instant::now(),
        })
    }

    /// End rendering a frame and present it
    pub async fn end_frame(&self, frame: Frame) -> Result<()> {
        let frame_time = frame.timestamp.elapsed().as_secs_f32() * 1000.0;

        // Submit commands to GPU
        self.queue.submit(std::iter::once(frame.encoder.finish()));

        // Present the frame
        frame.surface_texture.present();

        // Update debug info
        let mut debug_info = self.debug_info.write();
        debug_info.cpu_time_ms = frame_time;

        Ok(())
    }

    /// Resize the renderer
    pub async fn resize(&self, new_size: (u32, u32)) -> Result<()> {
        let (width, height) = new_size;
        if width == 0 || height == 0 {
            return Ok(()); // Skip zero-sized updates
        }

        info!("🔄 Resizing graphics context to: {}x{}", width, height);

        // Update surface configuration
        let mut surface_config = self.surface_config.write();
        surface_config.width = width;
        surface_config.height = height;
        self.surface.configure(&self.device, &surface_config);

        // Recreate render targets
        let new_render_targets = Self::create_render_targets(&self.device, &surface_config)?;
        *self.render_targets.write() = new_render_targets;

        // Recreate render passes
        let render_targets = self.render_targets.read();
        let new_render_passes = Self::create_render_passes(&render_targets)?;
        *self.render_passes.write() = new_render_passes;

        Ok(())
    }

    /// Create render targets for various rendering stages
    fn create_render_targets(device: &Device, config: &SurfaceConfiguration) -> Result<RenderTargets> {
        let width = config.width;
        let height = config.height;

        // Main color buffer (HDR)
        let main_color_texture = device.create_texture(&TextureDescriptor {
            label: Some("Main Color Buffer"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float, // HDR format
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let main_color = main_color_texture.create_view(&TextureViewDescriptor::default());

        // Main depth buffer
        let main_depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Main Depth Buffer"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let main_depth = main_depth_texture.create_view(&TextureViewDescriptor::default());

        // Shadow maps (cascade shadow maps)
        let mut shadow_maps = Vec::new();
        for i in 0..4 {
            let shadow_size = 2048 >> i; // Decreasing resolution for cascades
            let shadow_texture = device.create_texture(&TextureDescriptor {
                label: Some(&format!("Shadow Map {}", i)),
                size: Extent3d { width: shadow_size, height: shadow_size, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Depth32Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            shadow_maps.push(shadow_texture.create_view(&TextureViewDescriptor::default()));
        }

        // Reflection probes
        let mut reflection_probes = Vec::new();
        for i in 0..6 {
            let probe_texture = device.create_texture(&TextureDescriptor {
                label: Some(&format!("Reflection Probe {}", i)),
                size: Extent3d { width: 512, height: 512, depth_or_array_layers: 6 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            reflection_probes.push(probe_texture.create_view(&TextureViewDescriptor::default()));
        }

        // HDR buffer
        let hdr_texture = device.create_texture(&TextureDescriptor {
            label: Some("HDR Buffer"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba16Float,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let hdr_buffer = hdr_texture.create_view(&TextureViewDescriptor::default());

        // Bloom buffers (multiple resolutions)
        let mut bloom_buffers = Vec::new();
        for i in 0..5 {
            let scale = 1 << (i + 1);
            let bloom_texture = device.create_texture(&TextureDescriptor {
                label: Some(&format!("Bloom Buffer {}", i)),
                size: Extent3d { 
                    width: width / scale, 
                    height: height / scale, 
                    depth_or_array_layers: 1 
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            bloom_buffers.push(bloom_texture.create_view(&TextureViewDescriptor::default()));
        }

        // SSAO buffer (optional)
        let ssao_buffer = if device.features().contains(Features::SHADER_F64) {
            let ssao_texture = device.create_texture(&TextureDescriptor {
                label: Some("SSAO Buffer"),
                size: Extent3d { width: width / 2, height: height / 2, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::R8Unorm,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });
            Some(ssao_texture.create_view(&TextureViewDescriptor::default()))
        } else {
            None
        };

        // Temporal buffers for TAA
        let temporal_textures = [
            device.create_texture(&TextureDescriptor {
                label: Some("Temporal Buffer 0"),
                size: Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }),
            device.create_texture(&TextureDescriptor {
                label: Some("Temporal Buffer 1"),
                size: Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: TextureDimension::D2,
                format: TextureFormat::Rgba16Float,
                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            }),
        ];

        let temporal_buffers = [
            temporal_textures[0].create_view(&TextureViewDescriptor::default()),
            temporal_textures[1].create_view(&TextureViewDescriptor::default()),
        ];

        Ok(RenderTargets {
            main_color,
            main_depth,
            shadow_maps,
            reflection_probes,
            hdr_buffer,
            bloom_buffers,
            ssao_buffer,
            temporal_buffers,
            current_temporal_index: 0,
        })
    }

    /// Create render passes configuration  
    fn create_render_passes(_render_targets: &RenderTargets) -> Result<RenderPasses> {
        // Just mark as initialized - we'll create actual render passes when needed
        Ok(RenderPasses {
            initialized: true,
        })
    }

    /// Create render pipelines
    async fn create_pipelines(device: &Device, config: &SurfaceConfiguration) -> Result<RenderPipelines> {
        // Geometry pipeline (basic for now)
        let geometry_shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Geometry Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/geometry.wgsl").into()),
        });

        let geometry_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Geometry Pipeline"),
            layout: None,
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
                // compilation_options removed in current WGPU version
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Rgba16Float,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
                // compilation_options removed in current WGPU version
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            // cache: None, // Field removed in current WGPU version
        });

        // Create additional pipelines (simplified approach - reusing same shader for now)
        // In a real implementation, each would have unique shaders and configurations
        let lighting_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Lighting Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
                // compilation_options removed in current WGPU version
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                // compilation_options removed in current WGPU version
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            // cache: None, // Field removed in current WGPU version
        });

        // Create other pipelines (simplified - using same configuration)
        let shadow_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Shadow Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
                // compilation_options removed in current WGPU version
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
                // compilation_options removed in current WGPU version
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
            // cache: None, // Field removed in current WGPU version
        });

        // Create remaining pipelines with same pattern (simplified)
        let water_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Water Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main", 
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let terrain_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Terrain Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let particle_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Particle Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None, // Particles often don't use depth
            multisample: MultisampleState::default(),
            multiview: None,
        });

        let ui_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("UI Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None, // UI typically rendered without depth
            multisample: MultisampleState::default(),
            multiview: None,
        });

        // Create post-process pipelines
        let post_process_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Post Process Pipeline"),
            layout: None, // Simplified layout for demo purposes
            vertex: VertexState {
                module: &geometry_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &geometry_shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: TextureFormat::Bgra8UnormSrgb, // Common surface format
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });
        
        let post_process_pipelines = vec![post_process_pipeline];

        Ok(RenderPipelines {
            geometry_pipeline,
            lighting_pipeline,
            shadow_pipeline,
            water_pipeline,
            terrain_pipeline,
            particle_pipeline,
            ui_pipeline,
            post_process_pipelines,
        })
    }

    /// Get enabled features as strings
    fn get_enabled_features(device: &Device) -> Vec<String> {
        let mut features = Vec::new();
        let device_features = device.features();

        if device_features.contains(Features::MULTI_DRAW_INDIRECT) {
            features.push("Multi-Draw Indirect".to_string());
        }
        if device_features.contains(Features::TIMESTAMP_QUERY) {
            features.push("Timestamp Queries".to_string());
        }
        if device_features.contains(Features::TEXTURE_COMPRESSION_BC) {
            features.push("BC Texture Compression".to_string());
        }
        if device_features.contains(Features::RAY_TRACING_ACCELERATION_STRUCTURE) {
            features.push("Ray Tracing".to_string());
        }
        if device_features.contains(Features::SHADER_F64) {
            features.push("64-bit Shaders".to_string());
        }

        features
    }

    /// Get current debug information
    pub fn get_debug_info(&self) -> DebugInfo {
        (*self.debug_info.read()).clone()
    }

    /// Record draw call statistics
    pub fn record_draw_call(&self, vertex_count: u64) {
        let mut debug_info = self.debug_info.write();
        debug_info.draw_calls += 1;
        debug_info.vertices_rendered += vertex_count;
        debug_info.triangles_rendered += vertex_count / 3;
    }
}