#![allow(dead_code)]

use board::Board;

mod board;
mod magic;
mod moves;
mod perft;
mod types;
mod utils;

fn main() {
    let _ = Board::default();
}
