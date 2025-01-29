use super::{renderer::Renderer, Frame};
use winit::{error::EventLoopError, event::*, event_loop::EventLoop, window::WindowBuilder};

pub trait Application {
    fn on_draw(&self, frame: &mut Frame);

    async fn run(&self) -> Result<(), EventLoopError> {
        // Initialize the window and event handler
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        // Manage our wgpu stuff
        let mut renderer = Renderer::new(&window).await;
        renderer.resize(window.inner_size());

        // Handle events as they come in
        use WindowEvent::*;
        return event_loop.run(move |event, control_flow| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == renderer.window().id() => match event {
                CloseRequested => control_flow.exit(),
                Resized(physical_size) => renderer.resize(*physical_size),
                RedrawRequested => {
                    let mut frame = Frame::new(renderer.size);
                    self.on_draw(&mut frame);
                    renderer.render(frame).unwrap();
                }
                _ => {}
            },
            _ => {}
        });
    }
}
