use crate::mq;

/// TODO: figure out why there's a bug in the background being too short height, or just write my own
/// multiline text since mq is so hard to debug
pub fn bug_repro() {
    draw_multiline_left_aligned_text(
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
    draw_multiline_left_aligned_text(
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
}

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
    let mut total_height = 0f32;
    for text_dimensions in &text_line_dimensions {
        max_width = max_width.max(text_dimensions.width);
        // if total_height != 0.0 then increment height by spacing, but we don't have spacing

        // Hack to fix multiline empty string being ignored
        total_height += if text_dimensions.height <= 0.0 {
            font_size as f32
        } else {
            text_dimensions.height
        };
    }

    let text_x = text_center_point.x - (max_width / 2.0);
    let text_y = text_center_point.y - (total_height / 2.0); // TODO: should this be +?

    if let Some(background) = opt_background_rectangle {
        let first_y_offset = text_line_dimensions[0].offset_y;
        draw_text_background_rectangle(
            background,
            text_x,
            text_y - first_y_offset,
            max_width,
            total_height + first_y_offset,
        );
    }

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
