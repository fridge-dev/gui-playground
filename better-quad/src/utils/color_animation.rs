use crate::mq;
use crate::utils::infinite_iterator::InfiniteIterator;
use std::time::Duration;

/// Assume 120 FPS cuz YOLO
const ASSUMED_FPS: f32 = 120.0;
const ASSUMED_FRAME_DURATION_MS: f32 = 1000.0 / ASSUMED_FPS;

/// How long a transition between 2 colors should be.
#[derive(Debug, Copy, Clone)]
pub enum TransitionLength {
    TimedDuration(Duration),
    NumFrames(u32),
}

impl TransitionLength {
    fn to_num_frames(self) -> u32 {
        let frames_per_transition = match self {
            TransitionLength::TimedDuration(transition_duration) => {
                (transition_duration.as_millis() as f32 / ASSUMED_FRAME_DURATION_MS) as u32
            }
            TransitionLength::NumFrames(num_frames) => num_frames,
        };
        assert!(
            frames_per_transition > 1,
            "Transition length too small: {:?}",
            self
        );

        frames_per_transition
    }
}

/// Animate between a sequence of colors by iterating through the sequence.
pub struct StepColorAnimation {
    colors: InfiniteIterator<mq::Color>,
    frames_per_transition: u32,
    // current_frame is [0, frames_per_transition)
    current_frame: u32,
}

impl StepColorAnimation {
    pub fn new(colors: &[mq::Color], transition_length: TransitionLength) -> Self {
        Self {
            colors: InfiniteIterator::from(colors),
            frames_per_transition: transition_length.to_num_frames(),
            current_frame: 0,
        }
    }

    pub fn tick_frame(&mut self) {
        self.current_frame += 1;

        if self.current_frame == self.frames_per_transition {
            self.current_frame = 0;
            self.colors.advance();
        }
    }

    pub fn current_color(&self) -> mq::Color {
        *self.colors.current()
    }
}

/// Animate between a sequence of colors with smooth intermediate colors.
pub struct SmoothColorAnimation {
    color_transitions: InfiniteIterator<ColorTransition>,
    frames_per_transition: u32,
    // current_frame is [0, frames_per_transition)
    current_frame: u32,
}

struct ColorTransition {
    start: mq::Color,
    end: mq::Color,
}

impl SmoothColorAnimation {
    pub fn new(target_colors: &[mq::Color], transition_length: TransitionLength) -> Self {
        let mut colors = Vec::with_capacity(target_colors.len());
        for i in 0..target_colors.len() {
            let end_index = (i + 1) % target_colors.len();
            colors.push(ColorTransition {
                start: target_colors[i],
                end: target_colors[end_index],
            });
        }

        Self {
            color_transitions: InfiniteIterator::from(colors),
            frames_per_transition: transition_length.to_num_frames(),
            current_frame: 0,
        }
    }

    // idk if "frames" are a good abstraction to use. Maybe time based would be smoother and
    // handle intermittent scheduling issues better. Just going to start with this for now.
    pub fn tick_frame(&mut self) {
        self.current_frame += 1;

        if self.current_frame == self.frames_per_transition {
            self.current_frame = 0;
            self.color_transitions.advance();
        }
    }

    pub fn current_color(&self) -> mq::Color {
        let transition_percent = self.current_frame as f32 / self.frames_per_transition as f32;

        let transition = self.color_transitions.current();
        let start = transition.start;
        let end = transition.end;

        mq::Color {
            r: start.r + transition_percent * (end.r - start.r),
            g: start.g + transition_percent * (end.g - start.g),
            b: start.b + transition_percent * (end.b - start.b),
            a: start.a + transition_percent * (end.a - start.a),
        }
    }
}
