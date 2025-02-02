use crate::{board::Board, types::piece::Color};

use super::polyglot_array::POLYGLOT_RAND_ARRAY;

pub struct ZobristHasher {
    array: [u64; 781],
}

impl ZobristHasher {
    pub fn new() -> Self {
        Self { array: POLYGLOT_RAND_ARRAY }
    }

    fn hash_board(&self, board: &Board) -> u64 {
        let mut zobrist_hash = 0_u64;

        for (color, pieces) in board.pieces.iter().enumerate() {
            for (piece_type, piece_board) in pieces.iter().enumerate() {
                for square in piece_board.get_ones() {
                    let hash_index = color * 384 + piece_type * 64 + square.as_usize();
                    zobrist_hash ^= self.array[hash_index];
                }
            }
        }

        zobrist_hash
    }

    fn hash_castling(&self, board: &Board) -> u64 {
        let mut zobrist_hash = 0_u64;

        if board.state.castling.white_short() {
            zobrist_hash ^= self.array[768];
        }
        if board.state.castling.white_long() {
            zobrist_hash ^= self.array[769];
        }
        if board.state.castling.black_short() {
            zobrist_hash ^= self.array[770];
        }
        if board.state.castling.black_long() {
            zobrist_hash ^= self.array[771];
        }

        zobrist_hash
    }

    fn hash_en_passant(&self, board: &Board) -> u64 {
        if let Some(square) = board.state.en_passant {
            return self.array[772 + square.get_file() as usize]
        }
        0
    }

    fn hash_turn(&self, board: &Board) -> u64 {
        if board.state.turn == Color::WHITE {
            self.array[780]
        } else {
            0
        }
    }
    
    fn hash_everyting(&self, board: &Board) -> u64 {
        self.hash_board(board) ^ self.hash_castling(board) ^ self.hash_en_passant(board) ^ self.hash_turn(board)
    }
}
