pub mod piece {
    pub const NONE: u8 = 0b000;
    pub const PAWN: u8 = 0b001;
    pub const KNIGHT: u8 = 0b010;
    pub const BISHOP: u8 = 0b011;
    pub const ROOK: u8 = 0b100;
    pub const QUEEN: u8 = 0b101;
    pub const KING: u8 = 0b110;

    pub const WHITE: u8 = 0b0000;
    pub const BLACK: u8 = 0b1000;

    pub const COLOR_MASK: u8 = WHITE | BLACK;
    pub const KIND_MASK: u8 = 0b00111;

    #[inline(always)]
    pub const fn kind(p: u8) -> u8 {
        p & KIND_MASK
    }

    #[inline(always)]
    pub const fn color(p: u8) -> u8 {
        p & COLOR_MASK
    }

    #[inline(always)]
    pub const fn color_index(color_or_piece: u8) -> usize {
        (color_or_piece >> 3) as usize
    }

    #[inline(always)]
    pub const fn opposite(color: u8) -> u8 {
        color ^ BLACK
    }

    #[inline(always)]
    pub const fn get_piece_index(piece: u8, color: u8) -> usize {
        (kind(piece) as usize - 1) + color_index(color) * 6
    }

    #[inline(always)]
    pub const fn index(p: u8) -> usize {
        get_piece_index(p, p)
    }

    pub const fn from_fen_char(c: char) -> Option<u8> {
        let kind = match c.to_ascii_lowercase() {
            'p' => PAWN,
            'n' => KNIGHT,
            'b' => BISHOP,
            'r' => ROOK,
            'q' => QUEEN,
            'k' => KING,
            _ => return None,
        };

        Some(kind | if c.is_ascii_uppercase() { WHITE } else { BLACK })
    }
}
