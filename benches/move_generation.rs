
use std::time::Instant;

use rustier_chess::{board::Board, moves::move_mask_gen::MoveGenMasks, perft::play_game};

fn main() {
    let board = Board::default();
    let move_gen_masks = MoveGenMasks::load();

    let now = Instant::now();
    let n_iterations = play_game(&board, &move_gen_masks, 1, 6) as f64;
    let secs = now.elapsed().as_secs_f64();

    println!("It took {} seconds with {} nodes.", secs, n_iterations);
    println!("NPS: {:.2}M", n_iterations / (secs * 1e6));
}
