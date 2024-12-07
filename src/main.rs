use turn_time_tracker::snake_example::SnakeGameState;

#[macroquad::main("Tabletop Turn Time Tracker")]
async fn main() {
    turn_time_tracker::run_gui_default::<SnakeGameState>().await
}
