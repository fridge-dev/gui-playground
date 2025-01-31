use crate::bq::Timestamp;
use std::time::Duration;

/// Repeating animation based on time (as opposed to frames).
pub struct TimeBasedAnimationTicker {
    animation_start: Timestamp,
    animation_duration: Duration,
    current_animation_percent: f32, // [0, 1)
}

impl TimeBasedAnimationTicker {
    pub fn new(animation_start: Timestamp, animation_duration: Duration) -> Self {
        Self {
            animation_start,
            animation_duration,
            current_animation_percent: 0.0,
        }
    }

    pub fn tick(&mut self, now: Timestamp) {
        let animation_total_run_time = now - self.animation_start;
        let number_of_circles =
            animation_total_run_time.as_secs_f32() / self.animation_duration.as_secs_f32();
        self.current_animation_percent = number_of_circles % 1.0;
    }

    /// Returns percentage as a `[0.0, 1.0]` floating point number.
    pub fn current_animation_percent(&self) -> f32 {
        self.current_animation_percent
    }
}

/// Repeating animation based on frames (as opposed to time).
pub struct FrameBasedAnimationTicker {
    frames_per_animation: u32,
    // current_frame is [0, frames_per_transition)
    current_frame: u32,
}

impl FrameBasedAnimationTicker {
    pub fn for_num_frames(frames_per_animation: u32) -> Self {
        Self {
            frames_per_animation,
            current_frame: 0,
        }
    }

    pub fn for_duration(animation_duration: Duration, assumed_fps: f32) -> Self {
        let seconds_per_frame = 1.0 / assumed_fps;
        let frames_per_animation = animation_duration.as_secs_f32() / seconds_per_frame;
        Self::for_num_frames(frames_per_animation as u32)
    }

    pub fn tick_frame(&mut self) {
        self.current_frame = (self.current_frame + 1) % self.frames_per_animation
    }

    pub fn current_frame(&self) -> u32 {
        self.current_frame
    }

    /// Returns [0.0, 1.0) percent completion of the animation.
    pub fn animation_percent(&self) -> f32 {
        self.current_frame as f32 / self.frames_per_animation as f32
    }
}
