use crate::bq::{TextAlignment, TextAnchorPoint};
use crate::{bq, mq};

// TODO: make work with non-left click
const MOUSE_BUTTON: mq::MouseButton = mq::MouseButton::Left;

/// Button GUI element to track state and handling drawing math for you
pub struct SimpleButton {
    state: PressState,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum PressState {
    Unpressed,
    PressedContains,
    PressedNotContains,
}

pub enum ButtonAction {
    TriggerOnClick,
    NoAction,
}

impl ButtonAction {
    pub fn should_trigger_action(self) -> bool {
        matches!(self, Self::TriggerOnClick)
    }
}

impl SimpleButton {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            state: PressState::Unpressed,
            x,
            y,
            width,
            height,
        }
    }

    pub fn tick_state(&mut self) -> ButtonAction {
        let mut action = ButtonAction::NoAction;

        match &self.state {
            PressState::Unpressed => {
                // Check for click and position
                if mq::is_mouse_button_pressed(MOUSE_BUTTON) && self.is_cursor_on_button() {
                    self.state = PressState::PressedContains;
                }
            }
            PressState::PressedContains | PressState::PressedNotContains => {
                // Check for movement
                if self.is_cursor_on_button() {
                    self.state = PressState::PressedContains
                } else {
                    self.state = PressState::PressedNotContains
                }

                // Check for release
                if mq::is_mouse_button_released(MOUSE_BUTTON) {
                    // Click released, check position
                    if self.state == PressState::PressedContains {
                        action = ButtonAction::TriggerOnClick;
                    }

                    // Always unset `is_pressed` when releasing click
                    self.state = PressState::Unpressed;
                }
            }
        }

        action
    }

    fn is_cursor_on_button(&self) -> bool {
        let (mouse_x, mouse_y) = mq::mouse_position();
        let x_range = self.x..(self.x + self.width);
        let y_range = self.y..(self.y + self.height);
        x_range.contains(&mouse_x) && y_range.contains(&mouse_y)
    }

    pub fn is_pressed(&self) -> bool {
        matches!(self.state, PressState::PressedContains)
    }

    pub fn draw(
        &self,
        btn_color: mq::Color,
        btn_border_color: mq::Color,
        btn_border_thickness: f32,
        text: impl AsRef<str>,
        font_size: u16,
        text_color: mq::Color,
    ) {
        mq::draw_rectangle(self.x, self.y, self.width, self.height, btn_color);
        mq::draw_rectangle_lines(
            self.x,
            self.y,
            self.width,
            self.height,
            btn_border_thickness,
            btn_border_color,
        );
        bq::draw_text(
            text,
            TextAlignment::Center,
            None,
            font_size,
            text_color,
            TextAnchorPoint::Center {
                x: self.x + self.width / 2.0,
                y: self.y + self.height / 2.0,
            },
            None,
        );
    }
}
