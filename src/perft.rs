#[cfg(test)]
use {
    crate::board::Board, crate::moves::move_mask_gen::MoveGenMasks,
    crate::moves::moves_utils::Move, std::collections::VecDeque, std::fs,
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
fn test_game(
    board: &Board,
    move_gen_masks: &MoveGenMasks,
    depth: u8,
    max_depth: u8,
) -> Vec<Vec<Move>> {
    let mut queue: VecDeque<(u8, Move, Board, Vec<Move>)> = board
        .get_legal_moves(move_gen_masks)
        .into_iter()
        .map(|(x, y)| (depth, x, y, Vec::new()))
        .collect();

    let mut output: Vec<Vec<Move>> = Vec::with_capacity(100000);

    while let Some((current_depth, current_move, current_board, mut history)) = queue.pop_front() {
        history.push(current_move);

        if current_depth == max_depth {
            output.push(history);
            continue;
        }

        for (new_move, new_board) in current_board.get_legal_moves(move_gen_masks) {
            queue.push_back((current_depth + 1, new_move, new_board, history.clone()));
        }
    }

    output
}

// #[cfg(test)]
// fn aaa(
//     board: &Board,
//     move_gen_masks: &MoveGenMasks,
//     depth: u8,
//     max_depth: u8,
// ) {
//     let mut queue: VecDeque<(u8, Move, Board, Vec<(Move, Board)>)> = board
//         .get_legal_moves(move_gen_masks)
//         .into_iter()
//         .map(|(x, y)| (depth, x, y, Vec::new()))
//         .collect();

//     let mut output: Vec<Vec<(Move, Board)>> = Vec::with_capacity(100000);

//     while let Some((current_depth, current_move, current_board, mut history)) = queue.pop_front() {
//         history.push((current_move, current_board));

//         if current_depth == max_depth {
//             output.push(history);
//             continue;
//         }

//         for (new_move, new_board) in current_board.get_legal_moves(move_gen_masks) {
//             queue.push_back((current_depth + 1, new_move, new_board, history.clone()));
//         }
//     }

//     output.into_iter().map(
//         |history| {
//             if history[0].to_string() == ""
//         }
//     );
// }

#[cfg(test)]
fn save_test_output(moves: Vec<Vec<Move>>) {
    let move_strings: Vec<String> = moves
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
                .join("-")
        })
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

        let max_depth = 6;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 119060324)
    }

    #[test]
    fn test_position_2_kiwipete() {
        let board =
            Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")
                .unwrap();

        let max_depth = 5;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 193690690);
    }

    #[test]
    fn test_position_3() {
        let board = Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();

        let max_depth = 7;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);
        assert_eq!(n_moves, 178633661)
    }

    #[test]
    fn test_poition_4() {
        let board =
            Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1")
                .unwrap();

        let max_depth = 5;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        assert_eq!(n_moves, 15833292)
    }

    #[test]
    fn test_poition_4a() {
        let board =
            Board::from_fen("r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1")
                .unwrap();

        println!("{}", board);

        let max_depth = 1;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        // assert_eq!(n_moves, 15833292)
    }

    #[test]
    fn test_position_5() {
        let board =
            Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap();

        let max_depth = 5;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        assert_eq!(n_moves, 89941194)
    }

    #[test]
    fn test_position_6() {
        let board = Board::from_fen(
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        )
        .unwrap();

        let max_depth = 5;
        let n_moves = play_game(&board, &MOVE_GEN_MASKS, 1, max_depth);

        assert_eq!(n_moves, 164075551)
    }
}
