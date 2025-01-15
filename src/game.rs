use crate::board::Board;
use crate::piece::Color;

pub struct Game {
    pub board: Board,
    pub turn: Color,
    pub half_moves: u8,
    pub full_moves: u8,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            turn: Color::W,
            half_moves: 0,
            full_moves: 0,
        }
    }
}