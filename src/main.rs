#![allow(dead_code)]

mod bitboard;
mod board;
mod game;
mod moves;
mod piece;
mod square;
mod state;

use moves::move_gen::generate_moves;

use crate::board::Board;

fn main() {
    let a = Board::default();

    println!("{}", a);

    generate_moves();
}
