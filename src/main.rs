mod core;
mod ui;

use core::position::Position;
use ui::display::Display;

fn main() {
    let board = Position::new();
    board.print();
}