#![allow(dead_code)]

mod bitboard;
mod board;
mod piece;
mod game;

use crate::bitboard::BitBoard;

fn main() {
    let a = BitBoard::new(199);

    println!("{}", a);
}
