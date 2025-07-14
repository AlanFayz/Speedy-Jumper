use std::time::Duration;

use macroquad::prelude::*;
use macroquad::time::draw_fps;

mod sprite;
use sprite::*;
use timer::Timer;

pub fn window_config() -> Conf {
     Conf {
        window_title: "Speedy Jumper".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
     }
 }



pub async fn run() {
    let mut sprite = Sprite::new(String::from("character"), Vec2::new(20.0, 30.0), Vec2::new(150.0, 150.0)).await;

    loop {
        clear_background(DARKBLUE);
        draw_fps();

        sprite.update();

        next_frame().await;
    }

}