use better_quad::text::{TextBackground, TextCenterPoint, TextTopLeftPoint};
use better_quad::timestamp::Timestamp;
use better_quad::{fine_circle, mq, text, StatefulGui};

fn window_conf() -> mq::Conf {
    BugRepro::main_conf()
}

#[macroquad::main(window_conf)]
async fn main() {
    better_quad::initialize_engine();
    better_quad::run_gui_default::<BugRepro>().await
}

#[derive(Default)]
struct BugRepro;

impl StatefulGui for BugRepro {
    fn main_conf() -> mq::Conf {
        mq::Conf {
            window_width: 500,
            window_height: 500,
            ..Default::default()
        }
    }

    fn update(&mut self, _: Timestamp) {}

    fn draw(&self) {
        mq::clear_background(mq::BROWN);
        draw_mouse_coordinates();
        draw_text_examples();
    }
}

fn draw_mouse_coordinates() {
    let (mouse_x, mouse_y) = mq::mouse_position();
    let mouse_x = mouse_x as u32;
    let mouse_y = mouse_y as u32;
    text::draw_text(
        format!("({mouse_x:3}, {mouse_y:3})"),
        None,
        25,
        mq::BLACK,
        TextTopLeftPoint::new(0.0, 0.0),
        Some(TextBackground {
            color: mq::WHITE,
            x_padding: 0.0,
            y_padding: 0.0,
        }),
    )
}

fn draw_text_examples() {
    text::draw_multiline_left_aligned_text(
        "bbbb",
        None,
        30,
        mq::BLACK,
        TextCenterPoint::new(100.0, 100.0),
        Some(TextBackground {
            color: mq::WHITE,
            x_padding: 2.0,
            y_padding: 2.0,
        }),
    );
    fine_circle::draw(100.0, 100.0, 3.0, mq::RED);

    text::draw_multiline_left_aligned_text(
        "aaaa\nbbbb\ncccc",
        None,
        30,
        mq::BLACK,
        TextCenterPoint::new(200.0, 100.0),
        Some(TextBackground {
            color: mq::WHITE,
            x_padding: 2.0,
            y_padding: 2.0,
        }),
    );
    fine_circle::draw(200.0, 100.0, 3.0, mq::RED);
}
