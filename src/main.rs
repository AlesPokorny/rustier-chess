#![allow(dead_code)]

mod bitboard;
mod board;
mod game;
mod moves;
mod piece;
mod square;
mod state;

use std::time::Instant;

use moves::move_mask_gen::generate_moves;
use moves::move_mask_gen::MoveGenMasks;
use moves::moves_calculation::{get_knight_moves, get_pawn_moves};
use piece::Piece;
use piece::Pieces;
use square::Square;

use crate::board::Board;

fn main() {
    let board = Board::default();

    println!("{:x}", 0b11 << 12);
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
