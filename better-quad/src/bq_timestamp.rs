use crate::mq;
use std::ops::Sub;
use std::time::Duration;

/// Wrapper of time so we can have a non-primitive type for time to disambiguate UOM.
///
/// Why not SystemTime/etc? Those don't exist on WASM.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Timestamp {
    seconds: f64,
}

impl Timestamp {
    pub fn now() -> Self {
        Self {
            seconds: mq::miniquad::date::now(),
        }
    }

    pub fn as_sec_f64(&self) -> f64 {
        self.seconds
    }

    pub fn duration_since(&self, earlier: Timestamp) -> Option<Duration> {
        let delta_seconds = self.seconds - earlier.seconds;
        if delta_seconds >= 0.0 {
            Some(Duration::from_secs_f64(delta_seconds))
        } else {
            None
        }
    }

    // note: Shouldn't need `elapsed` as their should always be the provided `now` to do math with.
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs).unwrap()
    }
}
