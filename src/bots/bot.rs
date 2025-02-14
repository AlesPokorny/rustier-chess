use std::{collections::HashMap, sync::atomic::Ordering};

use rand::Rng;

use crate::{
    board::Board,
    moves::{move_mask_gen::MoveGenMasks, moves_utils::Move},
    types::{bitboard::BitBoard, piece::PIECE_VALUES_SETTING},
    utils::zobrist::{ZobristHash, ZobristHasher},
};

use crate::game::UCI_STOP;

use super::pesto::PeSTO;

const MIN_VALUE: i32 = -100000;
const MAX_VALUE: i32 = 100000;

pub struct Bot {
    evaluation_cache: HashMap<ZobristHash, i32>,
    piece_values: [i32; 6],
    max_depth: u8,
    pesto: PeSTO,
}

impl Bot {
    pub fn with_depth(max_depth: u8) -> Self {
        let mut piece_values = [0; 6];
        for (piece, value) in PIECE_VALUES_SETTING {
            piece_values[piece] = value;
        }
        Self {
            evaluation_cache: HashMap::with_capacity(1000),
            piece_values,
            max_depth,
            pesto: PeSTO::default(),
        }
    }

    pub fn set_depth(&mut self, max_depth: u8) {
        self.max_depth = max_depth
    }
    fn make_random_move(moves: Vec<(Move, BitBoard)>) -> (Move, BitBoard) {
        let mut rng = rand::rng();
        let i = rng.random_range(0..moves.len());

        moves.into_iter().nth(i).unwrap()
    }

    fn evaluate_position(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> i32 {
        if let Some(eval_value) = self.evaluation_cache.get(&board.zobrist) {
            return *eval_value;
        }
        let mut eval_value = 0;
        let n_legal_moves = self.get_number_of_moves(board, move_gen_masks, hasher);
        if n_legal_moves == 0 {
            if board.is_check(move_gen_masks) {
                return MAX_VALUE;
            } else {
                return 0;
            }
        }
        // eval_value += self.get_piece_values(board);
        eval_value += self.pesto.calculate_score(board);
        eval_value += n_legal_moves;
        self.evaluation_cache.insert(board.zobrist, eval_value);

        eval_value
    }

    fn get_piece_values(&self, board: &Board) -> i32 {
        let values: Vec<i32> = board
            .pieces
            .iter()
            .map(|color| {
                color
                    .iter()
                    .enumerate()
                    .map(|(piece, bb)| bb.get_ones().len() as i32 * self.piece_values[piece])
                    .sum::<i32>()
            })
            .collect();
        values[board.state.turn] - values[board.state.opponent]
    }

    fn get_number_of_moves(
        &self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> i32 {
        board.get_legal_moves(move_gen_masks, hasher).len() as i32
    }

    fn quiescence(
        &mut self,
        mut alpha: i32,
        beta: i32,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> i32 {
        if UCI_STOP.load(Ordering::Relaxed) {
            return 0;
        }
        let mut best_value = self.evaluate_position(board, move_gen_masks, hasher);

        if best_value >= beta {
            return best_value;
        }

        if alpha < best_value {
            alpha = best_value;
        }

        let capture_moves = board.get_capture_moves(move_gen_masks, hasher);

        for (_, new_board) in capture_moves {
            let score = -self.quiescence(-beta, -alpha, &new_board, move_gen_masks, hasher);

            if score >= beta {
                return score;
            }
            if score > best_value {
                best_value = score;
            }

            if score > alpha {
                alpha = score
            }
        }

        best_value
    }

    fn alpha_beta(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
        mut alpha: i32,
        beta: i32,
        depth: u8,
    ) -> i32 {
        // TODO: Figure out a better way to stop instead of returning 0
        if UCI_STOP.load(Ordering::Relaxed) {
            return 0;
        }
        if depth == self.max_depth {
            return self.quiescence(alpha, beta, board, move_gen_masks, hasher);
        }
        for (_, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            let score =
                -self.alpha_beta(&new_board, move_gen_masks, hasher, -beta, -alpha, depth + 1);
            if score >= beta {
                return beta;
            }
            if score > alpha {
                alpha = score;
            }
        }

        alpha
    }

    pub fn get_best_move(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> (Move, Board) {
        let mut best_score = MIN_VALUE;
        let mut best_move = Move::new();
        let mut best_board = Board::empty();
        for (new_move, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            if UCI_STOP.load(Ordering::Relaxed) {
                break;
            }
            let score =
                -self.alpha_beta(&new_board, move_gen_masks, hasher, MIN_VALUE, MAX_VALUE, 1);
            if score > best_score {
                best_score = score;
                best_move = new_move;
                best_board = new_board;
            }
        }
        (best_move, best_board)
    }
}

impl Default for Bot {
    fn default() -> Self {
        let mut piece_values = [0; 6];
        for (piece, value) in PIECE_VALUES_SETTING {
            piece_values[piece] = value;
        }
        Self {
            evaluation_cache: HashMap::with_capacity(1000),
            piece_values,
            max_depth: 5,
            pesto: PeSTO::default(),
        }
    }
}

#[cfg(test)]
mod test_bot_evaluation {
    use super::*;

    #[test]
    fn test_take_the_rook() {
        let move_gen_masks = MoveGenMasks::load();
        let hasher = ZobristHasher::load();
        let mut board = Board::from_fen("8/8/5K2/5R2/5r2/8/5k2/8 w - - 0 1", &hasher).unwrap();
        // let mut board = Board::from_fen("8/8/5K2/8/5R2/8/8/4k3 w - - 1 3", &hasher).unwrap();
        let mut bot = Bot::with_depth(2);

        // let (a, b) = bot.get_best_move(&board, &move_gen_masks, &hasher);

        // println!("{}", b);
        for _ in 0..4 {
            let (best_move, new_board) = bot.get_best_move(&board, &move_gen_masks, &hasher);
            board = new_board;
            println!("{}", board);
            println!("{}", board.get_fen());
        }

        // let (best_move, _) = bot.get_best_move(&board, &move_gen_masks, &hasher);
        // assert_eq!(best_move.to_long_string(), "f5f4");
    }

    #[test]
    fn test_a() {
        for i in 0..64 {
            println!("{}", i ^ 56);
        }
    }
}
