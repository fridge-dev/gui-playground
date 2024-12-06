use std::collections::LinkedList;
use std::ops::Add;
use macroquad::color::{DARKGRAY, DARKGREEN, GOLD, LIGHTGRAY, LIME, WHITE};
use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{clear_background, draw_line, draw_rectangle, draw_text, measure_text, next_frame, screen_height, screen_width};
use macroquad::rand;
use macroquad::time::{get_fps, get_time};

const SQUARES: i16 = 16;

/// "Speed" = number of seconds to wait before evaluating movement
/// Speed = `[0.3, 0.27, 0.243, 0.2187, ...]`
const INITIAL_SPEED: f64 = 0.2;
const FRUIT_SPEED_MULTIPLICATIVE_FACTOR: f64 = 0.9;

type Point = (i16, i16);

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Right => (self.0 + 1, self.1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Up => (self.0, self.1 - 1),
            Direction::Down => (self.0, self.1 + 1),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    next_dir: Direction,
    queued_dir: Option<Direction>,
    next_dir_locked: bool,
}

struct GameState {
    snake: Snake,
    fruit: Point,
    score: u64,
    speed: f64,
    last_update: f64,
    game_over: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            snake: Snake {
                head: (0, 0),
                body: LinkedList::new(),
                next_dir: Direction::Right,
                queued_dir: None,
                next_dir_locked: false,
            },
            fruit: (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES)),
            score: 0,
            speed: INITIAL_SPEED,
            last_update: get_time(),
            game_over: false,
        }
    }
}

pub async fn run_main() {
    let mut state = GameState::default();
    let mut fps_counter = FpsCounter::new();

    loop {
        evaluate_game(&mut state);
        draw_game(&state, &mut fps_counter);
        next_frame().await;
    }
}

fn evaluate_game(state: &mut GameState) {
    if !state.game_over {
        let dir_key_down = get_dir_key_down();
        if !state.snake.next_dir_locked {
            // check for change direction
            match dir_key_down {
                Some(Direction::Right) => {
                    if state.snake.next_dir != Direction::Left {
                        state.snake.next_dir = Direction::Right;
                        state.snake.next_dir_locked = true;
                    }
                }
                Some(Direction::Left) => {
                    if state.snake.next_dir != Direction::Right {
                        state.snake.next_dir = Direction::Left;
                        state.snake.next_dir_locked = true;
                    }
                }
                Some(Direction::Up) => {
                    if state.snake.next_dir != Direction::Down {
                        state.snake.next_dir = Direction::Up;
                        state.snake.next_dir_locked = true;
                    }
                }
                Some(Direction::Down) => {
                    if state.snake.next_dir != Direction::Up {
                        state.snake.next_dir = Direction::Down;
                        state.snake.next_dir_locked = true;
                    }
                }
                None => {}
            }
        }

        // Store the queued direction if the next move is already locked in
        if state.snake.next_dir_locked {
            match dir_key_down {
                Some(Direction::Right) => {
                    // TODO refactor both this and above to not allow re-sending the same input (QoL)
                    if state.snake.next_dir != Direction::Left {
                        state.snake.queued_dir = Some(Direction::Right);
                    }
                }
                Some(Direction::Left) => {
                    if state.snake.next_dir != Direction::Right {
                        state.snake.queued_dir = Some(Direction::Left);
                    }
                }
                Some(Direction::Up) => {
                    if state.snake.next_dir != Direction::Down {
                        state.snake.queued_dir = Some(Direction::Up);
                    }
                }
                Some(Direction::Down) => {
                    if state.snake.next_dir != Direction::Up {
                        state.snake.queued_dir = Some(Direction::Down);
                    }
                }
                None => {}
            }
        }

        // apply movement if time has elapsed
        if get_time() - state.last_update > state.speed {
            state.last_update = get_time();
            state.snake.body.push_front(state.snake.head);
            state.snake.head = state.snake.head + state.snake.next_dir;
            if state.snake.head == state.fruit {
                // Grow!
                state.fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                state.score += 100;
                state.speed *= FRUIT_SPEED_MULTIPLICATIVE_FACTOR;
            } else {
                // Normal movement.
                state.snake.body.pop_back();
            }

            // Apply queued dir, which is guaranteed to be valid compared to last applied dir.
            if let Some(qd) = state.snake.queued_dir {
                state.snake.next_dir = qd;
                state.snake.queued_dir = None;
            }

            // Check for wall collision
            if state.snake.head.0 < 0
                || state.snake.head.1 < 0
                || state.snake.head.0 >= SQUARES
                || state.snake.head.1 >= SQUARES
            {
                state.game_over = true;
            }
            // Check for body collision
            for (x, y) in &state.snake.body {
                if *x == state.snake.head.0 && *y == state.snake.head.1 {
                    state.game_over = true;
                }
            }
            state.snake.next_dir_locked = false;
        }
    } else {
        if is_key_down(KeyCode::Enter) {
            *state = GameState::default();
        }
    }
}

// TODO: actual key-down press queuing (like fancy mech keyboards) would be dope and feel really good.
fn get_dir_key_down() -> Option<Direction> {
    if is_key_down(KeyCode::Right) {
        Some(Direction::Right)
    } else if is_key_down(KeyCode::Left) {
        Some(Direction::Left)
    } else if is_key_down(KeyCode::Up) {
        Some(Direction::Up)
    } else if is_key_down(KeyCode::Down) {
        Some(Direction::Down)
    } else {
        None
    }
}

fn draw_game(state: &GameState, fps_counter: &mut FpsCounter) {
    if !state.game_over {
        // Draw game-in-progress state
        clear_background(LIGHTGRAY);

        let game_size = screen_width().min(screen_height());
        // 20 = total padding (10 lrud)
        let offset_x = (screen_width() - game_size + 50.) / 2.;
        let offset_y = (screen_height() - game_size + 50.) / 2.;
        let sq_size = (game_size - 50.) / SQUARES as f32;

        draw_rectangle(offset_x, offset_y, game_size - 50., game_size - 50., WHITE);

        for i in 1..SQUARES {
            draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                screen_height() - offset_y,
                2.,
                LIGHTGRAY,
            );
        }

        draw_rectangle(
            offset_x + state.snake.head.0 as f32 * sq_size,
            offset_y + state.snake.head.1 as f32 * sq_size,
            sq_size,
            sq_size,
            DARKGREEN,
        );

        for (x, y) in &state.snake.body {
            draw_rectangle(
                offset_x + *x as f32 * sq_size,
                offset_y + *y as f32 * sq_size,
                sq_size,
                sq_size,
                LIME,
            );
        }

        draw_rectangle(
            offset_x + state.fruit.0 as f32 * sq_size,
            offset_y + state.fruit.1 as f32 * sq_size,
            sq_size,
            sq_size,
            GOLD,
        );
    } else {
        // Draw game-over screen.
        clear_background(WHITE);
        let text = "Game Over. Press [enter] to play again.";
        let font_size = 30.;

        // Center the text
        let text_size = measure_text(text, None, font_size as _, 1.0);
        draw_text(
            text,
            screen_width() / 2. - text_size.width / 2.,
            screen_height() / 2. + text_size.height / 2.,
            font_size,
            DARKGRAY,
        );
    }

    // Unconditionally draw debug info
    draw_text(format!("mqFPS: {}fps", get_fps()).as_str(), 10., 50., 50., DARKGRAY);
    let (my_fps, my_fps_dur) = fps_counter.count_and_get();
    draw_text(format!("myFPS: {my_fps}fps ({my_fps_dur}s)").as_str(), 10., 110., 50., DARKGRAY);
}

struct FpsCounter {
    last_fps_update_time: f64,
    last_update_duration: f64,
    last_fps: u32,
    frames_this_update: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_fps_update_time: get_time(),
            last_update_duration: 0.0,
            last_fps: 0,
            frames_this_update: 0,
        }
    }

    pub fn count_and_get(&mut self) -> (u32, f64) {
        self.frames_this_update += 1;
        let now = get_time();
        let delta = now - self.last_fps_update_time;
        if delta >= 1.0 {
            self.last_fps_update_time = now;
            self.last_update_duration = delta;
            self.last_fps = (self.frames_this_update as f64 / delta) as u32;
            self.frames_this_update = 0;
        }

        (self.last_fps, self.last_update_duration)
    }
}