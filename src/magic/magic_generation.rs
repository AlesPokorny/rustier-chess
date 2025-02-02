use rayon::prelude::*;
use std::{collections::HashMap, fmt::Display, time::Instant};

use rand::Rng;

use crate::{
    moves::move_mask_gen::{
        generate_bishop_moves, generate_relevant_blockers, generate_rook_moves,
    },
    types::{bitboard::BitBoard, piece::Pieces, square::Square},
};

pub struct Magic {
    pub mask: BitBoard,
    pub magic: u64,
    pub shift: u8,
}

impl Magic {
    pub fn new(mask: BitBoard, magic: u64, shift: u8) -> Self {
        Self { mask, magic, shift }
    }

    pub fn get_index(&self, board_mask: BitBoard) -> usize {
        let blocker_mask = board_mask & self.mask;
        (blocker_mask.0.overflowing_mul(self.magic).0 >> self.shift) as usize
    }
}

impl Display for Magic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Magic: mask=0x{:x}, magic=0x{:x}, shift={}",
            self.mask.as_u64(),
            self.magic,
            self.shift
        )
    }
}

fn find_magic(square: &Square, piece: usize, timeout_seconds: u64) -> (u8, u64) {
    let mask = generate_relevant_blockers(square, piece);
    let (_, moves) = if piece == Pieces::ROOK {
        generate_rook_moves(square)
    } else {
        generate_bishop_moves(square)
    };
    let mut rng = rand::rng();
    let mut best_shift = 0;
    let mut best_magic = 0;

    'outer: for shift in 52..60 {
        let now = Instant::now();
        loop {
            let magic: u64 = rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>();
            let magic_struct = Magic { mask, magic, shift };
            if verify_magic(&moves, &magic_struct).is_ok() {
                best_shift = shift;
                best_magic = magic;
                break;
            }

            if now.elapsed().as_secs() > timeout_seconds {
                break 'outer;
            }
        }
    }

    (best_shift, best_magic)
}

struct IncompatibleMagic;

fn verify_magic(
    moves: &HashMap<BitBoard, BitBoard>,
    magic: &Magic,
) -> Result<(), IncompatibleMagic> {
    let mut resulting_map: HashMap<u64, BitBoard> = HashMap::new();
    let mut max_key = 0;
    for (blocker_mask, move_mask) in moves.iter() {
        let index_key = (blocker_mask.0.wrapping_mul(magic.magic)) >> magic.shift;
        if let Some(stored_mask) = resulting_map.get(&index_key) {
            if stored_mask != move_mask {
                return Err(IncompatibleMagic);
            }
            continue;
        }
        resulting_map.insert(index_key, *move_mask);
        max_key = max_key.max(index_key);
    }

    Ok(())
}

pub fn find_all_magics(piece: usize, timeout_seconds: u64) -> Vec<(u8, u64, u8)> {
    let mut output: Vec<(u8, u64, u8)> = Vec::new();

    (0..64)
        .into_par_iter()
        .map(|i| {
            let (best_shift, best_magic) = find_magic(&Square::new(i), piece, timeout_seconds);
            (i, best_magic, best_shift)
        })
        .collect_into_vec(&mut output);
    output
}

// #[cfg(test)]
// mod test_magics {
//     use crate::{magic::magics::ROOK_MAGICS, moves::move_mask_gen::generate_rook_moves, types::square::Square};

//     use super::Magic;

//     // #[test]
//     // fn test_rook_magics() {
//     //     for i in 0..64 {
//     //         let square = Square::new(i);
//     //         let (mask, moves) = generate_rook_moves(&square);
//     //         let (magic_value, shift) = ROOK_MAGICS[square.as_usize()];
//     //         let magic = Magic::new(mask, magic_value, shift);
//     //         let a = generate_rook_moves(&square);
//     //     }
//     // }
// }
