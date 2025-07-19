use std::time::Duration;
use std::vec::Vec;

use macroquad::audio::*;
use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use macroquad::time::draw_fps;
use macroquad::rand::RandGenerator;

use macroquad::ui::*;

mod sprite;
mod collectable;
mod background_pass;

use sprite::*;
use collectable::*;
use background_pass::*;

use crate::math::pixel_space;
use crate::math::Bounds2D;
use crate::timer::Timer;
use crate::client::*;

pub fn window_config() -> Conf {
     Conf {
        window_title: "Speedy Jumper".to_owned(),
        fullscreen: false,
        window_resizable: true,
        window_width: 1280,
        window_height: 720,
        platform: Platform { swap_interval: Some(1), ..Default::default() },
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
    pub background_pass: BackgroundPass,
    pub death_audio: Sound,
    pub start_audio: Sound, 
    pub soundtrack: Sound, 
    pub string_buffer: String
}

struct Game {
    game_state: GameState, 
    player: Sprite,
    jump_boosts: Vec<JumpBoost>,
    jump_boost_timer: Timer,
    dead_timer: Timer, 
    is_dead: bool,  
    resources: GameResources, 
    start_time: f64, 
    time_played: f64, 
    client: Client,
    show_name_already_exists: bool, 
    show_name_timer: Timer
}

pub static RANDOM: RandGenerator = RandGenerator::new();

static SOUND_EFFECT_VOLUME_RATIO: f32 = 0.8;
static SOUNDTRACK_VOLUME_RATIO: f32 = 0.1;

static SPRITE_LARGE_VIEW_RADIUS: f32 = 600.0 / 1280.0;
static SPRITE_SMALL_VIEW_RADIUS: f32 = 100.0 / 720.0;

pub async fn run() {
    let game_resources = create_game_resources().await;
    let mut game_info  = create_game(game_resources, GameState::Menu, Client::empty()).await;
    
    set_default_camera();

    let mut delta_time = 0.0;


    loop {
        let timer = Timer::new();

        match game_info.game_state {
            GameState::Menu => {
                menu_state(&mut game_info).await;
            }, 
            GameState::Playing => {
                playing_state(&mut game_info, delta_time);
            }, 
            GameState::EndScreen => {
                end_screen_state(&mut game_info).await;
            },
        }

        next_frame().await;
        delta_time = timer.elapsed().as_secs_f64();
    }
}

async fn new_sprite() -> Sprite {
    Sprite::new(
        "character".to_owned(), 
        Vec2::new(0.5, 0.5),
        Vec2::new(150.0 / 1280.0, 150.0 / 720.0), 
        SPRITE_LARGE_VIEW_RADIUS).await
}

async fn create_game(mut game_resources: GameResources, game_state: GameState, client: Client) -> Game {
    game_resources.background_pass.reset();

    Game {
        game_state,

        player: new_sprite().await,

        jump_boosts: Vec::new(),
        jump_boost_timer: Timer::new(),

        dead_timer: Timer::new(),
        is_dead: false,

        resources: game_resources,

        start_time: get_time(),
        time_played: 0.0, 
        client,
        show_name_already_exists: false,
        show_name_timer: Timer::new()
    }
} 

async fn reset_game(game_info: &mut Game) {
    game_info.resources.background_pass.reset();
    game_info.player = new_sprite().await;
    game_info.jump_boosts = Vec::new();
    game_info.jump_boost_timer = Timer::new();
    game_info.dead_timer = Timer::new();
    game_info.is_dead = false;
    game_info.start_time = get_time();
    game_info.time_played = 0.0;
    game_info.show_name_already_exists = false;
}


async fn create_game_resources() -> GameResources {
    GameResources {
        background_pass: BackgroundPass::new().await,
        death_audio: load_sound("assets/fail.wav").await.unwrap(),
        start_audio: load_sound("assets/game_start.wav").await.unwrap(), 
        soundtrack: load_sound("assets/colorful_potions.wav").await.unwrap(),
        string_buffer: String::new()
    }
}

fn playing_state(game_info: &mut Game, delta_time: f64) {
    game_info.resources.background_pass.render(
            delta_time, 
            game_info.start_time, 
            &game_info.player);

    draw_fps();
    draw_boost_count(game_info);
    draw_time(game_info);

    update_entities(game_info, delta_time);
    resolve_collisions(game_info);

    draw_entities(game_info);

    cleanup_boosts(game_info);
    spawn_boosts(game_info);

    if !game_info.is_dead {
        game_info.client.register_time(get_time() - game_info.start_time);
    }

    if game_info.is_dead && game_info.dead_timer.has_elapsed(Duration::from_secs(2)) {
        game_info.game_state = GameState::EndScreen;
    }

    if !game_info.is_dead && !game_info.player.get_bounds().intersects(screen_bounds()) {
        game_info.is_dead = true;
        game_info.dead_timer.reset();

        game_info.time_played = get_time() - game_info.start_time;

        stop_sound(&game_info.resources.soundtrack);
        play_sound(&game_info.resources.death_audio, PlaySoundParams { looped: false, volume: SOUND_EFFECT_VOLUME_RATIO });
    }

    draw_leaderboard(game_info);
}


fn screen_bounds() -> Bounds2D {
    Bounds2D::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0))
}

fn draw_boost_count(game_info: &mut Game) {
    draw_text(&format!("Boost Count: {}", game_info.player.boost_counter), 0.0, 16.0 * 3.0, 32.0, WHITE); 
}

fn draw_time(game_info: &mut Game) {
    draw_text(&format!("Time: {:.2}s", get_time() - game_info.start_time), 0.0, 16.0 * 6.0, 32.0, WHITE); 
}

fn cleanup_boosts(game_info: &mut Game) {
    let screen_bounds = screen_bounds();
    let player_bounds = game_info.player.get_bounds();

    game_info.jump_boosts.retain(|boost| boost.bounds.intersects(screen_bounds) && !boost.bounds.intersects(player_bounds));
}

fn random_boost_acceleration(game_info: &Game) -> Vec2 {
    let time_elapsed = get_time() - game_info.start_time;
    let mut upper_bound = clamp(1.0 - 1.0 / (time_elapsed * 0.05), 0.2, 1.0) as f32;
    upper_bound *= 1.0;

    Vec2::new(RANDOM.gen_range(-upper_bound, upper_bound), RANDOM.gen_range(-upper_bound, upper_bound))
}

fn gen_random_boost(game_info: &Game) -> JumpBoost {
    loop {
        let boost_position = Vec2::new(RANDOM.gen_range(0.0, 1.0), RANDOM.gen_range(0.0, 1.0));
        let boost_size = Vec2::splat(RANDOM.gen_range(25.0 / 1280.0, 50.0 / 1280.0));
        
        if !Bounds2D::new(boost_position, boost_size).intersects(game_info.player.get_bounds()) {
            break JumpBoost::new(
                    boost_position, 
                    boost_size, 
                    RANDOM.gen_range(0.0 as f64, 1.0 as f64).round() == 1.0, 
                    random_boost_acceleration(game_info))
        }
    }
}   

fn spawn_boosts(game_info: &mut Game) {
    let max_boosts = 15;
    let max_boosts_add = 7;

    const BOOST_SPAWN_COOLDOWN: Duration = Duration::new(1, 0);

    if game_info.jump_boosts.len() >= max_boosts || !game_info.jump_boost_timer.has_elapsed(BOOST_SPAWN_COOLDOWN) {
        return;
    }

    game_info.jump_boost_timer.reset();

    let boosts_to_add = max_boosts_add.min(max_boosts - game_info.jump_boosts.len());

    for _ in 0..boosts_to_add {
        game_info.jump_boosts.push(gen_random_boost(&game_info));
    }

}

fn update_entities(game_info: &mut Game, delta_time: f64) {
    for boost in &mut game_info.jump_boosts {
        boost.update(delta_time);
    }

    if !game_info.is_dead {
        game_info.player.update(delta_time);
    }
}

fn draw_entities(game_info: &Game) {
    let player_position = game_info.player
        .get_bounds() 
        .get_center();


    for boost in &game_info.jump_boosts {
        if player_position.distance(boost.bounds.get_center()) <= game_info.player.view_radius {
            boost.draw();
        }
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
            if boost.hurtful {
                game_info.player.boost_counter -= 1;
                game_info.player.view_radius = SPRITE_SMALL_VIEW_RADIUS;
            }
            else {
                game_info.player.boost_counter += 2;
                game_info.player.view_radius = SPRITE_LARGE_VIEW_RADIUS;
            }
        }
    }

    game_info.player.boost_counter = game_info.player.boost_counter.max(0);
}

async fn end_screen_state(game_info: &mut Game) {
    let fmt_text = format!("Stupid ahh guy bro only got {:.2}s", game_info.time_played);
    let text = &fmt_text.as_str();
    let font_size = 32.0;

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width / 2.0;
    let y = 32.0;

    draw_text(text, x, y, font_size, WHITE);

    let text = "Play Again";
    let font_size = 32.0;

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width * 3.0 / 2.0;
    let y = 128.0;

    if widgets::Button::new(text)
        .position(Vec2::new(x, y))
        .size(Vec2::new(text_dimensions.width, text_dimensions.height) * 3.0)
        .ui(&mut *root_ui())
        {
            game_info.game_state = GameState::Playing;
            reset_game(game_info).await;

            play_sound(&game_info.resources.start_audio, PlaySoundParams { looped: false, volume: SOUND_EFFECT_VOLUME_RATIO });
            play_sound(&game_info.resources.soundtrack, PlaySoundParams { looped: true, volume: SOUNDTRACK_VOLUME_RATIO });
        }

    let text = "Main Menu";
    let font_size = 32.0;

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width * 3.0 / 2.0;
    let y = 256.0;

    if widgets::Button::new(text)
        .position(Vec2::new(x, y))
        .size(Vec2::new(text_dimensions.width, text_dimensions.height) * 3.0)
        .ui(&mut *root_ui())
        {
            game_info.game_state = GameState::Menu;
            game_info.client.register_time(-1.0);
        }
}

fn draw_leaderboard(game_info: &mut Game) {
    game_info.client.sync();

    let title_text = "Leaderboard       ";
    let title_dim = measure_text(title_text, None, 32, 1.0);

    draw_text(title_text, screen_width() - title_dim.width, 0.0 + title_dim.height, 32.0, WHITE);

    let mut curr_y = title_dim.height;
    let mut count = 0;

    let leaderboard = game_info.client.get_leaderboard();

    if leaderboard.is_empty() {
        let text = "failed to connect to server";
        let dim = measure_text(text, None, 32, 1.0);

        draw_text(text, screen_width() - dim.width - 10.0, curr_y + dim.height + 10.0, 32.0, WHITE);
        
        return;
    }

    let max_name_width = leaderboard.iter()
        .take(10)
        .map(|(name, _)| measure_text(name, None, 32, 1.0).width)
        .fold(0.0, f32::max);

    let padding = 10.0;
    let score_x = screen_width() - padding;
    let name_x = score_x - 100.0 - max_name_width; 

    let mut vec: Vec<_> = leaderboard.iter().collect();
    vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap()); 

    for (name, score) in vec.iter().rev() {
        if count == 10 {
            break;
        }

        count += 1;

        let name_dim = measure_text(name, None, 32, 1.0);
        curr_y += name_dim.height + padding;

        let score_text = format!("{:.2}s", score);
        let score_dim = measure_text(&score_text, None, 32, 1.0);

        draw_text(name, name_x + (max_name_width - name_dim.width), curr_y, 32.0, WHITE);
        draw_text(&score_text, score_x - score_dim.width, curr_y, 32.0, WHITE);
    }
}   

async fn menu_state(game_info: &mut Game) {
    clear_background(BLACK);

    let text = "Speedy Jumper";
    let font_size = 32.0;

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width / 2.0;
    let y = 32.0;

    draw_text(text, x, y, font_size, WHITE);

    let text = "Play";

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width * 3.0 / 2.0;
    let y = 128.0;

    if widgets::Button::new("Play")
        .position(Vec2::new(x, y))
        .size(Vec2::new(text_dimensions.width, text_dimensions.height) * 3.0)
        .ui(&mut *root_ui())
        {
            if game_info.resources.string_buffer.len() == 0 {
                game_info.show_name_already_exists = true;
                game_info.show_name_timer = Timer::new();

                return;
            }

            let client = Client::new(game_info.resources.string_buffer.clone());

            if client.is_err() {
                game_info.show_name_already_exists = true;
                game_info.show_name_timer = Timer::new();
                return;
            }


            game_info.game_state = GameState::Playing;
            *game_info = create_game(game_info.resources.clone(), GameState::Playing, client.unwrap()).await;

            play_sound(&game_info.resources.start_audio, PlaySoundParams { looped: false, volume: SOUND_EFFECT_VOLUME_RATIO });
            play_sound(&game_info.resources.soundtrack, PlaySoundParams { looped: true, volume: SOUNDTRACK_VOLUME_RATIO });
        }

    let text = "   Player Name   ";

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width * 3.0 / 2.0;
    let y = 250.0;

    widgets::InputText::new(hash!())
        .position(Vec2::new(x, y))
        .size(Vec2::new(text_dimensions.width, text_dimensions.height) * 3.0)
        .ui(&mut *root_ui(), &mut game_info.resources.string_buffer); 


    let text = "Name already exists or you haven't entered a name";

    let text_dimensions = measure_text(text, None, font_size as u16, 1.0);
    let text_width = text_dimensions.width;

    let x = screen_width() / 2.0 - text_width / 2.0;
    let y = 350.0;

    if game_info.show_name_already_exists {
        draw_text(text, x, y, 32.0, WHITE);

        if game_info.show_name_timer.has_elapsed(Duration::from_secs(3)) {
            game_info.show_name_already_exists = false;
        }
    }

    
    let text = "Space/left click/tap to Jump\nMove mouse to direct where jump will go\nGreen guys good red guys bad\nLast as long as possible.";

    let x = screen_width() / 2.0 - measure_text("Space/left click/tap to Jump", None, font_size as u16, 1.0).width / 2.0;
    let y = 512.0;

    draw_multiline_text(text, x, y, font_size, None, WHITE);
}


pub fn draw_rectangle_screen(position: Vec2, size: Vec2, color: Color) {
    let position = pixel_space(position);
    let size = pixel_space(size);

    draw_rectangle(
        position.x, 
        position.y, 
        size.x, 
        size.y, 
        color
    );
}

pub fn draw_texture_screen(texture: &Texture2D, position: Vec2, size: Vec2, color: Color) {
    let position = pixel_space(position);
    let size = pixel_space(size);

    draw_texture_ex(
        texture,
        position.x,
        position.y,
        color,
        DrawTextureParams {
            dest_size: Some(size), 
            ..Default::default()
        },
    );
}

