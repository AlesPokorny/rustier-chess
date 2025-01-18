use crate::piece::PieceType;

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

    pub fn from_destination(destination: u8) -> Self {
        Self(destination as u16)
    }

    pub fn from_origin(origin: u8) -> Self {
        Self((origin as u16) << 5)
    }

    pub fn set_destination(&mut self, destination: u8) {
        self.0 |= destination as u16
    }

    pub fn set_origin(&mut self, origin: u8) {
        self.0 |= (origin as u16) << 5
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
