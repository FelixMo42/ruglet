use std::io::Write;

struct QoiHeader {}

struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Pixel {
    fn new() -> Self {
        Pixel {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    fn hash(&self) -> u8 {
        return (self.r * 3 + self.g * 5 + self.b * 7 + self.a * 11) % 64;
    }
}

const CHANNELS: usize = 4;

const QOI_HEADER_SIZE: usize = 14;
const QOI_END_MARKER: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
const QOI_END_MARKER_SIZE: usize = 8;

const QOI_OP_RUN: u8 = 0xc0;
const QOI_OP_INDEX: u8 = 0x00;
const QOI_OP_DIFF: u8 = 0x40;
const QOI_OP_LUMA: u8 = 0x80;
const QOI_OP_RGB: u8 = 0xfe;
const QOI_OP_RGBA: u8 = 0xff;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    fn hash(&self) -> usize {
        return (self.r as usize * 3
            + self.g as usize * 5
            + self.b as usize * 7
            + self.a as usize * 11)
            % 64;
    }
}

fn encode() {
    let mut seen = [Color::new(0, 0, 0, 255); 64];

    let pixels = [0, 0, 0, 255, 100, 0, 0, 255];
    let width: u32 = 1;
    let height: u32 = 2;

    let mut prev = Color::new(0, 0, 0, 255);

    let mut buffer: Vec<u8> = Vec::with_capacity((width * height) as usize);

    let mut run = 0;

    let last_pixel = pixels.len() - CHANNELS;

    buffer.extend(b"qoif");
    buffer.extend(width.to_be_bytes());
    buffer.extend(height.to_be_bytes());
    buffer.push(4); // num channels
    buffer.push(1); // linar RGB mode

    for i in (0..pixels.len()).step_by(CHANNELS) {
        let color = Color::new(pixels[i + 0], pixels[i + 1], pixels[i + 2], pixels[i + 3]);

        if color == prev {
            run += 1;

            if run == 62 || i == last_pixel {
                buffer.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }
        } else {
            if run > 0 {
                buffer.push(QOI_OP_RUN | (run - 1));
                run = 0;
            }

            let hash = color.hash();

            if color == seen[hash] {
                buffer.push(QOI_OP_INDEX | (hash as u8));
            } else {
                seen[hash] = color;

                let dr = color.r - prev.r;
                let dg = color.g - prev.g;
                let db = color.b - prev.b;
                let da = color.a - prev.a;

                let dr_dg = dr - dg;
                let db_dg = db - dg;

                if da != 0 {
                    buffer.push(QOI_OP_RGBA);
                    buffer.push(color.r);
                    buffer.push(color.g);
                    buffer.push(color.b);
                    buffer.push(color.a);
                } else if (dr >= 255 - 2 && dr <= 1)
                    && (dg >= 255 - 2 && dg <= 1)
                    && (db >= 255 - 2 && db <= 1)
                {
                    buffer.push(QOI_OP_DIFF | ((dr + 2) << 4) | ((dg + 2) << 2) | ((db + 2) << 0))
                } else if (dg >= 255 - 32 && dg <= 31)
                    && (dr_dg >= 255 - 8 && dr_dg <= 7)
                    && (db_dg >= 255 - 8 && db_dg <= 7)
                {
                    buffer.push(QOI_OP_LUMA | (dg + 32));
                    buffer.push(((dr_dg + 8) << 4) | (db_dg + 8));
                } else {
                    buffer.push(QOI_OP_RGB);
                    buffer.push(color.r);
                    buffer.push(color.g);
                    buffer.push(color.b);
                }
            }
        }

        prev = color;
    }

    for byte in QOI_END_MARKER {
        buffer.push(byte);
    }
}

fn decode() {}

fn main() {
    encode()
}
