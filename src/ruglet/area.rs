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
    pub fn zero() -> Area {
        return Area(Vec2::zero(), Vec2::zero());
    }

    pub fn w(&self) -> f32 {
        return self.1.x - self.0.x;
    }

    pub fn h(&self) -> f32 {
        return self.1.y - self.0.y;
    }

    pub fn size(&self) -> Vec2 {
        return Vec2::new(self.w(), self.h());
    }
}
