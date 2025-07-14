use macroquad::math::*;

pub fn rotate_around(direction: Vec2, position: Vec2, origin: Vec2) -> Vec2 {
    direction.rotate(position - origin) + origin
}

#[derive(Clone, Copy)]
pub struct Bounds2D {
    center: Vec2, 
    size:   Vec2
}

impl Bounds2D {
    pub fn new(top_left: Vec2, size: Vec2) -> Bounds2D {
        Bounds2D {
            center: top_left + size / 2.0,
            size: size
        }
    }

    pub fn inside(&self, point: Vec2) -> bool {
        point.x <= self.center.x + self.size.x / 2.0 && 
        point.x >= self.center.x - self.size.x / 2.0 && 
        point.y <= self.center.y + self.size.y / 2.0 && 
        point.y >= self.center.y - self.size.y / 2.0
    }
}
