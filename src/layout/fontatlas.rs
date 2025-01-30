use crate::ruglet::*;
use fontdue::*;

pub struct FontAtlas {
    font: Font,
    changed: bool,
    dimensions: (f32, f32),

    // Glyph info
    glyphsi: Vec<(char, f32, usize)>,
    bitmaps: Vec<Vec<u8>>,
    metrics: Vec<Metrics>,
    texarea: Vec<Area>,
}

impl FontAtlas {
    pub fn new() -> Self {
        // Load font
        let font = include_bytes!("../../res/OpenSans-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font, FontSettings::default()).unwrap();

        return FontAtlas {
            font,
            changed: false,
            dimensions: (0., 0.),

            // Glyph info
            glyphsi: vec![],
            bitmaps: vec![],
            metrics: vec![],
            texarea: vec![],
        };
    }

    pub fn size(&mut self, chr: char, px: f32) -> Vec2 {
        let tex = self.get(chr, px);
        let metrics = self.metrics[tex];

        return Vec2 {
            x: metrics.advance_width,
            y: px,
        };
    }

    pub fn metrics(&self, glyphsi: usize) -> Metrics {
        return self.metrics[glyphsi];
    }

    pub fn get(&mut self, chr: char, px: f32) -> usize {
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

    pub fn texture_area(&self, glyphsi: usize) -> Area {
        let area = self.texarea[glyphsi];
        return Area(
            Vec2::new(area.0.x / self.dimensions.0, area.0.y / self.dimensions.1),
            Vec2::new(area.1.x / self.dimensions.0, area.1.y / self.dimensions.1),
        );
    }

    pub fn build_texture(&mut self) -> TextureData {
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
        let dimensions = (1024u32, 1024u32);
        self.dimensions = (dimensions.0 as f32, dimensions.1 as f32);

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

                rgba[(sy + sx) * 4 + 3] = self.bitmaps[i][b];
            }
        }

        return TextureData::new(rgba, dimensions);
    }

    pub fn texture_changed(&self) -> bool {
        return self.changed;
    }
}
