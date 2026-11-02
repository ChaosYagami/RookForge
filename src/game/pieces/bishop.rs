use colored::Colorize;

use crate::game::state;

struct Bishop {
    color: state::PlayerSide,
}

impl Bishop {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " B ".green()),
            state::PlayerSide::Black => print!("{} ", "B ".red()),
        }
    }
}
