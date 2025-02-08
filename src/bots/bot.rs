use core::f32;
use std::{collections::HashMap, sync::atomic::Ordering};

use rand::Rng;

use crate::{
    board::Board,
    moves::{move_mask_gen::MoveGenMasks, moves_utils::Move},
    types::{
        bitboard::BitBoard,
        piece::{Color, PIECE_VALUES_SETTING},
    },
    utils::zobrist::{ZobristHash, ZobristHasher},
};

use crate::game::UCI_STOP;

pub struct Bot {
    evaluation_cache: HashMap<ZobristHash, f32>,
    piece_values: [f32; 6],
    max_depth: u8,
}

impl Bot {
    pub fn new(max_depth: u8) -> Self {
        let mut piece_values = [0.0; 6];
        for (piece, value) in PIECE_VALUES_SETTING {
            piece_values[piece] = value;
        }
        Self {
            evaluation_cache: HashMap::with_capacity(1000),
            piece_values,
            max_depth,
        }
    }

    fn make_random_move(moves: Vec<(Move, BitBoard)>) -> (Move, BitBoard) {
        let mut rng = rand::rng();
        let i = rng.random_range(0..moves.len());

        moves.into_iter().nth(i).unwrap()
    }

    fn evaluate_position(&mut self, board: &Board) -> f32 {
        if let Some(eval_value) = self.evaluation_cache.get(&board.zobrist) {
            return *eval_value;
        }
        let mut eval_value = 0_f32;
        eval_value += self.get_piece_values(board);
        self.evaluation_cache.insert(board.zobrist, eval_value);

        eval_value
    }

    fn get_piece_values(&self, board: &Board) -> f32 {
        let values: Vec<f32> = board
            .pieces
            .iter()
            .map(|color| {
                color
                    .iter()
                    .enumerate()
                    .map(|(piece, bb)| bb.get_ones().len() as f32 * self.piece_values[piece])
                    .sum::<f32>()
            })
            .collect();
        values[Color::WHITE] - values[Color::BLACK]
    }

    fn quiescence(
        &mut self,
        mut alpha: f32,
        beta: f32,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> f32 {
        let mut best_value = self.evaluate_position(board);

        if best_value >= beta {
            return beta;
        }

        if best_value > alpha {
            alpha = best_value;
        }

        let capture_moves = board.get_capture_moves(move_gen_masks, hasher);

        for (_, new_board) in capture_moves {
            let score = -self.quiescence(alpha, beta, &new_board, move_gen_masks, hasher);
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
        mut alpha: f32,
        beta: f32,
        depth: u8,
    ) -> f32 {
        if depth == self.max_depth {
            return self.quiescence(alpha, beta, board, move_gen_masks, hasher);
        }
        let mut best_value = f32::NEG_INFINITY;
        for (_, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            let score =
                -self.alpha_beta(&new_board, move_gen_masks, hasher, -beta, -alpha, depth + 1);
            if score > best_value {
                best_value = score;
                if score > alpha {
                    alpha = score;
                }
                if score >= beta {
                    return best_value;
                }
            }
        }

        best_value
    }

    fn get_best_move(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> (Move, Board) {
        let best_score = f32::NEG_INFINITY;
        let mut best_move = Move::new();
        let mut best_board = Board::empty();
        for (new_move, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            if UCI_STOP.load(Ordering::Relaxed) {
                break;
            }
            let score = self.alpha_beta(
                &new_board,
                move_gen_masks,
                hasher,
                f32::NEG_INFINITY,
                f32::INFINITY,
                1,
            );
            if score > best_score {
                best_move = new_move;
                best_board = new_board;
            }
        }
        (best_move, best_board)
    }
}

#[cfg(test)]
mod test_bot_evaluation {
    use crate::types::{piece::Pieces, square::Square};

    use super::*;

    #[test]
    fn test_count_piece_values() {
        let mut board = Board::default();
        let bot = Bot::new(0);

        assert_eq!(bot.get_piece_values(&board), 0.);

        board.pieces[Color::WHITE][Pieces::QUEEN].set_zero(&Square::new(3));
        assert_eq!(bot.get_piece_values(&board), -9.);

        board.pieces[Color::BLACK][Pieces::ROOK].set_zero(&Square::new(56));
        board.pieces[Color::BLACK][Pieces::ROOK].set_zero(&Square::new(63));

        assert_eq!(bot.get_piece_values(&board), 1.);
    }

    #[test]
    fn test_evaluate_position() {
        let board = Board::default();
        let mut bot = Bot::new(0);

        assert_eq!(bot.evaluation_cache.len(), 0);

        assert_eq!(bot.evaluate_position(&board), 0.);

        assert_eq!(bot.evaluation_cache.len(), 1);
        bot.evaluate_position(&board);
        assert_eq!(bot.evaluation_cache.len(), 1);
    }

    #[test]
    fn aaa() {
        let board = Board::default();
        let move_gen_masks = MoveGenMasks::load();
        let hasher = ZobristHasher::load();
        let mut bot = Bot::new(5);

        let (_, board) = bot.get_best_move(&board, &move_gen_masks, &hasher);
        println!("{}", board);
    }
}
