use crate::core::position::Position;
use crate::core::types::piece::Piece;

pub trait Display {
    fn print(&self);
}

impl Display for Position {
    fn print(&self) {
        println!("\n    a b c d e f g h\n");
        
        for rank in (0..8).rev() {
            
            print!("{}   ", rank + 1);

            for file in 0..8 {
                let square = rank * 8 + file;
                let piece_char = match self.get_piece_at(square) {
                    Some(Piece::WhitePawn) => 'P',
                    Some(Piece::WhiteKnight) => 'N',
                    Some(Piece::WhiteBishop) => 'B',
                    Some(Piece::WhiteRook) => 'R',
                    Some(Piece::WhiteQueen) => 'Q',
                    Some(Piece::WhiteKing) => 'K',
                    Some(Piece::BlackPawn) => 'p',
                    Some(Piece::BlackKnight) => 'n',
                    Some(Piece::BlackBishop) => 'b',
                    Some(Piece::BlackRook) => 'r',
                    Some(Piece::BlackQueen) => 'q',
                    Some(Piece::BlackKing) => 'k',
                    None => '.',
                };
                print!("{} ", piece_char);
            }
            
            println!("  {}", rank + 1);
        }

        println!("\n    a b c d e f g h\n");
    }
}
