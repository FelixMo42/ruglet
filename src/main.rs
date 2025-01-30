mod layout;
mod ruglet;

use fontdue::*;

use layout::*;
use ruglet::*;

struct MyApp {
    root: Div,
    font: Font,
}

impl MyApp {
    fn new() -> Self {
        // Load font
        let font = include_bytes!("../res/OpenSans-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();

        // Construct Dom
        let root = Div::new().pad(50.).bg([1., 1., 1.]).children(vec![
            Div::new().size(200.0, 200.0).bg([0.0, 0.5, 0.5]),
            Div::new().size(150.0, 150.0).bg([0.5, 0.5, 0.0]),
            Div::new().size(300.0, 300.0).bg([0.5, 0.0, 0.5]),
        ]);

        return MyApp { root, font };
    }
}

impl Application for MyApp {
    fn on_draw(&self, frame: &mut Frame) {
        // Rasterize and get the layout metrics for the letter 'g' at 17px.
        let (metrics, bitmap) = self.font.rasterize('g', 500.0);
        let mut rgba = vec![0xff; bitmap.len() * 4];
        for (i, &a) in bitmap.iter().enumerate() {
            rgba[i * 4 + 0] = a;
            rgba[i * 4 + 1] = a;
            rgba[i * 4 + 2] = a;
            rgba[i * 4 + 3] = a;
        }
        frame.set_texture(rgba, (metrics.width as u32, metrics.height as u32));

        // Render the dom
        self.root.render(frame, &frame.area.clone());
    }
}

fn main() {
    let app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
