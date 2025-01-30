mod layout;
mod ruglet;

use layout::*;
use ruglet::*;

struct MyApp {
    root: Div,
    font: FontAtlas,
}

impl MyApp {
    fn new(text: &str) -> Self {
        let mut font = FontAtlas::new();

        let divs: Vec<Div> = text
            .chars()
            .map(|c| {
                let texture = font.get(c, 200.0);
                let metrics = font.metrics(texture);

                let baseline = 175.0;
                let hoffset = baseline - (metrics.ymin as f32) - metrics.height as f32;

                Div::new()
                    .size(metrics.advance_width, 200.0)
                    .children(vec![Div::new()
                        .size(metrics.width as f32, metrics.height as f32)
                        .bg([0.0, 0.5, 0.5])
                        .offset(Vec2::new(metrics.xmin as f32, hoffset))
                        .texture(texture)])
            })
            .collect();

        let root = Div::new().pad(50.).bg([1., 1., 1.]).children(divs);

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
    let text = "Ruglet is cool!";

    let mut app = MyApp::new(text);

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
