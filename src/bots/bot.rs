use std::{
    collections::HashMap,
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use rand::Rng;

use crate::{
    board::Board,
    moves::{move_mask_gen::MoveGenMasks, moves_utils::Move},
    types::{bitboard::BitBoard, piece::PIECE_VALUES_SETTING},
    utils::zobrist::{ZobristHash, ZobristHasher},
};

use crate::game::UCI_STOP;

use super::{pesto::PeSTO, time_control::TimeControl};

const MIN_VALUE: i32 = -100000;
const MAX_VALUE: i32 = 100000;

pub struct Bot {
    evaluation_cache: HashMap<ZobristHash, i32>,
    piece_values: [i32; 6],
    max_depth: u8,
    pesto: PeSTO,
    time_control: TimeControl,
}

impl Bot {
    pub fn with_depth(max_depth: u8, time_control: TimeControl) -> Self {
        let mut piece_values = [0; 6];
        for (piece, value) in PIECE_VALUES_SETTING {
            piece_values[piece] = value;
        }
        Self {
            evaluation_cache: HashMap::with_capacity(1000),
            piece_values,
            max_depth,
            pesto: PeSTO::default(),
            time_control,
        }
    }

    pub fn set_depth(&mut self, max_depth: u8) {
        self.max_depth = max_depth
    }

    pub fn set_time_control(&mut self, time_control: TimeControl) {
        self.time_control = time_control
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
                return MIN_VALUE;
            } else {
                return 0;
            }
        }
        eval_value += self.pesto.calculate_score(board);
        eval_value += n_legal_moves;
        self.evaluation_cache.insert(board.zobrist, eval_value);

        eval_value
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
    ) -> (i32, u64) {
        if UCI_STOP.load(Ordering::Relaxed) {
            return (0, 0);
        }
        let mut best_value = self.evaluate_position(board, move_gen_masks, hasher);

        if best_value >= beta {
            return (best_value, 1);
        }

        let mut nodes_checked = 0;

        if alpha < best_value {
            alpha = best_value;
        }

        let capture_moves = board.get_capture_moves(move_gen_masks, hasher);

        for (_, new_board) in capture_moves {
            let (opponent_score, nodes) =
                self.quiescence(-beta, -alpha, &new_board, move_gen_masks, hasher);
            let score = -opponent_score;
            nodes_checked += nodes;

            if score >= beta {
                return (score, nodes_checked);
            }
            if score > best_value {
                best_value = score;
            }

            if score > alpha {
                alpha = score
            }
        }

        (best_value, nodes_checked)
    }

    fn alpha_beta(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
        mut alpha: i32,
        beta: i32,
        depth: u8,
    ) -> (i32, u64) {
        // TODO: Figure out a better way to stop instead of returning 0
        if UCI_STOP.load(Ordering::Relaxed) {
            return (0, 0);
        }
        if depth == 0 {
            return self.quiescence(alpha, beta, board, move_gen_masks, hasher);
        }
        let mut nodes_checked = 0;
        for (_, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            let (opponent_score, nodes) =
                self.alpha_beta(&new_board, move_gen_masks, hasher, -beta, -alpha, depth - 1);
            let score = -opponent_score;
            nodes_checked += nodes;

            if score >= beta {
                return (beta, nodes_checked);
            }
            if score > alpha {
                alpha = score;
            }
        }

        (alpha, nodes_checked)
    }

    pub fn get_best_move(
        &mut self,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> (Move, Board) {
        let move_start = Instant::now();
        let move_max_time = self.time_control.get_move_time(&board.state.turn) as u128;
        let time_thread = thread::spawn(move || loop {
            if move_start.elapsed().as_millis() > move_max_time {
                UCI_STOP.store(true, Ordering::Relaxed);
                return;
            }
            if UCI_STOP.load(Ordering::Relaxed) {
                return;
            }
            thread::sleep(Duration::from_millis(300));
        });

        let mut results: Vec<(Move, Board)> = Vec::with_capacity(self.max_depth as usize);

        for depth in 1..=self.max_depth {
            let best_move = self.get_best_move_for_depth(depth, board, move_gen_masks, hasher);
            if UCI_STOP.load(Ordering::Relaxed) {
                break;
            }
            results.push(best_move);
        }
        UCI_STOP.store(true, Ordering::Relaxed);
        time_thread.join().unwrap();
        results.into_iter().last().unwrap()
    }

    pub fn get_best_move_for_depth(
        &mut self,
        depth: u8,
        board: &Board,
        move_gen_masks: &MoveGenMasks,
        hasher: &ZobristHasher,
    ) -> (Move, Board) {
        let mut nodes_checked = 0;
        let mut best_move: (Move, Board) = (Move::new(), *board);
        let mut alpha = MIN_VALUE;
        let beta = MAX_VALUE;

        let start = Instant::now();

        for (new_move, new_board) in board.get_legal_moves(move_gen_masks, hasher) {
            if UCI_STOP.load(Ordering::Relaxed) {
                break;
            }
            let (opponent_score, nodes) =
                self.alpha_beta(&new_board, move_gen_masks, hasher, -beta, -alpha, depth);
            let score = -opponent_score;
            if score > alpha {
                alpha = score;
                best_move = (new_move, new_board);
            }
            nodes_checked += nodes;
        }
        let elapsed = start.elapsed().as_micros();

        println!("info depth {} seldepth {}", depth, self.max_depth);
        println!(
            "info score cp {}  depth {} nodes {}",
            alpha, depth, nodes_checked
        );
        println!("info nps {}", (nodes_checked as u128 * 1_000_000) / elapsed);
        println!("Checked {} nodes", nodes_checked);
        best_move
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
            time_control: TimeControl::max(),
        }
    }
}

// #[cfg(test)]
// mod test_bot_evaluation {
//     use super::*;

//     #[test]
//     fn test_take_the_rook() {
//         let move_gen_masks = MoveGenMasks::load();
//         let hasher = ZobristHasher::load();
//         let mut board = Board::from_fen("8/8/5K2/5R2/5r2/8/5k2/8 w - - 0 1", &hasher).unwrap();
//         // let mut board = Board::from_fen("8/8/5K2/8/5R2/8/8/4k3 w - - 1 3", &hasher).unwrap();
//         let mut bot = Bot::with_depth(2, TimeControl::max());

//         // let (a, b) = bot.get_best_move(&board, &move_gen_masks, &hasher);

//         // println!("{}", b);
//         for _ in 0..4 {
//             let (best_move, new_board) = bot.get_best_move(&board, &move_gen_masks, &hasher);
//             board = new_board;
//             println!("{}", board);
//             println!("{}", board.get_fen());
//         }

//         // let (best_move, _) = bot.get_best_move(&board, &move_gen_masks, &hasher);
//         // assert_eq!(best_move.to_long_string(), "f5f4");
//     }

//     #[test]
//     fn test_a() {
//         for i in 0..64 {
//             println!("{}", i ^ 56);
//         }
//     }
// }
