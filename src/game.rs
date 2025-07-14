use std::time::Duration;

use macroquad::prelude::*;
use macroquad::time::draw_fps;

mod sprite;
mod collectable;

use sprite::*;
use collectable::*;


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

enum GameState {
    Menu, 
    Playing, 
    EndScreen
}

struct Game {
    game_state: GameState, 
    player: Sprite
}


pub async fn run() {
    let mut game_info = Game {
        game_state: GameState::Playing,
        player: Sprite::new("character".to_owned(), Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(150.0, 150.0)).await
    };

    loop {
        match game_info.game_state {
            GameState::Menu => {

            }, 
            GameState::Playing => {
                playing_state(&mut game_info);
            }, 
            GameState::EndScreen => {

            },
        }

        next_frame().await;
    }

}

fn playing_state(game_info: &mut Game) {
    clear_background(DARKBLUE);
    draw_fps();

    game_info.player.update();
}