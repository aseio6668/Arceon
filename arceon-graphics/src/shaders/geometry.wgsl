// Basic geometry shader for Arceon graphics rendering

// Vertex shader
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    // Simple fullscreen triangle
    let x = f32(vertex_index / 2u) * 4.0 - 1.0;
    let y = f32(vertex_index % 2u) * 4.0 - 1.0;
    return vec4<f32>(x, y, 0.0, 1.0);
}

// Fragment shader
@fragment 
fn fs_main(@builtin(position) position: vec4<f32>) -> @location(0) vec4<f32> {
    // Simple gradient based on position
    let color = vec3<f32>(position.x / 800.0, position.y / 600.0, 0.5);
    return vec4<f32>(color, 1.0);
}
