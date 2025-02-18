use crate::{
    board::Board, types::{bitboard::BitBoard, piece::Pieces, square::Square, state::Castling}, utils::zobrist::ZobristHash
};
use std::{
    fmt::{self, Display},
    hash::{Hash, Hasher},
    str::FromStr,
};

pub struct UnmakeMoveHelper {
    pub origin: Square,
    pub destination: Square,
    pub move_bb: BitBoard,
    pub piece: usize,
    pub capture: Option<usize>,
    pub prev_en_passant: Option<Square>,
    pub prev_hash: ZobristHash,
    pub prev_halfmove: u8,
    pub prev_castling: Castling,
    pub was_promotion: bool,
    pub was_castling: bool,
}
#[derive(Clone)]
/// bit 0..5     destination
/// bit 6..11    origin
/// bit 12..13   promotion piece (0 queen, 1 rook, 2 bishop, 3 knight)
/// bit 14..15   1 - promotion flag, 2 en passant, 3 castling
// pub struct Move(pub u16);

pub struct Move {
    origin: Square,
    destination: Square,
    moving_piece: usize,
    promotion_piece: Option<usize>,
    special_move: Option<bool>,  // true = en_passant, false = castling
    capture: Option<usize>
}

impl Move {
    pub fn from_origin_and_destination(destination: &Square, origin: &Square, moving_piece: usize) -> Self {
        Self {
            origin: *origin,
            destination: *destination,
            moving_piece,
            promotion_piece: None,
            special_move: None,
            capture: None,
        }
    }

    pub fn set_castling(&mut self) {
        self.special_move = Some(false)
    }

    pub fn set_promotion(&mut self, piece_type: usize) {
        self.promotion_piece = Some(piece_type);
    }

    pub fn set_en_passant(&mut self) {
        self.special_move = Some(true)
    }

    pub fn get_destination(&self) -> Square {
        self.destination
    }

    pub fn get_origin(&self) -> Square {
        self.origin
    }

    pub fn get_promotion_piece(&self) -> Option<usize> {
        self.promotion_piece
    }

    /// None -> no special move
    /// true -> en_passant
    /// false -> castling
    pub fn get_special_move(&self) -> Option<bool> {
        self.special_move
    }

    pub fn from_long_str(input: &str, board: &Board) -> Self {
        let origin = Square::from_str(&input[0..=1]).unwrap();
        let destination = Square::from_str(&input[2..=3]).unwrap();

        let piece= board.get_piece_on_square(&origin).unwrap().piece;
        
        let mut output = Move::from_origin_and_destination(&destination, &origin, piece);
        
        if let Some(promotion_piece) = input.chars().nth(4) {
            output.set_promotion(match promotion_piece {
                'q' => Pieces::QUEEN,
                'n' => Pieces::KNIGHT,
                'r' => Pieces::ROOK,
                'b' => Pieces::BISHOP,
                _ => panic!("Unexpected promotion piece {}", promotion_piece),
            });
        }

        output
    }

    pub fn to_long_string(&self) -> String {
        let origin = self.get_origin();
        let destination = self.get_destination();
        let mut output = "".to_owned();
        output.push_str(&origin.to_string());
        output.push_str(&destination.to_string());

        if let Some(promotion_piece) = self.promotion_piece {
            let promotion_piece_string = match promotion_piece {
                Pieces::QUEEN => "q",
                Pieces::KNIGHT => "n",
                Pieces::ROOK => "r",
                Pieces::BISHOP => "b",
                _ => panic!("Unexpected promotion piece {}", promotion_piece),
            };
            output.push_str(promotion_piece_string);
        }

        output
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination && self.promotion_piece == other.promotion_piece
        // (self.0 & 0x3FFF) == (other.0 & 0x3FFF)
    }
}

impl Eq for Move {}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)?;

        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_long_string())
    }
}

// impl Hash for Move {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         state.write_u16(self.0);
//     }
// }

// impl Hasher for Move {
//     fn finish(&self) -> u64 {
//         self.0 as u64
//     }

//     fn write(&mut self, _bytes: &[u8]) {
//         panic!("This hasher only takes u16");
//     }

//     fn write_u16(&mut self, i: u16) {
//         self.0 = i;
//     }
// }
