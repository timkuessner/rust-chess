use crate::{board::Board, game::Player, moves::Move};

pub struct Bot {
    name: String,
}

impl Player for Bot {
    fn name(&self) -> &str {
        &self.name
    }

    fn is_human(&self) -> bool {
        false
    }

    fn choose_move(&mut self, _board: &Board, _legal_moves: &[Move]) -> Option<Move> {
        Some(_legal_moves[0])
    }
}

impl Bot {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}