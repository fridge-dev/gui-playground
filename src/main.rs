use macroquad::prelude as mq;
use turn_time_tracker::{StatefulGui, TurnTimeTrackerState};

fn window_conf() -> mq::Conf {
    TurnTimeTrackerState::main_conf()
}

// TODO:3 run in web
#[macroquad::main(window_conf)]
async fn main() {
    let mut state = TurnTimeTrackerState::new();

    // TODO:2 replace with dynamic player/color selection
    // https://github.com/not-fl3/particles-editor/blob/master/src/main.rs#L13-L130
    state.add_player("Marceline", mq::YELLOW);
    state.add_player("Bonnibel", mq::SKYBLUE);
    state.add_player("Dorian", mq::PINK);
    state.add_player("Leo", mq::GREEN);
    state.add_player("Tiger", mq::DARKBLUE);
    state.add_player("Russet", mq::ORANGE);
    state.add_player("Pudding", mq::DARKBROWN);
    state.add_player("Cranberry", mq::RED);

    turn_time_tracker::run_gui(state).await
}
