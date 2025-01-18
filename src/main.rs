#![allow(dead_code)]

mod bitboard;
mod board;
mod game;
mod moves;
mod piece;
mod square;

use crate::board::Board;

fn main() {
    let a = Board::default();

    println!("{}", a);
}
