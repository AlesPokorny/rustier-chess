use std::time::Instant;

use rustier_chess::{
    board::Board, moves::move_mask_gen::MoveGenMasks, perft::play_game,
    utils::zobrist::ZobristHasher,
};

fn main() {
    let hasher = ZobristHasher::load();
    let move_gen_masks = MoveGenMasks::load();
    let mut board = Board::new(&hasher);

    let now = Instant::now();
    let n_iterations = play_game(&mut board, &move_gen_masks, &hasher, 1, 6) as f64;
    let secs = now.elapsed().as_secs_f64();

    println!("It took {} seconds with {} nodes.", secs, n_iterations);
    println!("NPS: {:.2}M", n_iterations / (secs * 1e6));
}
