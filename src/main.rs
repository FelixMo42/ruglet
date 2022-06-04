pub mod ruglet;

use ruglet::prelude::*;

struct TestApp {
    color: [f32; 3],
    vertices: Vec<Vertex>,
}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {
            color: [1.0, 1.0, 1.0],
            vertices: vec![],
        };
    }
}

fn pos((x, y): (f32, f32)) -> [f32; 3] {
    return [x, y, 0.0];
}

impl Window for TestApp {
    fn on_mouse_moved(&mut self, (x, y): (f32, f32)) {
        self.vertices = vec![
            Vertex {
                position: pos((x - 100.0, y - 100.0)),
                tex_coords: [0.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: pos((x - 100.0, y + 100.0)),
                tex_coords: [0.0, 1.0],
                color: self.color.clone(),
            },
            Vertex {
                position: pos((x + 100.0, y - 100.0)),
                tex_coords: [1.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: pos((x + 100.0, y + 100.0)),
                tex_coords: [1.0, 1.0],
                color: self.color.clone(),
            },
            Vertex {
                position: pos((x + 100.0, y - 100.0)),
                tex_coords: [1.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: pos((x - 100.0, y + 100.0)),
                tex_coords: [0.0, 1.0],
                color: self.color.clone(),
            },
        ];
    }

    fn on_draw(&self) -> &[Vertex] {
        return &self.vertices;
    }
}

fn main() {
    pollster::block_on(run(TestApp::new()));
}
