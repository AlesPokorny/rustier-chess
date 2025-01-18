use crate::{bitboard::BitBoard, board::Board, piece::Color};

pub fn get_pawn_moves(
    pawn_position: BitBoard,
    bit: &u8,
    board: &Board,
    color: Color,
) -> (BitBoard, BitBoard) {
    let mut possible_positions;
    let not_all_pieces = !(board.black_pieces & board.white_pieces);

    let mut attacking_moves: BitBoard;
    if color == Color::W {
        possible_positions = pawn_position << 8;
        possible_positions &= not_all_pieces;
        if (8..16).contains(bit) && possible_positions.as_u64() != 0 {
            possible_positions.set_one(&(bit + 16));
            possible_positions &= not_all_pieces;
        }

        // Attacking moves
        attacking_moves = match bit % 8 {
            0 => pawn_position << 9,
            7 => pawn_position << 7,
            _ => pawn_position << 7 | pawn_position << 9,
        };
        attacking_moves &= board.black_pieces;
    } else {
        possible_positions = pawn_position >> 8;
        if (48..56).contains(bit) && possible_positions.as_u64() != 0 {
            possible_positions.set_one(&(bit - 16));
            possible_positions &= not_all_pieces;
        }

        // Attacking moves
        attacking_moves = match bit % 8 {
            0 => pawn_position >> 7,
            7 => pawn_position >> 9,
            _ => pawn_position >> 7 | pawn_position >> 9,
        };
        attacking_moves &= board.white_pieces;
    }
    (possible_positions, attacking_moves)
}

pub fn get_knight_moves(bit: &u8, board: &Board, color: &Color) -> BitBoard {
    let i8_bit = *bit as i8;
    let bit_col = (bit % 8) as i8;
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

    let mut destinations = BitBoard::zeros();

    for pony_move in pony_moves {
        let new_col = bit_col + pony_move.0;
        if (0..7).contains(&new_col) {
            continue;
        }
        let new_bit = pony_move.0 + pony_move.1 * 8 + i8_bit;
        if (0..=63).contains(&new_bit) {
            continue;
        }

        destinations.set_one(&(new_bit as u8));
    }

    destinations &= !(if color == &Color::W {
        board.white_pieces
    } else {
        board.black_pieces
    });
    destinations
}

#[cfg(test)]
mod test_move_gen {
    use crate::utils::coordinate_to_bit;
    use crate::{bitboard::BitBoard, board::Board, piece::Color};

    use super::*;

    #[test]
    fn test_get_white_pawn_moves() {
        let bit = 8;
        let mut bb = BitBoard::zeros();
        bb.set_one(&bit);
        let color = Color::W;
        let board = Board::default();

        let (moves, attacking_moves) = get_pawn_moves(bb, &bit, &board, color);

        assert_eq!(attacking_moves.as_u64(), 0);
        assert_eq!(moves.as_u64(), 0b1000000010000000000000000);
    }

    #[test]
    fn test_get_black_pawn_moves() {
        let bit = 49;
        let mut bb = BitBoard::zeros();
        bb.set_one(&bit);
        let color = Color::B;
        let board = Board::default();

        let (moves, attacking_moves) = get_pawn_moves(bb, &bit, &board, color);

        assert_eq!(attacking_moves.as_u64(), 0);
        println!("{}", moves);
        assert_eq!(moves.as_u64(), 0b100000001000000000000000000000000000000000);
    }

    #[test]
    fn test_get_knight_moves() {
        let board = Board::empty();
        let color = Color::W;

        let knight_moves = get_knight_moves(&coordinate_to_bit("a3"), &board, &color);
        assert_eq!(knight_moves.as_u64(), 0b000010100001000100000000);

        let knight_moves = get_knight_moves(&coordinate_to_bit("a1"), &board, &color);
        assert_eq!(knight_moves.as_u64(), 0b100000010000000000);

        let knight_moves = get_knight_moves(&coordinate_to_bit("d3"), &board, &color);
        assert_eq!(knight_moves.get_ones().len(), 8);
    }
}
