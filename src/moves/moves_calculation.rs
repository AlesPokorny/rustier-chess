use crate::moves::moves_utils::Move;
use crate::{board::Board, piece::Color, square::Square};

use super::move_mask_gen::MoveGenMasks;

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
        if board.colors[board.state.opponent_turn()].read_square(&attacking_square)
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

fn get_rook_moves(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let (all_moves_mask, move_map) = move_gen_masks.rook_moves.get(square.as_usize()).unwrap();
    let current_blockers = board.all_pieces & *all_moves_mask;
    let possible_moves =
        *move_map.get(&current_blockers).unwrap() & !board.colors[board.state.turn];

    let all_desitnations = possible_moves.get_ones();
    let mut all_moves: Vec<Move> = Vec::with_capacity(all_desitnations.len());

    for new_square in all_desitnations {
        all_moves.push(Move::from_origin_and_destination(&new_square, square))
    }
    all_moves
}

fn get_bishop_moves(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let (all_moves_mask, move_map) = move_gen_masks.bishop_moves.get(square.as_usize()).unwrap();
    let current_blockers = board.all_pieces & *all_moves_mask;
    let possible_moves =
        *move_map.get(&current_blockers).unwrap() & !board.colors[board.state.turn];

    let all_desitnations = possible_moves.get_ones();
    let mut all_moves: Vec<Move> = Vec::with_capacity(all_desitnations.len());

    for new_square in all_desitnations {
        all_moves.push(Move::from_origin_and_destination(&new_square, square))
    }
    all_moves
}

#[cfg(test)]
mod test_move_calculation {
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
}
