mod game;
mod math;
mod timer;
mod client;

use game::*;

#[macroquad::main(window_config)]
async fn main() {
    run().await;
}