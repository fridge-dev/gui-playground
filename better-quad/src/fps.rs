use crate::timestamp::Timestamp;
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
