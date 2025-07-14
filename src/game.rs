use std::time::Duration;
use std::vec::Vec;

use macroquad::prelude::*;
use macroquad::time::draw_fps;
use macroquad::rand::RandGenerator;

mod sprite;
mod collectable;


use sprite::*;
use collectable::*;

use crate::math::Bounds2D;
use crate::timer::Timer;

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
    player: Sprite,
    jump_boosts: Vec<JumpBoost>,
    jump_boost_timer: Timer,
    jump_boost_image: Texture2D
}


pub static RANDOM: RandGenerator = RandGenerator::new();

pub async fn run() {
    let mut game_info = Game {
        game_state: GameState::Playing,
        player: Sprite::new("character".to_owned(), Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(150.0, 150.0)).await,
        jump_boosts: Vec::new(),
        jump_boost_timer: Timer::new(),
        jump_boost_image: load_texture("assets/character_body.png").await.unwrap_or(Texture2D::empty())
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
    draw_boost_count(game_info);

    update_entities(game_info);
    resolve_collisions(game_info);

    draw_entities(game_info);

    cleanup_boosts(game_info);
    spawn_boosts(game_info);
}

fn draw_boost_count(game_info: &mut Game) {
    draw_text(&format!("Boost Count: {}", game_info.player.boost_counter), 0.0, 16.0 * 3.0, 32.0, WHITE); 
}

fn cleanup_boosts(game_info: &mut Game) {
    let screen_bounds = Bounds2D::new(
        Vec2::new(0.0, 0.0),
        Vec2::new(screen_width(), screen_height()),
    );

    let player_bounds = game_info.player.get_bounds();
    game_info.jump_boosts.retain(|boost| boost.bounds.intersects(screen_bounds) && !boost.bounds.intersects(player_bounds));
}

fn spawn_boosts(game_info: &mut Game) {
    const MAX_BOOSTS: usize = 10;
    const MAX_BOOSTS_ADD: usize = 3;
    const BOOST_SPAWN_COOLDOWN: Duration = Duration::new(1, 0);

    if game_info.jump_boosts.len() >= MAX_BOOSTS || !game_info.jump_boost_timer.has_elapsed(BOOST_SPAWN_COOLDOWN) {
        return;
    }

    game_info.jump_boost_timer.reset();

    let boosts_to_add = MAX_BOOSTS_ADD.min(MAX_BOOSTS - game_info.jump_boosts.len());

    for _ in 0..boosts_to_add {
        let boost_position = Vec2::new(RANDOM.gen_range(0.0, screen_width()), RANDOM.gen_range(0.0, screen_height() / 2.0));
        let boost_size = Vec2::splat(RANDOM.gen_range(25.0, 50.0));

        game_info.jump_boosts.push(JumpBoost::new(
            boost_position,
            boost_size,
            game_info.jump_boost_image.clone(),
            0.0, 10.0 ,
            0.0, 10.0
        ));
    }

}

fn update_entities(game_info: &mut Game) {
    for boost in &mut game_info.jump_boosts {
        boost.update();
    }

    game_info.player.update();
}

fn draw_entities(game_info: &Game) {
    for boost in &game_info.jump_boosts {
        boost.draw()
    }

    game_info.player.draw();
}

fn resolve_collisions(game_info: &mut Game) {
    for boost in &game_info.jump_boosts {
        if boost.bounds.intersects(game_info.player.get_bounds()) {
            game_info.player.boost_counter += 1;
        }
    }
}