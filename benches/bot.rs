use std::time::Instant;

use rustier_chess::{
    board::Board,
    bots::{bot::Bot, time_control::TimeControl},
    moves::move_mask_gen::MoveGenMasks,
    utils::zobrist::ZobristHasher,
};

fn main() {
    let hasher = ZobristHasher::load();
    let move_gen_masks = MoveGenMasks::load();
    let mut board = Board::new(&hasher);
    let mut bot = Bot::with_depth(7, TimeControl::max());

    let now = Instant::now();
    let _ = bot.get_best_move(&mut board, &move_gen_masks, &hasher);
    let secs = now.elapsed().as_secs_f64();

    println!("It took {} seconds", secs);
}
