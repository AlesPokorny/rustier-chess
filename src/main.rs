#![allow(dead_code)]

use types::bitboard::BitBoard;
mod board;
mod moves;
mod perft;
mod types;

// use std::time::Instant;

// use moves::move_mask_gen::generate_moves;
// use moves::move_mask_gen::MoveGenMasks;
// use moves::moves_calculation::{get_knight_moves, get_pawn_moves};
// use piece::Piece;
// use piece::Pieces;
// use square::Square;

// use crate::board::Board;

fn main() {
    // let board = Board::default();

    // generate_moves();
    println!("{}", BitBoard(0x000101010101017E));
    // println!("{}", BitBoard(0x01FE010101010101));
    // println!("{}", BitBoard(0x007E010101010100_u64.overflowing_mul(0x48FFFE99FECFAA00).0));

    // println!("{}", BitBoard(0x1C));

    // println!("{:x}", 0b11 << 12);
    // println!("{:x}", 0b111111000000);

    // let move_gen_mask = MoveGenMasks::load();

    // for square in board.pieces[board.state.turn][Pieces::PAWN].get_ones() {
    //     println!("{}", square);
    //     // let moves = get_pawn_moves(&square, &move_gen_mask, &board);
    //     let now = Instant::now();
    //     let moves = get_pawn_moves(square, &board);
    //     let duration = now.elapsed();
    //     println!("{:?}", duration);
    // }

    // println!("{}", Square::new(8));
}
