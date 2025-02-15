use crate::{moves::moves_utils::Move, utils::zobrist::ZobristHash};

pub enum TranspositionEntryType {
    Alpha,
    Beta,
    Exact,
}

pub struct TranspositionEntry {
    pub hash: ZobristHash,
    pub depth: u8,
    pub flag: TranspositionEntryType,
    pub score: i32,
    pub best_move: Option<Move>,
}

impl TranspositionEntry {
    fn empty() -> Self {
        Self {
            hash: ZobristHash::zero(),
            depth: 0,
            flag: TranspositionEntryType::Exact,
            score: 0,
            best_move: None,
        }
    }

    pub fn new(hash: ZobristHash, depth: u8, flag: TranspositionEntryType, score: i32, best_move: Option<Move>) -> Self {
        Self {
            hash,
            depth,
            flag,
            score,
            best_move,
        }
    }
}


pub struct TranspositionTable {
    table: Vec<TranspositionEntry>,
    size: usize,
    hits: u64,
    misses: u64,
}

impl TranspositionTable {
    pub fn clear(&mut self) {
        self.table.clear();
        self.hits = 0;
        self.misses = 0;
    }

    #[inline(always)]
    pub fn get(&mut self, hash: &ZobristHash) -> Option<&TranspositionEntry> {
        let index = hash.0 as usize % self.size;

        if let Some(entry) = self.table.get(index) {
            if &entry.hash == hash {
                self.hits += 1;
                return Some(entry);
            }
        }
        self.misses += 1;
        None
    }

    #[inline(always)]
    pub fn insert(&mut self, entry: TranspositionEntry) {
        let index = entry.hash.0 as usize % self.size;
        if let Some(old_entry) = self.table.get(index) {
            if old_entry.depth > entry.depth {
                return
            }
        }
        self.table[index] = entry;
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        let size = 2^30;
        Self {
            table: (0..size).map(|_| TranspositionEntry::empty()).collect(),
            size,
            hits: 0,
            misses: 0,
        }
    }
}