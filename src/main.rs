#![allow(dead_code)]

mod bitboard;
mod board;
mod game;
mod moves;
mod piece;

use crate::board::Board;

fn main() {
    let a = Board::default();

    println!("{}", a);
}
