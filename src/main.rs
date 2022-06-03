pub mod ruglet;

use ruglet::prelude::*;

struct Camera2D {
    scale_x: f32,
    scale_y: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Camera2D {
    fn new() -> Camera2D {
        return Camera2D {
            scale_x: 1.0,
            scale_y: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        };
    }

    fn project(&self, (x, y): (f32, f32)) -> [f32; 3] {
        return [
            (x + self.offset_x) * self.scale_x,
            (y + self.offset_y) * self.scale_y,
            0.0,
        ];
    }
}

struct TestApp {
    color: [f32; 3],
    camera: Camera2D,
    vertices: Vec<Vertex>,
}

impl TestApp {
    fn new() -> TestApp {
        return TestApp {
            color: [0.0, 1.0, 0.0],
            camera: Camera2D::new(),
            vertices: vec![],
        };
    }
}

impl Window for TestApp {
    fn on_resize(&mut self, (w, h): (f32, f32)) {
        self.camera.scale_x = 2.0 / w;
        self.camera.scale_y = 2.0 / h;
        self.camera.offset_x = -w / 2.0;
        self.camera.offset_y = h / 2.0;
    }

    fn on_mouse_moved(&mut self, (x, y): (f32, f32)) {
        self.vertices = vec![
            Vertex {
                position: self.camera.project((x, -y + 100.0)),
                tex_coords: [0.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: self.camera.project((x - 100.0, -y - 100.0)),
                tex_coords: [1.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: self.camera.project((x + 100.0, -y - 100.0)),
                tex_coords: [0.0, 1.0],
                color: self.color.clone(),
            },
        ];
    }

    fn on_key_down(&mut self, _chr: char) {
        self.vertices = vec![
            Vertex {
                position: [0.0, 0.5, 0.0],
                tex_coords: [0.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                tex_coords: [1.0, 0.0],
                color: self.color.clone(),
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
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
