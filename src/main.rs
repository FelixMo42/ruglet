mod layout;
mod ruglet;

use layout::*;
use ruglet::*;

struct MyApp {
    root: Div,
}

impl MyApp {
    fn new() -> Self {
        return MyApp {
            root: Div::new().pad(50.).bg([1., 1., 1.]).children(vec![
                Div::new().size(200.0, 200.0).bg([0.0, 0.5, 0.5]),
                Div::new().size(150.0, 150.0).bg([0.5, 0.5, 0.0]),
                Div::new().size(300.0, 300.0).bg([0.5, 0.0, 0.5]),
            ]),
        };
    }
}

impl Application for MyApp {
    fn on_draw(&self, frame: &mut Frame) {
        self.root.render(frame, &frame.area.clone());
    }
}

fn main() {
    let app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
