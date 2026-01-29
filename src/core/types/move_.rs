use super::piece::{ Piece, PieceType };

#[derive(Debug)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub piece: Piece,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub const fn new(from: u8, to: u8, piece: Piece) -> Self {
        Move { from, to, piece, promotion: None }
    }

    pub const fn promotion_move(from: u8, to: u8, piece: Piece, promotion: PieceType) -> Self {
        Move { from, to, piece, promotion: Some(promotion) }
    }
}