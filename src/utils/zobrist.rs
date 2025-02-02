use std::ops::{BitXor, BitXorAssign};

use crate::{
    board::Board,
    types::{piece::Color, square::Square, state::CastlingSide},
};

use super::polyglot_array::POLYGLOT_RAND_ARRAY;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ZobristHash(pub u64);

impl ZobristHash {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get_value(&self) -> u64 {
        self.0
    }
}

impl BitXor for ZobristHash {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for ZobristHash {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0
    }
}

pub struct ZobristHasher {
    array: [ZobristHash; 781],
}

impl ZobristHasher {
    pub fn load() -> Self {
        Self { array: POLYGLOT_RAND_ARRAY }
    }

    fn hash_board(&self, board: &Board) -> ZobristHash {
        let mut zobrist_hash = ZobristHash::new(0_u64);

        for (color, pieces) in board.pieces.iter().enumerate() {
            for (piece, piece_board) in pieces.iter().enumerate() {
                for square in piece_board.get_ones() {
                    zobrist_hash ^= self.hash_piece_at_square(&piece, &color, &square)
                }
            }
        }

        zobrist_hash
    }

    pub fn hash_specific_castling(&self, color: usize, side: CastlingSide) -> ZobristHash {
        if color == Color::WHITE && side == CastlingSide::Short {
            return self.array[768];
        }
        if color == Color::WHITE && side == CastlingSide::Long {
            return self.array[769];
        }
        if color == Color::BLACK && side == CastlingSide::Short {
            return self.array[770];
        }
        self.array[771]
    }

    pub fn hash_castling(&self, board: &Board) -> ZobristHash {
        let mut zobrist_hash = ZobristHash::new(0);

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

    pub fn hash_en_passant(&self, board: &Board) -> ZobristHash {
        if let Some(square) = board.state.en_passant {
            return self.array[772 + square.get_file() as usize];
        }
        ZobristHash::new(0)
    }

    pub fn hash_turn(&self, board: &Board) -> ZobristHash {
        if board.state.turn == Color::WHITE {
            self.array[780]
        } else {
            ZobristHash::new(0)
        }
    }

    pub fn hash_everyting(&self, board: &Board) -> ZobristHash {
        self.hash_board(board) ^ self.hash_castling(board) ^ self.hash_en_passant(board) ^ self.hash_turn(board)
    }

    pub fn hash_piece_at_square(&self, piece: &usize, color: &usize, square: &Square) -> ZobristHash {
        self.array[color * 384 + piece * 64 + square.as_usize()]
    }
}
