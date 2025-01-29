pub struct Style {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub padding: Option<f32>,
}

impl Style {
    pub fn new() -> Style {
        return Style {
            width: None,
            height: None,
            padding: None,
        };
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = Some(w);
        self.height = Some(h);

        return self;
    }

    pub fn pad(mut self, padding: f32) -> Self {
        self.padding = Some(padding);

        return self;
    }
}
