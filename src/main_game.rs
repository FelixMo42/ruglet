pub mod ruglet;

use ruglet::prelude::*;

struct TestApp {
    mouse: Vec2,
    quads: Vec<Sprite>,
}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {
            mouse: Vec2 { x: 0.0, y: 0.0 },
            quads: vec![],
        };
    }
}

impl Window for TestApp {
    fn on_mouse_moved(&mut self, (x, y): (f32, f32)) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn on_mouse_down(&mut self, _button: winit::event::MouseButton) {
        self.quads.push(Sprite::new(
            self.mouse.x - 100.0,
            self.mouse.y - 100.0,
            self.mouse.x + 100.0,
            self.mouse.y + 100.0,
        ))
    }

    fn on_draw(&self, renderer: &mut Renderer) {
        for quad in &self.quads {
            renderer.draw(quad);
        }

        renderer.draw(&Sprite::new(
            self.mouse.x - 100.0,
            self.mouse.y - 100.0,
            self.mouse.x + 100.0,
            self.mouse.y + 100.0,
        ));
    }
}

fn main() {
    pollster::block_on(run(TestApp::new()));
}
