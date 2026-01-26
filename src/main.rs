mod core;
mod ui;

use core::board::Board;
use ui::display::Display;

fn main() {
    let board = Board::new();
    board.print();
}