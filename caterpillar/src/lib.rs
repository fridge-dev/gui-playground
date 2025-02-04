use better_quad::bq::TextAlignment;
use better_quad::{bq, mq};
use better_quad::{
    bq::{FpsCounter, Timestamp},
    StatefulGui,
};
use std::collections::LinkedList;
use std::ops::Add;
use std::time::Duration;

const SQUARES: i16 = 16;

/// Tick movement every period
const INITIAL_MOVEMENT_TICK_SPEED: Duration = Duration::from_millis(200);
/// Gradually move faster.
/// Current config = `[0.2, 0.18, 0.162, 0.1458, ...]`
const MOVEMENT_TICK_SPEED_MULTIPLICATIVE_FACTOR: f64 = 0.9;

type Point = (i16, i16);

pub struct SnakeGameState {
    snake: Snake,
    fruit: Point,
    score: u64,
    movement_tick_speed: Duration,
    last_update: Timestamp,
    game_over: bool,
    fps_counter: FpsCounter,
}

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    next_dir: Direction,
    queued_dir: Option<Direction>,
    next_dir_locked: bool,
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

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

impl Default for SnakeGameState {
    fn default() -> Self {
        Self {
            snake: Snake {
                head: (0, 0),
                body: LinkedList::new(),
                next_dir: Direction::Right,
                queued_dir: None,
                next_dir_locked: false,
            },
            fruit: (
                mq::rand::gen_range(0, SQUARES),
                mq::rand::gen_range(0, SQUARES),
            ),
            score: 0,
            movement_tick_speed: INITIAL_MOVEMENT_TICK_SPEED,
            last_update: Timestamp::now(),
            game_over: false,
            fps_counter: FpsCounter::new(),
        }
    }
}

impl StatefulGui for SnakeGameState {
    fn update(&mut self, now: Timestamp) {
        evaluate_game(self, now);
    }

    fn draw(&self) {
        draw_game(self);
    }
}

fn evaluate_game(state: &mut SnakeGameState, now: Timestamp) {
    state.fps_counter.tick_frame(now);

    if state.game_over {
        if mq::is_key_down(mq::KeyCode::Enter) {
            *state = SnakeGameState::default();
        }
        return;
    }

    // game_over == false

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
    if now - state.last_update > state.movement_tick_speed {
        state.last_update = now;
        state.snake.body.push_front(state.snake.head);
        state.snake.head = state.snake.head + state.snake.next_dir;
        if state.snake.head == state.fruit {
            // Grow!
            state.fruit = (
                mq::rand::gen_range(0, SQUARES),
                mq::rand::gen_range(0, SQUARES),
            );
            state.score += 100;
            state.movement_tick_speed = Duration::from_secs_f64(
                state.movement_tick_speed.as_secs_f64() * MOVEMENT_TICK_SPEED_MULTIPLICATIVE_FACTOR,
            );
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
}

// TODO: actual key-down press queuing (like fancy mech keyboards) would be dope and feel really good.
fn get_dir_key_down() -> Option<Direction> {
    if mq::is_key_down(mq::KeyCode::Right) {
        Some(Direction::Right)
    } else if mq::is_key_down(mq::KeyCode::Left) {
        Some(Direction::Left)
    } else if mq::is_key_down(mq::KeyCode::Up) {
        Some(Direction::Up)
    } else if mq::is_key_down(mq::KeyCode::Down) {
        Some(Direction::Down)
    } else {
        None
    }
}

fn draw_game(state: &SnakeGameState) {
    if !state.game_over {
        // Draw game-in-progress state
        mq::clear_background(mq::LIGHTGRAY);

        let game_size = mq::screen_width().min(mq::screen_height());
        // 20 = total padding (10 lrud)
        let offset_x = (mq::screen_width() - game_size + 50.) / 2.;
        let offset_y = (mq::screen_height() - game_size + 50.) / 2.;
        let sq_size = (game_size - 50.) / SQUARES as f32;

        mq::draw_rectangle(
            offset_x,
            offset_y,
            game_size - 50.,
            game_size - 50.,
            mq::WHITE,
        );

        for i in 1..SQUARES {
            mq::draw_line(
                offset_x,
                offset_y + sq_size * i as f32,
                mq::screen_width() - offset_x,
                offset_y + sq_size * i as f32,
                2.,
                mq::LIGHTGRAY,
            );
        }

        for i in 1..SQUARES {
            mq::draw_line(
                offset_x + sq_size * i as f32,
                offset_y,
                offset_x + sq_size * i as f32,
                mq::screen_height() - offset_y,
                2.,
                mq::LIGHTGRAY,
            );
        }

        mq::draw_circle(
            offset_x + state.snake.head.0 as f32 * sq_size,
            offset_y + state.snake.head.1 as f32 * sq_size,
            sq_size / 2.5,
            mq::DARKGREEN,
        );

        for (x, y) in &state.snake.body {
            mq::draw_circle(
                offset_x + *x as f32 * sq_size,
                offset_y + *y as f32 * sq_size,
                sq_size / 2.5,
                mq::LIME,
            );
        }

        mq::draw_circle(
            offset_x + state.fruit.0 as f32 * sq_size,
            offset_y + state.fruit.1 as f32 * sq_size,
            sq_size / 2.5,
            mq::GOLD,
        );
    } else {
        // Draw game-over screen.
        mq::clear_background(mq::WHITE);
        bq::draw_text(
            "Game Over. Press [enter] to play again.",
            TextAlignment::Left,
            None,
            30,
            mq::DARKGRAY,
            bq::TextAnchorPoint::window_centered(),
            None,
        );
    }

    // Unconditionally draw debug info
    mq::draw_text(
        format!("mqFPS: {}fps", mq::get_fps()).as_str(),
        10.,
        50.,
        50.,
        mq::DARKGRAY,
    );
    let my_fps = state.fps_counter.fps();
    let my_fps_period = state.fps_counter.duration_of_last_period();
    mq::draw_text(
        format!("myFPS: {my_fps}fps ({}s)", my_fps_period.as_secs_f64()).as_str(),
        10.,
        110.,
        50.,
        mq::DARKGRAY,
    );
}
