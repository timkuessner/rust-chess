use crate::core::types::bitboard::{Bitboard};
use crate::core::types::piece::PieceType;
use super::types::piece::Piece;
use super::types::square::{RANK_1_AND_8, FILE_A, FILE_H};
use super::position::Position;
use super::types::color::Color;
use super::types::move_::Move;

pub struct MoveGenerator;

static KNIGHT_MOVES: [Bitboard; 64] = {
    let mut table = [Bitboard::empty(); 64];
    let offsets = [(2, 1), (2, -1), (-2, 1), (-2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2)];
    
    let mut sq: usize = 0;
    while sq < 64 {
        let file = (sq % 8) as i8;
        let rank = (sq / 8) as i8;
        let mut attacks = 0u64;
        
        let mut i = 0;
        while i < 8 {
            let (df, dr) = offsets[i];
            let nf = file + df;
            let nr = rank + dr;
            if nf >= 0 && nf <= 7 && nr >= 0 && nr <= 7 {
                attacks |= 1u64 << (nr * 8 + nf);
            }
            i += 1;
        }
        
        table[sq] = Bitboard(attacks);
        sq += 1;
    }
    table
};

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

        let piece = match side {
            Color::White => Piece::WhitePawn,
            Color::Black => Piece::BlackPawn,
        };

        let occupied = position.all_pieces();
        let opponent = position.pieces_of_color(side.opposite());

        let push_mask = pawns << shift & !occupied;

        let promotions_push_mask = push_mask & RANK_1_AND_8;
        let single_push_mask = push_mask & !RANK_1_AND_8;

        let double_shift = shift * 2;
        let double_push_mask = pawns << double_shift & !(occupied | (occupied << shift));

        let (left_shift, right_shift) = match side {
            Color::White => (7i8, 9i8),
            Color::Black => (-9i8, -7i8),
        };
        
        let left_cap_targets = (pawns & !FILE_A) << left_shift;
        
        let right_cap_targets = (pawns & !FILE_H) << right_shift;
        
        let en_passant_bitboard = match position.en_passant_square {
            Some(sq) => Bitboard(1u64 << sq),
            None => Bitboard::empty(),
        };
        
        let capture_targets = opponent | en_passant_bitboard;
        let left_captures = left_cap_targets & capture_targets;
        let right_captures = right_cap_targets & capture_targets;
        
        let left_promo_captures = left_captures & RANK_1_AND_8;
        let left_normal_captures = left_captures & !RANK_1_AND_8;
        let right_promo_captures = right_captures & RANK_1_AND_8;
        let right_normal_captures = right_captures & !RANK_1_AND_8;

        for to in single_push_mask.squares() {
            let from = (to as i8 - shift) as u8;
            moves.push(Move::new(from, to, piece));
        }

        for to in promotions_push_mask.squares() {
            let from = (to as i8 - shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, piece, promotion));
            }
        }

        for to in double_push_mask.squares() {
            let from = (to as i8 - double_shift) as u8;
            moves.push(Move::new(from, to, piece));
        }
        
        for to in left_normal_captures.squares() {
            let from = (to as i8 - left_shift) as u8;
            moves.push(Move::new(from, to, piece));
        }
        
        for to in left_promo_captures.squares() {
            let from = (to as i8 - left_shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, piece, promotion));
            }
        }
        
        for to in right_normal_captures.squares() {
            let from = (to as i8 - right_shift) as u8;
            moves.push(Move::new(from, to, piece));
        }
        
        for to in right_promo_captures.squares() {
            let from = (to as i8 - right_shift) as u8;
            for promotion in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                moves.push(Move::promotion_move(from, to, piece, promotion));
            }
        }
    }

    fn generate_knight_moves(position: &Position, side: &Color, moves: &mut Vec<Move>) {
        let knights = match side {
            Color::White => position.pieces[Piece::WhiteKnight],
            Color::Black => position.pieces[Piece::BlackKnight],
        };

        let own = position.pieces_of_color(side.clone());

        for from in knights.squares() {
            for to in (KNIGHT_MOVES[from as usize] & !own).squares() {
                moves.push(Move::new(from, to, match side {
                    Color::White => Piece::WhiteKnight,
                    Color::Black => Piece::BlackKnight,
                }));
            }
        }
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
}