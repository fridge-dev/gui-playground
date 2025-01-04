use crate::bq_rand;
use once_cell::sync::Lazy;

static INIT: Lazy<()> = Lazy::new(do_init);

fn do_init() {
    bq_rand::randomize_seed();
}

/// Must call before starting game engine.
///
/// # Why?
///
/// This currently only initializes the seeded RNG, needed because macroquad has an objectively bad
/// API for rand. See https://github.com/not-fl3/macroquad/issues/369
pub fn initialize_engine() {
    Lazy::force(&INIT);
}

pub(crate) fn is_initialized() -> bool {
    Lazy::get(&INIT).is_some()
}
