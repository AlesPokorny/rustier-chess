use crate::board::Board;
use crate::magic::magics::{BISHOP_MAGICS, ROOK_MAGICS};
use crate::moves::move_mask_gen::MoveGenMasks;
use crate::moves::moves_utils::Move;
use crate::types::{bitboard::BitBoard, piece::Color, piece::Pieces, square::Square};

const CASTLING_WHITE_LONG_CHECKING: BitBoard = BitBoard(0x1C);
const CASTLING_WHITE_LONG_BLOCKING: BitBoard = BitBoard(0xE);
const CASTLING_WHITE_SHORT: BitBoard = BitBoard(0x60);
const CASTLING_BLACK_LONG_CHECKING: BitBoard = BitBoard(0x1C00000000000000);
const CASTLING_BLACK_LONG_BLOCKING: BitBoard = BitBoard(0xE00000000000000);
const CASTLING_BLACK_SHORT: BitBoard = BitBoard(0x6000000000000000);
const PROMOTION_PIECES: [usize; 4] = [Pieces::QUEEN, Pieces::KNIGHT, Pieces::ROOK, Pieces::BISHOP];

// TODO: Clean this shit up
pub fn get_pawn_moves(square: Square, board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let direction = if board.state.turn == Color::WHITE {
        1_i8
    } else {
        -1
    };

    let new_square = square + direction * 8;

    if !(board.colors[0] | board.colors[1]).read_square(&new_square) {
        let new_move = Move::from_origin_and_destination(&new_square, &square);
        if !(1..=6).contains(&new_square.get_rank()) {
            for piece in PROMOTION_PIECES {
                let mut promotion_move = new_move.clone();
                promotion_move.set_promotion(piece);
                moves.push(promotion_move);
            }
        } else {
            moves.push(new_move);

            let base_rank = if board.state.turn == Color::WHITE {
                1
            } else {
                6
            };

            if square.get_rank() == base_rank {
                let new_square = square + direction * 16;
                if !(board.colors[0] | board.colors[1]).read_square(&new_square) {
                    let mut new_move = Move::from_origin_and_destination(&new_square, &square);
                    new_move.set_en_passant();
                    moves.push(new_move);
                }
            }
        }
    }

    let new_rank = new_square.get_rank() as i8;
    for offset in [1, -1] {
        let new_file = square.get_file() as i8 + offset;
        if !(0..=7).contains(&new_file) {
            continue;
        }
        let attacking_square = Square::new((new_rank * 8 + new_file) as u8);
        if board.colors[board.state.opponent].read_square(&attacking_square)
            | board.check_en_passant(&attacking_square)
        {
            let new_move = Move::from_origin_and_destination(&attacking_square, &square);
            if !(1..=6).contains(&new_rank) {
                for piece in PROMOTION_PIECES {
                    let mut promotion_move = new_move.clone();
                    promotion_move.set_promotion(piece);
                    moves.push(promotion_move);
                }
            } else {
                moves.push(new_move)
            }
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
    let mut moves: Vec<Move> = (move_gen_masks.king_moves[square.as_usize()]
        & !board.colors[board.state.turn])
        .get_ones()
        .into_iter()
        .map(|new_square| Move::from_origin_and_destination(&new_square, square))
        .collect();
    moves.append(&mut get_castling_moves(square, board, move_gen_masks));

    moves
}

fn get_rook_move_mask(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> BitBoard {
    let magic = &ROOK_MAGICS[square.as_usize()];
    let index = magic.get_index(board.all_pieces);
    move_gen_masks.rook_moves[square.as_usize()][index] & !board.colors[board.state.turn]
}

fn get_slidy_boii_moves(
    square: &Square,
    board: &Board,
    move_gen_masks: &MoveGenMasks,
    piece: usize,
) -> Vec<Move> {
    let possible_moves = if piece == Pieces::ROOK {
        get_rook_move_mask(square, board, move_gen_masks)
    } else if piece == Pieces::BISHOP {
        get_bishop_move_mask(square, board, move_gen_masks)
    } else {
        get_rook_move_mask(square, board, move_gen_masks)
            | get_bishop_move_mask(square, board, move_gen_masks)
    };

    let all_desitnations = possible_moves.get_ones();
    let mut all_moves: Vec<Move> = Vec::with_capacity(all_desitnations.len());

    for new_square in all_desitnations {
        all_moves.push(Move::from_origin_and_destination(&new_square, square))
    }
    all_moves
}

fn get_bishop_move_mask(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> BitBoard {
    let magic = &BISHOP_MAGICS[square.as_usize()];
    let index = magic.get_index(board.all_pieces);
    move_gen_masks.bishop_moves[square.as_usize()][index] & !board.colors[board.state.turn]
}

fn get_castling_moves(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
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
                .all(|square_to_check| !is_square_in_check(square_to_check, board, move_gen_masks))
            & !is_square_in_check(square, board, move_gen_masks)
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
        let (checking_mask, blocking_mask) = if board.state.turn == Color::WHITE {
            (CASTLING_WHITE_LONG_CHECKING, CASTLING_WHITE_LONG_BLOCKING)
        } else {
            (CASTLING_BLACK_LONG_CHECKING, CASTLING_BLACK_LONG_BLOCKING)
        };

        if (blocking_mask & board.all_pieces).is_empty()
            & checking_mask
                .get_ones()
                .iter()
                .all(|square_to_check| !is_square_in_check(square_to_check, board, move_gen_masks))
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

pub fn is_square_in_check(square: &Square, board: &Board, move_gen_masks: &MoveGenMasks) -> bool {
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
        let new_bit = square.as_u8() as i8 + pawn_direction * offset;
        if !(0..64).contains(&new_bit) {
            continue;
        }
        let attacking_square = Square::new(new_bit as u8);
        if (square.get_rank() as i8 - attacking_square.get_rank() as i8).abs() != 1 {
            continue;
        }
        if opponent_pieces[Pieces::PAWN].read_square(&attacking_square) {
            return true;
        }
    }

    false
}

pub fn get_all_moves(board: &Board, move_gen_masks: &MoveGenMasks) -> Vec<Move> {
    let mut all_moves: Vec<Move> = Vec::with_capacity(139); // maximum number of moves in a position
    for (piece, piece_board) in board.pieces[board.state.turn].iter().enumerate() {
        let all_squares = piece_board.get_ones();
        let mut piece_moves: Vec<Move> = all_squares
            .into_iter()
            .flat_map(|square| match piece {
                Pieces::PAWN => get_pawn_moves(square, board),
                Pieces::BISHOP => {
                    get_slidy_boii_moves(&square, board, move_gen_masks, Pieces::BISHOP)
                }
                Pieces::KNIGHT => get_knight_moves(&square, move_gen_masks, board),
                Pieces::ROOK => get_slidy_boii_moves(&square, board, move_gen_masks, Pieces::ROOK),
                Pieces::KING => get_king_moves(&square, move_gen_masks, board),
                Pieces::QUEEN => {
                    get_slidy_boii_moves(&square, board, move_gen_masks, Pieces::QUEEN)
                }
                _ => panic!("That is a weird piece"),
            })
            .collect();
        all_moves.append(&mut piece_moves);
    }

    all_moves
}

#[cfg(test)]
mod test_move_calculation {
    use std::str::FromStr;

    use once_cell::sync::Lazy;

    use crate::{board::Board, types::square::Square, utils::zobrist::ZobristHasher};

    use super::*;

    static HASHER: Lazy<ZobristHasher> = Lazy::new(ZobristHasher::load);

    #[test]
    fn test_get_white_pawn_moves() {
        let square = Square::new(8);
        let board = Board::new(&HASHER);

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
        let square = Square::from_str("b7").unwrap();
        let mut board = Board::new(&HASHER);
        board.state.change_turn();

        let moves = get_pawn_moves(square, &board);

        assert_eq!(
            moves[0],
            Move::from_origin_and_destination(&Square::from_str("b6").unwrap(), &square)
        );
        let mut new_move =
            Move::from_origin_and_destination(&Square::from_str("b5").unwrap(), &square);
        new_move.set_en_passant();
        assert_eq!(moves[1], new_move);
    }

    #[test]
    fn test_is_square_in_check() {
        let board = Board::from_fen(
            "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 4",
            &HASHER,
        )
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
            "r3k2r/pbpq1ppp/1pnp1n2/2b1p3/4P3/1PNB1N2/PBPPQPPP/R3K2R b KQkq - 3 9",
            &HASHER,
        )
        .unwrap();
        let move_gen_masks = MoveGenMasks::load();
        let king_square = board.pieces[Color::BLACK][Pieces::KING].get_one();

        let moves = get_castling_moves(&king_square, &board, &move_gen_masks);
        assert_eq!(moves.len(), 2);
        let short = moves.first().unwrap();
        assert_eq!(short.get_destination().to_string(), "g8");
        assert_eq!(short.get_origin().to_string(), "e8");
        assert!(short.special_move() == 3);

        let long = moves.get(1).unwrap();
        assert_eq!(long.get_destination().to_string(), "c8");
        assert_eq!(long.get_origin().to_string(), "e8");
        assert!(short.special_move() == 3);
    }

    #[test]
    fn test_get_castling_moves_white() {
        let board = Board::from_fen(
            "r1b1k2r/pppq1ppp/3p1n2/2b1p3/3nP3/1PNB1N2/PBPPQPPP/R3K2R w KQkq - 6 8",
            &HASHER,
        )
        .unwrap();
        let move_gen_masks = MoveGenMasks::load();
        let king_square = board.pieces[Color::WHITE][Pieces::KING].get_one();

        let moves = get_castling_moves(&king_square, &board, &move_gen_masks);
        assert_eq!(moves.len(), 2);
        let short = moves.first().unwrap();
        assert_eq!(short.get_destination().to_string(), "g1");
        assert_eq!(short.get_origin().to_string(), "e1");
        assert!(short.special_move() == 3);

        let long = moves.get(1).unwrap();
        assert_eq!(long.get_destination().to_string(), "c1");
        assert_eq!(long.get_origin().to_string(), "e1");
        assert!(short.special_move() == 3);
    }

    #[test]
    fn test_get_all_moves() {
        let mut board = Board::new(&HASHER);
        board.state.change_turn();
        let move_gen_masks = MoveGenMasks::load();

        let moves = get_all_moves(&board, &move_gen_masks);
        assert_eq!(moves.len(), 20);
    }
}
