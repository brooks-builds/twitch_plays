use std::fmt::Display;

pub mod vampire_survivors;

#[derive(Debug)]
pub enum Games {
    VampireSurvivors,
}

impl Display for Games {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Games::VampireSurvivors => "Vampire Survivors",
        };
        write!(f, "{}", name)
    }
}
