use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 200.0;
const COLORS: [Color; 5] = [RED, BLACK, PURPLE, PINK, LIGHTGRAY];
#[macroquad::main("space_ship")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let mut squares: Vec<Shape> = vec![];
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };

    loop {
        clear_background(DARKGREEN);
        let delta_time = get_frame_time();
        if is_key_down(KeyCode::W) {
            circle.y -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::S) {
            circle.y += MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::A) {
            circle.x -= MOVEMENT_SPEED * delta_time;
        }
        if is_key_down(KeyCode::D) {
            circle.x += MOVEMENT_SPEED * delta_time;
        }
        circle.x = clamp(circle.x, 16.0, screen_width() - 16.0);
        circle.y = clamp(circle.y, 16.0, screen_height() - 16.0);
        draw_circle(circle.x, circle.y, 16.0, circle.color);

        if rand::gen_range(0, 99) >= 95 {
            let size = rand::gen_range(16.0, 64.0);
            squares.push(Shape {
                size,
                speed: rand::gen_range(50.0, 150.0),
                x: rand::gen_range(0.0, screen_width() - size),
                y: -size,
                color: COLORS.choose().unwrap().clone(),
            })
        }
        for square in &mut squares {
            square.y += delta_time * square.speed;
        }
        squares.retain(|square| square.y < screen_height() + square.size);
        println!("{}",squares.len());
        for square in &squares {
            draw_rectangle(square.x, square.y, square.size, square.size, square.color);
        }
        next_frame().await;
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
    color: Color,
}
