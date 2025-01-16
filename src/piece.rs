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
        match self.piece {
            PieceType::P => {
                if self.color == Color::W {
                    write!(f, "P")
                } else {
                    write!(f, "p")
                }
            }
            PieceType::R => {
                if self.color == Color::W {
                    write!(f, "R")
                } else {
                    write!(f, "r")
                }
            }
            PieceType::N => {
                if self.color == Color::W {
                    write!(f, "N")
                } else {
                    write!(f, "n")
                }
            }
            PieceType::B => {
                if self.color == Color::W {
                    write!(f, "B")
                } else {
                    write!(f, "b")
                }
            }
            PieceType::Q => {
                if self.color == Color::W {
                    write!(f, "Q")
                } else {
                    write!(f, "q")
                }
            }
            PieceType::K => {
                if self.color == Color::W {
                    write!(f, "K")
                } else {
                    write!(f, "k")
                }
            }
        }
    }
}
