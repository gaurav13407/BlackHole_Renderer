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

    // Request the first redraw
    window.request_redraw();

    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                target.exit();
            }

            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                state.render().unwrap();
                window.request_redraw();
            }

            _ => {}
        })
        .unwrap();
}
