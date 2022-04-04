use crate::{
    buttons::{Button, LONG_DURATION, NORMAL_DURATION, SHORT_DURATION},
    command::Command,
};

pub fn handle_command(command: Command) -> Option<Button> {
    match command {
        Command::Down => Some(Button::S(NORMAL_DURATION)),
        Command::Left => Some(Button::A(NORMAL_DURATION)),
        Command::LongDown => Some(Button::S(LONG_DURATION)),
        Command::LongLeft => Some(Button::A(LONG_DURATION)),
        Command::LongRight => Some(Button::D(LONG_DURATION)),
        Command::LongUp => Some(Button::W(LONG_DURATION)),
        Command::Right => Some(Button::D(NORMAL_DURATION)),
        Command::ShortDown => Some(Button::S(SHORT_DURATION)),
        Command::ShortLeft => Some(Button::A(SHORT_DURATION)),
        Command::ShortRight => Some(Button::D(SHORT_DURATION)),
        Command::ShortUp => Some(Button::W(SHORT_DURATION)),
        Command::Up => Some(Button::W(NORMAL_DURATION)),
        Command::Enter => Some(Button::Enter(NORMAL_DURATION)),
        _ => None,
    }
}

pub fn help() -> String {
    "Commands are left(l), short left(sl), long left(ll), right(r), short right(sr), long right(lr), up(u), short up(su), long up(lu), down(d), short down(sd), long down(ld), enter"
    .to_owned()
}
