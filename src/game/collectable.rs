use macroquad::math::*;
use macroquad::prelude::*;

use crate::game::draw_rectangle_screen;
use crate::math::*;


pub struct JumpBoost {
    pub bounds: Bounds2D, 
    pub hurtful: bool,
    velocity: Vec2
}

impl JumpBoost {
    pub fn new(position: Vec2, size: Vec2, hurtful: bool) -> JumpBoost {
        JumpBoost { 
            bounds: Bounds2D::new(position, size), 
            hurtful,
            velocity: Vec2::new(0.0, 0.0)
        }
    }

    pub fn update(&mut self, gravity_force: f32) {
        self.velocity += Vec2::new(0.0, gravity_force);
        self.bounds.translate(self.velocity);
    }

    pub fn draw(&self) {
        if self.hurtful {
            draw_rectangle_screen(self.bounds.get_position(), self.bounds.get_size(), RED); 
        } else {
            draw_rectangle_screen(self.bounds.get_position(), self.bounds.get_size(), GREEN);
        }
    }
}
