use crate::mq;

const FONT_SCALE: f32 = 1.0;

/// Draws block of text at an anchor point.
pub fn draw_text(
    text: impl AsRef<str>,
    alignment: TextAlignment,
    font: Option<&mq::Font>,
    font_size: u16,
    text_color: mq::Color,
    text_anchor_point: TextAnchorPoint,
    opt_background_rectangle: Option<TextBackground>,
) -> TextContainer {
    let text = text.as_ref();
    let multiline_text_dimensions = measure_multiline_text(text, font, font_size);
    let text_container = TextContainer::compute(
        text_anchor_point,
        &multiline_text_dimensions,
        &opt_background_rectangle,
    );

    if let Some(background) = opt_background_rectangle {
        // Note: Background concept of "padding" could be decoupled from color, if padding is useful
        // without color.
        text_container.draw_rect(background.color);
    }

    let text_x_offset = text_container.rect_x + text_container.text_padding_x;

    // Starting with offset_y then incrementing by font_size is an artifact of refactoring mq::draw_multiline_text_ex().
    let mut text_y = text_container.rect_y
        + text_container.text_padding_y
        + multiline_text_dimensions.text_line_dimensions[0].offset_y;

    for (i, line) in text.lines().enumerate() {
        let text_x_diff = match alignment {
            TextAlignment::Left => 0.0,
            TextAlignment::Center => {
                (multiline_text_dimensions.max_width
                    - multiline_text_dimensions.text_line_dimensions[i].width)
                    / 2.0
            }
            TextAlignment::Right => {
                multiline_text_dimensions.max_width
                    - multiline_text_dimensions.text_line_dimensions[i].width
            }
        };

        let text_x = text_x_offset + text_x_diff;
        mq::draw_text(line, text_x, text_y, font_size as f32, text_color);
        text_y += font_size as f32;
    }

    text_container
}

struct MultilineTextDimensions {
    text_line_dimensions: Vec<mq::TextDimensions>,
    max_width: f32,
    total_height: f32,
}

/// Returns (max_width, total_height) of rect to contain text
fn measure_multiline_text(
    text: &str,
    font: Option<&mq::Font>,
    font_size: u16,
) -> MultilineTextDimensions {
    let text_line_dimensions = text
        .lines()
        .map(|line| mq::measure_text(line, font, font_size, FONT_SCALE))
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

    MultilineTextDimensions {
        text_line_dimensions,
        max_width,
        total_height,
    }
}

/// How text is aligned if there are multiple lines. If it's a single line, it doesn't matter.
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// Imagine you have a text box. This describes which part of the text box is positioned at the
/// specified (x,y) coord.
#[derive(Copy, Clone)]
pub enum TextAnchorPoint {
    TopLeft { x: f32, y: f32 },
    TopRight { x: f32, y: f32 },
    Center { x: f32, y: f32 },
    BottomLeft { x: f32, y: f32 },
    BottomRight { x: f32, y: f32 },
}

impl TextAnchorPoint {
    pub fn window_centered() -> Self {
        Self::Center {
            x: mq::screen_width() / 2.0,
            y: mq::screen_height() / 2.0,
        }
    }

    pub fn window_bottom_left() -> Self {
        Self::BottomLeft {
            x: 0.0,
            y: mq::screen_height(),
        }
    }

    pub fn window_bottom_right() -> Self {
        Self::BottomRight {
            x: mq::screen_width(),
            y: mq::screen_height(),
        }
    }

    pub fn window_top_right() -> Self {
        Self::TopRight {
            x: mq::screen_width(),
            y: 0.0,
        }
    }
}

/// A visual UI rectangle to be drawn.
#[derive(Copy, Clone)]
pub struct TextBackground {
    pub color: mq::Color,
    pub x_padding: f32,
    pub y_padding: f32,
}

/// Rectangle that contains text. May or may not be a visible rectangle (see optional text background).
/// This hopefully alleviates annoyance of dealing with shapes anchored to **top** left coordinate, but
/// text anchored to **bottom** left coordinate.
#[derive(Copy, Clone)]
pub struct TextContainer {
    pub rect_x: f32,
    pub rect_y: f32,
    pub rect_width: f32,
    pub rect_height: f32,
    pub text_padding_x: f32,
    pub text_padding_y: f32,
}

impl TextContainer {
    fn compute(
        text_anchor_point: TextAnchorPoint,
        multiline_text_dimensions: &MultilineTextDimensions,
        opt_background_rectangle: &Option<TextBackground>,
    ) -> Self {
        let (background_rect_padding_x, background_rect_padding_y) = match opt_background_rectangle
        {
            None => (0.0, 0.0),
            Some(bg_rect) => (bg_rect.x_padding, bg_rect.y_padding),
        };

        let total_width = multiline_text_dimensions.max_width + background_rect_padding_x * 2.0;
        let total_height = multiline_text_dimensions.total_height + background_rect_padding_y * 2.0;

        let (x, y) = match text_anchor_point {
            TextAnchorPoint::TopLeft { x, y } => (x, y),
            TextAnchorPoint::TopRight { x, y } => (x - total_width, y),
            TextAnchorPoint::Center { x, y } => (x - (total_width / 2.0), y - (total_height / 2.0)),
            TextAnchorPoint::BottomLeft { x, y } => (x, y - total_height),
            TextAnchorPoint::BottomRight { x, y } => (x - total_width, y - total_height),
        };

        Self {
            rect_x: x,
            rect_y: y,
            rect_width: total_width,
            rect_height: total_height,
            text_padding_x: background_rect_padding_x,
            text_padding_y: background_rect_padding_y,
        }
    }

    fn draw_rect(&self, color: mq::Color) {
        mq::draw_rectangle(
            self.rect_x,
            self.rect_y,
            self.rect_width,
            self.rect_height,
            color,
        );
    }
}
