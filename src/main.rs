mod layout;
mod ruglet;

use layout::*;
use ruglet::*;

struct MyApp {
    root: Div,
    font: FontAtlas,
}

impl MyApp {
    fn new() -> Self {
        let mut font = FontAtlas::new();

        let root = Div::new().pad(50.).bg([1., 1., 1.]).children(vec![
            Div::new()
                .size(200.0, 200.0)
                .bg([0.0, 0.5, 0.5])
                .texture(font.get('R', 20.0)),
            Div::new()
                .size(200.0, 200.0)
                .bg([0.5, 0.5, 0.0])
                .texture(font.get('u', 20.0)),
            Div::new()
                .size(200.0, 200.0)
                .bg([0.5, 0.0, 0.5])
                .texture(font.get('g', 20.0)),
            Div::new()
                .size(200.0, 200.0)
                .bg([0.0, 0.0, 0.5])
                .texture(font.get('l', 20.0)),
            Div::new()
                .size(200.0, 200.0)
                .bg([0.0, 0.5, 0.0])
                .texture(font.get('e', 20.0)),
            Div::new()
                .size(200.0, 200.0)
                .bg([0.0, 0.5, 0.0])
                .texture(font.get('t', 20.0)),
        ]);

        return MyApp { root, font };
    }
}

impl Application for MyApp {
    fn on_draw(&mut self, frame: &mut Frame) {
        // Update the texture if new glyphs have been added to the font atlas
        if self.font.texture_changed() {
            frame.set_texture(self.font.build_texture());
        }

        // Render the dom
        self.root.render(frame, &frame.area.clone(), &self.font);
    }
}

fn main() {
    let mut app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
