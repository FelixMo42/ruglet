use super::{renderer::Renderer, Frame};
use winit::{error::EventLoopError, event::*, event_loop::*, window::WindowBuilder};

pub trait Application {
    fn on_draw(&mut self, frame: &mut Frame);

    // Event functions
    fn on_mouse_scroll(&mut self, _dx: f32, _dy: f32) {}

    async fn run(&mut self) -> Result<(), EventLoopError> {
        // Initialize the window and event handler
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Manage our wgpu stuff
        let mut renderer = Renderer::new(&window).await;
        renderer.resize(window.inner_size());

        // Don't update if we don't have to
        event_loop.set_control_flow(ControlFlow::Wait);

        // Handle events as they come in
        use WindowEvent::*;
        return event_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.window().id() => match event {
                // Handle window events
                CloseRequested => control_flow.exit(),
                Resized(physical_size) => {
                    renderer.resize(*physical_size);
                    renderer.window().request_redraw();
                }
                RedrawRequested => {
                    let mut frame = Frame::new(renderer.size);
                    self.on_draw(&mut frame);
                    renderer.render(frame).unwrap();
                }

                // Handle user inputs
                MouseWheel { delta, .. } => match delta {
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.on_mouse_scroll(pos.x as f32, pos.y as f32);
                        renderer.window.request_redraw();
                    }
                    _ => println!("Unsupported scroll type!"),
                },

                // We don't care about the rest
                _ => {}
            },
            _ => {}
        });
    }
}
