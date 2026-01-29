mod core;
mod ui;

use core::position::Position;
use ui::display::Display;

fn main() {
    let position = Position::new();
    position.print();
    
    println!("  Side to move: {:?}", position.side_to_move);
    println!("  En passant: {:?}", position.en_passant_square);
    println!("  Halfmove clock: {}", position.halfmove_clock);
    println!("  Fullmove number: {}", position.fullmove_number);

    position.legal_moves();
}