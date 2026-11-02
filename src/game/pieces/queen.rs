use colored::Colorize;

use crate::game::state;

struct Queen {
    color: state::PlayerSide,
}

impl Queen {
    fn new(color: state::PlayerSide) -> Self {
        Self { color: color }
    }

    fn display(&self) {
        match self.color {
            state::PlayerSide::White => print!("{}", " Q ".green()),
            state::PlayerSide::Black => print!("{} ", "Q ".red()),
        }
    }
}
