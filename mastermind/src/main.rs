use better_quad::{mq, StatefulGui};
use mastermind::MastermindGame;

fn window_conf() -> mq::Conf {
    MastermindGame::main_conf()
}

#[macroquad::main(window_conf)]
async fn main() {
    better_quad::initialize_engine();
    better_quad::run_gui_default::<MastermindGame>().await
}
