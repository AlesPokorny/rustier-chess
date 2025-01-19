use crate::{bitboard::BitBoard, board::Board, piece::Color, square::Square};

use super::moves_utils::Move;

pub fn get_pawn_moves(
    pawn_position: BitBoard,
    square: &Square,
    board: &Board,
    color: usize,
) -> (BitBoard, BitBoard) {
    let mut possible_positions;
    let not_all_pieces = !(board.colors[0] & board.colors[1]);

    let mut attacking_moves: BitBoard;
    if color == Color::WHITE {
        possible_positions = pawn_position << 8;
        possible_positions &= not_all_pieces;
        if (8..16).contains(&square.as_u8()) && possible_positions.as_u64() != 0 {
            possible_positions.set_one(&(square.add(16)));
            possible_positions &= not_all_pieces;
        }

        // Attacking moves
        attacking_moves = match square.get_file() {
            0 => pawn_position << 9,
            7 => pawn_position << 7,
            _ => pawn_position << 7 | pawn_position << 9,
        };
        attacking_moves &= board.colors[Color::BLACK];
    } else {
        possible_positions = pawn_position >> 8;
        if (48..56).contains(&square.as_u8()) && possible_positions.as_u64() != 0 {
            possible_positions.set_one(&(square.sub(16)));
            possible_positions &= not_all_pieces;
        }

        // Attacking moves
        attacking_moves = match square.get_file() {
            0 => pawn_position >> 7,
            7 => pawn_position >> 9,
            _ => pawn_position >> 7 | pawn_position >> 9,
        };
        attacking_moves &= board.colors[Color::WHITE];
    }
    (possible_positions, attacking_moves)
}

pub fn get_knight_moves(square: &Square, board: &Board, color: &usize) -> Vec<Move> {
    let i8_bit = square.as_u8() as i8;
    let bit_col = (square.as_u8() % 8) as i8;
    let pony_moves = [
        (-1, 2),
        (-1, -2),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (2, 1),
        (2, -1),
    ];

    let mut destinations: Vec<Move> = Vec::new();

    for pony_move in pony_moves {
        let new_col = bit_col + pony_move.0;
        if (0..7).contains(&new_col) {
            continue;
        }
        let new_bit = pony_move.0 + pony_move.1 * 8 + i8_bit;
        if (0..=63).contains(&new_bit) {
            continue;
        }

        if (color == &Color::WHITE && !board.colors[Color::WHITE].read_square(square))
            | (color == &Color::BLACK && !board.colors[Color::BLACK].read_square(square))
        {
            destinations.push(Move::from_origin_and_destination(
                &Square::new(new_bit as u8),
                square,
            ));
        }
    }

    destinations
}

pub fn get_king_moves(square: &Square, board: &Board, color: &usize) -> Vec<Move> {
    let king_row = square.get_row();
    let king_file = square.get_file();
    let is_king_on_first_row = king_row == 0;
    let is_king_on_last_row = king_row == 7;
    let is_king_on_first_file = king_file == 0;
    let is_king_on_last_file = king_file == 7;

    let directions: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

    let mut moves: Vec<Move> = Vec::new();

    for direction in directions {
        if is_king_on_first_file && [1, 7, -9].contains(&direction) {
            continue;
        }
        if is_king_on_last_file && [-1, -7, 9].contains(&direction) {
            continue;
        }
        if is_king_on_first_row && (7..=9).contains(&direction) {
            continue;
        }
        if is_king_on_last_row && (-9..=-7).contains(&direction) {
            continue;
        }

        let new_square = *square << -3_i8;

        if !board.colors[*color].read_square(&new_square) {
            let new_move = Move::from_origin_and_destination(&new_square, square);
            moves.push(new_move)
        };
    }

    moves
}

#[cfg(test)]
mod test_move_gen {
    use crate::{bitboard::BitBoard, board::Board, piece::Color};

    use super::*;

    #[test]
    fn test_get_white_pawn_moves() {
        let bit = Square::new(8);
        let mut bb = BitBoard::zeros();
        bb.set_one(&bit);
        let color = Color::WHITE;
        let board = Board::default();

        let (moves, attacking_moves) = get_pawn_moves(bb, &bit, &board, color);

        assert_eq!(attacking_moves.as_u64(), 0);
        assert_eq!(moves.as_u64(), 0b1000000010000000000000000);
    }

    #[test]
    fn test_get_black_pawn_moves() {
        let bit = Square::new(49);
        let mut bb = BitBoard::zeros();
        bb.set_one(&bit);
        let color = Color::WHITE;
        let board = Board::default();

        let (moves, attacking_moves) = get_pawn_moves(bb, &bit, &board, color);

        assert_eq!(attacking_moves.as_u64(), 0);
        println!("{}", moves);
        assert_eq!(moves.as_u64(), 0b100000001000000000000000000000000000000000);
    }
}
