#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveFlag {
    None,
    EnPassantCapture,
    Castling,
    PromoteToQueen,
    PromoteToKnight,
    PromoteToRook,
    PromoteToBishop,
    PawnTwoUp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub flag: MoveFlag,
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            flag: MoveFlag::None,
        }
    }

    pub fn with_flag(from: usize, to: usize, flag: MoveFlag) -> Self {
        Self { from, to, flag }
    }
}
