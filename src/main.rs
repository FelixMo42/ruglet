pub mod layout;
pub mod ruglet;

use layout::prelude::*;
use ruglet::prelude::*;

struct TestApp {
    mouse: Vec2,
    root: Div,
}

struct Div {
    style: Style,
    children: Vec<Div>,
}

impl Div {
    fn render(&self, renderer: &mut Renderer, area: &Area) {
        let w = self.style.width.unwrap_or(area.w);
        let h = self.style.height.unwrap_or(area.h);

        renderer.quad(&Sprite::new(area.x, area.y, w, h));

        let mut new_area = area.clone();

        if let Some(padding) = self.style.padding {
            new_area = new_area.pad(padding);
        }

        for child in &self.children {
            child.render(renderer, &new_area);
        }
    }
}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {
            mouse: Vec2 { x: 0.0, y: 0.0 },
            root: Div {
                style: Style::new().pad(50.),
                children: vec![Div {
                    style: Style::new(),
                    children: vec![],
                }],
            },
        };
    }
}

impl Window for TestApp {
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
            w: 500.0,
            h: 500.0,
        };

        self.root.render(renderer, &area);
    }
}

fn main() {
    pollster::block_on(run(TestApp::new()));
}
