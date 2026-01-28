#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub const fn new(value: u64) -> Self {
        Bitboard(value)
    }

    pub const fn empty() -> Self {
        Bitboard(0)
    }

    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1u64 << square;
    }

    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1u64 << square);
    }

    pub fn get_bit(&self, square: u8) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Bitboard;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}
