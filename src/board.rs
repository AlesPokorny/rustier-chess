use std::fmt;

use crate::bitboard::BitBoard;


pub struct Board {
    pub pawns: BitBoard,
    pub rooks: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub queens: BitBoard,
    pub kings: BitBoard,
    pub white_pieces: BitBoard,
    pub black_pieces: BitBoard,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pawns: BitBoard::new(0b0000000011111111000000000000000000000000000000001111111100000000),
            rooks: BitBoard::new(0b1000000100000000000000000000000000000000000000000000000010000001),
            knights: BitBoard::new(0b0100001000000000000000000000000000000000000000000000000001000010),
            bishops: BitBoard::new(0b0010010000000000000000000000000000000000000000000000000000100100),
            queens: BitBoard::new(0b0001000000000000000000000000000000000000000000000000000000001000),
            kings: BitBoard::new(0b0000100000000000000000000000000000000000000000000000000000001000),
            white_pieces: BitBoard::new(0b1111111111111111000000000000000000000000000000000000000000000000),
            black_pieces: BitBoard::new(0b0000000000000000000000000000000000000000000000001111111111111111),
        }
    }
}

// impl fmt::Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
//         write!(f, "{}", self)
//     }
// }

#[derive(Clone, Copy)]
pub struct Position (u8, u8);

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> u8 {
        self.0
    }

    pub fn y(&self) -> u8 {
        self.1
    }

    pub fn xy(&self) -> (u8, u8) {
        (self.0, self.1)
    }
}