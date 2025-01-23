use macroquad::prelude as mq;
use std::sync::atomic;
use std::sync::atomic::AtomicU64;

/// Track the last set seed to be readable. Still use `mq::rand()` to produce randoms.
///
/// This pattern doesn't elegantly handle being uninitialized. We're relying on init.rs behavior to
/// enforce that a rand seed is set before starting a GUI, and we're somewhat assuming correctness in
/// multi-threaded environments is not strictly defined.
///
/// This re-uses the pattern of mq's RandGenerator global, so these should be fine assumptions.
static GLOBAL_STATE: RandState = RandState {
    last_set_seed: AtomicU64::new(0),
};

struct RandState {
    last_set_seed: AtomicU64,
}

/// Use this when you want to set any new seed and you don't care what it's set to.
pub fn randomize_rand_seed() {
    set_rand_seed(mq::miniquad::date::now() as _);
}

pub fn set_rand_seed(seed: u64) {
    GLOBAL_STATE
        .last_set_seed
        .store(seed, atomic::Ordering::Relaxed);
    mq::rand::srand(seed);
}

pub fn get_last_set_rand_seed() -> u64 {
    GLOBAL_STATE.last_set_seed.load(atomic::Ordering::Relaxed)
}
