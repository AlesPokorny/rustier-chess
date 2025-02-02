use rand::Rng;

use crate::{moves::moves_utils::Move, types::bitboard::BitBoard};

struct Bot;

impl Bot {
    fn make_random_move(moves: Vec<(Move, BitBoard)>) -> (Move, BitBoard) {
        let mut rng = rand::rng();
        let i = rng.random_range(0..moves.len());

        moves.into_iter().nth(i).unwrap()
    }
}
