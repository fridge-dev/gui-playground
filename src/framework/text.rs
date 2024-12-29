use macroquad::prelude as mq;

pub(crate) struct TextContainer {
    x_range: (f32, f32),
    y_range: (f32, f32),
}

impl TextContainer {
    pub(crate) fn new(x_range: (f32, f32), y_range: (f32, f32)) -> Self {
        assert!(x_range.1 > x_range.0);
        assert!(y_range.1 > y_range.0);
        Self { x_range, y_range }
    }

    pub(crate) fn window() -> Self {
        Self::new((0.0, mq::screen_width()), (0.0, mq::screen_height()))
    }
}

pub(crate) fn draw_centered_text(
    text: impl AsRef<str>,
    font: Option<&mq::Font>,
    font_size: u16,
    color: mq::Color,
    text_container: TextContainer,
) {
    let text_dimensions = mq::measure_text(text.as_ref(), font, font_size, 1.0);
    let x_offset =
        (text_container.x_range.1 - text_container.x_range.0 - text_dimensions.width) / 2.0;
    let y_offset =
        (text_container.y_range.1 - text_container.y_range.0 - text_dimensions.height) / 2.0;
    mq::draw_text(
        text.as_ref(),
        text_container.x_range.0 + x_offset,
        text_container.y_range.0 + y_offset,
        font_size as f32,
        color,
    );
}
