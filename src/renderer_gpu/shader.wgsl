// Fullscreen triangle — no vertex buffer.
@vertex
fn vs_main(@builtin(vertex_index) vi: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );
    return vec4<f32>(pos[vi], 0.0, 1.0);
}

struct Camera {
    position:   vec3<f32>,
    forward:    vec3<f32>,
    right:      vec3<f32>,
    up:         vec3<f32>,
    resolution: vec2<f32>,
}

@group(0) @binding(0) var<uniform> camera: Camera;
@group(0) @binding(1) var<uniform> time:   f32;

// ── Smooth value noise ────────────────────────────────────────────────────────
fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}
fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);   // smoothstep
    return mix(
        mix(hash2(i),                  hash2(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash2(i + vec2<f32>(0.0, 1.0)), hash2(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

// ── Blackbody colour: maps temperature 0‥1 → red/orange/yellow/white ─────────
fn blackbody(t: f32) -> vec3<f32> {
    let r = clamp(t * 1.5,         0.0, 1.0);
    let g = clamp(t * 1.5 - 0.3,  0.0, 1.0);
    let b = clamp(t * 3.0 - 2.0,  0.0, 1.0);
    return vec3<f32>(r, g * g, b * b);
}

// ── Disk colour: physically rotating coordinates ─────────────────────────────
fn disk_emission(r: f32, theta: f32, doppler: f32) -> vec3<f32> {
    // Keplerian speed: inner disk faster (v ∝ 1/√r, simplified as 3/r)
    let speed     = 3.0 / (r + 0.3);
    let rot_theta = theta + time * speed;

    // Rotate the sampling position in XZ — features PHYSICALLY ORBIT the BH
    let x_rot = cos(rot_theta) * r;
    let z_rot = sin(rot_theta) * r;

    // Temperature: white-hot inner edge, orange-red outer
    let temp  = clamp(1.0 - (r - 1.0) / 5.0, 0.0, 1.0);
    let temp2 = temp * temp;

    // Turbulence on the ROTATED position — plasma streamers that orbit
    let noise      = sin(x_rot * 20.0 + z_rot * 20.0);
    let turbulence = 0.6 + 0.4 * noise;

    // Multi-scale gas structure (value noise still useful for volume detail)
    let ruv = vec2<f32>(x_rot * 0.5, z_rot * 0.5);
    let n1  = vnoise(ruv * 4.0);
    let n2  = vnoise(ruv * 10.0 + vec2<f32>(1.7, 0.9));
    let gas = 0.55 + 0.45 * (n1 * 0.6 + n2 * 0.4);

    // Brightness: 1/r² × gas × turbulence × radial falloff
    let brightness = (1.0 / (r * r)) * gas * turbulence * exp(-r * 0.25);

    var col = blackbody(temp2) * brightness;

    // Relativistic beaming: power-4 (one side blazes)
    let beam = clamp(1.0 + doppler, 0.2, 4.0);
    col *= beam * beam * beam * beam;

    // Photon ring: white-hot spike at r ≈ 1.5 (photon sphere)
    let ring = exp(-abs(r - 1.5) * 8.0);
    col     += vec3<f32>(1.0, 0.92, 0.7) * ring * 1.5;

    return col;
}


// ── Sparse clean starfield sampled from a direction ───────────────────────────
fn stars(dir: vec3<f32>) -> vec3<f32> {
    let d   = normalize(dir);
    let h1  = fract(sin(dot(d.xy + d.z * 0.37, vec2<f32>(12.9898, 78.233))) * 43758.5453);
    let h2  = fract(sin(dot(d.yz + d.x * 0.53, vec2<f32>(93.989,  47.765))) * 31415.927);
    // Very sparse: only top 0.4% of hash values light up
    let s1  = step(0.996, h1);
    let s2  = step(0.994, h2) * 0.4;
    // Slightly warm vs cool
    return vec3<f32>(s1 + s2 * 1.1, s1 + s2 * 0.9, s1 + s2 * 0.7);
}

@fragment
fn fs_main(@builtin(position) frag_pos: vec4<f32>) -> @location(0) vec4<f32> {

    let uv  = (frag_pos.xy / camera.resolution) * 2.0 - 1.0;
    let asp = camera.resolution.x / camera.resolution.y;

    var ray_pos = camera.position;
    var ray_dir = normalize(camera.forward
        + uv.x * asp * camera.right
        + uv.y       * camera.up);

    // Accumulated disk light — ray passes THROUGH disk, collecting glow
    var color = vec3<f32>(0.0);

    // ── Raymarching loop ──────────────────────────────────────────────────────
    for (var i = 0; i < 400; i = i + 1) {

        let r = length(ray_pos);

        // Softened GR gravity
        let gravity = -ray_pos / (r * r * r + 0.001);
        ray_dir = normalize(ray_dir + gravity * 0.025);

        let jitter = 1.0 + 0.15 * fract(sin(f32(i) * 127.1) * 43758.5);
        ray_pos += ray_dir * (0.025 * jitter);

        // Event horizon — return whatever we've accumulated so far
        if (r < 1.0) {
            return vec4<f32>(color, 1.0);
        }

        // ── Volumetric accretion disk ─────────────────────────────────────────
        let disk_h = abs(ray_pos.y);
        if (disk_h < 0.3 && r > 1.1 && r < 8.0) {

            let theta = atan2(ray_pos.z, ray_pos.x);

            // Still thin but slightly softer → plasma visible all around
            let density = exp(-disk_h * 18.0);

            // Keplerian turbulence on rotated position
            let speed     = 3.0 / (r + 0.3);
            let rot_theta = theta + time * speed;
            let x_rot     = cos(rot_theta) * r;
            let z_rot     = sin(rot_theta) * r;
            let tnoise    = sin(x_rot * 15.0 + z_rot * 15.0);
            let turbulence = 0.7 + 0.3 * tnoise;

            // Radial falloff
            let radial = exp(-((r - 2.0) * (r - 2.0)) * 0.5);

            // Base disk emission
            let tangent = normalize(vec3<f32>(-ray_pos.z, 0.0, ray_pos.x));
            let doppler = dot(ray_dir, tangent);
            var col = disk_emission(r, theta, doppler);
            col    *= radial * turbulence;

            // Doppler boost: softer so dark side still glows
            let boost = clamp(1.0 + doppler * 1.8, 0.25, 4.0);
            col      *= boost;

            color += col * density * 0.012;

            // ── Inner bright ring (photon ring / ISCO edge) ───────────────────
            let ring_inner = smoothstep(1.5, 1.0, r);
            let ring_outer = smoothstep(6.0, 2.0, r);
            let ring       = ring_inner * ring_outer;
            color += vec3<f32>(3.0, 2.0, 1.5) * ring * density * 0.006;
        }
    }

    // ── Post-loop tone pipeline ───────────────────────────────────────────────
    // 1. Hard clamp to prevent bloom going infinite
    color = clamp(color, vec3<f32>(0.0), vec3<f32>(4.0));

    // 2. Subtle bloom on already-clamped value
    let lum   = dot(color, vec3<f32>(0.2126, 0.7152, 0.0722));
    color    += vec3<f32>(1.0, 0.85, 0.6) * (lum * lum) * 0.12;

    // 3. Reinhard tone mapping (denominator 1.0 = standard)
    color = color / (color + vec3<f32>(1.0));

    // 4. Gamma correction 1/2.2
    color = pow(max(color, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.2));

    // ── Inner glow + starfield ────────────────────────────────────────────────
    let r_final  = length(ray_pos);
    let inner    = exp(-r_final * 2.5);
    color       += vec3<f32>(1.0, 0.9, 0.7) * inner * 0.25;

    let bg_stars = stars(ray_dir) + vec3<f32>(0.0012, 0.0015, 0.003);
    let magnif   = 1.0 + 0.6 / (r_final * r_final + 1.5);

    return vec4<f32>(clamp(bg_stars * magnif + color, vec3<f32>(0.0), vec3<f32>(1.0)), 1.0);
}
