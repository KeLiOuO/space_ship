use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

const MOVEMENT_SPEED: f32 = 200.0;
const COLORS: [Color; 5] = [RED, BLACK, PURPLE, PINK, LIGHTGRAY];
#[macroquad::main("space_ship")]
async fn main() {
    //随机数种子
    rand::srand(miniquad::date::now() as u64);
    //方块集合
    let mut squares: Vec<Shape> = vec![];
    //圆
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
    };
    //游戏结束标志
    let mut gameover = false;
    loop {
        clear_background(DARKGREEN);
        let delta_time = get_frame_time();
        if !gameover {
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
            circle.x = clamp(
                circle.x,
                circle.size / 2.0,
                screen_width() - circle.size / 2.0,
            );
            circle.y = clamp(
                circle.y,
                circle.size / 2.0,
                screen_height() - circle.size / 2.0,
            );
        }
        draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);
        //生成方块
        if !gameover {
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
            //下落
            for square in &mut squares {
                square.y += delta_time * square.speed;
            }
            //移除外边的方块
            squares.retain(|square| square.y < screen_height() + square.size);
        }
        for square in &squares {
            draw_rectangle(square.x, square.y, square.size, square.size, square.color);
        }
        //碰撞检测
        if squares
            .iter()
            .any(|square| circle.circle_collides_with(square))
        {
            gameover = true;
        }

        if gameover {
            let text = "GAME OVER!";
            let text_dimension = measure_text(text, None, 64, 1.0);
            draw_text(
                text,
                (screen_width() - text_dimension.width) / 2.0,
                screen_height() / 2.0 - text_dimension.offset_y,
                64.0,
                BLACK,
            );
            if is_key_pressed(KeyCode::Space) {
                gameover = false;
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                squares.clear();
            }
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

impl Shape {
    fn circle_collides_with(&self, other: &Self) -> bool {
        self.circle().overlaps_rect(&other.rect())
    }
    fn rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }

    fn circle(&self) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            r: self.size / 2.0,
        }
    }
}
