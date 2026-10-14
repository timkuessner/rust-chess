use crate::moves::{Move, MoveFlag};
use crate::piece::piece;
use crate::utils::bitboard::bit;
use crate::utils::zobrist::{self, ZOBRIST};

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const WHITE_KINGSIDE: u8 = 0b0001;
pub const WHITE_QUEENSIDE: u8 = 0b0010;
pub const BLACK_KINGSIDE: u8 = 0b0100;
pub const BLACK_QUEENSIDE: u8 = 0b1000;

const ALL_CASTLING_RIGHTS: u8 = 0b1111;

const KINGSIDE_RIGHTS: [u8; 2] = [WHITE_KINGSIDE, BLACK_KINGSIDE];
const QUEENSIDE_RIGHTS: [u8; 2] = [WHITE_QUEENSIDE, BLACK_QUEENSIDE];

const CASTLING_RIGHTS_KEPT: [u8; 64] = {
    let mut kept = [ALL_CASTLING_RIGHTS; 64];

    kept[0] = ALL_CASTLING_RIGHTS & !WHITE_QUEENSIDE;
    kept[4] = ALL_CASTLING_RIGHTS & !(WHITE_KINGSIDE | WHITE_QUEENSIDE);
    kept[7] = ALL_CASTLING_RIGHTS & !WHITE_KINGSIDE;
    kept[56] = ALL_CASTLING_RIGHTS & !BLACK_QUEENSIDE;
    kept[60] = ALL_CASTLING_RIGHTS & !(BLACK_KINGSIDE | BLACK_QUEENSIDE);
    kept[63] = ALL_CASTLING_RIGHTS & !BLACK_KINGSIDE;

    kept
};

#[derive(Clone, Copy)]
struct UndoInfo {
    mv: Move,
    captured_piece: u8,
    castling_rights: u8,
    en_passant_file: Option<usize>,
    halfmove_clock: u32,
    zobrist_key: u64,
}

#[derive(Clone)]
pub struct Board {
    pub squares: [u8; 64],
    pub piece_bitboards: [u64; 12],
    pub color_bitboards: [u64; 2],
    pub color_to_move: u8,
    pub en_passant_file: Option<usize>,
    pub castling_rights: u8,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
    pub zobrist_key: u64,

    pub repetition_history: Vec<u64>,
    undo_stack: Vec<UndoInfo>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [piece::NONE; 64],
            piece_bitboards: Default::default(),
            color_bitboards: Default::default(),
            color_to_move: piece::WHITE,
            en_passant_file: None,
            castling_rights: 0,
            halfmove_clock: 0,
            fullmove_number: 1,
            zobrist_key: 0,
            repetition_history: Vec::new(),
            undo_stack: Vec::new(),
        }
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Self::default();
        board.load_position_from_fen(START_FEN);
        board
    }

    #[inline(always)]
    pub fn color_not_to_move(&self) -> u8 {
        piece::opposite(self.color_to_move)
    }

    #[inline(always)]
    pub fn pieces(&self, kind: u8, color: u8) -> u64 {
        self.piece_bitboards[piece::get_piece_index(kind, color)]
    }

    #[inline(always)]
    pub fn all_pieces(&self) -> u64 {
        self.color_bitboards[0] | self.color_bitboards[1]
    }

    #[inline(always)]
    pub fn orthogonal_sliders(&self, color: u8) -> u64 {
        self.pieces(piece::ROOK, color) | self.pieces(piece::QUEEN, color)
    }

    #[inline(always)]
    pub fn diagonal_sliders(&self, color: u8) -> u64 {
        self.pieces(piece::BISHOP, color) | self.pieces(piece::QUEEN, color)
    }

    #[inline(always)]
    pub fn king_square(&self, color: u8) -> usize {
        self.pieces(piece::KING, color).trailing_zeros() as usize
    }

    #[inline(always)]
    pub fn has_kingside_castle_right(&self, color_index: usize) -> bool {
        self.castling_rights & KINGSIDE_RIGHTS[color_index] != 0
    }

    #[inline(always)]
    pub fn has_queenside_castle_right(&self, color_index: usize) -> bool {
        self.castling_rights & QUEENSIDE_RIGHTS[color_index] != 0
    }

    pub fn load_position_from_fen(&mut self, fen: &str) {
        *self = Self::default();

        let mut sections = fen.split_whitespace();

        let mut file: usize = 0;
        let mut rank: usize = 7;

        for c in sections.next().unwrap_or("").chars() {
            match c {
                '/' => {
                    file = 0;
                    rank = rank.saturating_sub(1);
                }
                '1'..='8' => file += c as usize - '0' as usize,
                _ => {
                    if let Some(p) = piece::from_fen_char(c) {
                        self.add_piece(p, rank * 8 + file);
                        file += 1;
                    }
                }
            }
        }

        self.color_to_move = match sections.next() {
            Some("b") => piece::BLACK,
            _ => piece::WHITE,
        };

        self.castling_rights = sections.next().unwrap_or("-").chars().fold(0, |rights, c| {
            rights
                | match c {
                    'K' => WHITE_KINGSIDE,
                    'Q' => WHITE_QUEENSIDE,
                    'k' => BLACK_KINGSIDE,
                    'q' => BLACK_QUEENSIDE,
                    _ => 0,
                }
        });

        self.en_passant_file = sections
            .next()
            .and_then(|square| square.bytes().next())
            .and_then(|file_byte| file_byte.checked_sub(b'a'))
            .map(usize::from)
            .filter(|&file| file < 8);

        self.halfmove_clock = sections.next().and_then(|s| s.parse().ok()).unwrap_or(0);

        self.fullmove_number = sections.next().and_then(|s| s.parse().ok()).unwrap_or(1);

        self.zobrist_key = zobrist::compute_key(self);
    }

    pub fn make_move(&mut self, mv: Move) {
        let Move { from, to, flag } = mv;

        let own_color = self.color_to_move;
        let enemy_color = self.color_not_to_move();
        let is_white = own_color == piece::WHITE;
        let is_en_passant = flag == MoveFlag::EnPassantCapture;

        let moved_piece = self.squares[from];
        let captured_piece = if is_en_passant {
            piece::PAWN | enemy_color
        } else {
            self.squares[to]
        };

        self.undo_stack.push(UndoInfo {
            mv,
            captured_piece,
            castling_rights: self.castling_rights,
            en_passant_file: self.en_passant_file,
            halfmove_clock: self.halfmove_clock,
            zobrist_key: self.zobrist_key,
        });

        if piece::kind(moved_piece) == piece::PAWN || captured_piece != piece::NONE {
            self.halfmove_clock = 0;
            self.repetition_history.clear();
        } else {
            self.halfmove_clock += 1;
            self.repetition_history.push(self.zobrist_key);
        }

        if captured_piece != piece::NONE {
            let capture_square = match (is_en_passant, is_white) {
                (false, _) => to,
                (true, true) => to - 8,
                (true, false) => to + 8,
            };

            self.remove_piece(captured_piece, capture_square);
        }

        self.move_piece(moved_piece, from, to);

        if flag == MoveFlag::Castling {
            let (rook_from, rook_to) = if to > from {
                (to + 1, to - 1)
            } else {
                (to - 2, to + 1)
            };

            self.move_piece(piece::ROOK | own_color, rook_from, rook_to);
        }

        if let Some(promotion_kind) = promotion_kind(flag) {
            self.remove_piece(moved_piece, to);
            self.add_piece(promotion_kind | own_color, to);
        }

        let new_castling_rights =
            self.castling_rights & CASTLING_RIGHTS_KEPT[from] & CASTLING_RIGHTS_KEPT[to];
        self.zobrist_key ^= ZOBRIST.castling[self.castling_rights as usize]
            ^ ZOBRIST.castling[new_castling_rights as usize];
        self.castling_rights = new_castling_rights;

        if let Some(file) = self.en_passant_file {
            self.zobrist_key ^= ZOBRIST.en_passant_file[file];
        }
        self.en_passant_file = (flag == MoveFlag::PawnTwoUp).then_some(from % 8);
        if let Some(file) = self.en_passant_file {
            self.zobrist_key ^= ZOBRIST.en_passant_file[file];
        }

        if !is_white {
            self.fullmove_number += 1;
        }

        self.zobrist_key ^= ZOBRIST.side_to_move;
        self.color_to_move = enemy_color;
    }

    pub fn undo_move(&mut self) {
        let Some(undo) = self.undo_stack.pop() else {
            return;
        };
        let Move { from, to, flag } = undo.mv;

        let own_color = self.color_not_to_move();
        let is_white = own_color == piece::WHITE;

        self.color_to_move = own_color;
        if !is_white {
            self.fullmove_number -= 1;
        }

        if promotion_kind(flag).is_some() {
            let promoted = self.squares[to];
            self.remove_piece(promoted, to);
            self.add_piece(piece::PAWN | own_color, to);
        }

        let moved_piece = self.squares[to];
        self.move_piece(moved_piece, to, from);

        if flag == MoveFlag::Castling {
            let (rook_from, rook_to) = if to > from {
                (to + 1, to - 1)
            } else {
                (to - 2, to + 1)
            };
            self.move_piece(piece::ROOK | own_color, rook_to, rook_from);
        }

        if undo.captured_piece != piece::NONE {
            let capture_square = match (flag == MoveFlag::EnPassantCapture, is_white) {
                (false, _) => to,
                (true, true) => to - 8,
                (true, false) => to + 8,
            };
            self.add_piece(undo.captured_piece, capture_square);
        }

        self.castling_rights = undo.castling_rights;
        self.en_passant_file = undo.en_passant_file;
        self.halfmove_clock = undo.halfmove_clock;
        self.zobrist_key = undo.zobrist_key;
        self.repetition_history.pop();
    }

    #[inline(always)]
    fn add_piece(&mut self, p: u8, square: usize) {
        let index = piece::index(p);

        self.squares[square] = p;
        self.piece_bitboards[index] |= bit(square);
        self.color_bitboards[piece::color_index(p)] |= bit(square);
        self.zobrist_key ^= ZOBRIST.pieces[index][square];
    }

    #[inline(always)]
    fn remove_piece(&mut self, p: u8, square: usize) {
        let index = piece::index(p);

        self.squares[square] = piece::NONE;
        self.piece_bitboards[index] &= !bit(square);
        self.color_bitboards[piece::color_index(p)] &= !bit(square);
        self.zobrist_key ^= ZOBRIST.pieces[index][square];
    }

    #[inline(always)]
    fn move_piece(&mut self, p: u8, from: usize, to: usize) {
        let index = piece::index(p);
        let from_to = bit(from) | bit(to);

        self.squares[from] = piece::NONE;
        self.squares[to] = p;
        self.piece_bitboards[index] ^= from_to;
        self.color_bitboards[piece::color_index(p)] ^= from_to;
        self.zobrist_key ^= ZOBRIST.pieces[index][from] ^ ZOBRIST.pieces[index][to];
    }

    pub fn has_insufficient_material(&self) -> bool {
        const DARK_SQUARES: u64 = 0xAA55_AA55_AA55_AA55;

        let both = |kind: u8| self.pieces(kind, piece::WHITE) | self.pieces(kind, piece::BLACK);

        if both(piece::PAWN) | both(piece::ROOK) | both(piece::QUEEN) != 0 {
            return false;
        }

        let knights = both(piece::KNIGHT).count_ones();
        let bishop_bitboard = both(piece::BISHOP);
        let bishops = bishop_bitboard.count_ones();

        if knights + bishops <= 1 {
            return true;
        }

        knights == 0
            && (bishop_bitboard & DARK_SQUARES == 0 || bishop_bitboard & !DARK_SQUARES == 0)
    }

    pub fn is_three_fold_repetition(&self) -> bool {
        let same_side_positions = self.repetition_history.len() / 2;

        self.repetition_history
            .iter()
            .rev()
            .skip(1)
            .step_by(2)
            .take(same_side_positions.min(self.halfmove_clock as usize / 2))
            .filter(|&&key| key == self.zobrist_key)
            .nth(1)
            .is_some()
    }
}

fn promotion_kind(flag: MoveFlag) -> Option<u8> {
    match flag {
        MoveFlag::PromoteToQueen => Some(piece::QUEEN),
        MoveFlag::PromoteToRook => Some(piece::ROOK),
        MoveFlag::PromoteToKnight => Some(piece::KNIGHT),
        MoveFlag::PromoteToBishop => Some(piece::BISHOP),
        _ => None,
    }
}
