use crate::draw_cursor;
use better_quad::bq::Timestamp;
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

/// Animation to draw 4 cursors with different animations. Just having some fun.
pub(crate) struct VictoryMouseAnimations {
    north: StepColorAnimation,
    west: SmoothColorAnimation,
    east: SmoothColorAnimation,
    south: SmoothColorAnimation,
    animation_start: Timestamp,
    current_rotation_percent: f32, // [0, 1)
    cursor_offset: f32,
}

impl VictoryMouseAnimations {
    pub(crate) fn new(
        palette: Vec<mq::Color>,
        animation_start: Timestamp,
        cursor_offset: f32,
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
            animation_start,
            current_rotation_percent: 0.0,
            cursor_offset,
        }
    }

    pub(crate) fn tick(&mut self, now: Timestamp) {
        self.north.tick_frame();
        self.west.tick_frame();
        self.east.tick_frame();
        self.south.tick_frame();

        let animation_total_run_time = now - self.animation_start;
        let number_of_circles =
            animation_total_run_time.as_secs_f32() / FULL_CIRCLE_ROTATION_DURATION.as_secs_f32();
        self.current_rotation_percent = number_of_circles % 1.0;
    }

    pub(crate) fn draw(&self, mouse_x: f32, mouse_y: f32) {
        // let's get weird

        let rotate_point_fn = |x: f32, y: f32| -> (f32, f32) {
            geometry::rotate_point(x, y, mouse_x, mouse_y, self.current_rotation_percent)
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
    }
}
