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

#[allow(dead_code)] // premature abstractions, but I have relatively high confidence they may be useful, knowing std lib types
impl Timestamp {
    pub fn now() -> Self {
        Self {
            seconds: mq::get_time(),
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
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        self.duration_since(rhs).unwrap()
    }
}
