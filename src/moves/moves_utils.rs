use crate::{piece::PieceType, square::Square};

// bit 0..5     destination
// bit 6..11    origin
// bit 12..13   promotion piece (0 knight, 1 bishop, 2 rook, 3 queen)
// bit 14       promotion flag
// bit 15       en passant flag
pub struct Move(u16);

impl Move {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_destination(square: &Square) -> Self {
        Self(square.as_u16())
    }

    pub fn from_origin(origin: &Square) -> Self {
        Self(origin.as_u16() << 5)
    }

    pub fn from_origin_and_destination(destination: &Square, origin: &Square) -> Self {
        Self(destination.as_u16() | (origin.as_u16() << 5))
    }

    pub fn set_destination(&mut self, square: &Square) {
        self.0 |= square.as_u16()
    }

    pub fn set_origin(&mut self, square: &Square) {
        self.0 |= square.as_u16() << 5
    }

    pub fn set_promotion(&mut self, piece_type: PieceType) {
        let piece_value: u16 = match piece_type {
            PieceType::Q => 4,
            PieceType::R => 3,
            PieceType::B => 2,
            PieceType::N => 1,
            _ => panic!("Cannot promote to king nor pawn"),
        };
        self.0 |= piece_value << 12;
        self.0 |= 1 << 14;
    }

    pub fn set_en_passant(&mut self) {
        self.0 |= 1 << 15;
    }
}
