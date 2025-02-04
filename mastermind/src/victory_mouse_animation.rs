use crate::{draw_cursor, CURSOR_SIZE};
use better_quad::bq::{TextAlignment, TextAnchorPoint, Timestamp};
use better_quad::utils::animation_tickers::TimeBasedAnimationTicker;
use better_quad::utils::color_animation::{
    SmoothColorAnimation, StepColorAnimation, TransitionLength,
};
use better_quad::utils::geometry;
use better_quad::{bq, mq};
use std::time::Duration;

const TRANSITION_LENGTH_NORTH: TransitionLength =
    TransitionLength::TimedDuration(Duration::from_millis(550));
const TRANSITION_LENGTH_WEST: TransitionLength =
    TransitionLength::TimedDuration(Duration::from_millis(275));
const TRANSITION_LENGTH_EAST: TransitionLength =
    TransitionLength::TimedDuration(Duration::from_millis(80));
const TRANSITION_LENGTH_SOUTH: TransitionLength =
    TransitionLength::TimedDuration(Duration::from_millis(400));
const FULL_CIRCLE_ROTATION_DURATION: Duration = Duration::from_millis(1800);
const TRANSITION_LENGTH_WINNER_TEXT: TransitionLength =
    TransitionLength::TimedDuration(Duration::from_millis(3500));

/// Animation to draw 4 cursors with different animations. Just having some fun.
pub(crate) struct VictoryMouseAnimations {
    north: StepColorAnimation,
    west: SmoothColorAnimation,
    east: SmoothColorAnimation,
    south: SmoothColorAnimation,
    rotation: TimeBasedAnimationTicker,
    cursor_offset: f32,
    winner_text: String,
    winner_text_font_size: u16,
    winner_text_offset: f32,
    winner_text_animation: SmoothColorAnimation,
}

impl VictoryMouseAnimations {
    pub(crate) fn new(
        palette: Vec<mq::Color>,
        animation_start: Timestamp,
        cursor_offset: f32,
        winner_text: String,
        winner_text_font_size: u16,
        winner_text_offset: f32,
    ) -> Self {
        Self {
            north: StepColorAnimation::new(&palette, TRANSITION_LENGTH_NORTH),
            west: SmoothColorAnimation::new(&palette, TRANSITION_LENGTH_WEST),
            east: SmoothColorAnimation::new(
                &palette.clone().into_iter().rev().collect::<Vec<_>>(),
                TRANSITION_LENGTH_EAST,
            ),
            south: SmoothColorAnimation::new(
                &palette
                    .clone()
                    .into_iter()
                    .map(bq::invert_color)
                    .collect::<Vec<_>>(),
                TRANSITION_LENGTH_SOUTH,
            ),
            rotation: TimeBasedAnimationTicker::new(animation_start, FULL_CIRCLE_ROTATION_DURATION),
            cursor_offset,
            winner_text,
            winner_text_font_size,
            winner_text_offset,
            winner_text_animation: SmoothColorAnimation::new(
                &palette,
                TRANSITION_LENGTH_WINNER_TEXT,
            ),
        }
    }

    pub(crate) fn tick(&mut self, now: Timestamp) {
        self.north.tick_frame();
        self.west.tick_frame();
        self.east.tick_frame();
        self.south.tick_frame();
        self.rotation.tick(now);
        self.winner_text_animation.tick_frame();
    }

    pub(crate) fn draw(&self, mouse_x: f32, mouse_y: f32) {
        // let's get weird

        let rotate_point_fn = |x: f32, y: f32| -> (f32, f32) {
            geometry::rotate_point(
                x,
                y,
                mouse_x,
                mouse_y,
                self.rotation.current_animation_percent(),
            )
        };

        // North
        let (north_x, north_y) = rotate_point_fn(mouse_x, mouse_y - self.cursor_offset);
        draw_cursor(north_x, north_y, self.north.current_color());
        // West
        let (west_x, west_y) = rotate_point_fn(mouse_x - self.cursor_offset, mouse_y);
        draw_cursor(west_x, west_y, self.west.current_color());
        // East
        let (east_x, east_y) = rotate_point_fn(mouse_x + self.cursor_offset, mouse_y);
        draw_cursor(east_x, east_y, self.east.current_color());
        // South
        let (south_x, south_y) = rotate_point_fn(mouse_x, mouse_y + self.cursor_offset);
        draw_cursor(south_x, south_y, self.south.current_color());

        // Winner text
        let text_y_offset = self.cursor_offset + CURSOR_SIZE + self.winner_text_offset;
        bq::draw_text(
            &self.winner_text,
            TextAlignment::Left,
            None,
            self.winner_text_font_size,
            self.winner_text_animation.current_color(),
            TextAnchorPoint::Center {
                // should be bottom center :P but not supported yet
                x: mouse_x,
                y: mouse_y - text_y_offset,
            },
            None,
        );
    }
}
