use crate::bq_timestamp::Timestamp;
use macroquad::prelude as mq;
use std::hash::Hasher;
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
    // Hash current timestamp to use as rand seed. We could just use timestamp itself, but this makes
    // it visually appear that the seeds are non-sequential. That wouldn't affect randomness quality
    // in a PRNG, it's just visually satisfying when displaying the seed.
    let mut hasher = std::hash::DefaultHasher::new();
    hasher.write_u64(Timestamp::now().as_sec_f64() as u64);
    let seed = hasher.finish();

    set_rand_seed(seed);
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
