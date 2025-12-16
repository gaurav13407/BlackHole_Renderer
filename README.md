# Black Hole Renderer

A GPU-accelerated black hole visualization system using Rust and WebGPU, featuring physically accurate ray tracing through curved spacetime around a Schwarzschild black hole.

![Version](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-green)

## Features

- ✨ **Physically Accurate**: Implements Schwarzschild metric for realistic spacetime curvature
- 🚀 **GPU Accelerated**: Uses WebGPU compute shaders for real-time ray tracing
- 🔬 **RK4 Integration**: Fourth-order Runge-Kutta numerical integration for geodesic equations
- 💻 **CPU Reference**: Includes CPU-based renderer for validation and image generation
- 🎨 **Multiple Modes**: Real-time GPU rendering and offline CPU image generation

## Project Structure

```
blackhole_renderer/
├── src/
│   ├── lib.rs              # Library exports
│   ├── main.rs             # Real-time GPU renderer
│   ├── physics/            # Physics simulation
│   │   ├── mod.rs
│   │   ├── geodesic.rs     # Geodesic integration (RK4)
│   │   ├── integration.rs  # (Planned)
│   │   └── schwarzschild.rs # (Planned)
│   ├── gpu/                # GPU modules
│   │   ├── compute.rs      # (Planned)
│   │   └── state.rs        # (Planned)
│   └── renderer/           # Rendering modules
│       ├── camera.rs       # (Planned)
│       ├── cpu_reference.rs # (Planned)
│       └── scene.rs        # (Planned)
├── examples/
│   ├── ray_tracing.rs      # Terminal-based ray tracing demo
│   └── cpu_image.rs        # CPU-based image renderer
├── shaders/
│   ├── trace.wgsl          # Ray tracing compute shader
│   └── fullscreen.wgsl     # Fullscreen rendering shader
└── docs/
    └── project_documentation.md  # Comprehensive documentation
```

## Quick Start

### Prerequisites

- Rust (1.75+)
- GPU with Vulkan/DirectX 12/Metal support

### Installation

```bash
git clone https://github.com/yourusername/blackhole_renderer.git
cd blackhole_renderer
cargo build --release
```

### Running

#### Real-Time GPU Renderer
```bash
cargo run --release
```

#### CPU Image Generation
```bash
cargo run --release --example cpu_image
```
This generates `blackhole.ppm` - a 800×800 image of the black hole.

#### Ray Tracing Demo
```bash
cargo run --example ray_tracing
```
Watch a light ray spiral into the black hole in your terminal!

## Examples

### 1. **ray_tracing.rs** - Terminal Ray Simulation
Simulates a single photon trajectory around a black hole, showing:
- Radial distance (r) over time
- Angular position (phi)
- Whether the ray hits the event horizon or escapes

### 2. **cpu_image.rs** - Black Hole Image Generator
Generates a PPM image showing:
- Event horizon (black circle)
- Gravitational lensing effects
- Background gradient distorted by spacetime curvature

**Parameters you can adjust:**
- Image resolution (default: 800×800)
- Camera distance (default: r=20.0)
- Field of view
- Integration step size
- Maximum ray steps

## Physics Background

### Schwarzschild Metric
The renderer simulates light propagation using the Schwarzschild solution to Einstein's field equations:

$$ds^2 = -\left(1 - \frac{2M}{r}\right)dt^2 + \left(1 - \frac{2M}{r}\right)^{-1}dr^2 + r^2d\Omega^2$$

### Geodesic Integration
Light rays follow null geodesics, integrated using the RK4 method:
- **Step size**: 0.01 (configurable)
- **Order**: 4th order accuracy
- **Conserved quantities**: Energy (E) and angular momentum (L)

### Key Physics Features
- **Event Horizon**: r = 2.0 (in geometric units where M=1)
- **Photon Sphere**: r = 3.0 (unstable orbits)
- **Impact Parameter**: Determines ray trajectory based on initial conditions

## Development

### Building from Source
```bash
# Debug build
cargo build

# Release build (faster)
cargo build --release

# Run tests
cargo test

# Run specific example
cargo run --example <example_name>
```

### Adding New Examples
1. Create file in `examples/` directory
2. Import library: `use blackhole_renderer::physics::geodesic::*;`
3. Add to `Cargo.toml` if needed (optional for simple examples)

### Module Organization
- Add public modules to `src/lib.rs`
- Each module directory needs a `mod.rs` file
- Use `pub` keyword to export types/functions

## Dependencies

```toml
winit = "0.29"      # Window creation
wgpu = "0.19"       # WebGPU API
glam = "0.25"       # Math library
image = "0.24"      # Image I/O
anyhow = "1.0"      # Error handling
pollster = "0.3"    # Async runtime
```

## Roadmap

### Phase 1: Core Physics ✅
- [x] Schwarzschild geodesic equations
- [x] RK4 numerical integrator
- [x] CPU reference implementation
- [ ] Christoffel symbol calculations
- [ ] Adaptive step sizing

### Phase 2: GPU Ray Tracer 🚧
- [x] Basic compute pipeline
- [x] Storage texture output
- [ ] Physics in WGSL shader
- [ ] Camera ray generation
- [ ] Background environment

### Phase 3: Visual Features 📋
- [ ] Accretion disk geometry
- [ ] Doppler shifting
- [ ] Gravitational redshift
- [ ] Procedural star field

### Phase 4: Interactivity 📋
- [ ] Camera controls
- [ ] Parameter adjustment UI
- [ ] Performance metrics
- [ ] Screenshot capability

## Documentation

Comprehensive documentation available in:
- [docs/project_documentation.md](docs/project_documentation.md) - Full technical documentation
- Inline code comments
- Example programs with detailed explanations

## Contributing

Contributions welcome! Areas of interest:
- Physics accuracy improvements
- Performance optimizations
- Visual effects
- Documentation
- Bug fixes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- WebGPU team for excellent graphics API
- Rust community for amazing tools and libraries
- General relativity researchers for theoretical foundations

## References

1. **Schwarzschild, K.** (1916). "On the Gravitational Field of a Mass Point"
2. **Chandrasekhar, S.** (1983). "The Mathematical Theory of Black Holes"
3. **Marck, J.A.** (1996). "Short-cut method of solution of geodesic equations"

---

**Status**: Active Development  
**Version**: 0.1.0  
**Date**: December 2025
