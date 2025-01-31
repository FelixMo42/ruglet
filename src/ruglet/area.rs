use std::fmt::Debug;

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

    pub fn inside(&self, area: Area) -> bool {
        return area.0.x <= self.x
            && self.x <= area.1.x
            && area.0.y <= self.y
            && self.y <= area.1.y;
    }
}

#[derive(Clone, Copy)]
pub struct Area(pub Vec2, pub Vec2);

impl Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}-{},{}", self.0.x, self.0.y, self.1.x, self.1.y)
    }
}

impl Area {
    pub fn zero() -> Area {
        return Area(Vec2::zero(), Vec2::zero());
    }

    pub fn is_zero(&self) -> bool {
        return self.0.x == 0. && self.0.y == 0. && self.1.x == 0. && self.1.y == 0.;
    }

    pub fn contains(&self, target: Area) -> bool {
        return (self.0.x < target.0.x
            && target.0.x < self.1.x
            && self.0.y < target.0.y
            && target.0.y < self.1.y)
            || (self.0.x < target.1.x
                && target.0.x < self.1.x
                && self.0.y < target.1.y
                && target.0.y < self.1.y);
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
