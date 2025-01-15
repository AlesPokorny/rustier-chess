use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Clone, Copy)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn read_bit(&self, bit: u8) -> bool {
        self.0 & (1 << bit) != 0
    }

    pub fn set_zero(&mut self, bit: u8) {
        self.0 &= !(1 << bit);
    }

    pub fn set_one(&mut self, bit: u8) {
        self.0 |= 1 << bit;
    }

    pub fn get_ones(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut le_clone = *self;

        let mut n_zeros = le_clone.0.trailing_zeros() as u8;

        loop {
            if n_zeros == 64 {
                return result;
            }
            result.push(n_zeros);
            le_clone.set_zero(n_zeros);
            n_zeros = le_clone.0.trailing_zeros() as u8;
        }
    }

    pub fn get_zeros(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut le_clone = *self;

        let mut n_zeros = le_clone.0.trailing_ones() as u8;

        loop {
            if n_zeros == 64 {
                return result;
            }
            result.push(n_zeros);
            le_clone.set_one(n_zeros);
            n_zeros = le_clone.0.trailing_ones() as u8;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..8_u8 {
            for j in 0..8_u8 {
                write!(f, "{}", self.read_bit(i * 8 + j) as u8)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Binary for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 = self.0 & rhs.0
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 = self.0 | rhs.0
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 = self.0 ^ rhs.0
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

#[cfg(test)]
mod test_bitboard {
    use super::*;

    #[test]
    fn test_read_bit() {
        let bitboard = BitBoard::new(5);

        assert_eq!(bitboard.read_bit(0), true);
        assert_eq!(bitboard.read_bit(1), false);
        assert_eq!(bitboard.read_bit(2), true);
        assert_eq!(bitboard.read_bit(3), false);
    }

    #[test]
    fn test_set_zero() {
        let mut bitboard = BitBoard::new(5);

        bitboard.set_zero(2);
        assert_eq!(bitboard.0, 1);

        bitboard.set_zero(1);
        bitboard.set_zero(2);
        assert_eq!(bitboard.0, 1);
    }

    #[test]
    fn test_set_one() {
        let mut bitboard = BitBoard::new(5);

        bitboard.set_one(1);
        assert_eq!(bitboard.0, 7);

        bitboard.set_one(1);
        bitboard.set_one(2);
        assert_eq!(bitboard.0, 7);
    }

    #[test]
    fn test_get_ones() {
        let bitboard = BitBoard::new(5);

        let ones = bitboard.get_ones();

        assert_eq!(ones, vec![0, 2]);
    }

    #[test]
    fn test_get_zeros() {
        let bitboard = BitBoard::new(5);

        let ones = bitboard.get_zeros();
        let expected_result: Vec<u8> = (0..64).filter(|x| ![0, 2].contains(x)).collect();

        assert_eq!(ones, expected_result);
    }
}
