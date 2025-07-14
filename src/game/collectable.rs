use macroquad::math::*;

use crate::math::*;

pub trait Collectable {
    fn get_bounds() -> Bounds2D;
    fn get_boost_offset() -> f32;
    fn get_boost_cooldown_offset() -> f32;
}


