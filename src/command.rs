#[derive(Debug)]
pub enum Command {
    Down,
    Left,
    LongDown,
    LongLeft,
    LongRight,
    LongUp,
    Quit,
    Right,
    ShortDown,
    ShortLeft,
    ShortRight,
    ShortUp,
    Up,
    None,
}

impl From<String> for Command {
    fn from(message: String) -> Self {
        match message.trim().to_lowercase().as_str() {
            "down" => Self::Down,
            "left" => Self::Left,
            "right" => Self::Right,
            "up" => Self::Up,
            "long down" => Self::LongDown,
            "long right" => Self::LongRight,
            "long left" => Self::LongLeft,
            "long up" => Self::LongUp,
            "short down" => Self::ShortDown,
            "short right" => Self::ShortRight,
            "short left" => Self::ShortLeft,
            "short up" => Self::ShortUp,
            "quit" => Self::Quit,
            _ => Self::None,
        }
    }
}
