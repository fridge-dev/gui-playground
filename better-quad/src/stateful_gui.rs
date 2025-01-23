use crate::bq_timestamp::Timestamp;
use crate::init;
use crate::mq;

/// Helpful to be disciplined about separating state mutations and drawing, and to more easily
/// remember how to integrate with mq. Otherwise not a super useful abstraction.
pub trait StatefulGui {
    fn main_conf() -> mq::Conf {
        mq::Conf::default()
    }

    // Both called once per frame.
    fn update(&mut self, now: Timestamp);
    fn draw(&self);
}

pub async fn run_gui<T: StatefulGui>(mut gui: T) {
    assert!(
        init::is_initialized(),
        "Must call initialize_engine() before running any app"
    );

    loop {
        gui.update(Timestamp::now());
        gui.draw();
        mq::next_frame().await;
    }
}

pub async fn run_gui_default<T: StatefulGui + Default>() {
    run_gui(T::default()).await
}
