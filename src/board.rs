use std::error::Error;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use crate::moves::move_mask_gen::MoveGenMasks;
use crate::moves::moves_calculation::{get_all_moves, is_square_in_check};
use crate::moves::moves_utils::{Move, UnmakeMoveHelper};
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

#[derive(Clone)]
pub struct Board {
    pub colors: [BitBoard; 2],
    pub pieces: [[BitBoard; 6]; 2],
    pub all_pieces: BitBoard,
    pub state: State,
    pub zobrist: ZobristHash,
    pub position_history: Vec<ZobristHash>,
}

impl Board {
    pub fn new(hasher: &ZobristHasher) -> Self {
        Self::from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            hasher,
        )
        .unwrap()
    }

    pub fn check_repeat_draw(&self) -> bool {
        let pos_history_len = self.position_history.len();
        if self.state.half_moves < 6 {
            return false;
        }
        self.position_history[(pos_history_len - self.state.half_moves as usize)..pos_history_len]
            .iter()
            .filter(|history| history == &&self.zobrist)
            .collect::<Vec<&ZobristHash>>()
            .len()
            >= 3
    }

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
        &mut self,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> Vec<Move> {
        self.get_legal_moves(move_gen_masks, hasher)
            .into_iter()
            .filter(|legal_move| {
                self.colors[self.state.turn].read_square(&legal_move.get_destination())
            })
            .collect()
    }

    pub fn is_check(&self, move_gen_masks: &MoveGenMasks) -> bool {
        let king_square = self.pieces[self.state.turn][Pieces::KING].get_one();
        is_square_in_check(&king_square, self, move_gen_masks)
    }

    pub fn get_legal_moves(
        &mut self,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> Vec<Move> {
        let all_moves: Vec<Move> = get_all_moves(self, move_gen_masks)
            .into_iter()
            .filter_map(|the_move| {
                let unmake_move_helper = self.make_move(&the_move, hasher);

                // this turn changing is dumb. Can I make it better?
                self.state.change_turn();
                if !is_square_in_check(
                    &self.pieces[self.state.turn][Pieces::KING].get_one(),
                    self,
                    move_gen_masks,
                ) {
                    self.state.change_turn();
                    self.unmake_move(unmake_move_helper);
                    return Some(the_move);
                }
                self.state.change_turn();
                self.unmake_move(unmake_move_helper);
                None
            })
            .collect();

        all_moves
    }

    pub fn make_move(&mut self, the_move: &Move, hasher: &ZobristHasher) -> UnmakeMoveHelper {
        let origin = the_move.get_origin();
        let destination = the_move.get_destination();
        let mut capture: Option<usize> = None;
        let prev_en_passant = self.state.en_passant;
        let prev_halfmove = self.state.half_moves;
        let prev_hash = self.zobrist;
        let prev_castling: Castling = self.state.castling;

        let move_bb =
            BitBoard::zeros_with_one_bit(&origin) ^ BitBoard::zeros_with_one_bit(&destination);

        let is_capture = self.colors[self.state.opponent].read_square(&destination);

        self.state.increment_half_move();

        let mut moving_piece_type = 10;

        if self.state.en_passant.is_some() {
            self.zobrist ^= hasher.hash_en_passant(self, self.state.turn);
        }

        self.colors[self.state.turn] ^= move_bb;
        for (piece_type, piece_bitboard) in self.pieces[self.state.turn].iter_mut().enumerate() {
            if piece_bitboard.read_square(&origin) {
                self.zobrist ^= hasher.hash_piece_at_square(&piece_type, &self.state.turn, &origin);
                self.zobrist ^=
                    hasher.hash_piece_at_square(&piece_type, &self.state.turn, &destination);
                moving_piece_type = piece_type;
                *piece_bitboard ^= move_bb;
                if piece_type == Pieces::PAWN {
                    self.state.reset_half_move();
                }
                break;
            }
        }

        if is_capture {
            self.colors[self.state.opponent].set_zero(&destination);
            for (piece_type, piece_bitboard) in
                self.pieces[self.state.opponent].iter_mut().enumerate()
            {
                if piece_bitboard.read_square(&destination) {
                    capture = Some(piece_type);
                    *piece_bitboard &= !move_bb;
                    self.zobrist ^= hasher.hash_piece_at_square(
                        &piece_type,
                        &self.state.opponent,
                        &destination,
                    );
                    break;
                }
            }
            self.state.reset_half_move();
        } else if let Some(en_passant_square) = self.state.en_passant {
            // is en_passant capture
            if destination == en_passant_square && moving_piece_type == Pieces::PAWN {
                let capture_square = Square::new(origin.get_rank() * 8 + destination.get_file());
                self.clear_piece(&capture_square, Pieces::PAWN, self.state.opponent);
                self.zobrist ^= hasher.hash_piece_at_square(
                    &Pieces::PAWN,
                    &self.state.opponent,
                    &capture_square,
                );
            }
        }

        self.state.en_passant = None;

        match the_move.special_move() {
            0 => (),
            1 => {
                // 1 promotion
                self.pieces[self.state.turn][Pieces::PAWN] &= !move_bb;
                self.zobrist ^=
                    hasher.hash_piece_at_square(&Pieces::PAWN, &self.state.turn, &destination);
                let promotion_piece = the_move.get_promotion_piece();
                self.pieces[self.state.turn][promotion_piece].set_one(&destination);
                self.zobrist ^=
                    hasher.hash_piece_at_square(&promotion_piece, &self.state.turn, &destination);
            }
            2 => {
                // 2 en passant
                let en_passant_rank = if self.state.turn == Color::WHITE {
                    origin.get_rank().add(1)
                } else {
                    origin.get_rank().sub(1)
                };
                let en_passant_square = Square::new(en_passant_rank * 8 + destination.get_file());
                self.state.en_passant = Some(en_passant_square);
                self.zobrist ^= hasher.hash_en_passant(self, self.state.opponent);
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
                self.move_piece(
                    &rook_origin,
                    &rook_destination,
                    Pieces::ROOK,
                    self.state.turn,
                );
                self.zobrist ^=
                    hasher.hash_piece_at_square(&Pieces::ROOK, &self.state.turn, &rook_origin);
                self.zobrist ^=
                    hasher.hash_piece_at_square(&Pieces::ROOK, &self.state.turn, &rook_destination);
                self.state.castling.remove_color_castling(self.state.turn);
                self.state.reset_half_move();
            }
            _ => panic!("Boom, invalid special move"),
        }

        if self.state.castling.can_someone_castle() {
            if self.state.castling.white_long()
                && (self.pieces[Color::WHITE][Pieces::ROOK] & WHITE_LONG_ROOK_STARTING_MASK)
                    .is_empty()
            {
                self.state.castling.remove_white_long();
                self.zobrist ^= hasher.hash_castling_white_long();
            }
            if self.state.castling.white_short()
                && (self.pieces[Color::WHITE][Pieces::ROOK] & WHITE_SHORT_ROOK_STARTING_MASK)
                    .is_empty()
            {
                self.state.castling.remove_white_short();
                self.zobrist ^= hasher.hash_castling_white_short();
            }
            if self.state.castling.black_long()
                && (self.pieces[Color::BLACK][Pieces::ROOK] & BLACK_LONG_ROOK_STARTING_MASK)
                    .is_empty()
            {
                self.state.castling.remove_black_long();
                self.zobrist ^= hasher.hash_castling_black_long();
            }
            if self.state.castling.black_short()
                && (self.pieces[Color::BLACK][Pieces::ROOK] & BLACK_SHORT_ROOK_STARTING_MASK)
                    .is_empty()
            {
                self.state.castling.remove_black_short();
                self.zobrist ^= hasher.hash_castling_black_short();
            }
            if self.state.castling.can_color_castle(self.state.turn)
                && moving_piece_type == Pieces::KING
            {
                self.state.castling.remove_color_castling(self.state.turn);
                self.zobrist ^= hasher.hash_castling_color(self.state.turn);
            }
        }

        self.zobrist ^= hasher.turn_hash();
        self.sync_all_pieces();
        if self.state.turn == Color::BLACK {
            self.state.increment_full_move();
        }
        self.state.change_turn();
        self.position_history.push(self.zobrist);

        UnmakeMoveHelper {
            origin,
            destination,
            move_bb,
            piece: moving_piece_type,
            capture,
            prev_en_passant,
            prev_hash,
            prev_halfmove,
            prev_castling,
            special_move: the_move.special_move() as u8,
        }
    }

    pub fn unmake_move(&mut self, helper: UnmakeMoveHelper) {
        self.state.change_turn();
        self.colors[self.state.turn] ^= helper.move_bb;
        self.pieces[self.state.turn][helper.piece] ^= helper.move_bb;

        if let Some(captured_piece) = helper.capture {
            self.colors[self.state.opponent].set_one(&helper.destination);
            self.pieces[self.state.opponent][captured_piece].set_one(&helper.destination);
        }

        if let Some(en_passant) = helper.prev_en_passant {
            if helper.destination == en_passant && helper.piece == Pieces::PAWN {
                let capture_square =
                    Square::new(helper.origin.get_rank() * 8 + helper.destination.get_file());
                self.colors[self.state.opponent].set_one(&capture_square);
                self.pieces[self.state.opponent][Pieces::PAWN].set_one(&capture_square);
            }
        }

        match helper.special_move {
            // promotion
            1 => {
                for piece_bb in self.pieces[self.state.turn].iter_mut() {
                    piece_bb.set_zero(&helper.destination)
                }
            }
            // castling
            3 => {
                let rank = helper.destination.get_rank();
                let (origin_file, destination_file) = if helper.destination.get_file() == 2 {
                    // long
                    (0, 3)
                } else {
                    // short
                    (7, 5)
                };
                let rook_origin = Square::new(rank * 8 + origin_file);
                let rook_destination = Square::new(rank * 8 + destination_file);

                self.move_piece(
                    &rook_destination,
                    &rook_origin,
                    Pieces::ROOK,
                    self.state.turn,
                );
            }
            _ => (),
        }

        if self.state.turn == Color::BLACK {
            self.state.restore_full_move();
        }
        self.state.en_passant = helper.prev_en_passant;
        self.state.castling = helper.prev_castling;
        self.state.half_moves = helper.prev_halfmove;
        self.zobrist = helper.prev_hash;
        self.state.castling = helper.prev_castling;
        self.position_history.pop();
        self.sync_all_pieces();
    }

    pub fn check_and_make_move(
        &mut self,
        the_move: &Move,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) {
        for possible_move in self.get_legal_moves(move_gen_masks, hasher) {
            if &possible_move == the_move {
                self.make_move(&possible_move, hasher);
                return;
            }
        }
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
            position_history: Vec::with_capacity(50),
        }
    }

    pub fn check_en_passant(&self, square: &Square) -> bool {
        self.state.en_passant.is_some_and(|x| &x == square)
    }

    pub fn from_fen(fen: &str, hasher: &ZobristHasher) -> Result<Self, Box<dyn Error>> {
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
        let turn = if fen_parts[1].eq_ignore_ascii_case("w") {
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

        let mut board = Self {
            colors,
            pieces,
            all_pieces,
            state,
            zobrist: ZobristHash::new(0), // default board hash with polyglot randoms
            position_history: Vec::with_capacity(50),
        };
        board.zobrist = hasher.hash_everyting(&board);
        board.position_history.push(board.zobrist);

        Ok(board)
    }

    pub fn get_fen(&self) -> String {
        let mut fen = String::new();

        for i in (0..8).rev() {
            let mut empty_squares = 0;
            for j in 0..8 {
                let square = Square::new(i * 8 + j);
                match self.get_piece_on_square(&square) {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                        }
                        fen.push_str(&piece.to_string());
                        empty_squares = 0;
                    }
                    None => empty_squares += 1,
                };
            }

            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string())
            }
            if i > 0 {
                fen.push('/');
            }
        }

        fen.push(' ');

        if self.state.turn == Color::WHITE {
            fen.push('w');
        } else {
            fen.push('b');
        }

        fen.push(' ');

        if self.state.castling.white_short() {
            fen.push('K');
        }
        if self.state.castling.white_long() {
            fen.push('Q');
        }
        if self.state.castling.black_short() {
            fen.push('k');
        }
        if self.state.castling.black_long() {
            fen.push('q');
        }
        if !self.state.castling.can_someone_castle() {
            fen.push('-');
        }

        fen.push(' ');

        match self.state.en_passant {
            Some(square) => fen.push_str(&square.to_string()),
            None => fen.push('-'),
        }

        fen.push(' ');

        fen.push_str(&self.state.half_moves.to_string());

        fen.push(' ');

        fen.push_str(&self.state.full_moves.to_string());

        fen
    }

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
mod test_board {
    use super::*;

    // #[test]
    // fn test_a() {
    //     let hasher = ZobristHasher::load();
    //     let move_gen_masks = MoveGenMasks::load();
    //     let mut board = Board::new(&hasher);

    //     for legal_move in board.get_legal_moves(&move_gen_masks, &hasher) {
    //         let umh = board.make_move(&legal_move, &hasher);
    //         for second_move in board.get_legal_moves(&move_gen_masks, &hasher) {
    //             let umh2 = board.make_move(&second_move, &hasher);
    //             println!("{}", board);
    //             board.unmake_move(umh2);
    //         }
    //         board.unmake_move(umh);

    //     }

    //     println!("{}", board);
    // }

    #[test]
    fn test_to_fen() {
        let hasher = ZobristHasher::load();
        let fens = [
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        ];

        assert_eq!(
            Board::new(&hasher).get_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        );

        for fen in fens {
            assert_eq!(Board::from_fen(fen, &hasher).unwrap().get_fen(), fen);
        }
    }
}
