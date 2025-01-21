use crate::moves::moves_utils::Move;
use crate::{board::Board, piece::Color, square::Square};

fn get_pawn_moves(square: Square, board: &Board, color: usize, opponent_color: usize) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let direction = if color == Color::WHITE { 1_i8 } else { -1 };

    let new_square = square + direction * 8;

    if !(board.colors[0] & board.colors[1]).read_square(&new_square) {
        let new_move = Move::from_origin_and_destination(&new_square, &square);
        moves.push(new_move);

        let base_rank = if color == Color::WHITE { 1 } else { 6 };

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
        if board.colors[opponent_color].read_square(&attacking_square)
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

#[cfg(test)]
mod test_move_calculation {
    use crate::{board::Board, piece::Color, square::Square};

    use super::*;

    #[test]
    fn test_get_white_pawn_moves() {
        let square = Square::new(8);
        let board = Board::default();

        let moves = get_pawn_moves(square, &board, Color::WHITE, Color::BLACK);

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
        let board = Board::default();

        let moves = get_pawn_moves(square, &board, Color::BLACK, Color::WHITE);

        assert_eq!(
            moves[0],
            Move::from_origin_and_destination(&Square::new(47), &square)
        );
        let mut new_move = Move::from_origin_and_destination(&Square::new(39), &square);
        new_move.set_en_passant();
        assert_eq!(moves[1], new_move);
    }
}
