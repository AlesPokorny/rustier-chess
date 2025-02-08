#![allow(dead_code)]

mod board;
mod bots;
mod game;
mod magic;
mod moves;
mod perft;
mod types;
mod utils;

use game::UCIGame;

fn main() {
    let mut game = UCIGame::new();
    game.uci_io_loop();
}
