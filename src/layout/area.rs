use crate::ruglet::prelude::*;

pub struct Area {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Area {
    pub fn pad(&self, padding: f32) -> Area {
        return Area {
            x: self.x + padding,
            y: self.y + padding,
            w: self.w - padding * 2.,
            h: self.h - padding * 2.,
        };
    }

    pub fn resize(&self, size: Vec2) -> Area {
        return Area {
            x: self.x,
            y: self.y,
            w: size.x,
            h: size.y,
        };
    }
}

impl Clone for Area {
    fn clone(&self) -> Self {
        return Area {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        };
    }
}
