use crate::turn_time_tracker::infinite_iterator::InfiniteIterator;
use crate::StatefulGui;
use macroquad::prelude as mq;
use std::time::{Duration, SystemTime};

// Control consts
const KEY_NEXT_PLAYER: mq::KeyCode = mq::KeyCode::Space;
const KEY_PAUSE: mq::KeyCode = mq::KeyCode::P;
const KEY_DETAIL_MODE_TOGGLE: mq::KeyCode = mq::KeyCode::D;

// Draw consts
// TODO:3 dynamic size based on window
const PLAYER_TEXT_FONT_SIZE: u32 = 40;
const PLAYER_TEXT_LINE_BUFFER: u32 = 10;
const PLAYER_TEXT_X: f32 = 10.0;
const PLAYER_TEXT_Y: f32 = PIE_THICKNESS + PIE_Y + 10.0;

const PAUSED_TEXT_FONT_SIZE: u32 = PLAYER_TEXT_FONT_SIZE;
const PAUSED_TEXT_X: f32 = 10.0;
const PAUSED_TEXT_Y: f32 = PIE_Y + PIE_THICKNESS;

const PIE_X: f32 = 250.0;
const PIE_Y: f32 = 250.0;
const PIE_THICKNESS: f32 = 230.0;

pub struct TurnTimeTrackerState {
    players: InfiniteIterator<Player>,
    timer: TimerState,
    text_detail_mode: TextDetailMode,
}

impl StatefulGui for TurnTimeTrackerState {
    fn update(&mut self) {
        self.evaluate_state(SystemTime::now());
    }

    fn draw(&self) {
        self.draw_state();
    }
}

impl Default for TurnTimeTrackerState {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnTimeTrackerState {
    pub fn new() -> Self {
        Self {
            players: InfiniteIterator::new(),
            timer: TimerState::Paused,
            text_detail_mode: TextDetailMode::Concise,
        }
    }

    // TODO:3 remove `pub` and make it only accessible via UI interaction.
    pub fn add_player(&mut self, display_name: impl Into<String>, display_color: mq::Color) {
        self.players.push(Player::new(display_name, display_color));
    }

    fn evaluate_state(&mut self, now: SystemTime) {
        // Toggle detail mode if needed
        if mq::is_key_pressed(KEY_DETAIL_MODE_TOGGLE) {
            match self.text_detail_mode {
                TextDetailMode::Concise => self.text_detail_mode = TextDetailMode::Detailed,
                TextDetailMode::Detailed => self.text_detail_mode = TextDetailMode::Concise,
            }
        }

        match &mut self.timer {
            TimerState::Paused => {
                // Check for unpause
                if mq::is_key_pressed(KEY_PAUSE) {
                    self.timer = TimerState::Running { last_tick: now };
                }
            }
            TimerState::Running { ref mut last_tick } => {
                // Check for pause
                if mq::is_key_pressed(KEY_PAUSE) {
                    self.timer = TimerState::Paused;
                    return;
                }

                // Tick current player
                let current_player = self.players.current_mut();
                let elapsed_tick_time = now
                    .duration_since(*last_tick)
                    .expect("Elapsed tick time underflow");
                current_player.total_time += elapsed_tick_time;
                // Band-aid to fix num_turns not being set for initial player.
                if current_player.num_turns == 0 {
                    current_player.num_turns = 1;
                }

                *last_tick = now;

                // Change current player if needed. Do this AFTER ticking current player so previous
                // player is attributed the time until we process the player change.
                if mq::is_key_pressed(KEY_NEXT_PLAYER) {
                    self.players.increment();
                    self.players.current_mut().num_turns += 1;
                }
            }
        }
    }

    fn draw_state(&self) {
        let bg_color = match self.timer {
            TimerState::Paused => mq::DARKGRAY,
            TimerState::Running { .. } => mq::LIGHTGRAY,
        };
        mq::clear_background(bg_color);
        let (players, current_player_index) = self.players.raw();

        let mut all_total_time = Duration::ZERO;
        for player in players {
            all_total_time += player.total_time
        }

        // Draw text
        for (i, player) in players.iter().enumerate() {
            let text_line_name = format!(
                // TODO:3 replace '9' padding with dynamic name padding
                "{} {: <9}",
                if i == current_player_index {
                    "[X]"
                } else {
                    "[ ]"
                },
                player.display_name
            );

            let text_line_info = match self.text_detail_mode {
                TextDetailMode::Concise => format!(
                    "{} ({: >2.0}%)",
                    format_duration_concise(player.total_time),
                    100.0 * (player.total_time.as_secs_f32() / all_total_time.as_secs_f32()),
                ),
                TextDetailMode::Detailed => format!(
                    "{} ({: >2.0}%) -- ({} turns; avg {:.3} sec/turn)",
                    format_duration_detailed(player.total_time),
                    100.0 * (player.total_time.as_secs_f32() / all_total_time.as_secs_f32()),
                    player.num_turns,
                    player.total_time.as_secs_f32() / player.num_turns as f32,
                ),
            };

            // TODO:3 use friendlier font
            mq::draw_text(
                &format!("{text_line_name}: {text_line_info}"),
                PLAYER_TEXT_X,
                PLAYER_TEXT_Y
                    + ((PLAYER_TEXT_LINE_BUFFER + PLAYER_TEXT_FONT_SIZE) * (i as u32 + 1)) as f32,
                PLAYER_TEXT_FONT_SIZE as f32,
                player.display_color,
            );
        }

        Self::draw_pie(players, all_total_time);
        // TODO:1 larger thickness for current turn
        // TODO:1 red box around current turn text row
        // TODO:2 press 1-9 to fastswap to player turn

        if let TimerState::Paused = self.timer {
            mq::draw_text(
                "PAUSED",
                PAUSED_TEXT_X,
                PAUSED_TEXT_Y,
                PAUSED_TEXT_FONT_SIZE as f32,
                mq::WHITE,
            );
        }
    }

    fn draw_pie(players: &Vec<Player>, all_total_time: Duration) {
        let circle_sides = 100;
        let radius = 0.0;
        // Offset circle so 0 degrees is north.
        let rotation_offset = -90.0;

        let mut current_start_degree = 0.0;
        for player in players {
            // portion = [0, 1]
            let player_slice_portion =
                player.total_time.as_secs_f32() / all_total_time.as_secs_f32();
            let player_slice_degrees = 360.0 * player_slice_portion;
            mq::draw_arc(
                PIE_X,
                PIE_Y,
                circle_sides,
                radius,
                current_start_degree + rotation_offset,
                PIE_THICKNESS,
                player_slice_degrees,
                player.display_color,
            );

            current_start_degree += player_slice_degrees;
        }
    }
}

fn format_duration_concise(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

fn format_duration_detailed(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let hundredths = (100.0 * (duration.as_secs_f32() % 1.0)) as u32;

    format!("{hours:02}:{minutes:02}:{seconds:02}.{hundredths:02.0}")
}

#[derive(Copy, Clone)]
enum TimerState {
    Paused,
    Running { last_tick: SystemTime },
}

#[derive(Copy, Clone)]
enum TextDetailMode {
    Concise,
    Detailed,
}

struct Player {
    display_name: String,
    display_color: mq::Color,
    num_turns: usize,
    total_time: Duration,
}

impl Player {
    pub(crate) fn new(display_name: impl Into<String>, display_color: mq::Color) -> Self {
        Self {
            display_name: display_name.into(),
            display_color,
            num_turns: 0,
            total_time: Duration::ZERO,
        }
    }
}

mod infinite_iterator {
    pub(crate) struct InfiniteIterator<T> {
        items: Vec<T>,
        // Soft invariant: `current_index` is always a valid index into `items`.
        // Invariant holds as long as items is non-empty.
        current_index: usize,
    }

    impl<T> InfiniteIterator<T> {
        pub(crate) fn new() -> Self {
            Self {
                items: Vec::new(),
                current_index: 0,
            }
        }

        pub(crate) fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn check_invariants(&self, method_name: &'static str) {
            if self.items.is_empty() {
                panic!("Can't call {method_name}() on empty InfiniteIterator");
            }
            if self.current_index >= self.items.len() {
                panic!("InfiniteIterator-Invariant-Bug: called {method_name}() with current_index={} and len={}.", self.current_index, self.items.len());
            }
        }

        pub(crate) fn current_mut(&mut self) -> &mut T {
            self.check_invariants("current_mut");
            &mut self.items[self.current_index]
        }

        pub(crate) fn increment(&mut self) {
            if self.items.is_empty() {
                panic!("Can't call increment() on empty InfiniteIterator");
            }

            self.current_index = (self.current_index + 1) % self.items.len();
        }

        pub(crate) fn raw(&self) -> (&Vec<T>, usize) {
            self.check_invariants("raw");
            (&self.items, self.current_index)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    #[test]
    fn test_format_duration_detailed() {
        let test_cases = [
            // (input seconds, expected format)
            (2.99999, "00:00:02.99"),
            (3.00000, "00:00:03.00"),
        ];

        for (input_seconds, expected_output) in test_cases {
            let input = Duration::from_secs_f64(input_seconds);
            let actual_output = super::format_duration_detailed(input);
            assert_eq!(expected_output, &actual_output);
        }
    }
}
