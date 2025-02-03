use std::{
    hash::{Hash, Hasher},
    ops::{BitXor, BitXorAssign},
};

use crate::{
    board::Board,
    types::{
        piece::{Color, Pieces},
        square::Square,
    },
};

use super::polyglot_array::POLYGLOT_RAND_ARRAY;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ZobristHash(pub u64);

impl ZobristHash {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get_value(&self) -> u64 {
        self.0
    }

    pub fn zero() -> Self {
        Self(0)
    }
}

impl Hash for ZobristHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}

impl Hasher for ZobristHash {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _bytes: &[u8]) {
        panic!("This hasher only takes u64");
    }

    fn write_u64(&mut self, i: u64) {
        self.0 = i;
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

/// all info can be found here http://hgm.nubati.net/book_format.html
pub struct ZobristHasher {
    array: [ZobristHash; 781],
}

impl ZobristHasher {
    pub fn load() -> Self {
        Self {
            array: POLYGLOT_RAND_ARRAY,
        }
    }

    pub fn hash_board(&self, board: &Board) -> ZobristHash {
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

    pub fn hash_castling_white_short(&self) -> ZobristHash {
        self.array[768]
    }

    pub fn hash_castling_white_long(&self) -> ZobristHash {
        self.array[769]
    }

    pub fn hash_castling_black_short(&self) -> ZobristHash {
        self.array[770]
    }

    pub fn hash_castling_black_long(&self) -> ZobristHash {
        self.array[771]
    }

    pub fn hash_castling_color(&self, color: usize) -> ZobristHash {
        let i = 768 + (2 * color);
        self.array[i] ^ self.array[i + 1]
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

    /// Zobrist hashes en passant only if there is a pawn around that can possibly use it
    pub fn hash_en_passant(&self, board: &Board, turn: usize) -> ZobristHash {
        if let Some(square) = board.state.en_passant {
            let mask = if turn == Color::WHITE {
                board.pieces[Color::WHITE][Pieces::PAWN].shift_up(1)
            } else {
                board.pieces[Color::BLACK][Pieces::PAWN].shift_down(1)
            };
            if (mask.shift_left(1) | mask.shift_right(1)).read_square(&square) {
                return self.array[772 + square.get_file() as usize];
            }
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
        self.hash_board(board)
            ^ self.hash_castling(board)
            ^ self.hash_en_passant(board, board.state.turn)
            ^ self.hash_turn(board)
    }

    pub fn hash_piece_at_square(
        &self,
        piece: &usize,
        color: &usize,
        square: &Square,
    ) -> ZobristHash {
        self.array[color * 384 + piece * 64 + square.as_usize()]
    }
}

#[cfg(test)]
mod test_zobrist {
    use super::*;

    #[test]
    fn test_zobrist_castling_hash() {
        let board = Board::default();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_castling(&board),
            ZobristHash::new(0x7b3a2dabd781afe9)
        )
    }

    #[test]
    fn test_zobrist_turn_hash() {
        let board = Board::default();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_turn(&board),
            ZobristHash::new(0xf8d626aaaf278509)
        )
    }

    #[test]
    fn test_zobrist_hash_board() {
        let board = Board::default();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_board(&board),
            ZobristHash::new(0xc5d79d196e37d67c)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_1() {
        let board = Board::default();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x463b96181691fc9c)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_2() {
        let board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2")
                .unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x0756b94461c50fb0)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_3() {
        let board =
            Board::from_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2").unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x662fafb965db29d4)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_4() {
        let board =
            Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3")
                .unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x22a48b5a8e47ff78)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_5() {
        let board =
            Board::from_fen("rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR b kq - 0 3").unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x652a607ca3f242c1)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_6() {
        let board =
            Board::from_fen("rnbq1bnr/ppp1pkpp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR w - - 0 4").unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x00fdd303c946bdd9)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_8() {
        let board =
            Board::from_fen("rnbqkbnr/p1pppppp/8/8/PpP4P/8/1P1PPPP1/RNBQKBNR b KQkq c3 0 3")
                .unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x3c8123ea7b067637)
        )
    }

    #[test]
    fn test_zobrist_hash_everything_9() {
        let board = Board::from_fen("rnbqkbnr/p1pppppp/8/8/P6P/R1p5/1P1PPPP1/1NBQKBNR b Kkq - 0 4")
            .unwrap();
        let hasher = ZobristHasher::load();

        assert_eq!(
            hasher.hash_everyting(&board),
            ZobristHash::new(0x5c3f9b829b279560)
        )
    }
}
