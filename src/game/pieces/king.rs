use colored::Colorize;

use crate::game::state;

struct King {
    color: state::PlayerSide,
}

impl King {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " K ".green()),
            state::PlayerSide::Black => print!("{} ", "K ".red()),
        }
    }
}
