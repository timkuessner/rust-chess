use crate::core::types::bitboard::Bitboard;

pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;

pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;

pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;

pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;

pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;

pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;

pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;

pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

pub const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
pub const RANK_2: Bitboard = Bitboard(0x000000000000FF00);
pub const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
pub const RANK_4: Bitboard = Bitboard(0x00000000FF000000);
pub const RANK_5: Bitboard = Bitboard(0x000000FF00000000);
pub const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
pub const RANK_7: Bitboard = Bitboard(0x00FF000000000000);
pub const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

pub const FILE_A: Bitboard = Bitboard(0x0101010101010101);
pub const FILE_B: Bitboard = Bitboard(0x0202020202020202);
pub const FILE_C: Bitboard = Bitboard(0x0404040404040404);
pub const FILE_D: Bitboard = Bitboard(0x0808080808080808);
pub const FILE_E: Bitboard = Bitboard(0x1010101010101010);
pub const FILE_F: Bitboard = Bitboard(0x2020202020202020);
pub const FILE_G: Bitboard = Bitboard(0x4040404040404040);
pub const FILE_H: Bitboard = Bitboard(0x8080808080808080);

pub const NOT_A_FILE: Bitboard = Bitboard(0xFEFEFEFEFEFEFEFE);
pub const NOT_H_FILE: Bitboard = Bitboard(0x7F7F7F7F7F7F7F7F);
pub const NOT_AB_FILE: Bitboard = Bitboard(0xFCFCFCFCFCFCFCFC);
pub const NOT_GH_FILE: Bitboard = Bitboard(0x3F3F3F3F3F3F3F3F);

pub enum SquareParseError {
    InvalidLength,
    InvalidFile,
    InvalidRank,
}

pub struct Square;

impl Square {
    pub fn from_algebraic(s: &str) -> Result<u8, SquareParseError> {
        let bytes = s.as_bytes();
        
        if bytes.len() != 2 {
            return Err(SquareParseError::InvalidLength);
        }
        
        let file = match bytes[0] {
            b'a' => 0,
            b'b' => 1,
            b'c' => 2,
            b'd' => 3,
            b'e' => 4,
            b'f' => 5,
            b'g' => 6,
            b'h' => 7,
            _ => return Err(SquareParseError::InvalidFile),
        };
        
        let rank = match bytes[1] {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            _ => return Err(SquareParseError::InvalidRank),
        };
        
        Ok(rank * 8 + file)
    }
    
    pub fn to_algebraic(square: u8) -> String {
        let file = (b'a' + (square % 8)) as char;
        let rank = (b'1' + (square / 8)) as char;
        format!("{}{}", file, rank)
    }
    
    pub fn rank(square: u8) -> u8 {
        square / 8
    }
    
    pub fn file(square: u8) -> u8 {
        square % 8
    }
    
    pub fn from_coords(rank: u8, file: u8) -> u8 {
        rank * 8 + file
    }
    
    pub fn rank_bitboard(rank: u8) -> Bitboard {
        match rank {
            0 => RANK_1,
            1 => RANK_2,
            2 => RANK_3,
            3 => RANK_4,
            4 => RANK_5,
            5 => RANK_6,
            6 => RANK_7,
            7 => RANK_8,
            _ => Bitboard::empty(),
        }
    }
    
    pub fn file_bitboard(file: u8) -> Bitboard {
        match file {
            0 => FILE_A,
            1 => FILE_B,
            2 => FILE_C,
            3 => FILE_D,
            4 => FILE_E,
            5 => FILE_F,
            6 => FILE_G,
            7 => FILE_H,
            _ => Bitboard::empty(),
        }
    }
}