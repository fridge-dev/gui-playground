use macroquad::prelude as mq;
use turn_time_tracker::TurnTimeTrackerState;

#[macroquad::main("Tabletop Turn Time Tracker")]
async fn main() {
    let mut state = TurnTimeTrackerState::new();
    state.add_player("Bna", mq::YELLOW);
    state.add_player("Zla", mq::SKYBLUE);
    state.add_player("Dorian", mq::PINK);
    state.add_player("Russet", mq::DARKBROWN);
    state.add_player("Pudding", mq::BEIGE);
    state.add_player("Cranberry", mq::RED);

    turn_time_tracker::run_gui(state).await
}
