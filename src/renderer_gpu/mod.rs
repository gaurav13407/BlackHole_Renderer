pub mod state;

use std::sync::Arc;
use state::State;
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let window = Arc::new(
        WindowBuilder::new()
            .with_title("Black Hole Renderer — Phase 1")
            .build(&event_loop)
            .unwrap(),
    );

    let mut state = State::new(Arc::clone(&window)).await;

    window.request_redraw();

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    target.exit();
                }

                // Resize surface + update resolution uniform
                WindowEvent::Resized(new_size) => {
                    state.resize(new_size);
                }

                // HiDPI / scale change — query actual new size from window
                WindowEvent::ScaleFactorChanged { .. } => {
                    state.resize(window.inner_size());
                }

                WindowEvent::RedrawRequested => {
                    state.render().unwrap();
                    window.request_redraw();
                }

                _ => {}
            },

            _ => {}
        })
        .unwrap();
}
