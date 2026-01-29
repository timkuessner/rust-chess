use super::piece::PieceType;

pub struct Move {
    pub from: u8,
    pub to: u8,
    pub promotion: Option<PieceType>,
}

impl Move {
    pub const fn new(from: u8, to: u8) -> Self {
        Move { from, to, promotion: None }
    }

    pub const fn promotion_move(from: u8, to: u8, promotion: PieceType) -> Self {
        Move { from, to, promotion: Some(promotion) }
    }
}