#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn zero() -> Vec2 {
        return Vec2 { x: 0.0, y: 0.0 };
    }

    pub fn new(x: f32, y: f32) -> Vec2 {
        return Vec2 { x, y };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Area(pub Vec2, pub Vec2);

impl Area {
    pub fn w(&self) -> f32 {
        return self.1.x - self.0.x;
    }

    pub fn h(&self) -> f32 {
        return self.1.y - self.0.y;
    }

    pub fn pad(&self, padding: f32) -> Area {
        return Area(
            Vec2 {
                x: self.0.x + padding,
                y: self.0.y + padding,
            },
            Vec2 {
                x: self.1.x - padding,
                y: self.1.y - padding,
            },
        );
    }

    pub fn resize(&self, size: Vec2) -> Area {
        return Area(
            self.0,
            Vec2 {
                x: self.0.x + size.x,
                y: self.0.y + size.y,
            },
        );
    }
}
