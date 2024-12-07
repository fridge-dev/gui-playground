use macroquad::prelude as mq;

pub mod snake_example;

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
    // Both called once per frame. Helps separate state mutations and drawing.
    // TODO: provide timestamp now?
    fn update(&mut self);
    fn draw(&self);
}
