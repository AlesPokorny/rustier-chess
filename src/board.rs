use std::error::Error;
use std::fmt::{self};
use std::str::FromStr;

use crate::{
    bitboard::BitBoard,
    piece::{Color, Piece, Pieces},
    square::Square,
    state::{Castling, State},
};

pub struct Board {
    pub colors: [BitBoard; 2],
    pub pieces: [[BitBoard; 6]; 2],
    pub all_pieces: BitBoard,
    pub state: State,
}

impl Board {
    pub fn get_piece_on_square(&self, square: &Square) -> Option<Piece> {
        let mut piece_color: usize = 3;
        for (i, color) in self.colors.iter().enumerate() {
            if color.read_square(square) {
                piece_color = i;
                break;
            }
        }

        if piece_color == 3 {
            return None;
        }

        let mut piece_type = 9_usize;

        for (i, piece_bb) in self.pieces[piece_color].iter().enumerate() {
            if piece_bb.read_square(square) {
                piece_type = i;
                break;
            }
        }
        if piece_type == 9 {
            panic!("Found piece color but not piece type");
        }

        Some(Piece::new(piece_type, piece_color))
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Board {
            colors: [BitBoard::zeros(), BitBoard::zeros()],
            pieces: [
                [
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                ],
                [
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                    BitBoard::zeros(),
                ],
            ],
            all_pieces: BitBoard::zeros(),
            state: State::default(),
        }
    }

    pub fn check_en_passant(&self, square: &Square) -> bool {
        self.state.en_passant.is_some_and(|x| &x == square)
    }

    pub fn from_fen(fen: &str) -> Result<Self, Box<dyn Error>> {
        let fen_parts: Vec<&str> = fen.trim().split(" ").collect();

        if fen_parts.len() != 6 {
            return Err("Incorrect fen string")?;
        }

        let board_string = fen_parts[0];
        let board_string_parts: Vec<&str> = board_string
            .split("/")
            .collect::<Vec<&str>>()
            .into_iter()
            .rev()
            .collect();
        if board_string_parts.len() != 8 {
            return Err("Invorrect fen string")?;
        }

        let mut colors = [BitBoard::zeros(), BitBoard::zeros()];
        let mut pieces = [
            [
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
            ],
            [
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
                BitBoard::zeros(),
            ],
        ];

        for (rank, rank_str) in board_string_parts.iter().enumerate() {
            let mut file = 0_usize;
            for fen_char in rank_str.chars() {
                if (rank > 7) | (file > 7) {
                    panic!("Invalid fen");
                }

                let fen_char_digit = fen_char as usize;

                if (48..=56).contains(&fen_char_digit) {
                    let n_empty_spaces = fen_char_digit - '0' as usize;
                    file += n_empty_spaces
                } else {
                    let piece_color = if fen_char.is_lowercase() {
                        Color::BLACK
                    } else {
                        Color::WHITE
                    };

                    let piece_kind = match fen_char.to_ascii_lowercase() {
                        'p' => Pieces::PAWN,
                        'r' => Pieces::ROOK,
                        'n' => Pieces::KNIGHT,
                        'b' => Pieces::BISHOP,
                        'k' => Pieces::KING,
                        'q' => Pieces::QUEEN,
                        _ => panic!("Invalid fen char"),
                    };
                    let piece_square = Square::new((rank * 8 + file) as u8);
                    pieces[piece_color][piece_kind].set_one(&piece_square);
                    colors[piece_color].set_one(&piece_square);
                    file += 1;
                }
            }
        }

        let all_pieces = colors[0] | colors[1];
        let turn = if fen_parts[1] == "w" {
            Color::WHITE
        } else {
            Color::BLACK
        };

        let mut castling = Castling::new();
        for castling_char in fen_parts[2].chars() {
            match castling_char {
                'Q' => castling.set_white_long(),
                'K' => castling.set_white_short(),
                'q' => castling.set_black_long(),
                'k' => castling.set_black_short(),
                _ => return Err("Invalid castling char")?,
            }
        }

        let en_passant = if fen_parts[3] == "-" {
            None
        } else {
            match Square::from_str(fen_parts[3]) {
                Ok(square) => Some(square),
                Err(_) => return Err("Error parsing en passant square")?,
            }
        };

        let half_moves = match fen_parts[4].parse::<u8>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid half move string")?,
        };
        let full_moves = match fen_parts[5].parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid full move string")?,
        };

        let state = State {
            castling,
            en_passant,
            half_moves,
            full_moves,
            turn,
        };

        let board = Self {
            colors,
            pieces,
            all_pieces,
            state,
        };

        Ok(board)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..8_u8 {
            write!(f, "\n    -------------------------------")?;
            write!(f, "\n {} |", 8 - row)?;
            let row_i = 56 - row * 8;
            for col in 0..8_u8 {
                match self.get_piece_on_square(&Square::new(row_i + col)) {
                    // 63 - (row * 8 + 7 - col)
                    Some(piece) => write!(f, " {} |", piece)?,
                    None => write!(f, "   |")?,
                }
            }
        }

        writeln!(f, "\n    -------------------------------")?;
        write!(f, "     A   B   C   D   E   F   G   H")?;

        Ok(())
    }
}
