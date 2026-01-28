use super::color::Color;

#[derive(Debug)]
pub struct CastlingRights(u8);

impl CastlingRights {
    pub const NONE: u8 = 0;
    pub const WHITE_KINGSIDE: u8 = 1;
    pub const WHITE_QUEENSIDE: u8 = 2;
    pub const BLACK_KINGSIDE: u8 = 4;
    pub const BLACK_QUEENSIDE: u8 = 8;
    pub const WHITE_BOTH: u8 = 3;
    pub const BLACK_BOTH: u8 = 12;
    pub const ALL: u8 = 15;
    
    pub fn new(rights: u8) -> Self {
        CastlingRights(rights & Self::ALL)
    }
    
    pub fn all() -> Self {
        CastlingRights(Self::ALL)
    }
    
    pub fn has_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => (self.0 & Self::WHITE_KINGSIDE) != 0,
            Color::Black => (self.0 & Self::BLACK_KINGSIDE) != 0,
        }
    }
    
    pub fn has_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => (self.0 & Self::WHITE_QUEENSIDE) != 0,
            Color::Black => (self.0 & Self::BLACK_QUEENSIDE) != 0,
        }
    }
    
    pub fn remove_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= !Self::WHITE_KINGSIDE,
            Color::Black => self.0 &= !Self::BLACK_KINGSIDE,
        }
    }
    
    pub fn remove_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= !Self::WHITE_QUEENSIDE,
            Color::Black => self.0 &= !Self::BLACK_QUEENSIDE,
        }
    }
    
    pub fn remove_both(&mut self, color: Color) {
        match color {
            Color::White => self.0 &= !Self::WHITE_BOTH,
            Color::Black => self.0 &= !Self::BLACK_BOTH,
        }
    }
}