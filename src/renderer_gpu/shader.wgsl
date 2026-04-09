// Fullscreen triangle — covers the entire clip space with 3 vertices.
// No vertex buffer needed; positions are computed from vertex_index.
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32)
    -> @builtin(position) vec4<f32> {

    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );

    return vec4<f32>(pos[vertex_index], 0.0, 1.0);
}

// UV gradient: red = X, green = Y, blue = constant 0.5
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>)
    -> @location(0) vec4<f32> {

    let uv = pos.xy / vec2<f32>(800.0, 600.0);

    return vec4<f32>(uv, 0.5, 1.0);
}
