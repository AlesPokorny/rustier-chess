use std::error::Error;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::moves::move_mask_gen::MoveGenMasks;
use crate::moves::moves_calculation::{get_all_moves, is_square_in_check};
use crate::moves::moves_utils::Move;
use crate::types::state::{
    BLACK_LONG_ROOK_STARTING_MASK, BLACK_SHORT_ROOK_STARTING_MASK, WHITE_LONG_ROOK_STARTING_MASK,
    WHITE_SHORT_ROOK_STARTING_MASK,
};
use crate::types::{
    bitboard::BitBoard,
    piece::{Color, Piece, Pieces},
    square::Square,
    state::{Castling, State},
};
use crate::utils::zobrist::{ZobristHash, ZobristHasher};

#[derive(Clone, Copy)]
pub struct Board {
    pub colors: [BitBoard; 2],
    pub pieces: [[BitBoard; 6]; 2],
    pub all_pieces: BitBoard,
    pub state: State,
    pub zobrist: ZobristHash,
}

impl Board {
    pub fn clear_piece(&mut self, square: &Square, piece: usize, color: usize) {
        self.colors[color].set_zero(square);
        self.pieces[color][piece].set_zero(square);
    }

    pub fn move_piece(
        &mut self,
        origin: &Square,
        destination: &Square,
        piece: usize,
        color: usize,
    ) {
        self.colors[color].set_one(destination);
        self.pieces[color][piece].set_one(destination);
        self.clear_piece(origin, piece, color);
    }

    pub fn sync_all_pieces(&mut self) {
        self.all_pieces = self.colors[0] | self.colors[1];
    }

    pub fn get_capture_moves(
        &self,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> Vec<(Move, Board)> {
        self.get_legal_moves(move_gen_masks, hasher)
            .into_iter()
            .filter(|(legal_move, _)| {
                self.colors[self.state.turn].read_square(&legal_move.get_destination())
            })
            .collect()
    }

    pub fn is_check(&self, move_gen_masks: &MoveGenMasks) -> bool {
        let king_square = self.pieces[self.state.turn][Pieces::KING].get_one();
        is_square_in_check(&king_square, self, move_gen_masks)
    }

    pub fn get_legal_moves(
        &self,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> Vec<(Move, Board)> {
        let all_moves: Vec<(Move, Board)> = get_all_moves(self, move_gen_masks)
            .into_iter()
            .filter_map(|the_move| {
                let mut new_board = self.try_move(&the_move, hasher);
                if !is_square_in_check(
                    &new_board.pieces[new_board.state.turn][Pieces::KING].get_one(),
                    &new_board,
                    move_gen_masks,
                ) {
                    new_board.state.change_turn();
                    return Some((the_move, new_board));
                }
                None
            })
            .collect();

        all_moves
    }

    pub fn try_move(&self, the_move: &Move, hasher: &ZobristHasher) -> Board {
        let origin = the_move.get_origin();
        let destination = the_move.get_destination();
        let mut move_hash = ZobristHash::new(0_u64);

        let is_capture = self.colors[self.state.opponent].read_square(&destination);

        let mut new_board = *self;

        new_board.state.increment_half_move();

        let mut moving_piece_type = 10;

        new_board.colors[new_board.state.turn].set_zero(&origin);
        new_board.colors[new_board.state.turn].set_one(&destination);
        for (piece_type, piece_bitboard) in new_board.pieces[new_board.state.turn]
            .iter_mut()
            .enumerate()
        {
            if piece_bitboard.read_square(&origin) {
                move_hash ^=
                    hasher.hash_piece_at_square(&piece_type, &new_board.state.turn, &origin);
                move_hash ^=
                    hasher.hash_piece_at_square(&piece_type, &new_board.state.turn, &destination);
                moving_piece_type = piece_type;
                piece_bitboard.set_zero(&origin);
                piece_bitboard.set_one(&destination);
                if piece_type == Pieces::PAWN {
                    new_board.state.reset_half_move();
                }
                break;
            }
        }

        if is_capture {
            new_board.colors[new_board.state.opponent].set_zero(&destination);
            for (piece_type, piece_bitboard) in new_board.pieces[new_board.state.opponent]
                .iter_mut()
                .enumerate()
            {
                if piece_bitboard.read_square(&destination) {
                    piece_bitboard.set_zero(&destination);
                    move_hash ^= hasher.hash_piece_at_square(
                        &piece_type,
                        &new_board.state.opponent,
                        &destination,
                    );
                    break;
                }
            }
            new_board.state.reset_half_move();
        } else if let Some(en_passant_square) = new_board.state.en_passant {
            // is en_passant capture
            if destination == en_passant_square && moving_piece_type == Pieces::PAWN {
                let capture_square = Square::new(origin.get_rank() * 8 + destination.get_file());
                new_board.clear_piece(&capture_square, Pieces::PAWN, new_board.state.opponent);
                move_hash ^= hasher.hash_piece_at_square(
                    &Pieces::PAWN,
                    &new_board.state.opponent,
                    &capture_square,
                );
            }
        }

        move_hash ^= hasher.hash_en_passant(&new_board, new_board.state.turn);
        new_board.state.en_passant = None;

        match the_move.special_move() {
            0 => (),
            1 => {
                // 1 promotion
                new_board.clear_piece(&origin, Pieces::PAWN, new_board.state.turn);
                new_board.clear_piece(&destination, Pieces::PAWN, new_board.state.turn);
                move_hash ^=
                    hasher.hash_piece_at_square(&Pieces::PAWN, &new_board.state.turn, &destination);
                move_hash ^=
                    hasher.hash_piece_at_square(&Pieces::PAWN, &new_board.state.turn, &destination);
                new_board.pieces[new_board.state.turn][the_move.get_promotion_piece()]
                    .set_one(&destination);
                new_board.colors[new_board.state.turn].set_one(&destination);
            }
            2 => {
                // 2 en passant
                let en_passant_rank = if self.state.turn == Color::WHITE {
                    origin.get_rank().add(1)
                } else {
                    origin.get_rank().sub(1)
                };
                let en_passant_square = Square::new(en_passant_rank * 8 + destination.get_file());
                new_board.state.en_passant = Some(en_passant_square);
                move_hash ^= hasher.hash_en_passant(&new_board, new_board.state.opponent);
            }
            3 => {
                // 3 castling
                let rook_origin_file: u8;
                let rook_destination_file: u8;
                if destination.get_file() == 2 {
                    // long
                    rook_origin_file = 0;
                    rook_destination_file = 3;
                } else {
                    // short
                    rook_origin_file = 7;
                    rook_destination_file = 5;
                }
                let rank = origin.get_rank() * 8;
                let rook_origin = Square::new(rank + rook_origin_file);
                let rook_destination = Square::new(rank + rook_destination_file);
                new_board.move_piece(
                    &rook_origin,
                    &rook_destination,
                    Pieces::ROOK,
                    new_board.state.turn,
                );
                move_hash ^=
                    hasher.hash_piece_at_square(&Pieces::ROOK, &new_board.state.turn, &origin);
                move_hash ^=
                    hasher.hash_piece_at_square(&Pieces::ROOK, &new_board.state.turn, &destination);
            }
            _ => panic!("Boom, invalid special move"),
        }

        if new_board.state.castling.can_someone_castle() {
            if new_board.state.castling.white_long()
                && (new_board.pieces[Color::WHITE][Pieces::ROOK] & WHITE_LONG_ROOK_STARTING_MASK)
                    .is_empty()
            {
                new_board.state.castling.remove_white_long();
                move_hash ^= hasher.hash_castling_white_long();
            }
            if new_board.state.castling.white_short()
                && (new_board.pieces[Color::WHITE][Pieces::ROOK] & WHITE_SHORT_ROOK_STARTING_MASK)
                    .is_empty()
            {
                new_board.state.castling.remove_white_short();
                move_hash ^= hasher.hash_castling_white_short();
            }
            if new_board.state.castling.black_long()
                && (new_board.pieces[Color::BLACK][Pieces::ROOK] & BLACK_LONG_ROOK_STARTING_MASK)
                    .is_empty()
            {
                new_board.state.castling.remove_black_long();
                move_hash ^= hasher.hash_castling_black_long();
            }
            if new_board.state.castling.black_short()
                && (new_board.pieces[Color::BLACK][Pieces::ROOK] & BLACK_SHORT_ROOK_STARTING_MASK)
                    .is_empty()
            {
                new_board.state.castling.remove_black_short();
                move_hash ^= hasher.hash_castling_black_short();
            }
            if new_board
                .state
                .castling
                .can_color_castle(new_board.state.turn)
                && moving_piece_type == Pieces::KING
            {
                new_board
                    .state
                    .castling
                    .remove_color_castling(new_board.state.turn);
                move_hash ^= hasher.hash_castling_color(new_board.state.turn);
                move_hash ^= hasher.hash_castling_color(new_board.state.turn);
            }
        }

        new_board.zobrist ^= move_hash;
        new_board.sync_all_pieces();
        new_board.state.increment_full_move();

        new_board
    }

    pub fn check_and_make_move(
        &mut self,
        the_move: &Move,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> Option<Board> {
        self.get_legal_moves(move_gen_masks, hasher)
            .into_iter()
            .filter_map(|(possible_move, board)| {
                if &possible_move == the_move {
                    Some(board)
                } else {
                    None
                }
            })
            .nth(0)
    }

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
            zobrist: ZobristHash::zero(),
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
        let opponent = if turn == 1 { 0 } else { 1 };

        let mut castling = Castling::new();
        for castling_char in fen_parts[2].chars() {
            match castling_char {
                'Q' => castling.set_white_long(),
                'K' => castling.set_white_short(),
                'q' => castling.set_black_long(),
                'k' => castling.set_black_short(),
                '-' => break,
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
            opponent,
        };

        let board = Self {
            colors,
            pieces,
            all_pieces,
            state,
            zobrist: ZobristHash::new(0x463b96181691fc9c), // default board hash with polyglot randoms
        };

        Ok(board)
    }

    fn get_piece_on_square(&self, square: &Square) -> Option<Piece> {
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

#[cfg(test)]
mod test_aa {
    use super::Board;

    #[test]
    fn test_aa() {
        let board = Board::from_fen("8/8/5K2/5R2/5r2/8/5k2/8 w - - 0 1").unwrap();

        println!("{}", board);
    }
}
