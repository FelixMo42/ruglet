mod layout;
mod ruglet;

use fontdue::*;

use layout::*;
use ruglet::*;

struct FontAtlas {
    font: Font,
    changed: bool,

    // Glyph info
    glyphsi: Vec<(char, f32, usize)>,
    bitmaps: Vec<Vec<u8>>,
    metrics: Vec<Metrics>,
    texarea: Vec<Area>,
}

impl FontAtlas {
    fn new() -> Self {
        // Load font
        let font = include_bytes!("../res/OpenSans-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();

        return FontAtlas {
            font,
            changed: false,

            // Glyph info
            glyphsi: vec![],
            bitmaps: vec![],
            metrics: vec![],
            texarea: vec![],
        };
    }

    fn get(&mut self, chr: char, px: f32) -> usize {
        // Check if we already have this character
        for (c, p, i) in &self.glyphsi {
            if *c == chr && *p == px {
                return *i;
            }
        }

        // We didn't already have this character, so it's a change
        self.changed = true;

        // Rasterize the character
        let (metrics, bitmap) = self.font.rasterize(chr, px);

        // Save the metrics
        self.glyphsi.push((chr, px, self.glyphsi.len()));
        self.metrics.push(metrics);
        self.bitmaps.push(bitmap);
        self.texarea.push(Area::zero());

        // Return the index of the texture
        return self.glyphsi.len() - 1;
    }

    fn build_texture(&mut self) -> TextureData {
        // Sort glyphs by size
        self.glyphsi.sort_by(|a, b| {
            self.metrics[a.2]
                .height
                .partial_cmp(&self.metrics[b.2].height)
                .unwrap()
                .reverse()
        });

        // What if it dosen't fit?
        // How could we select the size more intelligently?
        let dimensions = (64u32, 64u32);

        // Figure out the position of all the glpyhs
        let mut x = 0f32;
        let mut y = 0f32;
        let mut max_h = 0f32;
        for (_, _, gi) in &self.glyphsi {
            // How big is this glyph?
            let size = self.metrics[*gi];
            let w = size.width as f32;
            let h = size.height as f32;

            // Update the max height of the row
            if h > max_h {
                max_h = h;
            }

            // If it doesn't fit, move to the next line
            if x + w > dimensions.0 as f32 {
                x = 0.;
                y += max_h;
                max_h = 0.;
            }

            // Update the position of the glyph in the texture
            self.texarea[*gi] = Area(Vec2::new(x, y), Vec2::new(x + w, y + h));

            // Move to the next position
            x += w;
        }

        // Pack the glyphs into the texture
        let texture_size = dimensions.0 * dimensions.1 * 4;
        let mut rgba = vec![0xff; texture_size as usize];
        for i in 0..self.bitmaps.len() {
            let area = self.texarea[i];

            let x = area.0.x as usize;
            let y = area.0.y as usize;
            let w = area.1.x as usize - x;

            for b in 0..self.bitmaps[i].len() {
                let col = b / w;
                let row = b % w;

                let sy = (y + col) * dimensions.0 as usize;
                let sx = x + row;

                println!("{} col:{} row:{}", b, col, row);

                rgba[(sy + sx) * 4 + 3] = self.bitmaps[i][b];
            }
        }

        return TextureData::new(rgba, dimensions);
    }

    fn texture_changed(&self) -> bool {
        return self.changed;
    }
}

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
                .texture(font.get('F', 20.0)),
            Div::new()
                .size(150.0, 150.0)
                .bg([0.5, 0.5, 0.0])
                .texture(font.get('e', 20.0)),
            Div::new()
                .size(300.0, 300.0)
                .bg([0.5, 0.0, 0.5])
                .texture(font.get('l', 20.0)),
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
        self.root.render(frame, &frame.area.clone());
    }
}

fn main() {
    let mut app = MyApp::new();

    if let Err(e) = pollster::block_on(app.run()) {
        eprintln!("Error: {:?}", e);
    }
}
