//! # Why?
//!
//! `mq::draw_circle()` and `mq::draw_circle_lines()` don't use the same number of polygon sides, so
//! you can't use them to overlap cleanly. Use this mod instead.
use crate::mq;

const SIDES: u8 = 50;
const ROTATION: f32 = 0.0;

pub fn draw(x: f32, y: f32, radius: f32, color: mq::Color) {
    mq::draw_poly(x, y, SIDES, radius, ROTATION, color);
}

pub fn draw_outline(x: f32, y: f32, radius: f32, thickness: f32, color: mq::Color) {
    mq::draw_poly_lines(x, y, SIDES, radius, ROTATION, thickness, color);
}
