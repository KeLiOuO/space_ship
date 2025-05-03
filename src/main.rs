use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
#[macroquad::main("space_ship")]
async fn main() {
    //随机数种子
    rand::srand(miniquad::date::now() as u64);
    //移动速度
    const MOVEMENT_SPEED: f32 = 200.0;
    //方块颜色组
    const COLORS: [Color; 5] = [RED, BLACK, PURPLE, PINK, LIGHTGRAY];
    //方块集合
    let mut squares: Vec<Shape> = vec![];
    //子弹集合
    let mut bullets: Vec<Shape> = vec![];
    //圆
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        collided: false,
    };
    //上次射击时间
    let mut shoot_interval = 0.0;
    //游戏结束标志
    let mut gameover = false;
    
    //游戏循环
    loop {
        //设置背景颜色
        clear_background(DARKGREEN);
        
        if !gameover {
            //当前帧和上一帧的时间间隔
            let delta_time = get_frame_time();
            
            //圆的移动
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
            //限定圆的移动范围
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
            
            //按下空格生成子弹 间隔0.1秒一次
            if is_key_pressed(KeyCode::Space) && (get_time() - shoot_interval) >= 0.1 {
                shoot_interval = get_time();
                bullets.push(Shape {
                    x: circle.x,
                    y: circle.y - circle.size / 2.0,
                    size: 5.0,
                    speed: circle.speed * 2.0,
                    color: MAGENTA,
                    collided: false,
                })
            }
            
            //生成方块
            if rand::gen_range(0, 99) >= 95 {
                let size = rand::gen_range(16.0, 64.0);
                squares.push(Shape {
                    size,
                    speed: rand::gen_range(50.0, 150.0),
                    x: rand::gen_range(0.0, screen_width() - size),
                    y: -size,
                    color: COLORS.choose().unwrap().clone(),
                    collided: false,
                })
            }
            
            //方块下落和子弹前进
            for square in &mut squares {
                square.y += delta_time * square.speed;
            }
            for bullet in &mut bullets {
                bullet.y -= bullet.speed * delta_time;
            }
            
            //移除外边的方块和子弹
            squares.retain(|square| square.y < screen_height() + square.size);
            bullets.retain(|bullet| bullet.y > -bullet.size);
            
            //移除碰撞过的方块和子弹
            squares.retain(|square| !square.collided);
            bullets.retain(|bullet| !bullet.collided);
        }
        
        //圆和方块的碰撞检测
        if squares
            .iter()
            .any(|square| circle.circle_collides_with(square))
        {
            gameover = true;
        }
        
        //子弹和方块的碰撞检测
        for bullet in bullets.iter_mut() {
            for square in squares.iter_mut() {
                if bullet.circle_collides_with(square) {
                    bullet.collided = true;
                    square.collided = true;
                }
            }
        }
        
        //图像绘制
        draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);
        for bullet in &bullets {
            draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
        }
        for square in &squares {
            draw_rectangle(square.x, square.y, square.size, square.size, square.color);
        }
        
        //游戏结束的提示和重开
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
            let tip = "press space to restart";
            let tip_dimension = measure_text("press space to restart", None, 32, 1.0);
            draw_text(
                tip,
                (screen_width() - tip_dimension.width) / 2.0,
                screen_height() / 2.0 + text_dimension.offset_y,
                32.0,
                BLACK,
            );
            if is_key_pressed(KeyCode::Space) {
                gameover = false;
                circle.x = screen_width() / 2.0;
                circle.y = screen_height() / 2.0;
                squares.clear();
                bullets.clear();
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
    collided: bool,
}

impl Shape {
    //判断圆和方块是否碰撞
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
