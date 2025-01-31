#[cfg(test)]
use {
    crate::board::Board, crate::moves::move_mask_gen::MoveGenMasks,
    crate::moves::moves_utils::Move, std::fs,
};

#[cfg(test)]
fn play_game(board: &Board, move_gen_masks: &MoveGenMasks, depth: u8, max_depth: u8) -> usize {
    let legal_moves = board.get_legal_moves(move_gen_masks);

    if depth == max_depth {
        return legal_moves.len();
    }

    let mut n_moves = 0;

    for (_, new_board) in legal_moves {
        n_moves += play_game(&new_board, move_gen_masks, depth + 1, max_depth);
    }

    n_moves
}

#[cfg(test)]
fn test_game(board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<(Move, Move)> {
    let mut output: Vec<(Move, Move)> = Vec::with_capacity(10000);
    for (legal_move, new_board) in board.get_legal_moves(move_gen_masks) {
        for (new_legal_move, _) in new_board.get_legal_moves(move_gen_masks) {
            output.push((legal_move.clone(), new_legal_move));
        }
    }
    output
}

#[cfg(test)]
fn save_test_output(moves: Vec<(Move, Move)>) {
    let move_strings: Vec<String> = moves
        .into_iter()
        .map(|(x, y)| format!("{}-{}", x, y))
        .collect();

    fs::write("/tmp/test.txt", move_strings.join("\n")).expect("");
}

#[cfg(test)]
mod test_perft {
    use super::*;
    use once_cell::sync::Lazy;

    static MOVE_GEN_MASKS: Lazy<MoveGenMasks> = Lazy::new(MoveGenMasks::load);

    #[test]
    fn test_position_1_default() {
        let board = Board::default();

        let max_depth = 5;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 4865609)
    }

    #[test]
    fn test_position_2_kiwipete() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();

        let max_depth = 2;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 2039);
    }

    #[test]
    fn test_position_3() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

        let max_depth = 2;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 191)
    }

    #[test]
    fn test_poition_4() {
        let board =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();

        let max_depth = 2;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        // save_test_output(test_game(&board, &MOVE_GEN_MASKS));

        assert_eq!(n_moves, 264)
    }

    #[test]
    fn test_position_5() {
        let board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

        let max_depth = 1;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        assert_eq!(n_moves, 44)
    }

    #[test]
    fn test_position_6() {
        let board = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        )
        .unwrap();

        let max_depth = 1;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        // let moves = test_game(&board, &MOVE_GEN_MASKS);
        // save_test_output(moves);

        assert_eq!(n_moves, 46)
    }

    #[test]
    fn test_position_7_en_passant() {
        let board =
            Board::from_fen("rnbqkbnr/ppp1ppp1/8/3pP2p/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3")
                .unwrap();

        let max_depth = 1;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        assert_eq!(n_moves, 31)
    }
}
