use macroquad::prelude as mq;

pub mod snake_example;
mod timestamp;
mod turn_time_tracker;

use crate::timestamp::Timestamp;
pub use turn_time_tracker::TurnTimeTrackerState;

pub async fn run_gui_default<T: StatefulGui + Default>() {
    run_gui(T::default()).await
}

pub async fn run_gui<T: StatefulGui>(mut gui: T) {
    loop {
        gui.update(Timestamp::now());
        gui.draw();
        mq::next_frame().await;
    }
}

pub trait StatefulGui {
    fn main_conf() -> mq::Conf {
        mq::Conf::default()
    }

    // Both called once per frame. Helps separate state mutations and drawing.
    fn update(&mut self, now: Timestamp);
    fn draw(&self);
}
