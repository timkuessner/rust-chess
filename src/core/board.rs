use super::types::bitboard::Bitboard;
use super::types::color::Color;
use super::types::piece::Piece;

pub struct Board {
    pub pieces: [Bitboard; Piece::COUNT],
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        let mut pieces = [Bitboard::empty(); Piece::COUNT];

        pieces[Piece::WhitePawn] = Bitboard::new(0x000000000000FF00);
        pieces[Piece::WhiteKnight] = Bitboard::new(0x0000000000000042);
        pieces[Piece::WhiteBishop] = Bitboard::new(0x0000000000000024);
        pieces[Piece::WhiteRook] = Bitboard::new(0x0000000000000081);
        pieces[Piece::WhiteQueen] = Bitboard::new(0x0000000000000008);
        pieces[Piece::WhiteKing] = Bitboard::new(0x0000000000000010);

        pieces[Piece::BlackPawn] = Bitboard::new(0x00FF000000000000);
        pieces[Piece::BlackKnight] = Bitboard::new(0x4200000000000000);
        pieces[Piece::BlackBishop] = Bitboard::new(0x2400000000000000);
        pieces[Piece::BlackRook] = Bitboard::new(0x8100000000000000);
        pieces[Piece::BlackQueen] = Bitboard::new(0x0800000000000000);
        pieces[Piece::BlackKing] = Bitboard::new(0x1000000000000000);

        Board { pieces, side_to_move: Color::White }
    }

    pub fn get_piece_at(&self, square: u8) -> Option<Piece> {
        for &piece in &Piece::all() {
            if self.pieces[piece].get_bit(square) {
                return Some(piece);
            }
        }
        None
    }
}