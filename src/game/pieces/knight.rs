use colored::Colorize;

use crate::game::state;

struct Knight {
    color: state::PlayerSide,
}

impl Knight {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " N ".green()),
            state::PlayerSide::Black => print!("{} ", "N ".red()),
        }
    }
}
