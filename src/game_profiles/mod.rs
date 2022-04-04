use std::fmt::Display;

use crate::{buttons::Button, command::Command};

pub mod vampire_survivors;

#[derive(Debug)]
pub enum Games {
    VampireSurvivors,
}

impl Games {
    pub fn get_button(&self, command: Command) -> Option<Button> {
        match self {
            Games::VampireSurvivors => vampire_survivors::handle_command(command),
        }
    }

    pub fn help(&self) -> String {
        match self {
            Games::VampireSurvivors => vampire_survivors::help(),
        }
    }
}

impl Display for Games {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Games::VampireSurvivors => "Vampire Survivors",
        };
        write!(f, "{}", name)
    }
}
