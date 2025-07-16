use macroquad::math::*;

pub fn rotate_around(direction: Vec2, position: Vec2, origin: Vec2) -> Vec2 {
    direction.rotate(position - origin) + origin
}

#[derive(Clone, Copy)]
pub struct Bounds2D {
    top_left: Vec2, 
    size:   Vec2
}

impl Bounds2D {
    pub fn new(top_left: Vec2, size: Vec2) -> Bounds2D {
        Bounds2D {
            top_left,
            size
        }
    }

    pub fn translate(&mut self, offset: Vec2) {
        self.top_left += offset;
    }
    
    pub fn get_position(&self) -> Vec2 {
        self.top_left
    }

    pub fn get_center(&self) -> Vec2 {
        return self.top_left + self.size / 2.0;
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn intersects(&self, other: Bounds2D) -> bool {
        !(
            self.top_left.x  + self.size.x  <= other.top_left.x  ||  //self is left of other
            other.top_left.x + other.size.x <= self.top_left.x   ||  //self is right of other
            self.top_left.y  + self.size.y  <= other.top_left.y  ||  //self is below other
            other.top_left.y + other.size.y <= self.top_left.y       //self is above other
        )
    }
}
