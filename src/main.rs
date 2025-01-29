pub mod layout;
pub mod ruglet;

use layout::prelude::*;
use ruglet::prelude::*;

struct TestApp {
    mouse: Vec2,
    size: Vec2,
    root: Div,
}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {
            size: Vec2 { x: 0.0, y: 0.0 },
            mouse: Vec2 { x: 0.0, y: 0.0 },
            root: Div::new().pad(50.).bg([1., 1., 1.]).children(vec![
                Div::new().size(200.0, 200.0).bg([0.0, 0.5, 0.5]),
                Div::new().size(150.0, 150.0).bg([0.5, 0.5, 0.]),
            ]),
        };
    }
}

impl Window for TestApp {
    fn on_resize(&mut self, (w, h): (f32, f32)) {
        self.size.x = w;
        self.size.y = h;
    }

    fn on_mouse_moved(&mut self, (x, y): (f32, f32)) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn on_mouse_down(&mut self, _button: winit::event::MouseButton) {
        // self.quads.push(Sprite::new(
        //     self.mouse.x - 100.0,
        //     self.mouse.y - 100.0,
        //     self.mouse.x + 100.0,
        //     self.mouse.y + 100.0,
        // ))
    }

    fn on_draw(&self, renderer: &mut Renderer) {
        let area = Area {
            x: 0.0,
            y: 0.0,
            w: self.size.x,
            h: self.size.y,
        };

        self.root.render(renderer, &area);
    }
}

fn main() {
    pollster::block_on(run(TestApp::new()));
}
