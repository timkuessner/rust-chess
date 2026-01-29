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
}