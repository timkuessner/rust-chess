mod core;
mod ui;

use rand::seq::{IndexedMutRandom};

use core::position::Position;
use ui::display::Display;

fn main() {
    let mut position = Position::new();
    position.print();

    for _ in 0..34 {
        let mut rng = rand::rng();
        let mut legal_moves = position.legal_moves();

        // choose_mut for a mutable reference
        if let Some(mv) = legal_moves.choose_mut(&mut rng) {
            position = position.apply_move(*mv);
            position.print();
        }
    }
}