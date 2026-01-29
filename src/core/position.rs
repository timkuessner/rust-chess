use crate::core::types::piece::PieceType;
use crate::core::types::square::{A1, A8, H1, H8};

use super::types::move_::Move;
use super::types::bitboard::Bitboard;
use super::types::color::Color;
use super::types::piece::Piece;
use super::types::castling::CastlingRights;
use super::move_generator::MoveGenerator;

#[derive(Debug, Clone)]
pub struct Position {
    pub pieces: [Bitboard; Piece::COUNT],
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<u8>,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,

}

impl Position {
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

        Position { pieces, side_to_move: Color::White, castling_rights: CastlingRights::all(), en_passant_square: None, halfmove_clock: 0, fullmove_number: 1 }
    }

    pub fn get_piece_at(&self, square: u8) -> Option<Piece> {
        for &piece in &Piece::all() {
            if self.pieces[piece].get_bit(square) {
                return Some(piece);
            }
        }
        None
    }

    pub fn all_pieces(&self) -> Bitboard {
        let mut bitboard = Bitboard::empty();
        for piece in Piece::all() {
            bitboard = Bitboard(bitboard.0 | self.pieces[piece].0);
        }
        bitboard
    }

    pub fn pieces_of_color(&self, color: Color) -> Bitboard {
        let mut bitboard = Bitboard::empty();
        for piece in Piece::all() {
            if piece.color() == color {
                bitboard = Bitboard(bitboard.0 | self.pieces[piece].0);
            }
        }
        bitboard
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        MoveGenerator::legal_moves(self)
    }

    pub fn apply_move(&self, move_: Move) -> Position {
        let mut new_position = self.clone();

        let piece = move_.piece;

        // Clear from
        new_position.pieces[piece].clear_bit(move_.from);

        // Set to, handle promotions
        if let Some(promotion) = move_.promotion {
            new_position.pieces[Piece::new(piece.color(), promotion)].set_bit(move_.to);
        } else {
            new_position.pieces[piece].set_bit(move_.to);
        }

        // Captures
        if let Some(captured_piece) = self.get_piece_at(move_.to) {
            new_position.pieces[captured_piece].clear_bit(move_.to);
        }

        // En passant captures
        if piece.piece_type() == PieceType::Pawn && self.en_passant_square == Some(move_.to) {
            let captured_square = match piece.color() {
                Color::White => move_.to - 8,
                Color::Black => move_.to + 8,
            };

            let captured_piece = Piece::new(
                piece.color().opposite(),
                PieceType::Pawn,
            );

            new_position.pieces[captured_piece].clear_bit(captured_square);
        }

        // Castling rook move
        if piece.piece_type() == PieceType::King {
            let difference = (move_.to as i8 - move_.from as i8).abs();
            if difference == 2 {
                let (rook_from, rook_to, rook) = match (piece.color(), move_.to) {
                    (Color::White, 6) => (7, 5, Piece::WhiteRook),
                    (Color::White, 2) => (0, 3, Piece::WhiteRook),
                    (Color::Black, 62) => (63, 61, Piece::BlackRook),
                    (Color::Black, 58) => (56, 59, Piece::BlackRook),
                    _ => unreachable!(),
                };
                new_position.pieces[rook].clear_bit(rook_from);
                new_position.pieces[rook].set_bit(rook_to);
            }
        }

        // Castling rights
        if piece.piece_type() == PieceType::King {
            new_position.castling_rights.remove_both(piece.color());
        }

        if piece.piece_type() == PieceType::Rook {
            match move_.from {
                A1 => new_position.castling_rights.remove_queenside(Color::White),
                H1 => new_position.castling_rights.remove_kingside(Color::White),
                A8 => new_position.castling_rights.remove_queenside(Color::Black),
                H8 => new_position.castling_rights.remove_kingside(Color::Black),
                _ => {},
            }
        }

        if let Some(piece) = self.get_piece_at(move_.to) {
            if piece.piece_type() == PieceType::Rook {
                match move_.to {
                    A1 => new_position.castling_rights.remove_queenside(Color::White),
                    H1 => new_position.castling_rights.remove_kingside(Color::White),
                    A8 => new_position.castling_rights.remove_queenside(Color::Black),
                    H8 => new_position.castling_rights.remove_kingside(Color::Black),
                    _ => {},
                }
            }
        }

        // En passant square
        if piece.piece_type() == PieceType::Pawn {
            let difference = (move_.to as i8 - move_.from as i8).abs();
            if difference == 16 {
                new_position.en_passant_square = Some((move_.from + move_.to) / 2);
            }
        }

        // Clock
        if piece.piece_type() == PieceType::Pawn || self.get_piece_at(move_.to).is_some() {
            new_position.halfmove_clock = 0;
        } else {
            new_position.halfmove_clock += 1;
        }

        if piece.color() == Color::Black {
            new_position.fullmove_number += 1;
        }

        // Side
        new_position.side_to_move = self.side_to_move.opposite();

        new_position
    }
}