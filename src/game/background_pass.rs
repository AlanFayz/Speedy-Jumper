use macroquad::prelude::*;

use crate::game::{sprite::Sprite, RANDOM};


#[derive(Clone)]
pub struct BackgroundPass {
    material: Material, 
    descent: f64, 
    time_elapsed: f64
}

impl BackgroundPass {
    pub async fn new() -> BackgroundPass {
        let fragment_shader_source = load_string("assets/shaders/fragment.glsl").await;
        let vertex_shader_source = load_string("assets/shaders/vertex.glsl").await;

        if vertex_shader_source.is_err() {
            error!("failed to load vertex shader");
        }

        if fragment_shader_source.is_err() {
            error!("failed to load fragment shader");
        }

        let bg_material = load_material(
            ShaderSource::Glsl { 
                fragment: &fragment_shader_source.expect("failed to load fragment shader"),
                vertex: &vertex_shader_source.expect("failed to load vertex shader")
            }, 
            MaterialParams {
                uniforms: vec![ UniformDesc::new("u_ScreenSize", UniformType::Float2), 
                                UniformDesc::new("u_Time", UniformType::Float1), 
                                UniformDesc::new("u_BouncesLeft", UniformType::Float1), 
                                UniformDesc::new("u_Descent", UniformType::Float1), 
                                UniformDesc::new("u_PlayerPosition", UniformType::Float2), 
                                UniformDesc::new("u_PlayerRadius", UniformType::Float1), 
                                UniformDesc::new("u_Random", UniformType::Float1), 
                                 ],
                
                ..Default::default()
            },
        );

        if bg_material.is_err() {
            error!("{:?}", bg_material);
        }

        BackgroundPass { 
            material: bg_material.expect("failed to load material"), 
            descent: 0.0, 
            time_elapsed: 0.0
        }
    }


    pub fn render(&mut self, delta_time: f64, start_time: f64, player: &Sprite) {
        clear_background(WHITE);

        gl_use_material(&self.material);

        self.material.set_uniform("u_ScreenSize", (screen_width(), screen_height()));
        self.material.set_uniform("u_Time", (get_time() - start_time) as f32);
        self.material.set_uniform("u_BouncesLeft", player.boost_counter as f32);
        self.material.set_uniform("u_Descent", self.descent as f32);
        self.material.set_uniform("u_PlayerPosition", player.get_bounds().get_center());
        self.material.set_uniform("u_PlayerRadius", player.view_radius);
        self.material.set_uniform("u_Random", RANDOM.gen_range(0.01, 0.03) as f32);


        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);

        gl_use_default_material();
        
        let multiplier = 1.0;

        self.time_elapsed += delta_time;
        self.descent += delta_time * multiplier;
    }

    pub fn reset(&mut self) {
        self.descent = 0.0;
        self.time_elapsed = 0.0;
    }
}