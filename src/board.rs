use std::fmt;

use crate::{
    bitboard::BitBoard,
    piece::{Color, Piece, PieceType},
};

pub struct Board {
    pub white_pawns: BitBoard,
    pub white_rooks: BitBoard,
    pub white_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub white_queens: BitBoard,
    pub white_king: BitBoard,
    pub black_pawns: BitBoard,
    pub black_rooks: BitBoard,
    pub black_knights: BitBoard,
    pub black_bishops: BitBoard,
    pub black_queens: BitBoard,
    pub black_king: BitBoard,
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

        let piece = if (self.white_pawns | self.black_pawns).read_bit(bit) {
            PieceType::P
        } else if (self.white_rooks | self.black_rooks).read_bit(bit) {
            PieceType::R
        } else if (self.white_knights | self.black_knights).read_bit(bit) {
            PieceType::N
        } else if (self.white_bishops | self.black_bishops).read_bit(bit) {
            PieceType::B
        } else if (self.white_queens | self.black_queens).read_bit(bit) {
            PieceType::Q
        } else if (self.white_king | self.black_king).read_bit(bit) {
            PieceType::K
        } else {
            panic!("Boom. Unexpected piece at bit {}", bit);
        };

        Some(Piece::new(piece, color))
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            white_pawns: BitBoard::new(0xFF00),
            white_rooks: BitBoard::new(0x81),
            white_knights: BitBoard::new(0x42),
            white_bishops: BitBoard::new(0x24),
            white_queens: BitBoard::new(0x8),
            white_king: BitBoard::new(0x10),
            black_pawns: BitBoard::new(0xFF000000000000),
            black_rooks: BitBoard::new(0x8100000000000000),
            black_knights: BitBoard::new(0x4200000000000000),
            black_bishops: BitBoard::new(0x2400000000000000),
            black_queens: BitBoard::new(0x800000000000000),
            black_king: BitBoard::new(0x1000000000000000),
            black_pieces: BitBoard::new(0xFFFF000000000000),
            white_pieces: BitBoard::new(0xFFFF),
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
