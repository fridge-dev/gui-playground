use crate::password::{Password, PasswordSource};
use crate::victory_mouse_animation::VictoryMouseAnimations;
use better_quad::bq::{BetterKeyCode, TextAnchorPoint};
use better_quad::{
    bq::{self, FpsCounter, TextBackground, Timestamp},
    StatefulGui,
};
use macroquad::prelude as mq;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::Duration;

mod victory_mouse_animation;

// Control consts
const KEY_SUBMIT: mq::KeyCode = mq::KeyCode::Space;
const KEY_REPLAY_PASSWORD: mq::KeyCode = mq::KeyCode::R;
const KEY_NEW_PASSWORD: mq::KeyCode = mq::KeyCode::Space;
const KEY_TOGGLE_NUMBER_OVERLAY: mq::KeyCode = mq::KeyCode::N;
const KEY_PLAYER_EDIT_PASSWORD: mq::KeyCode = mq::KeyCode::P;
const KEY_COPY_SEED: mq::KeyCode = mq::KeyCode::S;

// Game logic consts
const COLOR_PALETTE: [Color; 6] = [
    Color::Red,
    Color::Orange,
    Color::Yellow,
    // Color::NeonGreen,
    Color::Green,
    // Color::LightBlue,
    Color::Blue,
    Color::Purple,
    // Color::Pink,
];
const NUM_SLOTS_PER_ROW: usize = 4;
const NUM_GUESSES: usize = 8;

// Draw consts
const CURSOR_SIZE: f32 = 30.0;
const CURSOR_RADIUS: f32 = CURSOR_SIZE / 2.0;
const VICTORY_MULTI_CURSOR_OFFSET: f32 = CURSOR_SIZE;
const SLOTS_PER_ROW_F32: f32 = NUM_SLOTS_PER_ROW as f32;
const BOARD_OFFSET_X: f32 = 20.0;
const BOARD_OFFSET_Y: f32 = 20.0;
const ROW_SEPARATOR_HEIGHT: f32 = 1.0;
const SLOT_SIZE: f32 = 50.0;
const SLOT_RADIUS: f32 = SLOT_SIZE / 2.0;
const SLOT_PADDING: f32 = 5.0;
const WORKING_BOX_THICKNESS: f32 = 7.0;
const KEY_SIZE: f32 = 18.0;
const KEY_RADIUS: f32 = KEY_SIZE / 2.0;
const PEG_SIZE: f32 = 40.0;
const PEG_RADIUS: f32 = PEG_SIZE / 2.0;
const PEG_OUTER_PADDING: f32 = 10.0;
const SLOT_PEG_FONT_SIZE: u16 = 24;
const WIN_TITLES: [&str; 8] = [
    "LUCKER DUCKER",
    "lucker ducker",
    "goated mastermind",
    "mastermind",
    "genius",
    "vic royale",
    "silly goose",
    "dangerous warbler",
];
const SEED_FONT_SIZE: u16 = 27;
const SEED_TEXT_PADDING: f32 = 3.0;

struct BoardSizeDerivedConsts {
    row_width_guess: f32,
    row_height: f32,
    key_padding: f32,
    row_width_key: f32,
    board_height: f32,
}

impl BoardSizeDerivedConsts {
    fn get() -> Self {
        let row_width_guess =
            SLOT_SIZE * SLOTS_PER_ROW_F32 + SLOT_PADDING * (SLOTS_PER_ROW_F32 + 1.0);
        let row_height = SLOT_SIZE + SLOT_PADDING * 2.0;

        // Derive key padding such that a single guess row has 2 rows of keys.
        let key_padding = (row_height - KEY_SIZE * 2.0) / 3.0;
        let num_keys_top_key_row = (SLOTS_PER_ROW_F32 / 2.0).ceil();
        let row_width_key =
            num_keys_top_key_row * KEY_SIZE + key_padding * (num_keys_top_key_row + 1.0);

        let board_height =
            row_height * (NUM_GUESSES as f32 + 1.0) + ROW_SEPARATOR_HEIGHT * NUM_GUESSES as f32;

        Self {
            row_width_guess,
            row_height,
            key_padding,
            row_width_key,
            board_height,
        }
    }
}

// Features to do:
// - display controls
// - dynamically variable game params (num colors, num slots, num guesses)
// - pvp (https://docs.rs/gloo-net/latest/gloo_net )
// - display rules
// - add ability to seed run
pub struct MastermindGame {
    state: GameState,
    password: Password,
    // head: first guess; tail: most recent guess
    history: Vec<CompleteRow>,
    mouse_color: Color,
    // Work around annoying (0, 0) initialization issue with mq.
    mouse_moved: bool,
    number_overlay: NumberOverlay,
    fps_counter: FpsCounter,
}

enum GameState {
    InProgress {
        start_time: Timestamp,
        working_row: [Option<Color>; NUM_SLOTS_PER_ROW],
    },
    EditPassword,
    Victory {
        total_time: Duration,
        // Put the big struct in a box
        mouse_animations: Box<VictoryMouseAnimations>,
    },
    TooManyGuesses,
}

/// Separate mod to enforce RNG state and immutability.
mod password {
    use crate::{Color, COLOR_PALETTE, NUM_SLOTS_PER_ROW};
    use better_quad::bq;

    #[derive(Copy, Clone)]
    pub(super) struct Password {
        password: [Color; NUM_SLOTS_PER_ROW],
        source: PasswordSource,
    }

    #[derive(Copy, Clone)]
    pub(super) enum PasswordSource {
        Random { seed: u64 },
        Player,
    }

    impl Password {
        pub(super) fn random() -> Self {
            bq::randomize_rand_seed();
            Self {
                password: Color::random_array(&COLOR_PALETTE),
                source: PasswordSource::Random {
                    seed: bq::get_last_set_rand_seed(),
                },
            }
        }

        pub(super) fn player_specified(password: [Color; NUM_SLOTS_PER_ROW]) -> Self {
            Self {
                password,
                source: PasswordSource::Player,
            }
        }

        pub(super) fn password(&self) -> &[Color; NUM_SLOTS_PER_ROW] {
            &self.password
        }

        pub(super) fn source(&self) -> PasswordSource {
            self.source
        }
    }
}

impl GameState {
    fn new_game() -> Self {
        bq::randomize_rand_seed();
        Self::InProgress {
            start_time: Timestamp::now(),
            working_row: [None; NUM_SLOTS_PER_ROW],
        }
    }
}

impl StatefulGui for MastermindGame {
    fn main_conf() -> mq::Conf {
        mq::Conf {
            window_title: "Mastermind".to_string(),
            // TODO less brittle const
            window_width: 480,
            window_height: 770,
            ..Default::default()
        }
    }

    fn update(&mut self, now: Timestamp) {
        self.update(now);
    }

    fn draw(&self) {
        self.draw();
    }
}

impl Default for MastermindGame {
    fn default() -> Self {
        Self::new()
    }
}

impl MastermindGame {
    fn new() -> Self {
        Self {
            state: GameState::new_game(),
            password: Password::random(),
            history: Vec::with_capacity(NUM_GUESSES),
            mouse_color: COLOR_PALETTE[0],
            mouse_moved: false,
            number_overlay: NumberOverlay::Off,
            fps_counter: FpsCounter::new(),
        }
    }

    fn reset_with_same_password(&mut self) {
        self.state = GameState::new_game();
        self.history = Vec::with_capacity(NUM_GUESSES);
    }

    fn reset_with_new_password(&mut self) {
        self.reset_with_same_password();
        self.password = Password::random();
    }

    fn update(&mut self, now: Timestamp) {
        self.fps_counter.tick_frame(now);

        if !self.mouse_moved && mq::mouse_position() != (0.0, 0.0) {
            self.mouse_moved = true;
        }

        if mq::is_key_pressed(KEY_TOGGLE_NUMBER_OVERLAY) {
            self.number_overlay = match self.number_overlay {
                NumberOverlay::On => NumberOverlay::Off,
                NumberOverlay::Off => NumberOverlay::On,
            }
        }

        if mq::is_key_pressed(KEY_COPY_SEED) {
            // freaking clipboard isn't implemented anywhere except windows. Idk if this will work.
            mq::miniquad::window::clipboard_set(&format!("{}", bq::get_last_set_rand_seed()));
        }

        self.apply_state_specific_updates(now);
    }

    fn apply_state_specific_updates(&mut self, now: Timestamp) {
        match &mut self.state {
            GameState::InProgress {
                working_row,
                start_time,
            } => {
                // Update mouse color if needed
                if let Some(new_color) = Self::get_mouse_color_update() {
                    self.mouse_color = new_color;
                }

                // Set working row's color if needed
                if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                    let (mouse_x, mouse_y) = mq::mouse_position();
                    if let Some((i, j)) = guess_circles_ij::get_containing_ij(mouse_x, mouse_y) {
                        if j == NUM_GUESSES - self.history.len() {
                            working_row[i] = Some(self.mouse_color);
                        }
                    }
                }
                // Unset working row's color if needed
                if mq::is_mouse_button_pressed(mq::MouseButton::Right) {
                    let (mouse_x, mouse_y) = mq::mouse_position();
                    if let Some((i, j)) = guess_circles_ij::get_containing_ij(mouse_x, mouse_y) {
                        if j == NUM_GUESSES - self.history.len() {
                            working_row[i] = None;
                        }
                    }
                }

                // Apply guess if needed
                if mq::is_key_pressed(KEY_SUBMIT) {
                    if let Some(guess) = convert_working_row_if_completed(working_row) {
                        let complete_row = evaluate_guess(guess, *self.password.password());
                        self.history.push(complete_row);

                        if complete_row.num_correct_hits == NUM_SLOTS_PER_ROW {
                            self.state = GameState::Victory {
                                total_time: now - *start_time,
                                mouse_animations: Box::new(VictoryMouseAnimations::new(
                                    COLOR_PALETTE.map(|c| c.as_mq()).to_vec(),
                                    now,
                                    VICTORY_MULTI_CURSOR_OFFSET,
                                )),
                            };
                            return;
                        }

                        if self.history.len() == NUM_GUESSES {
                            self.state = GameState::TooManyGuesses;
                            return;
                        }

                        *working_row = [None; NUM_SLOTS_PER_ROW];
                    }
                }

                // Change to password edit mode if needed
                if mq::is_key_pressed(KEY_PLAYER_EDIT_PASSWORD) {
                    let working_row_empty = !working_row.iter().any(|c| c.is_some());
                    if self.history.is_empty() && working_row_empty {
                        self.state = GameState::EditPassword;
                    }
                }
            }
            GameState::EditPassword => {
                // Update mouse color if needed
                if let Some(new_color) = Self::get_mouse_color_update() {
                    self.mouse_color = new_color;
                }

                // Set password color if needed
                if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                    let (mouse_x, mouse_y) = mq::mouse_position();
                    if let Some((i, j)) = guess_circles_ij::get_containing_ij(mouse_x, mouse_y) {
                        if j == 0 {
                            let mut password = *self.password.password();
                            password[i] = self.mouse_color;
                            self.password = Password::player_specified(password);
                        }
                    }
                }

                // Change to InProgress mode if needed
                if mq::is_key_pressed(KEY_PLAYER_EDIT_PASSWORD) {
                    self.state = GameState::new_game();
                }
            }
            GameState::TooManyGuesses => {
                if mq::is_key_pressed(KEY_REPLAY_PASSWORD) {
                    self.reset_with_same_password();
                } else if mq::is_key_pressed(KEY_NEW_PASSWORD) {
                    self.reset_with_new_password();
                }
            }
            GameState::Victory {
                mouse_animations, ..
            } => {
                mouse_animations.tick(now);

                if mq::is_key_pressed(KEY_REPLAY_PASSWORD) {
                    self.reset_with_same_password();
                } else if mq::is_key_pressed(KEY_NEW_PASSWORD) {
                    self.reset_with_new_password();
                }
            }
        }
    }

    fn get_mouse_color_update() -> Option<Color> {
        if let Some(color) = Self::get_color_from_key_press() {
            return Some(color);
        }

        Self::get_color_from_mouse_click()
    }

    fn get_color_from_key_press() -> Option<Color> {
        let num_keys = [
            mq::KeyCode::Key1,
            mq::KeyCode::Key2,
            mq::KeyCode::Key3,
            mq::KeyCode::Key4,
            mq::KeyCode::Key5,
            mq::KeyCode::Key6,
            mq::KeyCode::Key7,
            mq::KeyCode::Key8,
            mq::KeyCode::Key9,
        ];

        let mut i = 0;
        loop {
            if i >= num_keys.len() || i >= COLOR_PALETTE.len() {
                return None;
            }

            if mq::is_key_pressed(num_keys[i]) {
                return Some(COLOR_PALETTE[i]);
            }

            i += 1;
        }
    }

    fn get_color_from_mouse_click() -> Option<Color> {
        if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
            let (mouse_x, mouse_y) = mq::mouse_position();
            if let Some(peg_i) = pegs_ij::get_containing_i(mouse_x, mouse_y) {
                return Some(COLOR_PALETTE[peg_i]);
            }
        }

        None
    }

    fn draw(&self) {
        mq::clear_background(mq::DARKBROWN);
        // Between BROWN and BEIGE
        let board_color = mq::Color::new(0.70, 0.60, 0.46, 1.0);

        let BoardSizeDerivedConsts {
            row_width_guess,
            row_height,
            key_padding,
            row_width_key,
            board_height,
        } = BoardSizeDerivedConsts::get();

        // Board
        mq::draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            row_width_guess + row_width_key,
            board_height,
            board_color,
        );

        // Vertical separator of Guess-Key
        mq::draw_rectangle(
            BOARD_OFFSET_X + row_width_guess,
            BOARD_OFFSET_Y,
            ROW_SEPARATOR_HEIGHT, // re-use "height" const for width :P
            board_height,
            mq::BLACK,
        );

        // Horizontal separators of Guess rows - Line goes at *bottom* of first n-1 rows
        for j in 0..NUM_GUESSES {
            let j = j as f32;
            mq::draw_rectangle(
                BOARD_OFFSET_X,
                BOARD_OFFSET_Y + row_height * (j + 1.0) + ROW_SEPARATOR_HEIGHT * j,
                row_width_guess + row_width_key,
                ROW_SEPARATOR_HEIGHT,
                mq::BLACK,
            );
        }

        // Password - overwrite space already drawn with Board
        let password_rectangle_color = match &self.state {
            GameState::InProgress { .. } => mq::BLACK,
            GameState::EditPassword => board_color,
            GameState::Victory { .. } => mq::GREEN,
            GameState::TooManyGuesses => mq::RED,
        };
        mq::draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            row_width_guess,
            row_height,
            password_rectangle_color,
        );

        // Password colors
        match self.state {
            GameState::InProgress { .. } => {
                for i in 0..self.password.password().len() {
                    guess_circles_ij::draw_password_text_overlay(i, 0);
                }
            }
            GameState::EditPassword | GameState::Victory { .. } | GameState::TooManyGuesses => {
                for (i, color) in self.password.password().iter().enumerate() {
                    guess_circles_ij::draw(i, 0, *color, self.number_overlay);
                }
            }
        }

        // Guesses - colored - history
        for (j, row) in self.history.iter().enumerate() {
            let j = NUM_GUESSES - j;
            for (i, color) in row.guess.iter().enumerate() {
                guess_circles_ij::draw(i, j, *color, self.number_overlay);
            }
        }

        // Guesses - colored - working
        if let GameState::InProgress { working_row, .. } = &self.state {
            let j = NUM_GUESSES - self.history.len();
            for (i, opt_color) in working_row.iter().enumerate() {
                if let Some(color) = opt_color {
                    guess_circles_ij::draw(i, j, *color, self.number_overlay);
                }
            }

            // Gold working box
            let j = (NUM_GUESSES - self.history.len()) as f32;
            mq::draw_rectangle_lines(
                BOARD_OFFSET_X,
                BOARD_OFFSET_Y + (row_height + ROW_SEPARATOR_HEIGHT) * j,
                row_width_guess,
                row_height,
                WORKING_BOX_THICKNESS,
                mq::GOLD,
            );
        }

        // Guesses - outlines
        for i in 0..NUM_SLOTS_PER_ROW {
            for j in 0..=NUM_GUESSES {
                guess_circles_ij::draw_outline(i, j);
            }
        }

        // Keys - colored
        for (j, row) in self.history.iter().enumerate() {
            let j = (NUM_GUESSES - j) as f32;
            let mut key_offset_index = 0;
            for _ in 0..row.num_correct_hits {
                let (key_offset_x, key_offset_y) =
                    get_key_offset(key_offset_index, NUM_SLOTS_PER_ROW, key_padding, KEY_RADIUS);
                bq::draw_circle(
                    BOARD_OFFSET_X + row_width_guess + key_offset_x,
                    BOARD_OFFSET_Y + (row_height + ROW_SEPARATOR_HEIGHT) * j + key_offset_y,
                    KEY_RADIUS,
                    mq::WHITE,
                );
                key_offset_index += 1;
            }

            for _ in 0..row.num_misplaced_hits {
                let (key_offset_x, key_offset_y) =
                    get_key_offset(key_offset_index, NUM_SLOTS_PER_ROW, key_padding, KEY_RADIUS);
                let medium_grey = mq::Color::new(0.38, 0.38, 0.38, 1.00);
                bq::draw_circle(
                    BOARD_OFFSET_X + row_width_guess + key_offset_x,
                    BOARD_OFFSET_Y + (row_height + ROW_SEPARATOR_HEIGHT) * j + key_offset_y,
                    KEY_RADIUS,
                    medium_grey,
                );
                key_offset_index += 1;
            }
        }

        // Keys - outlines
        #[allow(clippy::needless_range_loop)]
        for i in 0..NUM_SLOTS_PER_ROW {
            let (key_offset_x, key_offset_y) =
                get_key_offset(i, NUM_SLOTS_PER_ROW, key_padding, KEY_RADIUS);
            for j in 1..=NUM_GUESSES {
                let j = j as f32;
                bq::draw_circle_outline(
                    BOARD_OFFSET_X + row_width_guess + key_offset_x,
                    BOARD_OFFSET_Y + (row_height + ROW_SEPARATOR_HEIGHT) * j + key_offset_y,
                    KEY_RADIUS,
                    1.0,
                    mq::GOLD,
                );
            }
        }

        // Pegs
        let pegs_y = pegs_ij::compute_y_coordinate();
        for (i, color) in COLOR_PALETTE.iter().enumerate() {
            let x = pegs_ij::compute_x_coordinate(i);
            bq::draw_circle(x, pegs_y, PEG_RADIUS, color.as_mq());
            bq::draw_text_left_aligned(
                format!("{}", i + 1),
                None,
                SLOT_PEG_FONT_SIZE,
                mq::BLACK,
                TextAnchorPoint::Center { x, y: pegs_y },
                None,
            );
        }

        // Text - controls
        let controls_text = format!(
            "Press [number key] to select color\n\
            Press [{}] to submit guess\n\
            Press [{}] to toggle numbers display\n\
            Press [{}] to edit password",
            KEY_SUBMIT.to_lowercase(),
            KEY_TOGGLE_NUMBER_OVERLAY.to_lowercase(),
            KEY_PLAYER_EDIT_PASSWORD.to_lowercase(),
        );
        bq::draw_text_left_aligned(
            controls_text,
            None,
            25,
            mq::BLACK,
            TextAnchorPoint::TopLeft {
                x: BOARD_OFFSET_X,
                y: pegs_y + PEG_RADIUS + PEG_OUTER_PADDING + 5.0,
            },
            Some(TextBackground {
                color: mq::Color::new(1.0, 1.0, 1.0, 0.7),
                x_padding: 2.5,
                y_padding: 2.5,
            }),
        );

        // Text - new game
        let new_game_text = format!(
            "Press [{}] to replay the same password.\nPress [{}] for a new password.",
            KEY_REPLAY_PASSWORD.to_lowercase(),
            KEY_NEW_PASSWORD.to_lowercase(),
        );
        let new_game_text_background = TextBackground {
            color: mq::Color::new(0.78, 0.78, 0.78, 0.8),
            x_padding: 2.0,
            y_padding: 2.0,
        };
        match &self.state {
            GameState::InProgress { .. } | GameState::EditPassword => {}
            GameState::Victory { total_time, .. } => {
                let win_title = WIN_TITLES
                    .get(self.history.len() - 1)
                    .unwrap_or(WIN_TITLES.last().unwrap());
                bq::draw_text_left_aligned(
                    format!(
                        "You won in {} guesses! You are a {}!\nTime: {}\n\n{new_game_text}",
                        self.history.len(),
                        win_title,
                        format_duration(*total_time)
                    ),
                    None,
                    25,
                    mq::DARKGREEN,
                    TextAnchorPoint::window_centered(),
                    Some(new_game_text_background),
                );
            }
            GameState::TooManyGuesses => {
                bq::draw_text_left_aligned(
                    format!("You lose lmao\n\n{new_game_text}"),
                    None,
                    25,
                    mq::RED,
                    TextAnchorPoint::window_centered(),
                    Some(new_game_text_background),
                );
            }
        }

        // FPS
        let fps_text_top_left = bq::draw_fps_text_bottom_right(&self.fps_counter);

        // Seed
        let seed_text = match self.password.source() {
            PasswordSource::Random { seed } => format!("Seed: {seed}"),
            PasswordSource::Player => "Seed: N/A".to_string(),
        };
        bq::draw_text_left_aligned(
            seed_text,
            None,
            SEED_FONT_SIZE,
            mq::WHITE,
            TextAnchorPoint::BottomRight {
                x: fps_text_top_left.rect_x,
                y: mq::screen_height(),
            },
            Some(TextBackground {
                color: mq::BLACK,
                x_padding: SEED_TEXT_PADDING,
                y_padding: SEED_TEXT_PADDING,
            }),
        );

        // Mouse
        let (mouse_x, mouse_y) = mq::mouse_position();
        let mouse_on_screen = (0.0..=mq::screen_width()).contains(&mouse_x)
            && (0.0..=mq::screen_height()).contains(&mouse_y);
        if mouse_on_screen && self.mouse_moved {
            mq::show_mouse(false);

            match &self.state {
                GameState::InProgress { .. }
                | GameState::EditPassword
                | GameState::TooManyGuesses => {
                    draw_cursor(mouse_x, mouse_y, self.mouse_color.as_mq());
                }
                GameState::Victory {
                    mouse_animations, ..
                } => {
                    mouse_animations.draw(mouse_x, mouse_y);
                }
            };
        } else {
            mq::show_mouse(true);
        }
    }

    #[allow(dead_code)] // for debug/test purposes
    fn draw_ij_coordinates_on_cursor(mouse_x: f32, mouse_y: f32) {
        if let Some((i, j)) = guess_circles_ij::get_containing_ij(mouse_x, mouse_y) {
            mq::draw_text(
                &format!("({i}, {j})"),
                mouse_x - 10.0,
                mouse_y - 10.0,
                15.0,
                mq::GREEN,
            );
        }
    }
}

fn draw_cursor(x: f32, y: f32, color: mq::Color) {
    bq::draw_circle(x, y, CURSOR_RADIUS, color);
    bq::draw_circle(x, y, 1.0, mq::BLACK);
    bq::draw_circle_outline(x, y, CURSOR_RADIUS, 1.0, mq::BLACK);
}

/// Helper to manage grid of circles.
/// (x,y) = plain old pixel coordinates on display
/// (i,j) = coordinates of circles.
/// * i = `[0, 4)` left to right
/// * j = `[0, 9)` bottom to top
///
/// Other helpful indexes:
/// * history index is `j = NUM_GUESSES - j`
/// * working row is `j = NUM_GUESSES - history.len()`
///
/// Why? It makes it easier to index into history array.
///
/// ```text
///         <-- i -->
///          0 1 2 3
///         +-------+
///    ^  0 |       | <-- password
///    |  1 |       | <-- final guess
///    |  2 |       |
///       3 |       |
///    j  4 |       |
///       5 |       |
///    |  6 |       |
///    |  7 |       |
///    v  8 |       | <-- first guess
///         +-------+
/// ```
mod guess_circles_ij {
    use super::{
        Color, NumberOverlay, BOARD_OFFSET_X, BOARD_OFFSET_Y, COLOR_PALETTE, NUM_GUESSES,
        NUM_SLOTS_PER_ROW, ROW_SEPARATOR_HEIGHT, SLOT_PADDING, SLOT_PEG_FONT_SIZE, SLOT_RADIUS,
        SLOT_SIZE,
    };
    use better_quad::bq;
    use macroquad::prelude as mq;

    const CIRCLE_OUTLINE_THICKNESS: f32 = 1.0;

    fn compute_xy_coordinates(i: usize, j: usize) -> (f32, f32) {
        // explosive way to make sure I don't mis-use this function
        assert!(i < NUM_SLOTS_PER_ROW);
        assert!(j < NUM_GUESSES + 1); // + 1 accounts for password row
        let i = i as f32;
        let j = j as f32;

        let x = BOARD_OFFSET_X + SLOT_RADIUS + SLOT_SIZE * i + SLOT_PADDING * (i + 1.0);
        let y = BOARD_OFFSET_Y
            + SLOT_RADIUS
            + SLOT_SIZE * j
            + SLOT_PADDING * (j * 2.0 + 1.0)
            + ROW_SEPARATOR_HEIGHT * j;

        (x, y)
    }

    pub(crate) fn draw_outline(i: usize, j: usize) {
        let (x, y) = compute_xy_coordinates(i, j);
        bq::draw_circle_outline(x, y, SLOT_RADIUS, CIRCLE_OUTLINE_THICKNESS, mq::WHITE);
    }

    pub(crate) fn draw(i: usize, j: usize, color: Color, number_overlay: NumberOverlay) {
        let (x, y) = compute_xy_coordinates(i, j);
        bq::draw_circle(x, y, SLOT_RADIUS, color.as_mq());

        match number_overlay {
            NumberOverlay::On => {
                draw_text_overlay(
                    x,
                    y,
                    mq::BLACK,
                    format!(
                        "{}",
                        COLOR_PALETTE.iter().position(|c| *c == color).unwrap() + 1
                    ),
                );
            }
            NumberOverlay::Off => {}
        }
    }

    pub(crate) fn draw_password_text_overlay(i: usize, j: usize) {
        let (x, y) = compute_xy_coordinates(i, j);
        draw_text_overlay(x, y, mq::WHITE, "?");
    }

    fn draw_text_overlay(x: f32, y: f32, color: mq::Color, text: impl AsRef<str>) {
        bq::draw_text_left_aligned(
            text,
            None,
            SLOT_PEG_FONT_SIZE,
            color,
            bq::TextAnchorPoint::Center { x, y },
            None,
        );
    }

    pub(crate) fn get_containing_ij(mut x: f32, mut y: f32) -> Option<(usize, usize)> {
        x -= BOARD_OFFSET_X + SLOT_PADDING;
        let mut i = 0;
        loop {
            if x < 0.0 || i >= NUM_SLOTS_PER_ROW {
                return None;
            }
            if x <= SLOT_SIZE {
                break;
            }
            i += 1;
            x -= SLOT_SIZE + SLOT_PADDING;
        }

        y -= BOARD_OFFSET_Y + SLOT_PADDING;
        let mut j = 0;
        loop {
            #[allow(clippy::int_plus_one)]
            if y < 0.0 || j >= NUM_GUESSES + 1 {
                return None;
            }
            if y <= SLOT_SIZE {
                break;
            }
            j += 1;
            y -= SLOT_SIZE + SLOT_PADDING + ROW_SEPARATOR_HEIGHT + SLOT_PADDING;
        }

        Some((i, j))
    }
}

// I need to come up with a better re-usable method for drawing shapes and checking if mouse is within
// the shape boundaries. I am not loving this, but it works for now.
mod pegs_ij {
    use crate::{
        BoardSizeDerivedConsts, BOARD_OFFSET_X, BOARD_OFFSET_Y, COLOR_PALETTE, PEG_OUTER_PADDING,
        PEG_RADIUS, PEG_SIZE,
    };

    fn intra_peg_x_padding() -> f32 {
        let BoardSizeDerivedConsts {
            row_width_guess,
            row_width_key,
            ..
        } = BoardSizeDerivedConsts::get();

        // Question for future self: Do the local vars help readability?
        let board_width = row_width_guess + row_width_key;
        let peg_outer_padding = PEG_OUTER_PADDING * 2.0;
        let peg_total_width = COLOR_PALETTE.len() as f32 * PEG_SIZE;
        let num_intra_peg_spaces = COLOR_PALETTE.len() as f32 - 1.0;

        (board_width - (peg_outer_padding + peg_total_width)) / num_intra_peg_spaces
    }

    pub(crate) fn compute_x_coordinate(i: usize) -> f32 {
        // explosive way to make sure I don't mis-use this function
        assert!(i < COLOR_PALETTE.len());

        BOARD_OFFSET_X
            + PEG_OUTER_PADDING
            + (PEG_RADIUS * 2.0 + intra_peg_x_padding()) * i as f32
            + PEG_RADIUS
    }

    pub(crate) fn compute_y_coordinate() -> f32 {
        let derived_consts = BoardSizeDerivedConsts::get();
        BOARD_OFFSET_Y + derived_consts.board_height + PEG_OUTER_PADDING + PEG_RADIUS
    }

    pub(crate) fn get_containing_i(mut x: f32, y: f32) -> Option<usize> {
        x -= BOARD_OFFSET_X + PEG_OUTER_PADDING;
        let mut i = 0;
        loop {
            if x < 0.0 || i >= COLOR_PALETTE.len() {
                return None;
            }
            if x <= (PEG_RADIUS * 2.0) {
                break;
            }
            i += 1;
            x -= (PEG_RADIUS * 2.0) + intra_peg_x_padding();
        }

        let peg_y = compute_y_coordinate();
        if y < peg_y - PEG_RADIUS || y > peg_y + PEG_RADIUS {
            return None;
        }

        Some(i)
    }
}

/// Produce (x,y) key offset, assuming 2 rows for all keys.
fn get_key_offset(
    key_index: usize,
    num_slots_per_row: usize,
    key_padding: f32,
    key_radius: f32,
) -> (f32, f32) {
    // 4 -> 2
    // 5 -> 3
    // 6 -> 3
    // 7 -> 4
    let num_keys_top_key_row = (num_slots_per_row as f32 / 2.0).ceil() as usize;
    let (x_index, y_index) = if key_index < num_keys_top_key_row {
        (key_index, 0)
    } else {
        (key_index - num_keys_top_key_row, 1)
    };

    let x = (key_padding + key_radius * 2.0) * x_index as f32 + key_padding + key_radius;
    let y = (key_padding + key_radius * 2.0) * y_index as f32 + key_padding + key_radius;

    (x, y)
}

#[allow(dead_code)] // allow unused colors to be easily swapped in via const
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
    // OG 6
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    // Additional colors for fun
    Pink,
    LightBlue,
    NeonGreen,
}

impl Color {
    fn random_array<const N: usize>(palette: &[Self]) -> [Self; N] {
        [(); N].map(|_| Self::random(palette))
    }

    fn random(palette: &[Self]) -> Self {
        let index = mq::rand::gen_range(0, palette.len());
        palette[index]
    }

    fn as_mq(&self) -> mq::Color {
        match self {
            Self::Red => mq::RED,
            Self::Orange => mq::ORANGE,
            Self::Yellow => mq::YELLOW,
            Self::Green => mq::DARKGREEN,
            Self::Blue => mq::BLUE,
            Self::Purple => mq::VIOLET,
            Self::Pink => mq::MAGENTA,
            Self::LightBlue => mq::SKYBLUE,
            Self::NeonGreen => mq::Color::from_hex(0x39FF14),
        }
    }
}

#[derive(Copy, Clone)]
struct CompleteRow {
    guess: [Color; NUM_SLOTS_PER_ROW],
    num_correct_hits: usize,
    num_misplaced_hits: usize,
}

// None => Incomplete row
// Some => Completed row
fn convert_working_row_if_completed(
    working_row: &[Option<Color>; NUM_SLOTS_PER_ROW],
) -> Option<[Color; NUM_SLOTS_PER_ROW]> {
    if working_row.contains(&None) {
        return None;
    }

    Some(working_row.map(Option::unwrap))
}

fn evaluate_guess(
    guess: [Color; NUM_SLOTS_PER_ROW],
    password: [Color; NUM_SLOTS_PER_ROW],
) -> CompleteRow {
    let mut guess_colors_eligible_for_misplaced_hits = HashMap::new();
    let mut password_colors_eligible_for_misplaced_hits = HashMap::new();

    // First pass: check for correct hits
    let mut num_correct_hits = 0;
    for i in 0..NUM_SLOTS_PER_ROW {
        if guess[i] == password[i] {
            num_correct_hits += 1;
        } else {
            *guess_colors_eligible_for_misplaced_hits
                .entry(guess[i])
                .or_insert(0usize) += 1;
            *password_colors_eligible_for_misplaced_hits
                .entry(password[i])
                .or_insert(0usize) += 1;
        }
    }

    // Second pass: check for misplaced hits
    let mut num_misplaced_hits = 0;
    for (color, guess_color_count) in guess_colors_eligible_for_misplaced_hits {
        let password_color_count = password_colors_eligible_for_misplaced_hits
            .remove(&color)
            .unwrap_or(0);
        num_misplaced_hits += min(guess_color_count, password_color_count);
    }

    CompleteRow {
        guess,
        num_correct_hits,
        num_misplaced_hits,
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let hundredths = (100.0 * (duration.as_secs_f32() % 1.0)) as u32;

    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}.{hundredths:02.0}")
    } else {
        format!("{minutes:02}:{seconds:02}.{hundredths:02.0}")
    }
}

/// Whether or not numbers are shown over colors in the history and working row.
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum NumberOverlay {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    use super::{evaluate_guess, get_key_offset, Color, NUM_SLOTS_PER_ROW};

    // Janky names for readability defining test cases
    #[derive(Debug)]
    struct EvaluateGuessTestCase {
        // inputs
        pword: [Color; NUM_SLOTS_PER_ROW],
        guess: [Color; NUM_SLOTS_PER_ROW],
        // (expected correct, expected misplaced)
        pins: (usize, usize),
    }

    #[test]
    fn test_evaluate_guess() {
        for tc in evaluate_guess_test_cases() {
            let actual = evaluate_guess(tc.guess, tc.pword);
            let (expected_correct_hits, expected_misplaced_hits) = tc.pins;
            assert_eq!(
                actual.num_correct_hits, expected_correct_hits,
                "(Correct hits, phase1) {:?}",
                tc
            );
            assert_eq!(
                actual.num_misplaced_hits, expected_misplaced_hits,
                "(Misplaced hits, phase1) {:?}",
                tc
            );

            // Algorithm is not dependent on left/right, so swap them
            let actual = evaluate_guess(tc.pword, tc.guess);
            let (expected_correct_hits, expected_misplaced_hits) = tc.pins;
            assert_eq!(
                actual.num_correct_hits, expected_correct_hits,
                "(Correct hits, phase2) {:?}",
                tc
            );
            assert_eq!(
                actual.num_misplaced_hits, expected_misplaced_hits,
                "(Misplaced hits, phase2) {:?}",
                tc
            );
        }
    }

    fn evaluate_guess_test_cases() -> Vec<EvaluateGuessTestCase> {
        let a = Color::Red;
        let b = Color::Orange;
        let c = Color::Yellow;
        let d = Color::Green;

        vec![
            EvaluateGuessTestCase {
                pword: [a, a, a, a],
                guess: [a, a, a, a],
                pins: (4, 0),
            },
            EvaluateGuessTestCase {
                pword: [a, a, a, a],
                guess: [a, a, a, b],
                pins: (3, 0),
            },
            EvaluateGuessTestCase {
                pword: [a, a, a, a],
                guess: [a, b, b, b],
                pins: (1, 0),
            },
            EvaluateGuessTestCase {
                pword: [a, b, c, d],
                guess: [a, b, b, b],
                pins: (2, 0),
            },
            EvaluateGuessTestCase {
                pword: [a, b, c, d],
                guess: [a, c, a, b],
                pins: (1, 2),
            },
            EvaluateGuessTestCase {
                pword: [a, b, c, d],
                guess: [d, c, a, b],
                pins: (0, 4),
            },
            EvaluateGuessTestCase {
                pword: [a, b, a, b],
                guess: [a, b, c, d],
                pins: (2, 0),
            },
        ]
    }

    #[test]
    fn test_get_key_offset() {
        let key_padding = 5.0;
        let key_radius = 7.0;

        #[rustfmt::skip]
        get_key_offset_test_case(
            key_padding,
            key_radius,
            vec![
                (key_padding       + key_radius,       key_padding       + key_radius),
                (key_padding * 2.0 + key_radius * 3.0, key_padding       + key_radius),
                (key_padding       + key_radius,       key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 2.0 + key_radius * 3.0, key_padding * 2.0 + key_radius * 3.0),
            ],
        );

        #[rustfmt::skip]
        get_key_offset_test_case(
            key_padding,
            key_radius,
            vec![
                (key_padding       + key_radius,       key_padding       + key_radius),
                (key_padding * 2.0 + key_radius * 3.0, key_padding       + key_radius),
                (key_padding * 3.0 + key_radius * 5.0, key_padding       + key_radius),
                (key_padding       + key_radius,       key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 2.0 + key_radius * 3.0, key_padding * 2.0 + key_radius * 3.0),
            ],
        );

        #[rustfmt::skip]
        get_key_offset_test_case(
            key_padding,
            key_radius,
            vec![
                (key_padding       + key_radius,       key_padding       + key_radius),
                (key_padding * 2.0 + key_radius * 3.0, key_padding       + key_radius),
                (key_padding * 3.0 + key_radius * 5.0, key_padding       + key_radius),
                (key_padding       + key_radius,       key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 2.0 + key_radius * 3.0, key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 3.0 + key_radius * 5.0, key_padding * 2.0 + key_radius * 3.0),
            ],
        );

        #[rustfmt::skip]
        get_key_offset_test_case(
            key_padding,
            key_radius,
            vec![
                (key_padding       + key_radius,       key_padding       + key_radius),
                (key_padding * 2.0 + key_radius * 3.0, key_padding       + key_radius),
                (key_padding * 3.0 + key_radius * 5.0, key_padding       + key_radius),
                (key_padding * 4.0 + key_radius * 7.0, key_padding       + key_radius),
                (key_padding       + key_radius,       key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 2.0 + key_radius * 3.0, key_padding * 2.0 + key_radius * 3.0),
                (key_padding * 3.0 + key_radius * 5.0, key_padding * 2.0 + key_radius * 3.0),
            ],
        );
    }

    fn get_key_offset_test_case(
        key_padding: f32,
        key_radius: f32,
        expected_offsets: Vec<(f32, f32)>,
    ) {
        let slots_per_row = expected_offsets.len();
        for (key_index, expected_offset) in expected_offsets.into_iter().enumerate() {
            let actual_offset = get_key_offset(key_index, slots_per_row, key_padding, key_radius);
            assert_eq!(
                expected_offset.0, actual_offset.0,
                "Key index: {}, coord X",
                key_index
            );
            assert_eq!(
                expected_offset.1, actual_offset.1,
                "Key index: {}, coord Y",
                key_index
            );
        }
    }
}
