#![allow(dead_code)]

mod bitboard;
mod board;
mod game;
mod piece;

use crate::bitboard::BitBoard;

fn main() {
    let a = BitBoard::new(199);

    println!("{}", a);
}
