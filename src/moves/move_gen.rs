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
            possible_positions.set_one(bit + 16);
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
            possible_positions.set_one(bit - 16);
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

#[cfg(test)]
mod test_move_gen {
    use crate::{bitboard::BitBoard, board::Board, piece::Color};

    use super::get_pawn_moves;

    #[test]
    fn test_get_white_pawn_moves() {
        let bit = 8;
        let mut bb = BitBoard::zeros();
        bb.set_one(bit);
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
        bb.set_one(bit);
        let color = Color::B;
        let board = Board::default();

        let (moves, attacking_moves) = get_pawn_moves(bb, &bit, &board, color);

        assert_eq!(attacking_moves.as_u64(), 0);
        println!("{}", moves);
        assert_eq!(moves.as_u64(), 0b100000001000000000000000000000000000000000);
    }
}
