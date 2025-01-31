use crate::mq;
use crate::utils::animation_tickers::FrameBasedAnimationTicker;
use crate::utils::infinite_iterator::InfiniteIterator;
use std::time::Duration;

/// Assume 120 FPS cuz YOLO
const ASSUMED_FPS: f32 = 120.0;

/// How long a transition between 2 colors should be.
#[derive(Debug, Copy, Clone)]
pub enum TransitionLength {
    TimedDuration(Duration),
    NumFrames(u32),
}

impl TransitionLength {
    fn into_animation_ticker(self) -> FrameBasedAnimationTicker {
        match self {
            TransitionLength::TimedDuration(duration) => {
                FrameBasedAnimationTicker::for_duration(duration, ASSUMED_FPS)
            }
            TransitionLength::NumFrames(frames) => {
                FrameBasedAnimationTicker::for_num_frames(frames)
            }
        }
    }
}

/// Animate between a sequence of colors by iterating through the sequence.
pub struct StepColorAnimation {
    colors: InfiniteIterator<mq::Color>,
    animation_ticker: FrameBasedAnimationTicker,
}

impl StepColorAnimation {
    pub fn new(colors: &[mq::Color], transition_length: TransitionLength) -> Self {
        Self {
            colors: InfiniteIterator::from(colors),
            animation_ticker: transition_length.into_animation_ticker(),
        }
    }

    pub fn tick_frame(&mut self) {
        self.animation_ticker.tick_frame();

        if self.animation_ticker.current_frame() == 0 {
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
    animation_ticker: FrameBasedAnimationTicker,
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
            animation_ticker: transition_length.into_animation_ticker(),
        }
    }

    // idk if "frames" are a good abstraction to use. Maybe time based would be smoother and
    // handle intermittent scheduling issues better. Just going to start with this for now.
    // https://gafferongames.com/post/fix_your_timestep/
    pub fn tick_frame(&mut self) {
        self.animation_ticker.tick_frame();

        if self.animation_ticker.current_frame() == 0 {
            self.color_transitions.advance();
        }
    }

    pub fn current_color(&self) -> mq::Color {
        let transition_percent = self.animation_ticker.animation_percent();

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
