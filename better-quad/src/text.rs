use crate::mq;

/// Draws entire block of *left-aligned* text at a center point.
pub fn draw_multiline_left_aligned_text(
    text: impl AsRef<str>,
    font: Option<&mq::Font>,
    font_size: u16,
    text_color: mq::Color,
    text_center_point: TextCenterPoint,
    opt_background_rectangle: Option<TextBackground>,
) {
    let text = text.as_ref();
    let text_line_dimensions = text
        .lines()
        .map(|line| mq::measure_text(line, font, font_size, 1.0))
        .collect::<Vec<_>>();

    let mut max_width = 0f32;
    for text_dimensions in &text_line_dimensions {
        max_width = max_width.max(text_dimensions.width);
    }

    let mut height_of_last_line = text_line_dimensions.last().unwrap().height;
    if height_of_last_line <= 0.0 {
        // Hack to fix empty string last line being ignored
        height_of_last_line = font_size as f32;
    }
    let total_height =
        (text_line_dimensions.len() - 1) as f32 * font_size as f32 + height_of_last_line;

    let rect_x = text_center_point.x - (max_width / 2.0);
    let rect_y = text_center_point.y - (total_height / 2.0);
    if let Some(background) = opt_background_rectangle {
        draw_text_background_rectangle(background, rect_x, rect_y, max_width, total_height);
    }

    let text_x = rect_x;
    // mq is weird here. y is the y of the first line.
    let text_y = rect_y + text_line_dimensions[0].height;
    mq::draw_multiline_text(
        text,
        text_x,
        text_y,
        font_size as f32,
        Some(1.0),
        text_color,
    );
}

pub fn draw_centered_text(
    text: impl AsRef<str>,
    font: Option<&mq::Font>,
    font_size: u16,
    color: mq::Color,
    text_center_point: TextCenterPoint,
    opt_background_rectangle: Option<TextBackground>,
) {
    let text_dimensions = mq::measure_text(text.as_ref(), font, font_size, 1.0);
    let text_x = text_center_point.x - (text_dimensions.width / 2.0);
    let text_y = text_center_point.y + (text_dimensions.height / 2.0);

    if let Some(background) = opt_background_rectangle {
        draw_text_background_rectangle(
            background,
            text_x,
            text_y - text_dimensions.offset_y,
            text_dimensions.width,
            text_dimensions.height,
        )
    }

    mq::draw_text(text.as_ref(), text_x, text_y, font_size as f32, color);
}

// TODO: generic support for anchor points
pub struct TextCenterPoint {
    x: f32,
    y: f32,
}

impl TextCenterPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn for_window() -> Self {
        Self::new(mq::screen_width() / 2.0, mq::screen_height() / 2.0)
    }
}

pub struct TextTopLeftPoint {
    x: f32,
    y: f32,
}

impl TextTopLeftPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// TODO: support multiline text
pub fn draw_text(
    text: impl AsRef<str>,
    font: Option<&mq::Font>,
    font_size: u16,
    color: mq::Color,
    text_top_left_point: TextTopLeftPoint,
    opt_background_rectangle: Option<TextBackground>,
) {
    let text_dimensions = mq::measure_text(text.as_ref(), font, font_size, 1.0);

    if let Some(background) = opt_background_rectangle {
        draw_text_background_rectangle(
            background,
            text_top_left_point.x,
            text_top_left_point.y,
            text_dimensions.width,
            text_dimensions.height,
        )
    }

    mq::draw_text(
        text.as_ref(),
        text_top_left_point.x,
        text_top_left_point.y + text_dimensions.offset_y,
        font_size as f32,
        color,
    );
}

#[derive(Copy, Clone)]
pub struct TextBackground {
    pub color: mq::Color,
    pub x_padding: f32,
    pub y_padding: f32,
}

fn draw_text_background_rectangle(
    background: TextBackground,
    text_x: f32,
    text_y: f32,
    text_width: f32,
    text_height: f32,
) {
    mq::draw_rectangle(
        text_x - background.x_padding,
        text_y - background.y_padding,
        text_width + background.x_padding * 2.0,
        text_height + background.y_padding * 2.0,
        background.color,
    );
}
