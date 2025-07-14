use macroquad::math::*;

pub fn rotate_around(direction: Vec2, position: Vec2, origin: Vec2) -> Vec2 {
    direction.rotate(position - origin) + origin
}