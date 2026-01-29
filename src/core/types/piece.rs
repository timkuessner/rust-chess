use super::color::Color;
use super::bitboard::Bitboard;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    WhitePawn = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook = 3,
    WhiteQueen = 4,
    WhiteKing = 5,
    BlackPawn = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook = 9,
    BlackQueen = 10,
    BlackKing = 11,
}

impl Piece {
    pub const COUNT: usize = 12;

    pub fn new(color: Color, piece_type: PieceType) -> Self {
        match (color, piece_type) {
            (Color::White, PieceType::Pawn) => Piece::WhitePawn,
            (Color::White, PieceType::Knight) => Piece::WhiteKnight,
            (Color::White, PieceType::Bishop) => Piece::WhiteBishop,
            (Color::White, PieceType::Rook) => Piece::WhiteRook,
            (Color::White, PieceType::Queen) => Piece::WhiteQueen,
            (Color::White, PieceType::King) => Piece::WhiteKing,
            (Color::Black, PieceType::Pawn) => Piece::BlackPawn,
            (Color::Black, PieceType::Knight) => Piece::BlackKnight,
            (Color::Black, PieceType::Bishop) => Piece::BlackBishop,
            (Color::Black, PieceType::Rook) => Piece::BlackRook,
            (Color::Black, PieceType::Queen) => Piece::BlackQueen,
            (Color::Black, PieceType::King) => Piece::BlackKing,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Piece::WhitePawn | Piece::WhiteKnight | Piece::WhiteBishop 
            | Piece::WhiteRook | Piece::WhiteQueen | Piece::WhiteKing => Color::White,
            Piece::BlackPawn | Piece::BlackKnight | Piece::BlackBishop 
            | Piece::BlackRook | Piece::BlackQueen | Piece::BlackKing => Color::Black,
        }
    }

    pub fn piece_type(&self) -> PieceType {
        match self {
            Piece::WhitePawn | Piece::BlackPawn => PieceType::Pawn,
            Piece::WhiteKnight | Piece::BlackKnight => PieceType::Knight,
            Piece::WhiteBishop | Piece::BlackBishop => PieceType::Bishop,
            Piece::WhiteRook | Piece::BlackRook => PieceType::Rook,
            Piece::WhiteQueen | Piece::BlackQueen => PieceType::Queen,
            Piece::WhiteKing | Piece::BlackKing => PieceType::King,
        }
    }

    pub const fn all() -> [Piece; 12] {
        [
            Piece::WhitePawn, Piece::WhiteKnight, Piece::WhiteBishop,
            Piece::WhiteRook, Piece::WhiteQueen, Piece::WhiteKing,
            Piece::BlackPawn, Piece::BlackKnight, Piece::BlackBishop,
            Piece::BlackRook, Piece::BlackQueen, Piece::BlackKing,
        ]
    }
}

impl Index<Piece> for [Bitboard; Piece::COUNT] {
    type Output = Bitboard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self[piece as usize]
    }
}

impl IndexMut<Piece> for [Bitboard; Piece::COUNT] {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self[piece as usize]
    }
}
