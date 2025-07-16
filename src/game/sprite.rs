
use std::time::Duration;

use macroquad::audio::*;
use macroquad::math::*;
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

use macroquad::texture::Texture2D;

use crate::timer::*;
use crate::math::*;

pub struct Sprite {
    pub boost_counter: i32,
    pub view_radius: f32,

    position: Vec2, 
    size: Vec2,

    velocity: Vec2,

    body: Texture2D,
    eye: Texture2D,

    boost_timer: Timer,
    boost_cooldown: Duration,
    boost_speed_increase: f32,
    boost_sound: Sound, 
    
    boing_sound: Sound
}

static SPRITE_SOUND_EFFECT_VOLUME_RATIO: f32 = 0.8;

impl Sprite { 
    async fn load_sprite_texture(name: &str, part: &str) -> Texture2D {
        let asset_path = "assets/".to_owned() + name + "_" + part + ".png";
        
        match load_texture(asset_path.as_str()).await {
            Ok(texture) => {
                texture.set_filter(FilterMode::Nearest);
                texture 
            },
            Err(_) => {
                macroquad::logging::error!("failed to load {asset_path}");
                Texture2D::empty()
            },
        }
    }

    pub async fn new(name: String, position: Vec2, size: Vec2, starting_view_radius: f32) -> Self {
        Sprite {
            boost_counter: 10,
            view_radius: starting_view_radius,
            position,
            size, 

            velocity: Vec2::new(0.0, 0.0),

            body: Sprite::load_sprite_texture(name.as_str(), "body").await, 
            eye: Sprite::load_sprite_texture(name.as_str(),  "eye").await, 

            boost_timer: Timer::new(),
            boost_cooldown: Duration::from_millis(500),
            boost_speed_increase: 20.0,
            boost_sound: load_sound("assets/jump.wav").await.unwrap(), 
            boing_sound: load_sound("assets/boing.wav").await.unwrap()
        }
    }

    pub fn get_bounds(&self) -> Bounds2D {
        Bounds2D::new(self.position, self.size)
    }

    fn handle_movement(&mut self) {
        let center = self.position + self.size / 2.0;

        let direction = (Vec2::from(mouse_position()) - center)
            .normalize_or(Vec2::new(0.0, 0.0));

        if direction.length() == 0.0 {
            return;
        }

        let mut scalar = 0.0;

        if is_key_down(KeyCode::Space) && self.boost_timer.has_elapsed(self.boost_cooldown) && self.boost_counter > 0 {
            scalar += self.boost_speed_increase;
            self.boost_timer.reset();
            self.boost_counter -= 1;

            play_sound(&self.boost_sound, PlaySoundParams { looped: false, volume: SPRITE_SOUND_EFFECT_VOLUME_RATIO });
        }   

        self.velocity += direction * scalar;
    }

    fn handle_gravity(&mut self) {
        const GRAVITY_CONSTANT: f32 = 0.15;
        self.velocity += Vec2::new(0.0, GRAVITY_CONSTANT);
    }

    fn draw_eye(&self, mut eye_center: Vec2, eye_size: Vec2, eye_origin: Vec2) {
        let mouse_to_eye = Vec2::from(mouse_position()) - eye_center;
        let distance = mouse_to_eye.length() / Vec2::from(screen_size());

        let eye_direction = mouse_to_eye.normalize_or(Vec2::new(1.0, 0.0)) * distance;

        eye_center = rotate_around(eye_direction, eye_center, eye_origin);
        let eye_position = eye_center - eye_size / 2.0;

        draw_texture_ex(
            &self.eye,
            eye_position.x,
            eye_position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(eye_size), 
                ..Default::default()
            },
        );
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.body,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(self.size), 
                ..Default::default()
            },
        );

        let center = self.position + self.size / 2.0;

        let eye_center = center - Vec2::new(-0.3, 0.1) * self.size; 
        let eye_origin = center - Vec2::new(0.0, 0.1) * self.size;

        let eye_size = self.size / 2.0;

        self.draw_eye(eye_center, eye_size, eye_origin);
    }

    fn handle_border(&mut self) {
        let player_bounds = self.get_bounds();

        let mut force = Vec2::new(0.0, 0.0);

        if player_bounds.get_position().x == 0.0 {
            force += Vec2::new(1.0, 0.0);
        } else if player_bounds.get_position().x == screen_width() - self.size.x {
            force += Vec2::new(-1.0, 0.0);
        }

        if player_bounds.get_position().y == 0.0 {
            force += Vec2::new(0.0, 1.0);
        }

        if force.length() > 0.0 {
            play_sound(&self.boing_sound, PlaySoundParams { looped: false, volume: SPRITE_SOUND_EFFECT_VOLUME_RATIO });
        }

        // probably not the most accurate but want to get the "bouncy" effect off the walls
        force = force.normalize_or_zero() * self.velocity.length() * 2.0; 
        self.velocity += force;
    }

    pub fn update(&mut self) {
        self.handle_movement();
        self.handle_gravity();
        self.handle_border();

        self.velocity = self.velocity.clamp_length(0.0, 10.0);
        self.position += self.velocity;
        self.velocity = self.velocity.lerp(Vec2::new(0.0, 0.0), 0.01);

        self.position = self.position.clamp(
            Vec2::new(0.0, 0.0), 
            Vec2::new(screen_width() - self.size.x, f32::MAX),
        );
    }
}