use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;
use macroquad_particles::{ColorCurve, Emitter, EmitterConfig};
use std::fs;

const FRAGMENT_SHADER: &str = include_str!("starfield-shader.glsl");
const VERTEX_SHADER: &str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
varying float iTime;

uniform mat4 Model;
uniform mat4 Projection;
uniform vec4 _Time;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    iTime = _Time.x;
}
";
#[macroquad::main("space_ship")]
async fn main() {
    //随机数种子
    rand::srand(miniquad::date::now() as u64);
    //移动速度
    const MOVEMENT_SPEED: f32 = 200.0;
    //方块颜色组
    const COLORS: [Color; 5] = [RED, SKYBLUE, PURPLE, PINK, LIGHTGRAY];
    //射击间隔
    const SHOOT_INTERVAL: f64 = 0.5;
    //游戏状态
    let mut game_state = GameState::MianMenu;
    //上次射击时间
    let mut shoot_time = 0.0;
    //当前分数
    let mut score: u32 = 0;
    //最高分数
    let mut high_score: u32 = fs::read_to_string("highscore.dat")
        .map_or(Ok(0), |i| i.parse::<u32>())
        .unwrap_or(0);
    //方块集合
    let mut squares: Vec<Shape> = vec![];
    //子弹集合
    let mut bullets: Vec<Shape> = vec![];
    //爆炸集合
    let mut explosions: Vec<(Emitter, Vec2)> = vec![];
    //圆
    let mut circle = Shape {
        size: 32.0,
        speed: MOVEMENT_SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0,
        color: YELLOW,
        collided: false,
    };
    //圆的粒子
    let mut circle_explosion = Emitter::new(EmitterConfig {
        one_shot: false,
        amount: circle.size.round() as u32 * 2,
        initial_direction: Vec2::new(0.0, 1.0),
        initial_direction_spread: 1.0,
        initial_velocity: 100.0,
        ..particle_explosion()
    });
    //shader
    let mut direction_modifier = 0.0;
    let render_target = render_target(320, 150);
    render_target.texture.set_filter(FilterMode::Nearest);
    let material = load_material(
        ShaderSource::Glsl {
            vertex: VERTEX_SHADER,
            fragment: FRAGMENT_SHADER,
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("iResolution", UniformType::Float2),
                UniformDesc::new("direction_modifier", UniformType::Float1),
            ],
            ..Default::default()
        },
    )
    .unwrap();
    //游戏循环
    loop {
        //设置背景颜色
        clear_background(DARKGREEN);
        material.set_uniform("iResolution", (screen_width(), screen_height()));
        material.set_uniform("direction_modifier", direction_modifier);
        gl_use_material(&material);
        draw_texture_ex(
            &render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        gl_use_default_material();
        //当前帧和上一帧的时间间隔
        let delta_time = get_frame_time();
        match game_state {
            GameState::MianMenu => {
                if is_key_pressed(KeyCode::Escape) {
                    std::process::exit(0);
                }
                if is_key_pressed(KeyCode::Space) {
                    circle.x = screen_width() / 2.0;
                    circle.y = screen_height() / 2.0;
                    score = 0;
                    squares.clear();
                    bullets.clear();
                    explosions.clear();
                    game_state = GameState::Playing;
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
                for square in &mut squares {
                    square.y += delta_time * square.speed;
                }
                for square in &squares {
                    draw_rectangle(square.x, square.y, square.size, square.size, square.color);
                }
                let name = "Space Ship";
                let name_dimensions = measure_text(name, None, 100, 1.0);
                draw_text(
                    name,
                    screen_width() / 2.0 - name_dimensions.width / 2.0,
                    name_dimensions.offset_y + 20.0,
                    100.0,
                    WHITE,
                );
                let text = "Press space";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
            }
            GameState::GameOver => {
                let text = "GAME OVER!";
                let text_dimension = measure_text(text, None, 64, 1.0);
                draw_text(
                    text,
                    (screen_width() - text_dimension.width) / 2.0,
                    screen_height() / 2.0,
                    64.0,
                    WHITE,
                );
                let tip = "press space to main menu";
                let tip_dimension = measure_text(tip, None, 32, 1.0);
                draw_text(
                    tip,
                    (screen_width() - tip_dimension.width) / 2.0,
                    screen_height() / 2.0 + text_dimension.height + 10.0,
                    32.0,
                    WHITE,
                );
                if score == high_score {
                    let congratulation = "new highscore, congratulation";
                    let congratulation_dimension = measure_text(congratulation, None, 32, 1.0);
                    draw_text(
                        congratulation,
                        (screen_width() - congratulation_dimension.width) / 2.0,
                        screen_height() / 2.0 + text_dimension.height + tip_dimension.height + 20.0,
                        32.0,
                        WHITE,
                    );
                }
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::MianMenu;
                }
            }
            GameState::Pause => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Playing;
                }
                draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
                }
                for square in &squares {
                    draw_rectangle(square.x, square.y, square.size, square.size, square.color);
                }
                let text = "Paused. Press space to continue";
                let text_dimensions = measure_text(text, None, 50, 1.0);
                draw_text(
                    text,
                    screen_width() / 2.0 - text_dimensions.width / 2.0,
                    screen_height() / 2.0,
                    50.0,
                    WHITE,
                );
                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High Score: {}", high_score);
                let text_dimension = measure_text(highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimension.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            }
            GameState::Playing => {
                //圆的移动
                if is_key_down(KeyCode::W) {
                    circle.y -= MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::S) {
                    circle.y += MOVEMENT_SPEED * delta_time;
                }
                if is_key_down(KeyCode::A) {
                    circle.x -= MOVEMENT_SPEED * delta_time;
                    direction_modifier -= 0.1 * delta_time;
                }
                if is_key_down(KeyCode::D) {
                    circle.x += MOVEMENT_SPEED * delta_time;
                    direction_modifier += 0.1 * delta_time;
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

                //按下space暂停
                if is_key_pressed(KeyCode::Escape) {
                    game_state = GameState::Pause;
                }
                //按下空格生成子弹 间隔0.1秒一次
                if is_key_down(KeyCode::Space) && (get_time() - shoot_time) >= SHOOT_INTERVAL {
                    shoot_time = get_time();
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
                explosions.retain(|(explosion, _)| explosion.config.emitting);

                //移除碰撞过的方块和子弹
                squares.retain(|square| !square.collided);
                bullets.retain(|bullet| !bullet.collided);

                //圆和方块的碰撞检测
                if squares
                    .iter()
                    .any(|square| circle.circle_collides_with(square))
                {
                    if score == high_score {
                        fs::write("highscore.dat", high_score.to_string()).ok();
                    }
                    game_state = GameState::GameOver
                }

                //子弹和方块的碰撞检测
                for bullet in bullets.iter_mut() {
                    for square in squares.iter_mut() {
                        if bullet.circle_collides_with(square) {
                            bullet.collided = true;
                            square.collided = true;
                            score += square.size.round() as u32;
                            high_score = high_score.max(score);
                            explosions.push((
                                Emitter::new(EmitterConfig {
                                    amount: square.size.round() as u32 * 2,
                                    ..particle_explosion()
                                }),
                                vec2(square.x, square.y),
                            ));
                        }
                    }
                }

                //图像绘制
                draw_circle(circle.x, circle.y, circle.size / 2.0, circle.color);
                circle_explosion.draw(Vec2::new(circle.x, circle.y + circle.size / 2.0));
                for bullet in &bullets {
                    draw_circle(bullet.x, bullet.y, bullet.size / 2.0, bullet.color);
                }
                for square in &squares {
                    draw_rectangle(square.x, square.y, square.size, square.size, square.color);
                }
                for (explosion, coords) in explosions.iter_mut() {
                    explosion.draw(*coords);
                }
                draw_text(
                    format!("Score: {}", score).as_str(),
                    10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
                let highscore_text = format!("High Score: {}", high_score);
                let text_dimension = measure_text(highscore_text.as_str(), None, 25, 1.0);
                draw_text(
                    highscore_text.as_str(),
                    screen_width() - text_dimension.width - 10.0,
                    35.0,
                    25.0,
                    WHITE,
                );
            }
        }
        next_frame().await;
    }
}
fn particle_explosion() -> EmitterConfig {
    EmitterConfig {
        local_coords: false,
        one_shot: true,
        emitting: true,
        lifetime: 0.6,
        lifetime_randomness: 0.3,
        explosiveness: 0.65,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 300.0,
        initial_velocity_randomness: 0.8,
        size: 3.0,
        size_randomness: 0.3,
        colors_curve: ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
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

enum GameState {
    MianMenu,
    Playing,
    Pause,
    GameOver,
}
