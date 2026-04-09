# 🕳️ BlackHole Renderer

> Real-time GPU black hole renderer with gravitational lensing, accretion disk simulation, and interactive orbit camera — built with Rust + WGPU.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![WGPU](https://img.shields.io/badge/WGPU-0.19-blue)
![License](https://img.shields.io/badge/License-MIT-green)

---

## ✨ Features

- **Schwarzschild geodesic ray marching** — photon paths bent by GR gravity in real time
- **Volumetric accretion disk** — Keplerian differential rotation, plasma turbulence, orbital streaks
- **Relativistic Doppler beaming** — approaching side blazes, receding side dims
- **Photon ring** — sharp bright spike at the innermost stable photon sphere (r ≈ 1.5)
- **Einstein lensing** — stars behind the BH warp into arcs; secondary disk image wraps over the top
- **Procedural starfield** — 3-layer lensed starfield, sampled from the final bent ray direction
- **Interactive orbit camera** — orbit, zoom, and mouse-look around the black hole
- **Filmic tone mapping** — Reinhard + gamma correction, no blowout

---

## 🚀 Quick Start

```bash
git clone https://github.com/gaurav13407/BlackHole_Renderer
cd BlackHole_Renderer
cargo run
```

Requires a GPU with Vulkan / Metal / DX12 support.

---

## 🎮 Controls

| Input | Action |
|---|---|
| **Left-click** | Capture mouse |
| **Mouse drag** | Orbit around black hole |
| **Scroll wheel** | Zoom in / out |
| **W / S** | Zoom in / out (keyboard) |
| **A / D** | Orbit left / right |
| **Q / E** | Orbit up / down |
| **Escape** | Release mouse |

---

## 🔬 Physics

| Parameter | Value | Meaning |
|---|---|---|
| Schwarzschild radius | `r = 1.0` | Event horizon |
| Photon sphere | `r = 1.5` | Innermost unstable photon orbit |
| Disk inner edge | `r = 1.1` | Near ISCO |
| Disk outer edge | `r = 8.0` | Outer accretion region |
| Integration steps | 400 | Per pixel, at dt ≈ 0.025 |

Gravity formula (Schwarzschild null geodesic approximation):
```
a = -ray_pos / (r³ + ε)
```

Keplerian orbital speed used for disk turbulence:
```
speed = 3.0 / (r + 0.3)   →   inner plasma rotates faster
```

---

## 🏗️ Architecture

```
src/
├── main.rs                  Entry point
└── renderer_gpu/
    ├── mod.rs               Event loop, orbit camera, input handling
    ├── state.rs             WGPU state, uniform buffers, render pipeline
    └── shader.wgsl          Fragment shader — all physics + visuals
```

**Uniforms sent to GPU each frame:**
- `Camera` (80 bytes) — position, forward, right, up, resolution
- `time` (f32) — drives plasma animation and Keplerian rotation

---

## 📸 Gallery

> Run the renderer and orbit around with your mouse for the best view.

---

## 📄 License

MIT © [Gaurav](https://github.com/gaurav13407)
