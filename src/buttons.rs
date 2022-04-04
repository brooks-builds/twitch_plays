use std::time::Duration;

pub const NORMAL: u64 = 500;
pub const NORMAL_DURATION: Duration = Duration::from_millis(NORMAL);
pub const LONG_DURATION: Duration = Duration::from_millis(NORMAL * 4);
pub const SHORT_DURATION: Duration = Duration::from_millis(NORMAL / 4);

use enigo::Key;

#[derive(Clone, Copy)]
pub enum Button {
    UpArrow(Duration),
    DownArrow(Duration),
    RightArrow(Duration),
    LeftArrow(Duration),
    Enter(Duration),
    W(Duration),
    A(Duration),
    S(Duration),
    D(Duration),
    None,
}

impl Button {
    pub fn get_duration(&self) -> Duration {
        match self {
            Button::UpArrow(duration) => duration.clone(),
            Button::DownArrow(duration) => duration.clone(),
            Button::RightArrow(duration) => duration.clone(),
            Button::LeftArrow(duration) => duration.clone(),
            Button::Enter(duration) => duration.clone(),
            Button::None => unreachable!(),
            Button::W(duration) => duration.clone(),
            Button::A(duration) => duration.clone(),
            Button::S(duration) => duration.clone(),
            Button::D(duration) => duration.clone(),
        }
    }
}

impl Into<Key> for Button {
    fn into(self) -> Key {
        match self {
            Button::UpArrow(_) => Key::UpArrow,
            Button::DownArrow(_) => Key::DownArrow,
            Button::RightArrow(_) => Key::RightArrow,
            Button::LeftArrow(_) => Key::LeftArrow,
            Button::Enter(_) => Key::Return,
            Button::None => unreachable!(),
            Button::W(_) => Key::Layout('w'),
            Button::A(_) => Key::Layout('a'),
            Button::S(_) => Key::Layout('s'),
            Button::D(_) => Key::Layout('d'),
        }
    }
}
