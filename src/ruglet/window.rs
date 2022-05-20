pub use crate::ruglet::state::State;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu::*;

pub trait Window {
    fn on_key_down(&self) {}

    fn on_draw(&self) {}
}

pub async fn run<App: Window>(app: App) {
    //
    let event_loop = EventLoop::new();

    //
    let window = WindowBuilder::new()
        .with_title("title")
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window).await;

    //
    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            // Match the window event if its out id matches our window.
            // This should alwais be the case. window_id is used if you
            // spawn multiple windows.

            // close handlers
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }

            // resize handlers
            WindowEvent::Resized(size) => {
                state.resize(*size);
                window.request_redraw();
            }

            // ignore everything else
            _ => {}
        },

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            match state.render() {
                Ok(_) => {}

                // Reconfigure the surface if lost
                Err(SurfaceError::Lost) => state.resize(state.size),

                // The system is out of memory, we should probably quit
                Err(SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        _ => {}
    })
}
