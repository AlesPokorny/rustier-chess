use crate::bitboard::BitBoard;
use crate::moves::moves_utils::Move;
use crate::piece::Pieces;
use crate::{board::Board, piece::Color, square::Square};

use super::move_mask_gen::MoveGenMasks;

const CASTLING_WHITE_LONG: BitBoard = BitBoard(0xC);
const CASTLING_WHITE_SHORT: BitBoard = BitBoard(0x60);
const CASTLING_BLACK_LONG: BitBoard = BitBoard(0xC00000000000000);
const CASTLING_BLACK_SHORT: BitBoard = BitBoard(0x6000000000000000);

pub fn get_pawn_moves(square: Square, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let direction = if board.state.turn == Color::WHITE {
        1_i8
    } else {
        -1
    };

    let new_square = square + direction * 8;

    if !(board.colors[0] & board.colors[1]).read_square(&new_square) {
        let new_move = Move::from_origin_and_destination(&new_square, &square);
        moves.push(new_move);

        let base_rank = if board.state.turn == Color::WHITE {
            1
        } else {
            6
        };

        if square.get_rank() == base_rank {
            let new_square = square + direction * 16;
            if !(board.colors[0] & board.colors[1]).read_square(&new_square) {
                let mut new_move = Move::from_origin_and_destination(&new_square, &square);
                new_move.set_en_passant();
                moves.push(new_move);
            }
        }
    }

    for offset in [7, 9] {
        let attacking_square = square + direction * offset;
        if (square.get_rank() as i8 - attacking_square.get_rank() as i8).abs() > 1 {
            continue;
        }
        if board.colors[board.state.opponent].read_square(&attacking_square)
            | board.check_en_passant(&attacking_square)
        {
            moves.push(Move::from_origin_and_destination(
                &attacking_square,
                &square,
            ))
        }
    }

    moves
}

pub fn get_knight_moves(
    square: &Square,
    move_gen_masks: &MoveGenMasks,
    board: &Board,
) -> Vec<Move> {
    (move_gen_masks.knight_moves[square.as_usize()] & !board.colors[board.state.turn])
        .get_ones()
        .into_iter()
        .map(|new_square| Move::from_origin_and_destination(&new_square, square))
        .collect()
}

fn get_king_moves(square: &Square, move_gen_masks: &MoveGenMasks, board: &Board) -> Vec<Move> {
    (move_gen_masks.king_moves[square.as_usize()] & !board.colors[board.state.turn])
        .get_ones()
        .into_iter()
        .map(|new_square| Move::from_origin_and_destination(&new_square, square))
        .collect()
}

fn get_rook_move_mask(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> BitBoard {
    let (move_mask, move_map) = move_gen_masks.rook_moves.get(square.as_usize()).unwrap();
    let blocker_mask = board.all_pieces & *move_mask;
    *move_map.get(&blocker_mask).unwrap() & !board.colors[board.state.turn]
}

fn get_rook_moves(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let possible_moves = get_rook_move_mask(square, board, move_gen_masks);
    let all_desitnations = possible_moves.get_ones();
    let mut all_moves: Vec<Move> = Vec::with_capacity(all_desitnations.len());

    for new_square in all_desitnations {
        all_moves.push(Move::from_origin_and_destination(&new_square, square))
    }
    all_moves
}

fn get_bishop_move_mask(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> BitBoard {
    let (move_mask, move_map) = move_gen_masks.bishop_moves.get(square.as_usize()).unwrap();
    let blocker_mask = *move_mask & board.all_pieces;
    *move_map.get(&blocker_mask).unwrap()
}

fn get_bishop_moves(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let possible_moves_mask = get_bishop_move_mask(square, board, move_gen_masks);
    let all_desitnations = possible_moves_mask.get_ones();
    let mut all_moves: Vec<Move> = Vec::with_capacity(all_desitnations.len());

    for new_square in all_desitnations {
        all_moves.push(Move::from_origin_and_destination(&new_square, square))
    }
    all_moves
}

fn get_castling_moves(board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let (can_short, can_long) = board.state.castling.can_castle(board.state.turn);
    let mut castling_moves: Vec<Move> = Vec::with_capacity(1);
    if can_short {
        let mask = if board.state.turn == Color::WHITE {
            CASTLING_WHITE_SHORT
        } else {
            CASTLING_BLACK_SHORT
        };
        if (mask & board.all_pieces).is_empty()
            & mask
                .get_ones()
                .iter()
                .all(|square| !is_square_in_check(square, board, move_gen_masks))
        {
            let (origin, destination) = if board.state.turn == Color::WHITE {
                (Square::new(4), Square::new(6))
            } else {
                (Square::new(60), Square::new(62))
            };
            let mut castling_move = Move::from_origin_and_destination(&destination, &origin);
            castling_move.set_castling();
            castling_moves.push(castling_move);
        }
    }

    if can_long {
        let mask = if board.state.turn == Color::WHITE {
            CASTLING_WHITE_LONG
        } else {
            CASTLING_BLACK_LONG
        };
        if (mask & board.all_pieces).is_empty()
            & mask
                .get_ones()
                .iter()
                .all(|square| !is_square_in_check(square, board, move_gen_masks))
        {
            let (origin, destination) = if board.state.turn == Color::WHITE {
                (Square::new(4), Square::new(2))
            } else {
                (Square::new(60), Square::new(58))
            };
            let mut castling_move = Move::from_origin_and_destination(&destination, &origin);
            castling_move.set_castling();
            castling_moves.push(castling_move);
        }
    }

    castling_moves
}

fn is_square_in_check(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> bool {
    let square_usize = square.as_usize();
    let opponent_pieces = board.pieces[board.state.opponent];

    if !(move_gen_masks.knight_moves[square_usize] & opponent_pieces[Pieces::KNIGHT]).is_empty() {
        return true;
    }

    if !(move_gen_masks.king_moves[square_usize] & opponent_pieces[Pieces::KING]).is_empty() {
        return true;
    }

    let rook_move_mask = get_rook_move_mask(square, board, move_gen_masks);
    if !(rook_move_mask & (opponent_pieces[Pieces::ROOK] | opponent_pieces[Pieces::QUEEN]))
        .is_empty()
    {
        return true;
    }

    let bishop_move_mask = get_bishop_move_mask(square, board, move_gen_masks);
    if !(bishop_move_mask & (opponent_pieces[Pieces::BISHOP] | opponent_pieces[Pieces::QUEEN]))
        .is_empty()
    {
        return true;
    }

    let pawn_direction = if board.state.turn == Color::WHITE {
        1_i8
    } else {
        -1
    };
    for offset in [7, 9] {
        let attacking_square = *square + pawn_direction * offset;
        if (square.get_rank() as i8 - attacking_square.get_rank() as i8).abs() != 1 {
            continue;
        }
        if opponent_pieces[Pieces::PAWN].read_square(&attacking_square) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_move_calculation {
    use std::str::FromStr;

    use crate::{board::Board, square::Square};

    use super::*;

    #[test]
    fn test_get_white_pawn_moves() {
        let square = Square::new(8);
        let board = Board::default();

        let moves = get_pawn_moves(square, &board);

        assert_eq!(
            moves[0],
            Move::from_origin_and_destination(&Square::new(16), &square)
        );
        let mut new_move = Move::from_origin_and_destination(&Square::new(24), &square);
        new_move.set_en_passant();
        assert_eq!(moves[1], new_move);
    }

    #[test]
    fn test_get_black_pawn_moves() {
        let square = Square::new(55);
        let mut board = Board::default();
        board.state.change_turn();

        let moves = get_pawn_moves(square, &board);

        assert_eq!(
            moves[0],
            Move::from_origin_and_destination(&Square::new(47), &square)
        );
        let mut new_move = Move::from_origin_and_destination(&Square::new(39), &square);
        new_move.set_en_passant();
        assert_eq!(moves[1], new_move);
    }

    #[test]
    fn test_is_square_in_check() {
        let board =
            Board::from_fen("r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 4")
                .unwrap();
        let move_gen_masks = MoveGenMasks::load();

        let in_check_positions = ["a5", "d5", "e5", "f5", "e7", "d6", "f6", "d5", "h4"];

        assert!(in_check_positions.into_iter().all(|square_str| {
            let square = Square::from_str(square_str).unwrap();
            is_square_in_check(&square, &board, &move_gen_masks)
        }));

        let not_in_check_positions = ["a3", "g4", "e4", "h5"];

        assert!(!not_in_check_positions.into_iter().any(|square_str| {
            let square = Square::from_str(square_str).unwrap();
            is_square_in_check(&square, &board, &move_gen_masks)
        }));
    }

    #[test]
    fn test_get_castling_moves_black() {
        let board = Board::from_fen(
            "r1b1k2r/pppq1ppp/2np1n2/2b1p3/4P3/1PNB1N2/PBPPQPPP/R3K2R b KQkq - 5 7",
        )
        .unwrap();
        let move_gen_masks = MoveGenMasks::load();

        let moves = get_castling_moves(&board, &move_gen_masks);
        let expected_from = Square::from_str("e8").unwrap();
        let expected_to = Square::from_str("g8").unwrap();
        assert_eq!(moves.len(), 1);
        let the_move = moves.get(0).unwrap();
        assert_eq!(the_move.get_destination(), expected_to);
        assert_eq!(the_move.get_origin(), expected_from);
        assert!(the_move.is_castling());
    }

    #[test]
    fn test_get_castling_moves_white() {
        let board = Board::from_fen(
            "r1b1k2r/pppq1ppp/3p1n2/2b1p3/3nP3/1PNB1N2/PBPPQPPP/R3K2R w KQkq - 6 8",
        )
        .unwrap();
        let move_gen_masks = MoveGenMasks::load();

        let moves = get_castling_moves(&board, &move_gen_masks);
        assert_eq!(moves.len(), 2);
        let short = moves.get(0).unwrap();
        assert_eq!(short.get_destination(), Square::from_str("g1").unwrap());
        assert_eq!(short.get_origin(), Square::from_str("e1").unwrap());
        assert!(short.is_castling());

        let long = moves.get(1).unwrap();
        assert_eq!(long.get_destination(), Square::from_str("c1").unwrap());
        assert_eq!(long.get_origin(), Square::from_str("e1").unwrap());
        assert!(long.is_castling());
    }
}
