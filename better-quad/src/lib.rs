//! Better abstractions and utilities than macroquad ("mq").
pub(crate) mod bq_button;
pub(crate) mod bq_circle;
pub(crate) mod bq_color;
pub(crate) mod bq_fps;
pub(crate) mod bq_keys;
pub(crate) mod bq_rand;
pub(crate) mod bq_text;
pub(crate) mod bq_timestamp;
pub(crate) mod init;
pub(crate) mod stateful_gui;

pub mod utils;

pub use macroquad::prelude as mq;

pub use init::initialize_engine;
pub use stateful_gui::{run_gui, run_gui_default, StatefulGui};

pub mod bq {
    //! prelude
    pub use crate::bq_button::*;
    pub use crate::bq_circle::*;
    pub use crate::bq_color::*;
    pub use crate::bq_fps::*;
    pub use crate::bq_keys::*;
    pub use crate::bq_rand::*;
    pub use crate::bq_text::*;
    pub use crate::bq_timestamp::*;
}
