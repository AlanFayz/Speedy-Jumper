mod game;
mod math;
mod timer;

use game::*;

#[macroquad::main(window_config)]
async fn main() {
    run().await;
}