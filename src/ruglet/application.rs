use winit::{error::EventLoopError, event::*, event_loop::EventLoop, window::WindowBuilder};

use super::{renderer::Renderer, Vertex};

pub trait Application {
    fn on_draw(&self) -> Vec<Vertex>;

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
                RedrawRequested => renderer.render(self.on_draw()).unwrap(),
                _ => {}
            },
            _ => {}
        });
    }
}
