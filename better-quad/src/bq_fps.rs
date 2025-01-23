use crate::bq_text::{TextBackground, TextTopLeftPoint};
use crate::bq_timestamp::Timestamp;
use crate::{bq_text, mq};
use std::time::Duration;

pub struct FpsCounter {
    last_update_time: Timestamp,
    last_period_duration: Duration,
    last_period_fps: u32,
    current_period_frames: u32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            last_update_time: Timestamp::now(),
            last_period_duration: Duration::ZERO,
            last_period_fps: 0,
            current_period_frames: 0,
        }
    }

    pub fn tick_frame(&mut self, now: Timestamp) {
        self.current_period_frames += 1;
        self.check_for_second_rollover(now);
    }

    fn check_for_second_rollover(&mut self, now: Timestamp) {
        let delta = now - self.last_update_time;
        if delta.as_secs_f64() >= 1.0 {
            self.last_update_time = now;
            self.last_period_duration = delta;
            self.last_period_fps = (self.current_period_frames as f64 / delta.as_secs_f64()) as u32;
            self.current_period_frames = 0;
        }
    }

    pub fn fps(&self) -> u32 {
        self.last_period_fps
    }

    /// Not really needed, this was only used to initially confirm the duration is nearly always 1.000 seconds.
    pub fn duration_of_last_period(&self) -> Duration {
        self.last_period_duration
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Opinionated, single-purpose way to display FPS.
pub fn draw_fps_text_bottom_right(fps_counter: &FpsCounter) -> TextTopLeftPoint {
    const FPS_FONT_SIZE: u16 = 20;
    const FPS_TEXT_PADDING: f32 = 3.0;

    let fps_text = format!("{} FPS", fps_counter.fps());
    let fps_text_dim = mq::measure_text("120 FPS", None, FPS_FONT_SIZE, 1.0);
    let fps_text_x = mq::screen_width() - fps_text_dim.width - (FPS_TEXT_PADDING * 2.0);
    let fps_text_y = mq::screen_height() - fps_text_dim.offset_y - (FPS_TEXT_PADDING * 2.0);
    let top_left_point = TextTopLeftPoint::new(fps_text_x, fps_text_y);
    bq_text::draw_text(
        fps_text,
        None,
        FPS_FONT_SIZE,
        mq::BLACK,
        top_left_point,
        Some(TextBackground {
            color: mq::Color::new(1.00, 1.00, 1.00, 0.9),
            x_padding: FPS_TEXT_PADDING,
            y_padding: FPS_TEXT_PADDING,
        }),
    );

    top_left_point
}
