use bincode::{deserialize_from, serialize_into};
use std::collections::HashMap;
use std::fs::File;

use crate::types::{bitboard::BitBoard, piece::Pieces, square::Square};

pub struct MoveGenMasks {
    pub king_moves: Vec<BitBoard>,
    pub knight_moves: Vec<BitBoard>,
    pub rook_moves: Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>,
    pub bishop_moves: Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>,
}

impl MoveGenMasks {
    pub fn load() -> Self {
        let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, KING_MOVES_FILE)).unwrap();
        let king_moves = deserialize_from::<&mut File, Vec<BitBoard>>(&mut reader).unwrap();

        let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, KNIGHT_MOVES_FILE)).unwrap();
        let knight_moves = deserialize_from::<&mut File, Vec<BitBoard>>(&mut reader).unwrap();

        let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, ROOK_MOVES_FILE)).unwrap();
        let rook_moves =
            deserialize_from::<&mut File, Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>>(
                &mut reader,
            )
            .unwrap();

        let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, BISHOP_MOVES_FILE)).unwrap();
        let bishop_moves = deserialize_from::<
            &mut File,
            Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>,
        >(&mut reader)
        .unwrap();

        Self {
            king_moves,
            knight_moves,
            rook_moves,
            bishop_moves,
        }
    }
}

const MOVES_FOLDER_PATH: &str = "./src/moves/data/";
const KING_MOVES_FILE: &str = "king.bin";
const KNIGHT_MOVES_FILE: &str = "knight.bin";
const ROOK_MOVES_FILE: &str = "rook.bin";
const BISHOP_MOVES_FILE: &str = "bishop.bin";

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
    let king_row = square.get_rank();
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
    let row = square.get_rank();

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
fn generate_rook_moves(square: &Square) -> (BitBoard, HashMap<BitBoard, BitBoard>) {
    let full_mask = generate_rook_moves_mask(square);

    let blockers_mask = create_blocker_boards(&full_mask);
    let legal_moves_mask =
        generate_slider_moves_from_blockers(square, &blockers_mask, Pieces::ROOK);

    let mut output: HashMap<BitBoard, BitBoard> = HashMap::with_capacity(blockers_mask.len());

    for (blockers, legal_moves) in blockers_mask.into_iter().zip(legal_moves_mask) {
        output.insert(blockers, legal_moves);
    }
    (full_mask, output)
}

fn generate_bishop_moves(square: &Square) -> (BitBoard, HashMap<BitBoard, BitBoard>) {
    let all_move_mask = generate_bishop_move_mask(square);
    let blockers = create_blocker_boards(&all_move_mask);
    let legal_moves = generate_slider_moves_from_blockers(square, &blockers, Pieces::BISHOP);

    let mut output: HashMap<BitBoard, BitBoard> = HashMap::with_capacity(blockers.len());

    for (blockers, legal_moves) in blockers.into_iter().zip(legal_moves) {
        output.insert(blockers, legal_moves);
    }
    (all_move_mask, output)
}

fn generate_slider_moves_from_blockers(
    square: &Square,
    blockers: &[BitBoard],
    piece: usize,
) -> Vec<BitBoard> {
    let directions: [(i8, i8); 4] = match piece {
        Pieces::ROOK => [(-1, 0), (1, 0), (0, 1), (0, -1)],
        Pieces::BISHOP => [(-1, -1), (1, 1), (1, -1), (-1, 1)],
        _ => panic!("Wrong piece"),
    };

    let mut legal_moves_vec: Vec<BitBoard> = Vec::with_capacity(blockers.len());
    let file = square.get_file() as i8;
    let row = square.get_rank() as i8;
    let max_step = [row, file, 7 - row, 7 - file].into_iter().max().unwrap();

    for blocker in blockers {
        let mut legal_moves = BitBoard::zeros();
        for direction in directions {
            for step in 1..=max_step {
                let new_row = row + step * direction.0;
                let new_file = file + step * direction.1;

                if !(0..8).contains(&new_row) | !(0..8).contains(&new_file) {
                    break;
                }

                let new_square = Square::new((new_row * 8 + new_file) as u8);
                legal_moves.set_one(&new_square);

                if blocker.read_square(&new_square) {
                    break;
                }
            }
        }
        legal_moves_vec.push(legal_moves);
    }

    legal_moves_vec
}

fn generate_bishop_legal_moves_from_blockers(
    square: &Square,
    blockers: &[BitBoard],
) -> Vec<BitBoard> {
    let mut legal_moves_vec: Vec<BitBoard> = Vec::with_capacity(blockers.len());
    let file = square.get_file() as i8;
    let row = square.get_rank() as i8;
    let directions: [(i8, i8); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];
    let max_step = [row, file, 7 - row, 7 - file].into_iter().max().unwrap();

    for blocker in blockers {
        let mut legal_moves = BitBoard::zeros();
        for direction in directions {
            for step in 1..=max_step {
                let new_row = row + step * direction.0;
                let new_file = file + step * direction.1;

                if !(0..8).contains(&new_row) | !(0..8).contains(&new_file) {
                    break;
                }

                let new_square = Square::new((new_row * 8 + new_file) as u8);
                legal_moves.set_one(&new_square);

                if blocker.read_square(&new_square) {
                    break;
                }
            }
        }
        legal_moves_vec.push(legal_moves);
    }

    legal_moves_vec
}

fn generate_bishop_move_mask(square: &Square) -> BitBoard {
    let file = square.get_file() as i8;
    let row = square.get_rank() as i8;
    let directions: [(i8, i8); 4] = [(-1, -1), (1, 1), (1, -1), (-1, 1)];
    let max_step = [row, file, 7 - row, 7 - file].into_iter().max().unwrap();

    let mut sliding_move_bb = BitBoard::zeros();

    for direction in directions {
        for step in 1..=max_step {
            let new_row = row + step * direction.0;
            let new_file = file + step * direction.1;

            if !(0..8).contains(&new_row) | !(0..8).contains(&new_file) {
                break;
            }
            sliding_move_bb.set_one(&Square::new((new_row * 8 + new_file) as u8));
        }
    }

    sliding_move_bb
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
    let mut king_moves: Vec<BitBoard> = Vec::with_capacity(64);
    let mut knight_moves: Vec<BitBoard> = Vec::with_capacity(64);
    let mut rook_moves: Vec<(BitBoard, HashMap<BitBoard, BitBoard>)> = Vec::with_capacity(64);
    let mut bishop_moves: Vec<(BitBoard, HashMap<BitBoard, BitBoard>)> = Vec::with_capacity(64);
    for i in 0..64 {
        let square = Square::new(i);

        let king_bb = generate_king_moves(&square);
        king_moves.push(king_bb);

        let knight_bb = generate_knight_moves(&square);
        knight_moves.push(knight_bb);

        let rook_mapping = generate_rook_moves(&square);
        rook_moves.push(rook_mapping);

        let bishop_mapping = generate_bishop_moves(&square);
        bishop_moves.push(bishop_mapping);
    }

    save_move_file(KING_MOVES_FILE, king_moves);
    save_move_file(KNIGHT_MOVES_FILE, knight_moves);
    save_sliding_move_file(ROOK_MOVES_FILE, rook_moves);
    save_sliding_move_file(BISHOP_MOVES_FILE, bishop_moves);
}

pub fn save_move_file(file_name: &str, moves: Vec<BitBoard>) {
    let file = File::create(format!("{}{}", MOVES_FOLDER_PATH, file_name)).unwrap();
    serialize_into(file, &moves).unwrap();
}

pub fn save_sliding_move_file(
    file_name: &str,
    moves: Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>,
) {
    let file = File::create(format!("{}{}", MOVES_FOLDER_PATH, file_name)).unwrap();
    serialize_into(file, &moves).unwrap();
}

pub fn read_sliding_moves() {
    let mut reader = File::open(format!("{}{}", MOVES_FOLDER_PATH, KING_MOVES_FILE)).unwrap();
    let _ =
        deserialize_from::<&mut File, Vec<(BitBoard, HashMap<BitBoard, BitBoard>)>>(&mut reader)
            .unwrap();
}

#[cfg(test)]
mod test_move_gen {
    use super::*;

    #[test]
    fn test_generate_bishop_move_mask() {
        let square = Square::new(30);
        assert_eq!(
            generate_bishop_move_mask(&square).as_u64(),
            0x40810a000a01008
        );
    }

    #[test]
    fn test_generate_rook_move_mask() {
        let square = Square::new(30);
        assert_eq!(
            generate_rook_moves_mask(&square).as_u64(),
            0x40404040bf404040
        );
    }
}
