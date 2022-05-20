pub use crate::ruglet::state::State;
pub use crate::ruglet::vertices::Vertex;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use wgpu::*;

pub trait Window {
    fn on_draw(&self) -> &[Vertex];

    fn on_key_down(&mut self, _chr: char) {}
    fn on_mouse_moved(&mut self, _cord: (f32, f32)) {}
    fn on_resize(&mut self, _size: (f32, f32)) {}
}

pub async fn run<App: 'static + Window>(mut app: App) {
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
                app.on_resize((size.width as f32, size.height as f32));
                window.request_redraw();
            }

            WindowEvent::CursorMoved { position, .. } => {
                app.on_mouse_moved((position.x as f32, position.y as f32));
                window.request_redraw();
            }

            WindowEvent::ReceivedCharacter(chr) => {
                app.on_key_down(*chr);
                window.request_redraw();
            }

            // ignore everything else
            _ => {}
        },

        Event::RedrawRequested(window_id) if window_id == window.id() => {
            match state.render(app.on_draw()) {
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
