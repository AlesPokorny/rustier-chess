use std::fmt;

use crate::square::Square;

#[derive(PartialEq, Eq)]
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
        self.0 |= 3 << 14
    }

    pub fn set_promotion(&mut self, piece_type: usize) {
        let piece_value: u16 = match piece_type {
            (0..=3) => piece_type as u16,
            _ => panic!("Cannot promote to king nor pawn"),
        };
        self.0 |= piece_value << 12;
        self.0 |= 1 << 14;
    }

    pub fn set_en_passant(&mut self) {
        self.0 |= 1 << 15;
    }

    pub fn get_destination(&self) -> Square {
        Square::new((self.0 & 0b111111) as u8)
    }

    pub fn get_origin(&self) -> Square {
        Square::new(((self.0 & 0b111111000000) >> 6) as u8)
    }

    pub fn is_castling(&self) -> bool {
        self.0 >> 14 == 3
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.get_origin();
        let destination = self.get_destination();
        writeln!(f, "from: {}, to: {}", origin, destination)?;

        Ok(())
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.get_origin();
        let destination = self.get_destination();
        write!(f, "from: {}, to: {}", origin, destination)?;

        Ok(())
    }
}
