use std::fmt::Display;

#[derive(PartialEq, Eq)]
pub enum Color {
    W,
    B,
}

#[derive(PartialEq, Eq)]
pub enum PieceType {
    P,
    R,
    N,
    B,
    Q,
    K,
}

pub struct Piece {
    pub piece: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(piece: PieceType, color: Color) -> Self {
        Self { piece, color }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.color {
            Color::W => match self.piece {
                PieceType::P => write!(f, "P"),
                PieceType::R => write!(f, "R"),
                PieceType::N => write!(f, "N"),
                PieceType::B => write!(f, "B"),
                PieceType::Q => write!(f, "Q"),
                PieceType::K => write!(f, "K"),
            },
            Color::B => match self.piece {
                PieceType::P => write!(f, "p"),
                PieceType::R => write!(f, "r"),
                PieceType::N => write!(f, "n"),
                PieceType::B => write!(f, "b"),
                PieceType::Q => write!(f, "q"),
                PieceType::K => write!(f, "k"),
            },
        }
    }
}
