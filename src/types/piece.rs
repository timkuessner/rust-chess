use super::color::Color;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece_type: PieceType, color: Color) -> Self {
        Piece { piece_type, color }
    }

    pub fn to_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::King, Color::White) => 'K',
            (PieceType::Pawn, Color::Black) => 'p',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::King, Color::Black) => 'k',
        }
    }
}