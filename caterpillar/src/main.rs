use better_quad::{mq, StatefulGui};
use caterpillar::SnakeGameState;

fn window_conf() -> mq::Conf {
    SnakeGameState::main_conf()
}

#[macroquad::main(window_conf)]
async fn main() {
    better_quad::initialize_engine();
    better_quad::run_gui_default::<SnakeGameState>().await
}
