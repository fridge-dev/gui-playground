use better_quad::StatefulGui;
use macroquad::prelude as mq;
use turn_time_tracker::TurnTimeTracker;

fn window_conf() -> mq::Conf {
    TurnTimeTracker::main_conf()
}

#[macroquad::main(window_conf)]
async fn main() {
    better_quad::initialize_engine();
    better_quad::run_gui(example_turn_time_tracker()).await
}

fn example_turn_time_tracker() -> TurnTimeTracker {
    // TODO:2 replace with dynamic player/color selection
    // https://github.com/not-fl3/particles-editor/blob/master/src/main.rs#L13-L130
    // https://docs.rs/macroquad/latest/src/events/events.rs.html
    let players = vec![
        ("Marceline", mq::YELLOW),
        ("Bonnibel", mq::SKYBLUE),
        ("Dorian", mq::PINK),
        ("Leo", mq::GREEN),
        ("Tiger", mq::DARKBLUE),
        ("Russet", mq::ORANGE),
        ("Pudding", mq::DARKBROWN),
        ("Cranberry", mq::RED),
    ];

    TurnTimeTracker::with_players(players)
}
