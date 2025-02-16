use crate::types::{bitboard::BitBoard, piece::Color, square::Square};

pub const WHITE_LONG_ROOK_STARTING_MASK: BitBoard = BitBoard(0b1);
pub const WHITE_SHORT_ROOK_STARTING_MASK: BitBoard = BitBoard(0x80);
pub const BLACK_LONG_ROOK_STARTING_MASK: BitBoard = BitBoard(0x100000000000000);
pub const BLACK_SHORT_ROOK_STARTING_MASK: BitBoard = BitBoard(0x8000000000000000);

/// bit 0:      white short
/// bit 1:      white long
/// bit 2:      black short
/// bit 3:      black long
/// bits 4-7:   unused
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Castling(u8);

impl Default for Castling {
    fn default() -> Self {
        Self(0b1111)
    }
}

impl Castling {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn read_bit(&self, bit: u8) -> bool {
        self.0 & (1 << bit) != 0
    }

    pub fn set_zero(&mut self, bit: u8) {
        self.0 &= !(1 << bit);
    }

    pub fn set_one(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    pub fn set_white_short(&mut self) {
        self.set_one(0)
    }

    pub fn set_white_long(&mut self) {
        self.set_one(1)
    }

    pub fn set_black_short(&mut self) {
        self.set_one(2)
    }

    pub fn set_black_long(&mut self) {
        self.set_one(3)
    }

    pub fn white_short(&self) -> bool {
        self.read_bit(0)
    }

    pub fn white_long(&self) -> bool {
        self.read_bit(1)
    }

    pub fn black_short(&self) -> bool {
        self.read_bit(2)
    }

    pub fn black_long(&self) -> bool {
        self.read_bit(3)
    }

    pub fn remove_white_short(&mut self) {
        self.set_zero(0)
    }

    pub fn remove_white_long(&mut self) {
        self.set_zero(1)
    }

    pub fn remove_black_short(&mut self) {
        self.set_zero(2)
    }

    pub fn remove_black_long(&mut self) {
        self.set_zero(3)
    }

    pub fn remove_color_castling(&mut self, color: usize) {
        if color == Color::WHITE {
            self.0 &= !0b11;
        } else {
            self.0 &= !0b1100;
        }
    }

    pub fn can_color_castle(&self, color: usize) -> bool {
        if color == Color::WHITE {
            self.0 & 0b11 != 0
        } else {
            self.0 & 0b1100 != 0
        }
    }

    pub fn can_someone_castle(&self) -> bool {
        self.0 != 0
    }

    /// return (short, long) tuple
    pub fn can_castle(&self, color: usize) -> (bool, bool) {
        if color == Color::WHITE {
            (self.white_short(), self.white_long())
        } else {
            (self.black_short(), self.black_long())
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct State {
    pub castling: Castling,
    pub en_passant: Option<Square>,
    pub half_moves: u8,
    pub full_moves: u16,
    pub turn: usize,
    pub opponent: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            castling: Castling::default(),
            en_passant: None,
            half_moves: 0,
            full_moves: 0,
            turn: Color::WHITE,
            opponent: Color::BLACK,
        }
    }
}

impl State {
    pub fn set_en_passant(&mut self, square: Square) {
        self.en_passant = Some(square)
    }

    /// increments after each move
    /// resets after each pawn or capture move
    /// if reaches 100, game is a draw
    pub fn increment_half_move(&mut self) {
        self.half_moves += 1;
    }

    pub fn reset_half_move(&mut self) {
        self.half_moves = 0;
    }

    /// Increments after each black move
    pub fn increment_full_move(&mut self) {
        self.full_moves += 1;
    }

    pub fn restore_full_move(&mut self) {
        self.full_moves -= 1;
    }

    pub fn change_turn(&mut self) {
        self.opponent = self.turn;
        self.turn ^= 1;
    }

    pub fn remove_castling_rights(&mut self, color: usize, side: usize) {
        self.castling.set_zero((color | (side << 1)) as u8);
    }
}
