use std::fmt;

use crate::{
    bitboard::BitBoard,
    piece::{Color, Piece, PieceType},
};

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

impl Board {
    pub fn get_piece_at_bit(&self, bit: u8) -> Option<Piece> {
        let color = if self.white_pieces.read_bit(bit) {
            Color::W
        } else if self.black_pieces.read_bit(bit) {
            Color::B
        } else {
            return None;
        };

        let piece = if self.pawns.read_bit(bit) {
            PieceType::P
        } else if self.rooks.read_bit(bit) {
            PieceType::R
        } else if self.knights.read_bit(bit) {
            PieceType::N
        } else if self.bishops.read_bit(bit) {
            PieceType::B
        } else if self.queens.read_bit(bit) {
            PieceType::Q
        } else if self.kings.read_bit(bit) {
            PieceType::K
        } else {
            panic!("Boom. Unexpected piece");
        };

        Some(Piece::new(piece, color))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            pawns: BitBoard::new(
                0b0000000011111111000000000000000000000000000000001111111100000000,
            ),
            rooks: BitBoard::new(
                0b1000000100000000000000000000000000000000000000000000000010000001,
            ),
            knights: BitBoard::new(
                0b0100001000000000000000000000000000000000000000000000000001000010,
            ),
            bishops: BitBoard::new(
                0b0010010000000000000000000000000000000000000000000000000000100100,
            ),
            queens: BitBoard::new(
                0b0000100000000000000000000000000000000000000000000000000000001000,
            ),
            kings: BitBoard::new(
                0b0001000000000000000000000000000000000000000000000000000000010000,
            ),
            black_pieces: BitBoard::new(
                0b1111111111111111000000000000000000000000000000000000000000000000,
            ),
            white_pieces: BitBoard::new(
                0b0000000000000000000000000000000000000000000000001111111111111111,
            ),
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8_u8 {
            write!(f, "\n    -------------------------------")?;
            write!(f, "\n {} |", 8 - row)?;
            let row_i = 56 - row * 8;
            for col in 0..8_u8 {
                match self.get_piece_at_bit(row_i + col) {
                    // 63 - (row * 8 + 7 - col)
                    Some(piece) => write!(f, " {} |", piece)?,
                    None => write!(f, "   |")?,
                }
            }
        }

        writeln!(f, "\n    -------------------------------")?;
        write!(f, "     A   B   C   D   E   F   G   H")?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct Position(u8, u8);

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
