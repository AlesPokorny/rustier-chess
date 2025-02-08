use crate::types::{piece::Pieces, square::Square};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Clone)]
/// bit 0..5     destination
/// bit 6..11    origin
/// bit 12..13   promotion piece (0 queen, 1 rook, 2 bishop, 3 knight)
/// bit 14..15   1 - promotion flag, 2 en passant, 3 castling
pub struct Move(pub u16);

impl Move {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_destination(square: &Square) -> Self {
        Self(square.as_u16())
    }

    pub fn from_origin(origin: &Square) -> Self {
        Self(origin.as_u16() << 6)
    }

    pub fn from_origin_and_destination(destination: &Square, origin: &Square) -> Self {
        Self(destination.as_u16() | (origin.as_u16() << 6))
    }

    pub fn set_destination(&mut self, square: &Square) {
        self.0 |= square.as_u16()
    }

    pub fn set_origin(&mut self, square: &Square) {
        self.0 |= square.as_u16() << 6
    }

    pub fn set_castling(&mut self) {
        self.0 |= 0xc000 // 3 << 14
    }

    pub fn set_promotion(&mut self, piece_type: usize) {
        let piece_value: u16 = match piece_type {
            (0..=3) => piece_type as u16,
            _ => panic!("Cannot promote to king nor pawn"),
        };
        self.0 |= piece_value << 12;
        self.0 |= 0x4000; // 1 << 14
    }

    pub fn set_en_passant(&mut self) {
        self.0 |= 0x8000; // 1 << 15
    }

    pub fn get_destination(&self) -> Square {
        Square::new((self.0 & 0x3f) as u8) // 0b111111
    }

    pub fn get_origin(&self) -> Square {
        Square::new(((self.0 & 0xfc0) >> 6) as u8) // 0b111111000000
    }

    ///1 - promotion flag, 2 en passant, 3 castling
    pub fn special_move(&self) -> u16 {
        self.0 >> 14
    }

    pub fn get_promotion_piece(&self) -> usize {
        ((self.0 & 0x3000) >> 12) as usize
    }
    pub fn from_long_str(input: &str) -> Self {
        let origin = Square::from_str(&input[0..=1]).unwrap();
        let destination = Square::from_str(&input[2..=3]).unwrap();

        let mut output = Move::from_origin_and_destination(&destination, &origin);

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

        if self.special_move() != 1 {
            return output;
        }

        let promotion_piece = match self.get_promotion_piece() {
            Pieces::QUEEN => "q",
            Pieces::KNIGHT => "n",
            Pieces::ROOK => "r",
            Pieces::BISHOP => "b",
            _ => panic!("Unexpected promotion piece {}", self.get_promotion_piece()),
        };
        output.push_str(promotion_piece);
        output
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        (self.0 & 0x3FFF) == (other.0 & 0x3FFF)
    }
}

impl Default for Move {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)?;

        Ok(())
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let promotion_str = if self.special_move() == 1 {
            match self.get_promotion_piece() {
                Pieces::QUEEN => "q",
                Pieces::KNIGHT => "n",
                Pieces::ROOK => "r",
                Pieces::BISHOP => "b",
                _ => panic!("wrong promotion piece"),
            }
        } else {
            ""
        };

        write!(
            f,
            "{}{}{}",
            self.get_origin(),
            self.get_destination(),
            promotion_str,
        )
    }
}
