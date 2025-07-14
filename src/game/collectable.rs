use std::sync::PoisonError;

use macroquad::math::*;
use macroquad::prelude::*;

use crate::math::*;
use crate::game::RANDOM;


pub struct JumpBoost {
    pub bounds: Bounds2D, 
    pub boost_offset: f32, 
    pub boost_cooldown_offset: f32,
    image: Texture2D,
    velocity: Vec2
}

impl JumpBoost {
    pub fn new(position: Vec2, size: Vec2, image: Texture2D, boost_start: f32, boost_range: f32, cooldown_start: f32, cooldown_range: f32) -> JumpBoost {
        JumpBoost { 
            bounds: Bounds2D::new(position, size), 
            boost_offset: RANDOM.gen_range(boost_start, boost_start + boost_range),
            boost_cooldown_offset: RANDOM.gen_range(cooldown_start, cooldown_start + cooldown_range),
            image, 
            velocity: Vec2::new(0.0, 0.0)
        }
    }

    pub fn update(&mut self) {
        const GRAVITY_CONSTANT: f32 = 0.05; //TODO: maybe add some random x movement for each jump boost?
        self.velocity += Vec2::new(0.0, GRAVITY_CONSTANT);

        self.bounds.translate(self.velocity);
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.image,
            self.bounds.get_position().x,
            self.bounds.get_position().y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.bounds.get_size()), 
                ..Default::default()
            },
        );
    }
}
