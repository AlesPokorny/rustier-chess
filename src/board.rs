use std::fmt;

use crate::{bitboard::BitBoard, piece::Piece, square::Square, state::State};

pub struct Board {
    pub colors: [BitBoard; 2],
    pub pieces: [[BitBoard; 6]; 2],
    pub state: State,
}

impl Board {
    pub fn get_piece_on_square(&self, square: &Square) -> Option<Piece> {
        let mut piece_color: usize = 3;
        for (i, color) in self.colors.iter().enumerate() {
            if color.read_square(square) {
                piece_color = i;
                break;
            }
        }

        if piece_color == 3 {
            return None;
        }

        let mut piece_type = 9_usize;

        for (i, piece_bb) in self.pieces[piece_color].iter().enumerate() {
            if piece_bb.read_square(square) {
                piece_type = i;
                break;
            }
        }
        if piece_type == 9 {
            panic!("Found piece color but not piece type");
        }

        Some(Piece::new(piece_type, piece_color))
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Board {
            colors: [BitBoard::zeros(), BitBoard::zeros()],
            pieces: [
                [
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                ],
                [
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                ],
            ],
            state: State::default(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            colors: [BitBoard::new(0xFFFF), BitBoard::new(0xFFFF000000000000)],
            pieces: [
                [
                    BitBoard::new(0x8),    // queens
                    BitBoard::new(0x81),   // rooks
                    BitBoard::new(0x24),   // bishops
                    BitBoard::new(0x42),   // knights
                    BitBoard::new(0xFF00), // pawns
                    BitBoard::new(0x10),   // king
                ],
                [
                    BitBoard::new(0x800000000000000),  // queens
                    BitBoard::new(0x8100000000000000), // rooks
                    BitBoard::new(0x2400000000000000), // bishops
                    BitBoard::new(0x4200000000000000), // knights
                    BitBoard::new(0xFF000000000000),   // pawns
                    BitBoard::new(0x1000000000000000), // king
                ],
            ],
            state: State::default(),
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
                match self.get_piece_on_square(&Square::new(row_i + col)) {
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
