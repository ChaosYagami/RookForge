use colored::Colorize;

use crate::game::state;

struct Pawn {
    color: state::PlayerSide,
}

impl Pawn {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " P ".green()),
            state::PlayerSide::Black => print!("{} ", "P ".red()),
        }
    }
}
