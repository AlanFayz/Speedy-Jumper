mod game;
use game::*;


#[macroquad::main(window_config)]
async fn main() {
    run().await;
}