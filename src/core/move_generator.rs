use crate::core::types::bitboard::{Bitboard};
use crate::core::types::piece::PieceType;
use super::types::piece::Piece;
use super::types::square::{RANK_1_AND_8, FILE_A, FILE_H};
use super::position::Position;
use super::types::color::Color;
use super::types::move_::Move;

pub struct MoveGenerator;

impl MoveGenerator {
    pub fn legal_moves(position: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(40);
        let side = &position.side_to_move;

        Self::generate_pawn_moves(position, side, &mut moves);
        Self::generate_knight_moves(position, side, &mut moves);
        Self::generate_bishop_moves(position, side, &mut moves);
        Self::generate_rook_moves(position, side, &mut moves);
        Self::generate_queen_moves(position, side, &mut moves);
        Self::generate_king_moves(position, side, &mut moves);

        moves.into_iter().filter(|move_| !Self::is_move_illegal(position, move_)).collect()
    }

    fn generate_pawn_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        let shift: i8 = match side {
            Color::White => 8,
            Color::Black => -8,
        };

        let pawns = match side {
            Color::White => position.pieces[Piece::WhitePawn],
            Color::Black => position.pieces[Piece::BlackPawn],
        };

        let occupied = position.all_pieces();
        let opponent = position.pieces_of_color(side.opposite());

        let push_mask = Bitboard(pawns.0 << shift) & !occupied;

        let promotions_push_mask = Bitboard(push_mask.0 & RANK_1_AND_8.0);
        let single_push_mask = Bitboard(push_mask.0 & !RANK_1_AND_8.0);

        let double_shift = shift * 2;
        let double_push_mask = Bitboard(pawns.0 << double_shift) & !Bitboard(occupied.0 | (occupied.0 << shift));

        let (left_shift, right_shift) = match side {
            Color::White => (7i8, 9i8),
            Color::Black => (-9i8, -7i8),
        };
        
        let left_cap_targets = Bitboard((pawns.0 & !FILE_A.0) << left_shift);
        
        let right_cap_targets = Bitboard((pawns.0 & !FILE_H.0) << right_shift);
        
        let en_passant_bitboard = match position.en_passant_square {
            Some(sq) => Bitboard(1u64 << sq),
            None => Bitboard::empty(),
        };
        
        let capture_targets = opponent | en_passant_bitboard;
        let left_captures = left_cap_targets & capture_targets;
        let right_captures = right_cap_targets & capture_targets;
        
        let left_promo_captures = Bitboard(left_captures.0 & RANK_1_AND_8.0);
        let left_normal_captures = Bitboard(left_captures.0 & !RANK_1_AND_8.0);
        let right_promo_captures = Bitboard(right_captures.0 & RANK_1_AND_8.0);
        let right_normal_captures = Bitboard(right_captures.0 & !RANK_1_AND_8.0);

        for to in single_push_mask.squares() {
            let from = (to as i8 - shift) as u8;
            moves.push(Move::new(from, to));
        }

        for to in promotions_push_mask.squares() {
            let from = (to as i8 - shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, promotion));
            }
        }

        for to in double_push_mask.squares() {
            let from = (to as i8 - double_shift) as u8;
            moves.push(Move::new(from, to));
        }
        
        for to in left_normal_captures.squares() {
            let from = (to as i8 - left_shift) as u8;
            moves.push(Move::new(from, to));
        }
        
        for to in left_promo_captures.squares() {
            let from = (to as i8 - left_shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, promotion));
            }
        }
        
        for to in right_normal_captures.squares() {
            let from = (to as i8 - right_shift) as u8;
            moves.push(Move::new(from, to));
        }
        
        for to in right_promo_captures.squares() {
            let from = (to as i8 - right_shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, promotion));
            }
        }
    }

    fn generate_knight_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        
    }

    fn generate_bishop_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        
    }

    fn generate_rook_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        
    }

    fn generate_queen_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        
    }

    fn generate_king_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        
    }

    fn is_move_illegal(position: &Position, move_: &Move) -> bool {
        false
    }

    fn serialize_pawn_moves(bitboard: Bitboard, offset: i8, promotion: bool, moves: &mut Vec<Move>) {

    }
}