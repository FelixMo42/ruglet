mod ruglet;

use ruglet::*;

struct MyApp {}

impl MyApp {
    fn new() -> Self {
        return MyApp {};
    }
}

impl Application for MyApp {
    fn on_draw(&self) -> Vec<Vertex> {
        return vec![
            Vertex {
                position: [100., 100., 0.0],
                tex_coords: [0., 0.],
                color: [0.0, 0.5, 0.5],
            },
            Vertex {
                position: [100., 200.0, 0.0],
                tex_coords: [0.0868241, 0.00759614],
                color: [0.5, 0.0, 0.5],
            },
            Vertex {
                position: [200.0, 200.0, 0.0],
                tex_coords: [0.5868241, 0.99240386],
                color: [0.5, 0.0, 0.5],
            },
        ];
    }
}

fn main() {
    let app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
