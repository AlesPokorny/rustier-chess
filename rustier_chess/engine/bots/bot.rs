use std::collections::HashMap;

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

struct Bot {
    evaluation_cache: HashMap<ZobristHash, f32>,
    piece_values: [f32; 6],
    max_depth: u8,
}

impl Bot {
    fn new(max_depth: u8) -> Self {
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

    fn evaluate(&mut self, board: &Board) -> f32 {
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
            // TODO: Impl quiesce
            // return quiesce( alpha, beta );
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
    fn test_evaluate() {
        let board = Board::default();
        let mut bot = Bot::new(0);

        assert_eq!(bot.evaluation_cache.len(), 0);

        assert_eq!(bot.evaluate(&board), 0.);

        assert_eq!(bot.evaluation_cache.len(), 1);
        bot.evaluate(&board);
        assert_eq!(bot.evaluation_cache.len(), 1);
    }
}
