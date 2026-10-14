use crate::board::Board;
use crate::moves::{Move, MoveFlag};
use crate::piece::piece;
use crate::utils::bitboard::{self, bit, contains_square, squares};
use crate::utils::magic::{bishop_attacks, rook_attacks};
use crate::utils::tables::{BETWEEN, CASTLING, KING_MOVES, KNIGHT_MOVES, LINE, PAWN_ATTACKS};

const MAX_MOVES: usize = 218;

pub enum MoveGeneration {
    Moves(Vec<Move>),
    Checkmate { winner: bool },
    Draw(DrawReason),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawReason {
    ThreefoldRepetition,
    FiftyMoveRule,
    InsufficientMaterial,
    Stalemate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionMode {
    All,
    QueenOnly,
    QueenAndKnight,
}

static PROMOTIONS: [MoveFlag; 4] = [
    MoveFlag::PromoteToQueen,
    MoveFlag::PromoteToKnight,
    MoveFlag::PromoteToRook,
    MoveFlag::PromoteToBishop,
];

struct PawnRules {
    push: i32,
    capture_left: i32,
    capture_right: i32,

    capture_left_files: u64,
    capture_right_files: u64,
    promotion_rank: u64,
    double_push_rank: u64,
    en_passant_rank: usize,
}

const PAWN_RULES: [PawnRules; 2] = [
    PawnRules {
        push: 8,
        capture_left: 7,
        capture_right: 9,
        capture_left_files: bitboard::NOT_A_FILE,
        capture_right_files: bitboard::NOT_H_FILE,
        promotion_rank: bitboard::RANK_8,
        double_push_rank: bitboard::RANK_4,
        en_passant_rank: 5,
    },
    PawnRules {
        push: -8,
        capture_left: -7,
        capture_right: -9,
        capture_left_files: bitboard::NOT_H_FILE,
        capture_right_files: bitboard::NOT_A_FILE,
        promotion_rank: bitboard::RANK_1,
        double_push_rank: bitboard::RANK_5,
        en_passant_rank: 2,
    },
];

pub struct MoveGenerator<'a> {
    board: &'a Board,
    own_color: u8,
    enemy_color: u8,
    own_index: usize,
    enemy_index: usize,
    own_king: usize,
    own_bitboard: u64,
    enemy_bitboard: u64,
    all_bitboard: u64,
    enemy_attack_bitboard: u64,

    move_type_bitmask: u64,
    generate_quiet_moves: bool,
    promotions_to_generate: PromotionMode,
    in_check: bool,
    in_double_check: bool,

    check_ray_bitmask: u64,
    pin_rays_bitmask: u64,
    moves: Vec<Move>,
}

impl<'a> MoveGenerator<'a> {
    pub fn generate_moves(board: &Board) -> MoveGeneration {
        if board.has_insufficient_material() {
            return MoveGeneration::Draw(DrawReason::InsufficientMaterial);
        }

        if board.is_three_fold_repetition() {
            return MoveGeneration::Draw(DrawReason::ThreefoldRepetition);
        }

        let (moves, in_check) = MoveGenerator::generate(board, false);

        if moves.is_empty() {
            if in_check {
                return MoveGeneration::Checkmate {
                    winner: board.color_to_move != piece::WHITE,
                };
            }
            return MoveGeneration::Draw(DrawReason::Stalemate);
        }

        if board.halfmove_clock >= 100 {
            return MoveGeneration::Draw(DrawReason::FiftyMoveRule);
        }

        MoveGeneration::Moves(moves)
    }

    fn generate(board: &'a Board, captures_only: bool) -> (Vec<Move>, bool) {
        let mut move_generator = MoveGenerator::new(board, captures_only);

        move_generator.calculate_attack_data();

        move_generator.generate_king_moves();

        if !move_generator.in_double_check {
            move_generator.generate_sliding_moves();
            move_generator.generate_knight_moves();
            move_generator.generate_pawn_moves();
        }

        (move_generator.moves, move_generator.in_check)
    }

    fn new(board: &'a Board, captures_only: bool) -> Self {
        let own_color = board.color_to_move;
        let enemy_color = board.color_not_to_move();
        let own_index = piece::color_index(own_color);
        let enemy_index = piece::color_index(enemy_color);
        let enemy_bitboard = board.color_bitboards[enemy_index];

        Self {
            board,
            own_color,
            enemy_color,
            own_index,
            enemy_index,
            own_king: board.king_square(own_color),
            own_bitboard: board.color_bitboards[own_index],
            enemy_bitboard,
            all_bitboard: board.all_pieces(),
            enemy_attack_bitboard: 0,
            move_type_bitmask: if captures_only { enemy_bitboard } else { !0 },
            generate_quiet_moves: !captures_only,
            promotions_to_generate: PromotionMode::All,
            in_check: false,
            in_double_check: false,
            check_ray_bitmask: !0,
            pin_rays_bitmask: 0,
            moves: Vec::with_capacity(MAX_MOVES),
        }
    }

    fn calculate_attack_data(&mut self) {
        let board = self.board;
        let king = self.own_king;

        let enemy_orthogonal = board.orthogonal_sliders(self.enemy_color);
        let enemy_diagonal = board.diagonal_sliders(self.enemy_color);
        let enemy_knights = board.pieces(piece::KNIGHT, self.enemy_color);
        let enemy_pawns = board.pieces(piece::PAWN, self.enemy_color);

        let blockers = self.all_bitboard & !bit(king);

        let mut enemy_attack_bitboard = 0;

        for from_square in squares(enemy_orthogonal) {
            enemy_attack_bitboard |= rook_attacks(from_square, blockers);
        }

        for from_square in squares(enemy_diagonal) {
            enemy_attack_bitboard |= bishop_attacks(from_square, blockers);
        }

        let slider_candidates = (rook_attacks(king, self.enemy_bitboard) & enemy_orthogonal)
            | (bishop_attacks(king, self.enemy_bitboard) & enemy_diagonal);

        let mut check_ray_bitmask = 0;
        let mut num_checks = 0;

        for slider_square in squares(slider_candidates) {
            let between = BETWEEN[king][slider_square];

            match (between & self.all_bitboard).count_ones() {
                0 => {
                    check_ray_bitmask |= between | bit(slider_square);
                    num_checks += 1;
                }
                1 => self.pin_rays_bitmask |= between | bit(slider_square),

                _ => {}
            }
        }

        let non_slider_checkers = (KNIGHT_MOVES[king] & enemy_knights)
            | (PAWN_ATTACKS[self.own_index][king] & enemy_pawns);

        check_ray_bitmask |= non_slider_checkers;
        num_checks += non_slider_checkers.count_ones();

        self.in_check = num_checks > 0;
        self.in_double_check = num_checks > 1;
        self.check_ray_bitmask = if self.in_check { check_ray_bitmask } else { !0 };

        for knight_square in squares(enemy_knights) {
            enemy_attack_bitboard |= KNIGHT_MOVES[knight_square];
        }

        enemy_attack_bitboard |= bitboard::pawn_attacks(enemy_pawns, self.enemy_index);
        enemy_attack_bitboard |= KING_MOVES[board.king_square(self.enemy_color)];

        self.enemy_attack_bitboard = enemy_attack_bitboard;
    }

    fn generate_king_moves(&mut self) {
        let legal_bitmask = !(self.enemy_attack_bitboard | self.own_bitboard);
        let king_move_bitboard = KING_MOVES[self.own_king] & legal_bitmask & self.move_type_bitmask;

        self.add_moves(self.own_king, king_move_bitboard);

        if self.in_check || !self.generate_quiet_moves {
            return;
        }

        let castling = &CASTLING[self.own_index];
        let sides = [
            (
                self.board.has_kingside_castle_right(self.own_index),
                &castling.kingside,
            ),
            (
                self.board.has_queenside_castle_right(self.own_index),
                &castling.queenside,
            ),
        ];

        for (has_right, side) in sides {
            if has_right
                && side.empty & self.all_bitboard == 0
                && side.safe & self.enemy_attack_bitboard == 0
            {
                self.moves.push(Move::with_flag(
                    self.own_king,
                    side.king_to,
                    MoveFlag::Castling,
                ));
            }
        }
    }

    fn generate_sliding_moves(&mut self) {
        let board = self.board;
        let move_bitmask = !self.own_bitboard & self.check_ray_bitmask & self.move_type_bitmask;

        let mut orthogonal_sliders_bitboard = board.orthogonal_sliders(self.own_color);
        let mut diagonal_sliders_bitboard = board.diagonal_sliders(self.own_color);

        if self.in_check {
            orthogonal_sliders_bitboard &= !self.pin_rays_bitmask;
            diagonal_sliders_bitboard &= !self.pin_rays_bitmask;
        }

        for from_square in squares(orthogonal_sliders_bitboard) {
            let move_square_bitboard = rook_attacks(from_square, self.all_bitboard)
                & move_bitmask
                & self.pin_line(from_square);

            self.add_moves(from_square, move_square_bitboard);
        }

        for from_square in squares(diagonal_sliders_bitboard) {
            let move_square_bitboard = bishop_attacks(from_square, self.all_bitboard)
                & move_bitmask
                & self.pin_line(from_square);

            self.add_moves(from_square, move_square_bitboard);
        }
    }

    fn generate_knight_moves(&mut self) {
        let knight_bitboard =
            self.board.pieces(piece::KNIGHT, self.own_color) & !self.pin_rays_bitmask;
        let move_bitmask = !self.own_bitboard & self.check_ray_bitmask & self.move_type_bitmask;

        for from_square in squares(knight_bitboard) {
            self.add_moves(from_square, KNIGHT_MOVES[from_square] & move_bitmask);
        }
    }

    fn generate_pawn_moves(&mut self) {
        let board = self.board;
        let rules = &PAWN_RULES[self.own_index];

        let pawns = board.pieces(piece::PAWN, self.own_color);
        let empty_bitboard = !self.all_bitboard;
        let check_ray_bitmask = self.check_ray_bitmask;
        let promotion_rank = rules.promotion_rank;

        let single_push = bitboard::shift(pawns, rules.push) & empty_bitboard;

        let capture_left = bitboard::shift(pawns & rules.capture_left_files, rules.capture_left)
            & self.enemy_bitboard
            & check_ray_bitmask;
        let capture_right = bitboard::shift(pawns & rules.capture_right_files, rules.capture_right)
            & self.enemy_bitboard
            & check_ray_bitmask;

        if self.generate_quiet_moves {
            self.add_pawn_moves(
                single_push & !promotion_rank & check_ray_bitmask,
                rules.push,
                MoveFlag::None,
            );

            let double_push = bitboard::shift(single_push, rules.push)
                & empty_bitboard
                & rules.double_push_rank
                & check_ray_bitmask;

            self.add_pawn_moves(double_push, rules.push * 2, MoveFlag::PawnTwoUp);
        }

        self.add_pawn_moves(
            capture_left & !promotion_rank,
            rules.capture_left,
            MoveFlag::None,
        );
        self.add_pawn_moves(
            capture_right & !promotion_rank,
            rules.capture_right,
            MoveFlag::None,
        );

        self.add_pawn_promotions(single_push & promotion_rank & check_ray_bitmask, rules.push);
        self.add_pawn_promotions(capture_left & promotion_rank, rules.capture_left);
        self.add_pawn_promotions(capture_right & promotion_rank, rules.capture_right);

        if let Some(ep_file_index) = board.en_passant_file {
            let to_square = rules.en_passant_rank * 8 + ep_file_index;
            let captured_pawn_square = (to_square as i32 - rules.push) as usize;

            if contains_square(check_ray_bitmask, captured_pawn_square) {
                let pawns_that_can_capture_ep = pawns & PAWN_ATTACKS[self.enemy_index][to_square];

                for from_square in squares(pawns_that_can_capture_ep) {
                    if self.can_move_along_pin(from_square, to_square)
                        && !self.in_check_after_en_passant(
                            from_square,
                            to_square,
                            captured_pawn_square,
                        )
                    {
                        self.moves.push(Move::with_flag(
                            from_square,
                            to_square,
                            MoveFlag::EnPassantCapture,
                        ));
                    }
                }
            }
        }
    }

    fn in_check_after_en_passant(
        &self,
        from_square: usize,
        to_square: usize,
        ep_capture_square: usize,
    ) -> bool {
        let masked_blockers =
            self.all_bitboard ^ (bit(ep_capture_square) | bit(from_square) | bit(to_square));

        let rook_attackers = rook_attacks(self.own_king, masked_blockers)
            & self.board.orthogonal_sliders(self.enemy_color);
        let bishop_attackers = bishop_attacks(self.own_king, masked_blockers)
            & self.board.diagonal_sliders(self.enemy_color);

        rook_attackers | bishop_attackers != 0
    }

    fn add_moves(&mut self, from_square: usize, targets: u64) {
        self.moves
            .extend(squares(targets).map(|to_square| Move::new(from_square, to_square)));
    }

    fn add_pawn_moves(&mut self, targets: u64, offset: i32, flag: MoveFlag) {
        for to_square in squares(targets) {
            let from_square = (to_square as i32 - offset) as usize;

            if self.can_move_along_pin(from_square, to_square) {
                self.moves
                    .push(Move::with_flag(from_square, to_square, flag));
            }
        }
    }

    fn add_pawn_promotions(&mut self, targets: u64, offset: i32) {
        let promotions = self.promotion_flags();

        for to_square in squares(targets) {
            let from_square = (to_square as i32 - offset) as usize;

            if self.can_move_along_pin(from_square, to_square) {
                self.moves.extend(
                    promotions
                        .iter()
                        .map(|&flag| Move::with_flag(from_square, to_square, flag)),
                );
            }
        }
    }

    fn promotion_flags(&self) -> &'static [MoveFlag] {
        if !self.generate_quiet_moves {
            return &PROMOTIONS[..1];
        }

        match self.promotions_to_generate {
            PromotionMode::All => &PROMOTIONS,
            PromotionMode::QueenOnly => &PROMOTIONS[..1],
            PromotionMode::QueenAndKnight => &PROMOTIONS[..2],
        }
    }

    #[inline(always)]
    fn pin_line(&self, square: usize) -> u64 {
        if contains_square(self.pin_rays_bitmask, square) {
            LINE[square][self.own_king]
        } else {
            !0
        }
    }

    #[inline(always)]
    fn can_move_along_pin(&self, from_square: usize, to_square: usize) -> bool {
        contains_square(self.pin_line(from_square), to_square)
    }
}
