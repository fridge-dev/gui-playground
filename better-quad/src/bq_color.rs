use crate::mq;

pub fn invert_color(color: mq::Color) -> mq::Color {
    mq::Color {
        r: 1.0 - color.r,
        g: 1.0 - color.g,
        b: 1.0 - color.b,
        a: color.a,
    }
}
