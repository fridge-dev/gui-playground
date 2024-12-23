use crate::framework::timestamp::Timestamp;
use macroquad::prelude as mq;

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
    loop {
        gui.update(Timestamp::now());
        gui.draw();
        mq::next_frame().await;
    }
}

pub async fn run_gui_default<T: StatefulGui + Default>() {
    run_gui(T::default()).await
}
