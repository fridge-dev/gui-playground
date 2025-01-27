use crate::mq;

pub trait BetterKeyCode {
    fn to_lowercase(&self) -> String;
}

impl BetterKeyCode for mq::KeyCode {
    fn to_lowercase(&self) -> String {
        format!("{self:?}").to_lowercase()
    }
}
