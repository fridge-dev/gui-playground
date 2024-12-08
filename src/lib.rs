use macroquad::prelude as mq;

pub mod snake_example;
mod turn_time_tracker;

pub use turn_time_tracker::TurnTimeTrackerState;

pub async fn run_gui_default<T: StatefulGui + Default>() {
    run_gui(T::default()).await
}

pub async fn run_gui<T: StatefulGui>(mut gui: T) {
    loop {
        gui.update();
        gui.draw();
        mq::next_frame().await;
    }
}

pub trait StatefulGui {
    fn main_conf() -> mq::Conf {
        mq::Conf::default()
    }

    // Both called once per frame. Helps separate state mutations and drawing.
    // TODO: provide timestamp now?
    fn update(&mut self);
    fn draw(&self);
}
