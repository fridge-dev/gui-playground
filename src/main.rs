use macroquad::prelude as mq;
use turn_time_tracker::TurnTimeTrackerState;

// TODO run in web
#[macroquad::main("Tabletop Turn Time Tracker")]
async fn main() {
    let mut state = TurnTimeTrackerState::new();

    // TODO: replace with dynamic player/color selection
    // https://github.com/not-fl3/particles-editor/blob/master/src/main.rs#L13-L130
    state.add_player("Bna", mq::YELLOW);
    state.add_player("Zla", mq::SKYBLUE);
    state.add_player("Dorian", mq::PINK);
    state.add_player("Leo", mq::GREEN);
    state.add_player("Tiger", mq::MAGENTA);
    state.add_player("Russet", mq::ORANGE);
    state.add_player("Pudding", mq::DARKBLUE);
    state.add_player("Cranberry", mq::RED);

    turn_time_tracker::run_gui(state).await
}
