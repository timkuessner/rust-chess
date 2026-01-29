mod core;
mod ui;

use core::position::Position;
use ui::display::Display;

fn main() {
    let mut position = Position::new();
    position.print();

    for _ in 0..10 {
        position = position.apply_move(position.legal_moves()[0]);

        position.print();
    }
}