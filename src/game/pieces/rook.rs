use colored::Colorize;

use crate::game::state;

struct Rook {
    color: state::PlayerSide,
}

impl Rook {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " R ".green()),
            state::PlayerSide::Black => print!("{} ", "R ".red()),
        }
    }
}
