use std::{sync::atomic::Ordering, time::Instant};

use rustier_chess::{
    board::Board,
    bots::{bot::Bot, time_control::TimeControl},
    game::UCI_STOP,
    moves::move_mask_gen::MoveGenMasks,
    utils::zobrist::ZobristHasher,
};

fn main() {
    UCI_STOP.store(false, Ordering::Relaxed);
    let hasher = ZobristHasher::load();
    let move_gen_masks = MoveGenMasks::load();
    let mut board = Board::new(&hasher);
    let mut bot = Bot::with_depth(7, TimeControl::max());

    let now = Instant::now();
    let _ = bot.get_best_move(&mut board, &move_gen_masks, &hasher);
    let secs = now.elapsed().as_secs_f64();

    println!("Search depth 7 took {} seconds", secs);

    UCI_STOP.store(false, Ordering::Relaxed);
    let mut bot = Bot::with_depth(5, TimeControl::max());

    let now = Instant::now();
    for _ in 0..20 {
        let best_move = bot.get_best_move(&mut board, &move_gen_masks, &hasher);
        board.make_move(&best_move, &hasher);
        UCI_STOP.store(false, Ordering::Relaxed);
    }

    let secs = now.elapsed().as_secs_f64();
    println!("Playing 8 moves depth 5 took {} seconds", secs);
}
