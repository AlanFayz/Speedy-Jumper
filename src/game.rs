use std::time::Duration;
use std::vec::Vec;

use macroquad::file::load_string;
use macroquad::prelude::*;
use macroquad::time::draw_fps;
use macroquad::rand::RandGenerator;

use macroquad::ui::*;

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

#[derive(Clone)]
struct GameResources {
    pub jump_boost_image: Texture2D,
    pub background_material: Material
}

struct Game {
    game_state: GameState, 
    player: Sprite,
    jump_boosts: Vec<JumpBoost>,
    jump_boost_timer: Timer,
    dead_timer: Timer, 
    is_dead: bool,  // TODO: move death semantics to sprite
    resources: GameResources
}



pub static RANDOM: RandGenerator = RandGenerator::new();

pub async fn run() {
    next_frame().await;

    let game_resources = create_game_resources().await;
    let mut game_info  = create_game(game_resources).await;
    
    set_default_camera();

    loop {
        match game_info.game_state {
            GameState::Menu => {

            }, 
            GameState::Playing => {
                playing_state(&mut game_info);
            }, 
            GameState::EndScreen => {
                end_screen_state(&mut game_info).await;
            },
        }

        next_frame().await;
    }
}

async fn create_game(game_resources: GameResources) -> Game {
    Game {
        game_state: GameState::Playing,
        player: Sprite::new("character".to_owned(), Vec2::new(screen_width() / 2.0, screen_height() / 2.0), Vec2::new(150.0, 150.0)).await,
        jump_boosts: Vec::new(),
        jump_boost_timer: Timer::new(),
        dead_timer: Timer::new(),
        is_dead: false,
        resources: game_resources
    }
} 

async fn create_game_resources() -> GameResources {

    let fragment_shader_source = load_string("assets/shaders/fragment.glsl").await;
    let vertex_shader_source = load_string("assets/shaders/vertex.glsl").await;

    if vertex_shader_source.is_err() {
        macroquad::logging::error!("failed to load vertex shader");
    }

    if fragment_shader_source.is_err() {
        macroquad::logging::error!("failed to load fragment shader");
    }

    let bg_material: Result<Material, macroquad::Error> = load_material(
            ShaderSource::Glsl { 
                fragment: &fragment_shader_source.expect("failed to load fragment shader"),
                vertex: &vertex_shader_source.expect("failed to load vertex shader")
            }, 
            MaterialParams {
                uniforms: vec![ UniformDesc::new("u_ScreenSize", UniformType::Float2), 
                                UniformDesc::new("u_Time", UniformType::Float1) ],
             ..Default::default()
            },
        );

    if bg_material.is_err() {
        macroquad::logging::error!("{:?}", bg_material);
    }

    GameResources {
        jump_boost_image: load_texture("assets/character_body.png").await.unwrap_or(Texture2D::empty()),
        background_material: bg_material.expect("failed to load material")
    }
}

fn playing_state(game_info: &mut Game) {
    draw_background(game_info);

    draw_fps();
    draw_boost_count(game_info);

    update_entities(game_info);
    resolve_collisions(game_info);

    draw_entities(game_info);

    cleanup_boosts(game_info);
    spawn_boosts(game_info);

    if game_info.is_dead && game_info.dead_timer.has_elapsed(Duration::from_secs(2)) {
        game_info.game_state = GameState::EndScreen;
    }

    if !game_info.is_dead && (game_info.player.boost_counter == 0 || !game_info.player.get_bounds().intersects(screen_bounds())) {
        game_info.is_dead = true;
        game_info.dead_timer.reset();
    }
}

fn draw_background(game_info: &mut Game) {
    clear_background(WHITE);

    gl_use_material(&game_info.resources.background_material);

    game_info.resources.background_material.set_uniform("u_ScreenSize", (screen_width(), screen_height()));
    game_info.resources.background_material.set_uniform("u_Time", get_time() as f32);

    draw_rectangle(0.0, 0.0, screen_width(), screen_height(), WHITE);

    gl_use_default_material();
}

fn screen_bounds() -> Bounds2D {
    Bounds2D::new(Vec2::new(0.0, 0.0), Vec2::new(screen_width(), screen_height()))
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
            game_info.resources.jump_boost_image.clone(),
            0.0, 10.0 ,
            0.0, 10.0
        ));
    }

}

fn update_entities(game_info: &mut Game) {
    for boost in &mut game_info.jump_boosts {
        boost.update();
    }

    if !game_info.is_dead {
        game_info.player.update();
    }
}

fn draw_entities(game_info: &Game) {
    for boost in &game_info.jump_boosts {
        boost.draw()
    }

    if !game_info.is_dead {
        game_info.player.draw();
    }
}

fn resolve_collisions(game_info: &mut Game) {
    if game_info.is_dead {
        return;
    }

    for boost in &game_info.jump_boosts {
        if boost.bounds.intersects(game_info.player.get_bounds()) {
            game_info.player.boost_counter += 1;
        }
    }
}

async fn end_screen_state(game_info: &mut Game) {
    if widgets::Button::new("Play Again")
        .position(vec2(100.0, 100.0))
        .size(Vec2::new(100.0, 50.0))
        .ui(&mut *root_ui())
        {

            game_info.game_state = GameState::Playing;
            *game_info = create_game(game_info.resources.clone()).await;
        }
}