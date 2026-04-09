// Fullscreen triangle — no vertex buffer needed.
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

// Actual window resolution from Rust — no more hardcoding
@group(0) @binding(0)
var<uniform> resolution: vec2<f32>;

// Phase 4 (fixed): gravity bending with real resolution
@fragment
fn fs_main(@builtin(position) pos: vec4<f32>)
    -> @location(0) vec4<f32> {

    // NDC UV in [-1, 1], aspect-corrected
    let uv = (pos.xy / resolution) * 2.0 - 1.0;
    let aspect = resolution.x / resolution.y;
    let uv_corrected = vec2<f32>(uv.x * aspect, uv.y);

    var ray_origin = vec3<f32>(0.0, 0.0, -5.0);
    var ray_dir    = normalize(vec3<f32>(uv_corrected, 1.0));

    let bh_pos = vec3<f32>(0.0, 0.0, 0.0);

    var pos_ray = ray_origin;
    var dir     = ray_dir;

    for (var i = 0; i < 200; i = i + 1) {

        let r    = pos_ray - bh_pos;
        let dist = length(r);

        // Swallowed → black
        if (dist < 1.0) {
            return vec4<f32>(0.0, 0.0, 0.0, 1.0);
        }

        // 1/r³ inverse-cube bending — closer to relativistic photon deflection
        let r_dir    = normalize(-r);
        let strength = 1.0 / (dist * dist * dist);
        dir     = normalize(dir + r_dir * strength * 0.1);
        pos_ray = pos_ray + dir * 0.03;
    }

    // Escaped → white
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
