pub mod state;

use std::collections::HashSet;
use std::sync::Arc;
use state::State;
use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{CursorGrabMode, WindowBuilder},
};

/// Recompute camera position and basis from orbit angles + distance.
/// Camera always looks at the origin (black hole location).
fn apply_orbit(cam: &mut state::Camera, yaw: f32, pitch: f32, dist: f32) {
    let (sy, cy) = yaw.sin_cos();
    let (sp, cp) = pitch.sin_cos();

    // Camera sits on a sphere of radius `dist` around origin
    cam.position = [dist * sy * cp, dist * sp, dist * cy * cp];

    // Forward = direction toward origin
    let fx = -sy * cp;
    let fy = -sp;
    let fz = -cy * cp;
    cam.forward = [fx, fy, fz];

    // right = normalize(cross(worldUp=[0,1,0], forward)) = normalize([fz, 0, -fx])
    let rx   = fz;
    let rz   = -fx;
    let rlen = (rx * rx + rz * rz).sqrt().max(0.0001);
    cam.right = [rx / rlen, 0.0, rz / rlen];

    // up = cross(forward, right) = [fy*rz - fz*0, fz*rx - fx*rz, fx*0 - fy*rx]
    let (rx, rz) = (cam.right[0], cam.right[2]);
    cam.up = [fy * rz, fz * rx - fx * rz, -fy * rx];
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Black Hole Renderer — Orbit Camera")
            .build(&event_loop)
            .unwrap(),
    );

    let mut state = State::new(Arc::clone(&window)).await;
    let mut held: HashSet<KeyCode> = HashSet::new();

    // Orbit parameters
    let mut yaw:   f32 = 0.0;
    let mut pitch: f32 = 0.35;  // above equator
    let mut dist:  f32 = 6.0;
    let mut mouse_captured = false;

    apply_orbit(&mut state.camera, yaw, pitch, dist);

    window.request_redraw();

    event_loop
        .run(move |event, target| match event {

            // ── Raw mouse delta ─────────────────────────────────────────────
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                if mouse_captured {
                    let sens = 0.004_f32;
                    yaw   += dx as f32 * sens;
                    pitch  = (pitch - dy as f32 * sens).clamp(-1.4, 1.4);
                    apply_orbit(&mut state.camera, yaw, pitch, dist);
                }
            }

            // ── Window events ────────────────────────────────────────────────
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => target.exit(),
                WindowEvent::Resized(s)     => state.resize(s),
                WindowEvent::ScaleFactorChanged { .. } => state.resize(window.inner_size()),

                // Click → capture cursor
                WindowEvent::MouseInput {
                    state: ElementState::Pressed,
                    button: MouseButton::Left, ..
                } => {
                    mouse_captured = true;
                    let _ = window.set_cursor_grab(CursorGrabMode::Locked)
                        .or_else(|_| window.set_cursor_grab(CursorGrabMode::Confined));
                    window.set_cursor_visible(false);
                }

                // Scroll wheel → zoom
                WindowEvent::MouseWheel { delta, .. } => {
                    let scroll = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y,
                        MouseScrollDelta::PixelDelta(p)   => p.y as f32 * 0.02,
                    };
                    dist = (dist - scroll * 0.5).clamp(1.5, 20.0);
                    apply_orbit(&mut state.camera, yaw, pitch, dist);
                }

                // Track held keys
                WindowEvent::KeyboardInput {
                    event: KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: key_state, ..
                    }, ..
                } => {
                    match key_state {
                        ElementState::Pressed  => { held.insert(key); }
                        ElementState::Released => { held.remove(&key); }
                    }
                    if key == KeyCode::Escape && key_state == ElementState::Pressed {
                        mouse_captured = false;
                        let _ = window.set_cursor_grab(CursorGrabMode::None);
                        window.set_cursor_visible(true);
                    }
                }

                // Every frame: apply held-key movement THEN render
                WindowEvent::RedrawRequested => {
                    let orbit = 0.025_f32;
                    let zoom  = 0.06_f32;
                    let mut dirty = false;

                    if held.contains(&KeyCode::KeyA) || held.contains(&KeyCode::ArrowLeft)  { yaw   -= orbit; dirty = true; }
                    if held.contains(&KeyCode::KeyD) || held.contains(&KeyCode::ArrowRight) { yaw   += orbit; dirty = true; }
                    if held.contains(&KeyCode::KeyW) || held.contains(&KeyCode::ArrowUp)    { dist   = (dist - zoom).clamp(1.5, 20.0); dirty = true; }
                    if held.contains(&KeyCode::KeyS) || held.contains(&KeyCode::ArrowDown)  { dist   = (dist + zoom).clamp(1.5, 20.0); dirty = true; }
                    if held.contains(&KeyCode::KeyQ) { pitch = (pitch + orbit).clamp(-1.4, 1.4); dirty = true; }
                    if held.contains(&KeyCode::KeyE) { pitch = (pitch - orbit).clamp(-1.4, 1.4); dirty = true; }

                    if dirty { apply_orbit(&mut state.camera, yaw, pitch, dist); }

                    state.render().unwrap();
                    window.request_redraw();
                }

                _ => {}
            },
            _ => {}
        })
        .unwrap();
}
