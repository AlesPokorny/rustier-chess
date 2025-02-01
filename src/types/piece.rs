use std::fmt::Display;

pub struct Color;
impl Color {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;

    pub fn other_color(color: usize) -> usize {
        if color == Self::WHITE {
            Self::BLACK
        } else {
            Self::WHITE
        }
    }
}

pub struct Pieces;
impl Pieces {
    pub const QUEEN: usize = 0;
    pub const ROOK: usize = 1;
    pub const BISHOP: usize = 2;
    pub const KNIGHT: usize = 3;
    pub const PAWN: usize = 4;
    pub const KING: usize = 5;
}

pub struct Piece {
    pub piece: usize,
    pub color: usize,
}

impl Piece {
    pub fn new(piece: usize, color: usize) -> Self {
        Self { piece, color }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            0 => match self.piece {
                0 => write!(f, "Q"),
                1 => write!(f, "R"),
                2 => write!(f, "B"),
                3 => write!(f, "N"),
                4 => write!(f, "P"),
                5 => write!(f, "K"),
                _ => panic!("Unexpected piece type"),
            },
            1 => match self.piece {
                0 => write!(f, "q"),
                1 => write!(f, "r"),
                2 => write!(f, "b"),
                3 => write!(f, "n"),
                4 => write!(f, "p"),
                5 => write!(f, "k"),
                _ => panic!("Unexpected piece type"),
            },
            _ => panic!("Unexpected piece color"),
        }
    }
}
