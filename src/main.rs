use macroquad::prelude::*;
#[macroquad::main("space_ship")]
async fn main() {
    loop {
        clear_background(DARKGREEN);
        next_frame().await;
    }
}
