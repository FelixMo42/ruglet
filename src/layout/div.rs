use crate::layout::prelude::*;
use crate::ruglet::prelude::*;

pub struct Div {
    style: Style,
    children: Vec<Div>,
}

impl Div {
    pub fn new() -> Div {
        return Div {
            style: Style::new(),
            children: vec![],
        };
    }

    pub fn children(mut self, children: Vec<Div>) -> Self {
        self.children = children;

        return self;
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.style.width = Some(w);
        self.style.height = Some(h);

        return self;
    }

    pub fn pad(mut self, padding: f32) -> Self {
        self.style.padding = Some(padding);

        return self;
    }

    pub fn bg(mut self, color: [f32; 3]) -> Self {
        self.style.bg = Some(color);

        return self;
    }
}

impl Div {
    fn calc_size(&self, max: &Area) -> Vec2 {
        let w = self.style.width.unwrap_or(max.w);
        let h = self.style.height.unwrap_or(max.h);

        return Vec2 { x: w, y: h };
    }

    pub fn render(&self, renderer: &mut Renderer, area: &Area) -> Area {
        // How big am I?
        let my_area = area.resize(self.calc_size(area));

        // Draw a background if needed
        if let Some(color) = self.style.bg {
            renderer.quad(&Sprite::new_with_color(&my_area, color));
        }

        // Calculare the total amount area for my children
        let mut child_area = my_area.clone();
        if let Some(padding) = self.style.padding {
            child_area = child_area.pad(padding);
        }

        // Draw children
        for child in &self.children {
            let size = child.calc_size(&child_area);

            // Wrap
            if child_area.x + size.x > my_area.x + my_area.w {
                child_area.x = my_area.x + self.style.padding.unwrap_or(0.0);
                child_area.y += 100.0;
            }

            // Blah
            let area = child.render(renderer, &child_area);
            child_area.x = area.x + area.w;
        }

        // Return my area so that my parents knows where I am.
        return my_area;
    }
}
