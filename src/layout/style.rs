use crate::ruglet::Vec2;

pub struct Style {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Option<f32>,
    pub bg: Option<[f32; 3]>,
    pub offset: Option<Vec2>,
    pub texture: Option<usize>,
}

impl Style {
    pub fn new() -> Style {
        return Style {
            width: None,
            height: None,
            padding: None,
            bg: None,
            texture: None,
            offset: None,
        };
    }
}
