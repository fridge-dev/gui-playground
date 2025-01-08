use better_quad::timestamp::Timestamp;
use better_quad::StatefulGui;
use infinite_iterator::InfiniteIterator;
use macroquad::prelude as mq;
use std::cmp::max;
use std::collections::BinaryHeap;
use std::time::Duration;

// Control consts
const KEY_NEXT_PLAYER: mq::KeyCode = mq::KeyCode::Space;
const KEY_PAUSE: mq::KeyCode = mq::KeyCode::P;
const KEY_TIME_DISPLAY_TOGGLE: mq::KeyCode = mq::KeyCode::H;
const KEY_DETAIL_MODE_TOGGLE: mq::KeyCode = mq::KeyCode::D;

// Draw consts
// TODO:3 dynamic size based on window
const PIE_X: f32 = 300.0;
const PIE_Y: f32 = 300.0;
const PIE_THICKNESS: f32 = 230.0;
const PIE_THICKNESS_CURRENT_TURN_MULTIPLIER: f32 = 1.2;

const PAUSED_TEXT_FONT_SIZE: f32 = PLAYER_TEXT_FONT_SIZE;
const PAUSED_TEXT_X: f32 = 10.0;
const PAUSED_TEXT_Y: f32 = PIE_Y + PIE_THICKNESS;

const PLAYER_TEXT_FONT_SIZE: f32 = 40.0;
const PLAYER_TEXT_LINE_BUFFER: f32 = 10.0;
const PLAYER_TEXT_X: f32 = 10.0;
const PLAYER_TEXT_Y: f32 = PIE_THICKNESS + PIE_Y + 20.0;
const PLAYER_RECTANGLE_THICKNESS: f32 = 6.0;

pub struct TurnTimeTracker {
    players: InfiniteIterator<Player>,
    timer: TimerState,
    time_display_mode: TimeDisplayMode,
    text_detail_mode: TextDetailMode,
}

#[derive(Copy, Clone)]
enum TimerState {
    Paused,
    Running { last_tick: Timestamp },
}

#[derive(Copy, Clone)]
enum TextDetailMode {
    Concise,
    Detailed,
}

#[derive(Copy, Clone)]
enum TimeDisplayMode {
    Shown,
    Hidden,
}

impl StatefulGui for TurnTimeTracker {
    fn main_conf() -> mq::Conf {
        mq::Conf {
            window_title: "Tabletop Turn Time Tracker".to_string(),
            window_width: (PIE_X + PIE_THICKNESS * PIE_THICKNESS_CURRENT_TURN_MULTIPLIER + 20.0)
                as i32,
            window_height: 1000,
            ..Default::default()
        }
    }

    fn update(&mut self, now: Timestamp) {
        self.evaluate_state(now);
    }

    fn draw(&self) {
        self.draw_state();
    }
}

impl Default for TurnTimeTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl TurnTimeTracker {
    pub fn new() -> Self {
        Self {
            players: InfiniteIterator::new(),
            timer: TimerState::Paused,
            time_display_mode: TimeDisplayMode::Shown,
            text_detail_mode: TextDetailMode::Concise,
        }
    }

    // TODO:3 remove `pub` and make it only accessible via UI interaction.
    pub fn add_player(&mut self, display_name: impl Into<String>, display_color: mq::Color) {
        self.players.push(Player::new(display_name, display_color));
    }

    fn evaluate_state(&mut self, now: Timestamp) {
        // Toggle time display if needed
        if mq::is_key_pressed(KEY_TIME_DISPLAY_TOGGLE) {
            self.time_display_mode = match self.time_display_mode {
                TimeDisplayMode::Shown => TimeDisplayMode::Hidden,
                TimeDisplayMode::Hidden => TimeDisplayMode::Shown,
            };
        }

        // Toggle detail mode if needed
        if mq::is_key_pressed(KEY_DETAIL_MODE_TOGGLE) {
            self.text_detail_mode = match self.text_detail_mode {
                TextDetailMode::Concise => TextDetailMode::Detailed,
                TextDetailMode::Detailed => TextDetailMode::Concise,
            };
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
                let elapsed_tick_time = now
                    .duration_since(*last_tick)
                    .expect("Elapsed tick time underflow");
                self.players.current_mut().tick_frame(elapsed_tick_time);

                *last_tick = now;

                // Change current player if needed. Do this AFTER ticking current player so previous
                // player is attributed the time until we process the player change.
                if mq::is_key_pressed(KEY_NEXT_PLAYER) {
                    self.players.current_mut().stats.end_turn();
                    self.players.advance();
                }

                // TODO:2 press 1-9 to fastswap to player turn
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

        match self.time_display_mode {
            TimeDisplayMode::Shown => Self::draw_pie(players, current_player_index, all_total_time),
            TimeDisplayMode::Hidden => {}
        }
        self.draw_player_text(players, current_player_index, all_total_time);

        if let TimerState::Paused = self.timer {
            mq::draw_text(
                "PAUSED",
                PAUSED_TEXT_X,
                PAUSED_TEXT_Y,
                PAUSED_TEXT_FONT_SIZE,
                mq::WHITE,
            );
        }
    }

    fn draw_player_text(
        &self,
        players: &[Player],
        current_player_index: usize,
        all_total_time: Duration,
    ) {
        for (i, player) in players.iter().enumerate() {
            let text_line_name = format!(
                // Names longer than 8 chars will push the line out a little bit :P oh well
                "{} {: <8}",
                if i == current_player_index {
                    "[X]"
                } else {
                    "[ ]"
                },
                player.display_name
            );

            let text_line_info = match (self.time_display_mode, self.text_detail_mode) {
                (TimeDisplayMode::Hidden, _) => {
                    if i == current_player_index {
                        format_duration_concise(player.stats.current_turn_duration)
                    } else {
                        "".to_string()
                    }
                }
                (TimeDisplayMode::Shown, TextDetailMode::Concise) => format!(
                    "{} ({: >2.0}%)",
                    format_duration_concise(player.total_time),
                    100.0 * (player.total_time.as_secs_f32() / all_total_time.as_secs_f32()),
                ),
                (TimeDisplayMode::Shown, TextDetailMode::Detailed) => format!(
                    "{} ({: >2.0}%) -- ({} turns; avg: {}, max: {}, median: {})",
                    format_duration_detailed(player.total_time),
                    100.0 * (player.total_time.as_secs_f32() / all_total_time.as_secs_f32()),
                    player.stats.num_turns(),
                    format_duration_stats(if player.stats.num_turns() == 0 {
                        None
                    } else {
                        Some(player.total_time / player.stats.num_turns() as u32)
                    }),
                    format_duration_stats(player.stats.max_turn()),
                    format_duration_stats(player.stats.median_turn())
                ),
            };

            let text_line = if text_line_info.is_empty() {
                text_line_name
            } else {
                format!("{text_line_name}: {text_line_info}")
            };

            // TODO:3 use friendlier monospace font
            let player_text_y = PLAYER_TEXT_Y
                + ((PLAYER_TEXT_LINE_BUFFER + PLAYER_TEXT_FONT_SIZE) * (i as f32 + 1.0));
            mq::draw_text(
                &text_line,
                PLAYER_TEXT_X,
                player_text_y,
                PLAYER_TEXT_FONT_SIZE,
                player.display_color,
            );

            if i == current_player_index {
                let text_dimension =
                    mq::measure_text(&text_line, None, PLAYER_TEXT_FONT_SIZE as u16, 1.0);
                // Magic numbers are rectangle padding, which just "looks right".
                mq::draw_rectangle_lines(
                    PLAYER_TEXT_X - 5.0,
                    player_text_y - text_dimension.height - 4.0,
                    text_dimension.width + 10.0,
                    text_dimension.height + 18.0,
                    PLAYER_RECTANGLE_THICKNESS,
                    mq::WHITE,
                );
            }
        }
    }

    fn draw_pie(players: &[Player], current_player_index: usize, all_total_time: Duration) {
        let circle_sides = 100;
        let radius = 0.0;
        // Offset circle so 0 degrees is north.
        let rotation_offset = -90.0;

        let mut current_start_degree = 0.0;
        for (i, player) in players.iter().enumerate() {
            // portion = [0, 1]
            let player_slice_portion =
                player.total_time.as_secs_f32() / all_total_time.as_secs_f32();
            let player_slice_degrees = 360.0 * player_slice_portion;
            let thickness_multiplier = if i == current_player_index {
                PIE_THICKNESS_CURRENT_TURN_MULTIPLIER
            } else {
                1.0
            };
            mq::draw_arc(
                PIE_X,
                PIE_Y,
                circle_sides,
                radius,
                current_start_degree + rotation_offset,
                PIE_THICKNESS * thickness_multiplier,
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

fn format_duration_stats(duration: Option<Duration>) -> String {
    let total_seconds = duration.unwrap_or_default().as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{minutes:02}:{seconds:02}")
}

struct Player {
    display_name: String,
    display_color: mq::Color,
    total_time: Duration,
    stats: PlayerTurnDurationStats,
}

impl Player {
    pub(crate) fn new(display_name: impl Into<String>, display_color: mq::Color) -> Self {
        Self {
            display_name: display_name.into(),
            display_color,
            total_time: Duration::ZERO,
            stats: PlayerTurnDurationStats::new(),
        }
    }

    pub(crate) fn tick_frame(&mut self, elapsed_tick_time: Duration) {
        self.total_time += elapsed_tick_time;
        self.stats.tick_frame(elapsed_tick_time);
    }
}

struct PlayerTurnDurationStats {
    current_turn_duration: Duration,
    completed_turn_durations: BinaryHeap<Duration>,
}

impl PlayerTurnDurationStats {
    // Don't count turns towards stats if they're a "quick" press to skip to the next player.
    const DONT_COUNT_TURN_THRESHOLD: Duration = Duration::from_millis(700);

    pub(crate) fn new() -> Self {
        Self {
            current_turn_duration: Duration::ZERO,
            completed_turn_durations: BinaryHeap::new(),
        }
    }

    pub(crate) fn end_turn(&mut self) {
        if self.current_turn_duration >= Self::DONT_COUNT_TURN_THRESHOLD {
            self.completed_turn_durations
                .push(self.current_turn_duration);
        }
        self.current_turn_duration = Duration::ZERO;
    }

    pub(crate) fn tick_frame(&mut self, elapsed_tick_time: Duration) {
        self.current_turn_duration += elapsed_tick_time;
    }

    pub(crate) fn num_turns(&self) -> usize {
        let current_turn = if self.current_turn_duration.is_zero() {
            0
        } else {
            1
        };

        current_turn + self.completed_turn_durations.len()
    }

    pub(crate) fn max_turn(&self) -> Option<Duration> {
        let opt_max_completed = self.completed_turn_durations.peek();
        let opt_current = if self.current_turn_duration.is_zero() {
            None
        } else {
            Some(self.current_turn_duration)
        };

        match (opt_max_completed, opt_current) {
            (Some(a), Some(b)) => Some(max(*a, b)),
            (Some(a), None) => Some(*a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
    }

    // Sub-optimal, but whatever
    pub(crate) fn median_turn(&self) -> Option<Duration> {
        let mut sorted_turns = self.completed_turn_durations.clone();
        if !self.current_turn_duration.is_zero() {
            sorted_turns.push(self.current_turn_duration);
        }
        let sorted_turns_vec = sorted_turns.into_sorted_vec();

        if sorted_turns_vec.is_empty() {
            return None;
        }

        if 0 == sorted_turns_vec.len() % 2 {
            // even length
            let median_index_1 = sorted_turns_vec.len() / 2 - 1;
            let median_index_2 = sorted_turns_vec.len() / 2;
            Some((sorted_turns_vec[median_index_1] + sorted_turns_vec[median_index_2]) / 2)
        } else {
            // odd length
            let median_index = sorted_turns_vec.len() / 2;
            Some(sorted_turns_vec[median_index])
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

        pub(crate) fn advance(&mut self) {
            self.check_invariants("advance");
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
