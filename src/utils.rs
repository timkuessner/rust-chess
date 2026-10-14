pub mod bitboard {
    use std::iter::FusedIterator;

    pub const FILE_A: u64 = 0x0101_0101_0101_0101;
    pub const FILE_H: u64 = FILE_A << 7;

    pub const NOT_A_FILE: u64 = !FILE_A;
    pub const NOT_H_FILE: u64 = !FILE_H;

    pub const RANK_1: u64 = 0b1111_1111;
    pub const RANK_4: u64 = RANK_1 << 24;
    pub const RANK_5: u64 = RANK_1 << 32;
    pub const RANK_8: u64 = RANK_1 << 56;

    #[inline(always)]
    pub const fn bit(square: usize) -> u64 {
        1u64 << square
    }

    #[inline(always)]
    pub const fn contains_square(bitboard: u64, square: usize) -> bool {
        (bitboard >> square) & 1 != 0
    }

    pub const fn on_board(file: i32, rank: i32) -> bool {
        file >= 0 && file < 8 && rank >= 0 && rank < 8
    }

    #[inline(always)]
    pub const fn shift(bitboard: u64, num_squares_to_shift: i32) -> u64 {
        if num_squares_to_shift > 0 {
            bitboard << num_squares_to_shift
        } else {
            bitboard >> -num_squares_to_shift
        }
    }

    #[inline(always)]
    pub const fn pawn_attacks(pawns: u64, color_index: usize) -> u64 {
        if color_index == 0 {
            ((pawns << 9) & NOT_A_FILE) | ((pawns << 7) & NOT_H_FILE)
        } else {
            ((pawns >> 7) & NOT_A_FILE) | ((pawns >> 9) & NOT_H_FILE)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Squares(u64);

    #[inline(always)]
    pub const fn squares(bitboard: u64) -> Squares {
        Squares(bitboard)
    }

    impl Iterator for Squares {
        type Item = usize;

        #[inline(always)]
        fn next(&mut self) -> Option<usize> {
            if self.0 == 0 {
                return None;
            }

            let square = self.0.trailing_zeros() as usize;

            self.0 &= self.0 - 1;
            Some(square)
        }

        #[inline(always)]
        fn size_hint(&self) -> (usize, Option<usize>) {
            let count = self.0.count_ones() as usize;
            (count, Some(count))
        }
    }

    impl ExactSizeIterator for Squares {}
    impl FusedIterator for Squares {}
}

pub mod tables {
    use super::bitboard::{bit, on_board};

    pub static KNIGHT_MOVES: [u64; 64] = leaper_attacks([
        (1, 2),
        (2, 1),
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
    ]);

    pub static KING_MOVES: [u64; 64] = leaper_attacks([
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ]);

    pub static PAWN_ATTACKS: [[u64; 64]; 2] = generate_pawn_attacks();

    pub static LINE: [[u64; 64]; 64] = generate_lines();

    pub static BETWEEN: [[u64; 64]; 64] = generate_between();

    pub struct CastleSide {
        pub empty: u64,
        pub safe: u64,
        pub king_to: usize,
    }

    pub struct Castling {
        pub kingside: CastleSide,
        pub queenside: CastleSide,
    }

    pub static CASTLING: [Castling; 2] = [
        Castling {
            kingside: CastleSide {
                empty: bit(5) | bit(6),
                safe: bit(5) | bit(6),
                king_to: 6,
            },
            queenside: CastleSide {
                empty: bit(1) | bit(2) | bit(3),
                safe: bit(2) | bit(3),
                king_to: 2,
            },
        },
        Castling {
            kingside: CastleSide {
                empty: bit(61) | bit(62),
                safe: bit(61) | bit(62),
                king_to: 62,
            },
            queenside: CastleSide {
                empty: bit(57) | bit(58) | bit(59),
                safe: bit(58) | bit(59),
                king_to: 58,
            },
        },
    ];

    const fn leaper_attacks(offsets: [(i32, i32); 8]) -> [u64; 64] {
        let mut attacks = [0u64; 64];

        let mut square = 0;
        while square < 64 {
            let mut i = 0;
            while i < 8 {
                let file = (square % 8) as i32 + offsets[i].0;
                let rank = (square / 8) as i32 + offsets[i].1;

                if on_board(file, rank) {
                    attacks[square] |= bit((rank * 8 + file) as usize);
                }

                i += 1;
            }

            square += 1;
        }

        attacks
    }

    const fn generate_pawn_attacks() -> [[u64; 64]; 2] {
        let mut attacks = [[0u64; 64]; 2];

        let mut color = 0;
        while color < 2 {
            let forward = if color == 0 { 1 } else { -1 };

            let mut square = 0;
            while square < 64 {
                let file = (square % 8) as i32;
                let rank = (square / 8) as i32 + forward;

                if on_board(file - 1, rank) {
                    attacks[color][square] |= bit((rank * 8 + file - 1) as usize);
                }
                if on_board(file + 1, rank) {
                    attacks[color][square] |= bit((rank * 8 + file + 1) as usize);
                }

                square += 1;
            }

            color += 1;
        }

        attacks
    }

    const fn alignment(a: usize, b: usize) -> Option<(i32, i32)> {
        let dx = (b % 8) as i32 - (a % 8) as i32;
        let dy = (b / 8) as i32 - (a / 8) as i32;

        if a != b && (dx == 0 || dy == 0 || dx.abs() == dy.abs()) {
            Some((dx.signum(), dy.signum()))
        } else {
            None
        }
    }

    const fn generate_lines() -> [[u64; 64]; 64] {
        let mut lines = [[0u64; 64]; 64];

        let mut a = 0;
        while a < 64 {
            let mut b = 0;
            while b < 64 {
                if let Some((dx, dy)) = alignment(a, b) {
                    let ax = (a % 8) as i32;
                    let ay = (a / 8) as i32;

                    let mut i = -7;
                    while i <= 7 {
                        let x = ax + dx * i;
                        let y = ay + dy * i;

                        if on_board(x, y) {
                            lines[a][b] |= bit((y * 8 + x) as usize);
                        }

                        i += 1;
                    }
                }

                b += 1;
            }

            a += 1;
        }

        lines
    }

    const fn generate_between() -> [[u64; 64]; 64] {
        let mut between = [[0u64; 64]; 64];

        let mut a = 0;
        while a < 64 {
            let mut b = 0;
            while b < 64 {
                if let Some((dx, dy)) = alignment(a, b) {
                    let bx = (b % 8) as i32;
                    let by = (b / 8) as i32;

                    let mut x = (a % 8) as i32 + dx;
                    let mut y = (a / 8) as i32 + dy;

                    while x != bx || y != by {
                        between[a][b] |= bit((y * 8 + x) as usize);
                        x += dx;
                        y += dy;
                    }
                }

                b += 1;
            }

            a += 1;
        }

        between
    }
}

pub mod magic {
    use std::sync::LazyLock;

    use super::bitboard::{bit, on_board};

    const ROOK_DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    const BISHOP_DIRECTIONS: [(i32, i32); 4] = [(-1, -1), (-1, 1), (1, 1), (1, -1)];

    const ROOK_SHIFTS: [u32; 64] = [
        52, 52, 52, 52, 52, 52, 52, 52, 53, 53, 53, 54, 53, 53, 54, 53, 53, 54, 54, 54, 53, 53, 54,
        53, 53, 54, 53, 53, 54, 54, 54, 53, 52, 54, 53, 53, 53, 53, 54, 53, 52, 53, 54, 54, 53, 53,
        54, 53, 53, 54, 54, 54, 53, 53, 54, 53, 52, 53, 53, 53, 53, 53, 53, 52,
    ];

    const BISHOP_SHIFTS: [u32; 64] = [
        58, 60, 59, 59, 59, 59, 60, 58, 60, 59, 59, 59, 59, 59, 59, 60, 59, 59, 57, 57, 57, 57, 59,
        59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 55, 55, 57, 59, 59, 59, 59, 57, 57, 57, 57,
        59, 59, 60, 60, 59, 59, 59, 59, 60, 60, 58, 60, 59, 59, 59, 59, 59, 58,
    ];

    const ROOK_MAGICS: [u64; 64] = [
        468374916371625120,
        18428729537625841661,
        2531023729696186408,
        6093370314119450896,
        13830552789156493815,
        16134110446239088507,
        12677615322350354425,
        5404321144167858432,
        2111097758984580,
        18428720740584907710,
        17293734603602787839,
        4938760079889530922,
        7699325603589095390,
        9078693890218258431,
        578149610753690728,
        9496543503900033792,
        1155209038552629657,
        9224076274589515780,
        1835781998207181184,
        509120063316431138,
        16634043024132535807,
        18446673631917146111,
        9623686630121410312,
        4648737361302392899,
        738591182849868645,
        1732936432546219272,
        2400543327507449856,
        5188164365601475096,
        10414575345181196316,
        1162492212166789136,
        9396848738060210946,
        622413200109881612,
        7998357718131801918,
        7719627227008073923,
        16181433497662382080,
        18441958655457754079,
        1267153596645440,
        18446726464209379263,
        1214021438038606600,
        4650128814733526084,
        9656144899867951104,
        18444421868610287615,
        3695311799139303489,
        10597006226145476632,
        18436046904206950398,
        18446726472933277663,
        3458977943764860944,
        39125045590687766,
        9227453435446560384,
        6476955465732358656,
        1270314852531077632,
        2882448553461416064,
        11547238928203796481,
        1856618300822323264,
        2573991788166144,
        4936544992551831040,
        13690941749405253631,
        15852669863439351807,
        18302628748190527413,
        12682135449552027479,
        13830554446930287982,
        18302628782487371519,
        7924083509981736956,
        4734295326018586370,
    ];

    const BISHOP_MAGICS: [u64; 64] = [
        16509839532542417919,
        14391803910955204223,
        1848771770702627364,
        347925068195328958,
        5189277761285652493,
        3750937732777063343,
        18429848470517967340,
        17870072066711748607,
        16715520087474960373,
        2459353627279607168,
        7061705824611107232,
        8089129053103260512,
        7414579821471224013,
        9520647030890121554,
        17142940634164625405,
        9187037984654475102,
        4933695867036173873,
        3035992416931960321,
        15052160563071165696,
        5876081268917084809,
        1153484746652717320,
        6365855841584713735,
        2463646859659644933,
        1453259901463176960,
        9808859429721908488,
        2829141021535244552,
        576619101540319252,
        5804014844877275314,
        4774660099383771136,
        328785038479458864,
        2360590652863023124,
        569550314443282,
        17563974527758635567,
        11698101887533589556,
        5764964460729992192,
        6953579832080335136,
        1318441160687747328,
        8090717009753444376,
        16751172641200572929,
        5558033503209157252,
        17100156536247493656,
        7899286223048400564,
        4845135427956654145,
        2368485888099072,
        2399033289953272320,
        6976678428284034058,
        3134241565013966284,
        8661609558376259840,
        17275805361393991679,
        15391050065516657151,
        11529206229534274423,
        9876416274250600448,
        16432792402597134585,
        11975705497012863580,
        11457135419348969979,
        9763749252098620046,
        16960553411078512574,
        15563877356819111679,
        14994736884583272463,
        9441297368950544394,
        14537646123432199168,
        9888547162215157388,
        18140215579194907366,
        18374682062228545019,
    ];

    #[derive(Clone, Copy, Default)]
    struct Magic {
        mask: u64,
        magic: u64,
        shift: u32,
        offset: usize,
    }

    impl Magic {
        #[inline(always)]
        fn index(&self, occupancy: u64) -> usize {
            ((occupancy & self.mask).wrapping_mul(self.magic) >> self.shift) as usize
        }
    }

    struct SlidingAttacks {
        magics: [Magic; 64],
        table: Box<[u64]>,
    }

    impl SlidingAttacks {
        fn new(directions: &[(i32, i32); 4], magics: &[u64; 64], shifts: &[u32; 64]) -> Self {
            let mut entries = [Magic::default(); 64];
            let mut table: Vec<u64> = Vec::new();

            for square in 0..64 {
                let mask = relevant_occupancy(square, directions);
                let entry = Magic {
                    mask,
                    magic: magics[square],
                    shift: shifts[square],
                    offset: table.len(),
                };

                table.resize(entry.offset + (1usize << (64 - entry.shift)), 0);

                let mut blockers = 0u64;
                loop {
                    table[entry.offset + entry.index(blockers)] =
                        slider_attacks(square, blockers, directions);

                    blockers = blockers.wrapping_sub(mask) & mask;
                    if blockers == 0 {
                        break;
                    }
                }

                entries[square] = entry;
            }

            Self {
                magics: entries,
                table: table.into_boxed_slice(),
            }
        }

        #[inline(always)]
        fn attacks(&self, square: usize, occupancy: u64) -> u64 {
            let magic = &self.magics[square];
            self.table[magic.offset + magic.index(occupancy)]
        }
    }

    static ROOK: LazyLock<SlidingAttacks> =
        LazyLock::new(|| SlidingAttacks::new(&ROOK_DIRECTIONS, &ROOK_MAGICS, &ROOK_SHIFTS));

    static BISHOP: LazyLock<SlidingAttacks> =
        LazyLock::new(|| SlidingAttacks::new(&BISHOP_DIRECTIONS, &BISHOP_MAGICS, &BISHOP_SHIFTS));

    #[inline(always)]
    pub fn rook_attacks(square: usize, occupancy: u64) -> u64 {
        ROOK.attacks(square, occupancy)
    }

    #[inline(always)]
    pub fn bishop_attacks(square: usize, occupancy: u64) -> u64 {
        BISHOP.attacks(square, occupancy)
    }

    fn relevant_occupancy(square: usize, directions: &[(i32, i32); 4]) -> u64 {
        let mut mask = 0u64;

        for &(dx, dy) in directions {
            let mut x = (square % 8) as i32 + dx;
            let mut y = (square / 8) as i32 + dy;

            while on_board(x + dx, y + dy) {
                mask |= bit((y * 8 + x) as usize);
                x += dx;
                y += dy;
            }
        }

        mask
    }

    fn slider_attacks(square: usize, blockers: u64, directions: &[(i32, i32); 4]) -> u64 {
        let mut attacks = 0u64;

        for &(dx, dy) in directions {
            let mut x = (square % 8) as i32 + dx;
            let mut y = (square / 8) as i32 + dy;

            while on_board(x, y) {
                let target = bit((y * 8 + x) as usize);
                attacks |= target;

                if blockers & target != 0 {
                    break;
                }

                x += dx;
                y += dy;
            }
        }

        attacks
    }
}

pub mod zobrist {
    use crate::board::Board;
    use crate::piece::piece;

    const fn splitmix64(state: u64) -> (u64, u64) {
        let state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        (state, z ^ (z >> 31))
    }

    pub struct Zobrist {
        pub pieces: [[u64; 64]; 12],
        pub castling: [u64; 16],
        pub en_passant_file: [u64; 8],
        pub side_to_move: u64,
    }

    const fn generate() -> Zobrist {
        let mut state = 0x1234_5678_9ABC_DEF0u64;
        let mut z = Zobrist {
            pieces: [[0; 64]; 12],
            castling: [0; 16],
            en_passant_file: [0; 8],
            side_to_move: 0,
        };

        let mut i = 0;
        while i < 12 {
            let mut s = 0;
            while s < 64 {
                let (next, value) = splitmix64(state);
                state = next;
                z.pieces[i][s] = value;
                s += 1;
            }
            i += 1;
        }

        i = 0;
        while i < 16 {
            let (next, value) = splitmix64(state);
            state = next;
            z.castling[i] = value;
            i += 1;
        }

        i = 0;
        while i < 8 {
            let (next, value) = splitmix64(state);
            state = next;
            z.en_passant_file[i] = value;
            i += 1;
        }

        let (_, value) = splitmix64(state);
        z.side_to_move = value;

        z
    }

    pub static ZOBRIST: Zobrist = generate();

    pub fn compute_key(board: &Board) -> u64 {
        let mut key = 0u64;

        for (square, &p) in board.squares.iter().enumerate() {
            if p != piece::NONE {
                key ^= ZOBRIST.pieces[piece::index(p)][square];
            }
        }

        key ^= ZOBRIST.castling[board.castling_rights as usize];

        if let Some(file) = board.en_passant_file {
            key ^= ZOBRIST.en_passant_file[file];
        }

        if board.color_to_move == piece::BLACK {
            key ^= ZOBRIST.side_to_move;
        }

        key
    }
}
