use std::collections::HashMap;

use crate::{bitboard::BitBoard, board::Board, piece::Color, square::Square};

use bincode::{deserialize_from, serialize_into};

use std::fs::File;

const MOVES_FOLDER_PATH: &str = "./src/moves/data/";
const KING_MOVES_FILE: &str = "king.bin";
const KNIGHT_MOVES_FILE: &str = "knight.bin";
const ROOK_MOVES_FILE: &str = "rook.bin";

fn get_pawn_moves(
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

fn generate_knight_moves(square: &Square) -> BitBoard {
    let i8_bit = square.as_u8() as i8;
    let square_file = square.get_file() as i8;
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

    let mut moves = BitBoard::zeros();

    for pony_move in pony_moves {
        let new_file = square_file + pony_move.0;
        if !(0..=7).contains(&new_file) {
            continue;
        }
        let new_bit = pony_move.0 + pony_move.1 * 8 + i8_bit;
        if !(0..=63).contains(&new_bit) {
            continue;
        }

        moves.set_one(&Square::new(new_bit as u8));
    }

    moves
}

fn generate_king_moves(square: &Square) -> BitBoard {
    let king_row = square.get_row();
    let king_file = square.get_file();
    let is_king_on_first_row = king_row == 0;
    let is_king_on_last_row = king_row == 7;
    let is_king_on_first_file = king_file == 0;
    let is_king_on_last_file = king_file == 7;

    let directions: [i8; 8] = [1, 7, 8, 9, -1, -7, -8, -9];

    let mut moves = BitBoard::zeros();

    for direction in directions {
        if is_king_on_first_file && [-1, 7, -9].contains(&direction) {
            continue;
        }
        if is_king_on_last_file && [1, -7, 9].contains(&direction) {
            continue;
        }
        if is_king_on_first_row && (-9..=-7).contains(&direction) {
            continue;
        }
        if is_king_on_last_row && (7..=9).contains(&direction) {
            continue;
        }

        moves.set_one(&(*square + direction));
    }
    moves
}

fn generate_rook_moves_mask(square: &Square) -> BitBoard {
    let file = square.get_file();
    let row = square.get_row();

    let mut sliding_move_bb = BitBoard::zeros();

    for i in 0..8_u8 {
        if i != file {
            sliding_move_bb.set_one(&Square::new(row * 8 + i));
        }

        if i != row {
            sliding_move_bb.set_one(&Square::new(i * 8 + file));
        }
    }
    sliding_move_bb
}

// TODO: This needs magic
fn generate_rook_moves(square: &Square) -> HashMap<BitBoard, BitBoard> {
    let full_mask = generate_rook_moves_mask(square);

    let blockers_mask = create_blocker_boards(&full_mask);
    let legal_moves_mask = get_rook_legal_moves_from_blockers(square, &full_mask, &blockers_mask);

    let mut output: HashMap<BitBoard, BitBoard> = HashMap::with_capacity(blockers_mask.len());

    for (blockers, legal_moves) in blockers_mask.into_iter().zip(legal_moves_mask) {
        output.insert(blockers, legal_moves);
    }
    output
}

/// TODO: This function needs to be cleaned up
fn get_rook_legal_moves_from_blockers(
    square: &Square,
    bitboard: &BitBoard,
    blockers: &[BitBoard],
) -> Vec<BitBoard> {
    let mut legal_moves_vec: Vec<BitBoard> = Vec::with_capacity(blockers.len());
    let starting_row = square.get_row() as i8;
    let starting_file = square.get_file() as i8;

    for blocker_board in blockers.iter() {
        let mut new_bitboard = *bitboard;

        for direction in [-1, 1] {
            let mut file_blocked = false;
            let mut row_blocked = false;

            for distance in 1_i8..8 {
                let offset = direction * distance;
                let new_row = starting_row + offset;
                let new_file = starting_file + offset;

                if (0..8).contains(&new_row) {
                    let new_square = Square::new((new_row * 8 + starting_file) as u8);
                    if row_blocked {
                        new_bitboard.set_zero(&new_square);
                    }
                    if blocker_board.read_square(&new_square) {
                        row_blocked = true;
                    };
                }

                if (0..8).contains(&new_file) {
                    let new_square = Square::new((starting_row * 8 + new_file) as u8);
                    if file_blocked {
                        new_bitboard.set_zero(&new_square);
                    }
                    if blocker_board.read_square(&new_square) {
                        file_blocked = true;
                    };
                }
            }
        }
        legal_moves_vec.push(new_bitboard);
    }

    legal_moves_vec
}

fn create_blocker_boards(bitboard: &BitBoard) -> Vec<BitBoard> {
    let set_bits_indices = bitboard.get_ones();
    let n_patterns = 1 << set_bits_indices.len(); // 2^n
    let mut blocker_boards: Vec<BitBoard> = Vec::with_capacity(n_patterns);

    for pattern_i in 0..n_patterns {
        let mut new_bitboard = BitBoard::zeros();
        for (bit_i, new_square) in set_bits_indices.iter().enumerate() {
            let bit = ((pattern_i >> bit_i) & 1) as u64;
            new_bitboard |= BitBoard::new(bit << new_square.as_u64());
        }

        blocker_boards.push(new_bitboard);
    }
    blocker_boards
}

pub fn generate_moves() {
    let mut king_moves: HashMap<Square, BitBoard> = HashMap::with_capacity(64);
    let mut knight_moves: HashMap<Square, BitBoard> = HashMap::with_capacity(64);
    let mut rook_moves: HashMap<Square, HashMap<BitBoard, BitBoard>> = HashMap::with_capacity(64);
    for i in 0..63 {
        let square = Square::new(i);

        let king_bb = generate_king_moves(&square);
        king_moves.insert(square, king_bb);

        let knight_bb = generate_knight_moves(&square);
        knight_moves.insert(square, knight_bb);

        let rook_mapping = generate_rook_moves(&square);
        rook_moves.insert(square, rook_mapping);
    }

    save_move_file(KING_MOVES_FILE, king_moves);
    save_move_file(KNIGHT_MOVES_FILE, knight_moves);
    save_sliding_move_file(ROOK_MOVES_FILE, rook_moves);
}

pub fn save_move_file(file_name: &str, moves: HashMap<Square, BitBoard>) {
    let file = File::create(format!("{}{}", MOVES_FOLDER_PATH, file_name)).unwrap();
    serialize_into(file, &moves).unwrap();
}

pub fn read_moves() {
    let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, KING_MOVES_FILE)).unwrap();
    let a = deserialize_from::<&mut File, HashMap<Square, BitBoard>>(&mut reader).unwrap();
    println!("{}", a.get(&Square::new(2)).unwrap());
}

pub fn save_sliding_move_file(
    file_name: &str,
    moves: HashMap<Square, HashMap<BitBoard, BitBoard>>,
) {
    let file = File::create(format!("{}{}", MOVES_FOLDER_PATH, file_name)).unwrap();
    serialize_into(file, &moves).unwrap();
}

pub fn read_sliding_moves() {
    let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, KING_MOVES_FILE)).unwrap();
    let _ =
        deserialize_from::<&mut File, HashMap<Square, HashMap<BitBoard, BitBoard>>>(&mut reader)
            .unwrap();
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
