use crate::framework::timestamp::Timestamp;
use crate::StatefulGui;
use macroquad::prelude as mq;
use rand::Rng;
use std::cmp::min;
use std::collections::HashMap;

// Control consts
const KEY_SUBMIT: mq::KeyCode = mq::KeyCode::Space;

// Game logic consts
const NUM_COLORS: usize = 6;
const COLOR_PALETTE: [Color; NUM_COLORS] = [
    Color::Red,
    Color::Orange,
    Color::Yellow,
    Color::Green,
    Color::Blue,
    Color::Purple,
];
const NUM_SLOTS_PER_ROW: usize = 4;
const NUM_GUESSES: usize = 8;

// Draw consts
const CURSOR_SIZE: f32 = 10.0;
const SLOTS_PER_ROW_F32: f32 = NUM_SLOTS_PER_ROW as f32;
const BOARD_OFFSET_X: f32 = 20.0;
const BOARD_OFFSET_Y: f32 = 20.0;
const ROW_SEPARATOR_HEIGHT: f32 = 1.0;
const SLOT_SIZE: f32 = 50.0;
const SLOT_RADIUS: f32 = SLOT_SIZE / 2.0;
const SLOT_PADDING: f32 = 5.0;
// Keys will fit within a single guess slot.
const KEY_SIZE: f32 = 8.0;
const KEY_PADDING: f32 = 10.0;

// Features to do:
// - player selects password
// - pvp
// - custom attributes
// - time-based
// - show numbers on colors and ???? text on password
pub struct MastermindGame {
    state: GameState,
    password: [Color; NUM_SLOTS_PER_ROW],
    // head: first guess; tail: most recent guess
    history: Vec<CompleteRow>,
    mouse_color: Color,
    // Work around annoying (0, 0) initialization issue with mq.
    mouse_moved: bool,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GameState {
    InProgress {
        working_row: [Option<Color>; NUM_SLOTS_PER_ROW],
    },
    Victory,
    TooManyGuesses,
}

impl MastermindGame {
    // TODO delete
    pub fn setup_example() -> Self {
        let mut game = Self::new();
        game.state = GameState::InProgress {
            working_row: [Some(Color::Yellow), None, Some(Color::Green), None],
        };
        game.history.extend([
            CompleteRow {
                guess: [Color::Red, Color::Orange, Color::Yellow, Color::Green],
                num_correct_hits: 0,
                num_misplaced_hits: 1,
            },
            CompleteRow {
                guess: [Color::Blue, Color::Purple, Color::Yellow, Color::Orange],
                num_correct_hits: 1,
                num_misplaced_hits: 2,
            }
        ]);

        game
    }
}

impl StatefulGui for MastermindGame {
    fn main_conf() -> mq::Conf {
        mq::Conf {
            window_title: "Mastermind Head2Head".to_string(),
            // TODO less brittle const
            window_width: 325,
            window_height: 650,
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
            state: GameState::InProgress {
                working_row: [None; NUM_SLOTS_PER_ROW],
            },
            password: [
                Color::random(),
                Color::random(),
                Color::random(),
                Color::random(),
            ],
            history: Vec::with_capacity(NUM_GUESSES),
            mouse_color: COLOR_PALETTE[0],
            mouse_moved: false,
        }
    }

    fn update(&mut self, _now: Timestamp) {
        if !self.mouse_moved && mq::mouse_position() != (0.0, 0.0) {
            self.mouse_moved = true;
        }

        match self.state {
            GameState::InProgress { working_row } => {
                // Update mouse color if needed
                if let Some(new_color) = Self::get_color_from_key_press() {
                    self.mouse_color = new_color;
                }

                // Update working row's color if needed
                if mq::is_mouse_button_pressed(mq::MouseButton::Left) {
                    let (mouse_x, mouse_y) = mq::mouse_position();
                    // TODO: check if x,y is in valid cell and update
                }

                // Apply guess if needed
                if mq::is_key_pressed(KEY_SUBMIT) {
                    if let Some(guess) = convert_working_row_if_completed(working_row) {
                        let complete_row = evaluate_guess(guess, self.password);
                        self.history.push(complete_row);

                        if complete_row.num_correct_hits == NUM_SLOTS_PER_ROW {
                            self.state = GameState::Victory;
                        } else if self.history.len() == NUM_GUESSES {
                            self.state = GameState::TooManyGuesses;
                        } else {
                            self.state = GameState::InProgress {
                                working_row: [None; NUM_SLOTS_PER_ROW],
                            };
                        }
                    }
                }
            }
            GameState::Victory => {
                // TODO: no update? Check for restart keypress
            }
            GameState::TooManyGuesses => {
                // TODO: no update? Check for restart keypress
            }
        }
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

    fn draw(&self) {
        mq::clear_background(mq::DARKBROWN);
        let row_width = SLOT_SIZE * SLOTS_PER_ROW_F32 + SLOT_PADDING * (SLOTS_PER_ROW_F32 + 1.0);
        let row_height = SLOT_SIZE + SLOT_PADDING * 2.0;

        // Board
        mq::draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            row_width,
            row_height * (NUM_GUESSES as f32 + 1.0) + ROW_SEPARATOR_HEIGHT * NUM_GUESSES as f32,
            mq::BROWN,
        );

        // Separators - Line goes at *bottom* of first n-1 rows
        for j in 0..NUM_GUESSES {
            let j = j as f32;
            mq::draw_rectangle(
                BOARD_OFFSET_X,
                BOARD_OFFSET_Y + row_height * (j + 1.0) + ROW_SEPARATOR_HEIGHT * j,
                row_width,
                ROW_SEPARATOR_HEIGHT,
                mq::BLACK,
            );
        }

        // Password - overwrite space already drawn with Board
        mq::draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            row_width,
            row_height,
            mq::BLACK,
        );

        // Circles - colored - history
        for (j, row) in self.history.iter().enumerate() {
            let j = NUM_GUESSES - j;
            for (i, color) in row.guess.iter().enumerate() {
                circle::draw_ij(i, j, *color);
            }
        }

        // Circles - colored - working
        if let GameState::InProgress { working_row } = &self.state {
            let j = NUM_GUESSES - self.history.len();
            for (i, opt_color) in working_row.iter().enumerate() {
                if let Some(color) = opt_color {
                    circle::draw_ij(i, j, *color);
                }
            }

            // Gold working box
            let j = (NUM_GUESSES - self.history.len()) as f32;
            mq::draw_rectangle_lines(
                BOARD_OFFSET_X,
                BOARD_OFFSET_Y + (row_height + ROW_SEPARATOR_HEIGHT) * j,
                row_width,
                row_height,
                4.0,
                mq::GOLD,
            );
        }

        // Circles - outlines
        for i in 0..NUM_SLOTS_PER_ROW {
            for j in 0..NUM_GUESSES+1 {
                circle::draw_outline_ij(i, j);
            }
        }

        // Mouse
        let (mouse_x, mouse_y) = mq::mouse_position();
        let mouse_on_screen = (0.0..=mq::screen_width()).contains(&mouse_x) && (0.0..=mq::screen_height()).contains(&mouse_y);
        if mouse_on_screen && self.mouse_moved {
            mq::draw_circle(mouse_x, mouse_y, CURSOR_SIZE, self.mouse_color.as_mq());
            mq::show_mouse(false);
        } else {
            mq::show_mouse(true);
        }
    }
}

/// Helper to manage grid of circles
mod circle {
    use super::{Color, NUM_GUESSES, NUM_SLOTS_PER_ROW, BOARD_OFFSET_Y, BOARD_OFFSET_X, SLOT_RADIUS, SLOT_SIZE, SLOT_PADDING, ROW_SEPARATOR_HEIGHT};
    use macroquad::prelude as mq;

    const CIRCLE_SIDES: u8 = 30;
    const CIRCLE_ROTATION: f32 = 0.0;
    const CIRCLE_OUTLINE_THICKNESS: f32 = 1.0;

    pub(crate) fn compute_xy_coordinates(i: usize, j: usize) -> (f32, f32) {
        // explosive way to make sure I don't mis-use this function
        assert!(i < NUM_SLOTS_PER_ROW);
        assert!(j < NUM_GUESSES + 1); // + 1 accounts for password row
        let i = i as f32;
        let j = j as f32;

        let x = BOARD_OFFSET_X + SLOT_RADIUS + SLOT_SIZE * i + SLOT_PADDING * (i + 1.0);
        let y = BOARD_OFFSET_Y + SLOT_RADIUS + SLOT_SIZE * j + SLOT_PADDING * (j * 2.0 + 1.0) + ROW_SEPARATOR_HEIGHT * j;

        (x, y)
    }

    pub(crate) fn draw_ij(i: usize, j: usize, color: Color) {
        let (x, y) = compute_xy_coordinates(i, j);
        draw(x, y, color);
    }

    pub(crate) fn draw_outline_ij(i: usize, j: usize) {
        let (x, y) = compute_xy_coordinates(i, j);
        draw_outline(x, y);
    }

    pub(crate) fn draw(x: f32, y: f32, color: Color) {
        mq::draw_poly(
            x,
            y,
            CIRCLE_SIDES,
            SLOT_RADIUS,
            CIRCLE_ROTATION,
            color.as_mq(),
        );
    }

    pub(crate) fn draw_outline(x: f32, y: f32) {
        mq::draw_poly_lines(
            x,
            y,
            CIRCLE_SIDES,
            SLOT_RADIUS,
            CIRCLE_ROTATION,
            CIRCLE_OUTLINE_THICKNESS,
            mq::WHITE,
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Color {
    fn random() -> Self {
        let r = rand::thread_rng().gen_range(0..COLOR_PALETTE.len());
        COLOR_PALETTE[r]
    }

    fn as_mq(&self) -> mq::Color {
        match self {
            Color::Red => mq::RED,
            Color::Orange => mq::ORANGE,
            Color::Yellow => mq::YELLOW,
            Color::Green => mq::GREEN,
            Color::Blue => mq::BLUE,
            Color::Purple => mq::PURPLE,
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
    working_row: [Option<Color>; NUM_SLOTS_PER_ROW],
) -> Option<[Color; NUM_SLOTS_PER_ROW]> {
    if working_row.contains(&None) {
        return None;
    }

    // More brittle than I'd like :P but trying to move fast.
    // This could be made better by using Vec<> everywhere.
    assert_eq!(
        4, NUM_SLOTS_PER_ROW,
        "changed SLOTS_PER_ROW const without changing hard-coded indexes"
    );
    Some([
        working_row[0].unwrap(),
        working_row[1].unwrap(),
        working_row[2].unwrap(),
        working_row[3].unwrap(),
    ])
}

fn evaluate_guess(guess: [Color; NUM_SLOTS_PER_ROW], password: [Color; NUM_SLOTS_PER_ROW]) -> CompleteRow {
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

#[cfg(test)]
mod tests {
    use crate::mastermind::{evaluate_guess, Color, NUM_SLOTS_PER_ROW};

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
}
