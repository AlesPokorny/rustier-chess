use std::fmt;

#[derive(Clone, Copy)]
pub struct BitMap {
    value: u64,
}

impl BitMap {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn read_bit(&self, bit: u8) -> bool {
        self.value & (1 << bit) != 0
    }

    pub fn set_zero(&mut self, bit: u8) {
        self.value &= !(1 << bit);
    }

    pub fn set_one(&mut self, bit: u8) {
        self.value |= 1 << bit;
    }

    pub fn get_ones(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut le_clone = *self;

        let mut n_zeros = le_clone.value.trailing_zeros() as u8;

        loop {
            if n_zeros == 64 {
                return result;
            }
            result.push(n_zeros);
            le_clone.set_zero(n_zeros);
            n_zeros = le_clone.value.trailing_zeros() as u8;
        }
    }

    pub fn get_zeros(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut le_clone = *self;

        let mut n_zeros = le_clone.value.trailing_ones() as u8;

        loop {
            if n_zeros == 64 {
                return result;
            }
            result.push(n_zeros);
            le_clone.set_one(n_zeros);
            n_zeros = le_clone.value.trailing_ones() as u8;
        }
    }
}

impl fmt::Display for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for BitMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.value, f)
    }
}

#[cfg(test)]
mod test_bitmap {
    use super::*;

    #[test]
    fn test_read_bit() {
        let bitmap = BitMap::new(5);

        assert_eq!(bitmap.read_bit(0), true);
        assert_eq!(bitmap.read_bit(1), false);
        assert_eq!(bitmap.read_bit(2), true);
        assert_eq!(bitmap.read_bit(3), false);
    }

    #[test]
    fn test_set_zero() {
        let mut bitmap = BitMap::new(5);

        bitmap.set_zero(2);
        assert_eq!(bitmap.value, 1);

        bitmap.set_zero(1);
        bitmap.set_zero(2);
        assert_eq!(bitmap.value, 1);
    }

    #[test]
    fn test_set_one() {
        let mut bitmap = BitMap::new(5);

        bitmap.set_one(1);
        assert_eq!(bitmap.value, 7);

        bitmap.set_one(1);
        bitmap.set_one(2);
        assert_eq!(bitmap.value, 7);
    }

    #[test]
    fn test_get_ones() {
        let bitmap = BitMap::new(5);

        let ones = bitmap.get_ones();

        assert_eq!(ones, vec![0, 2]);
    }

    #[test]
    fn test_get_zeros() {
        let bitmap = BitMap::new(5);

        let ones = bitmap.get_zeros();
        let expected_result: Vec<u8> = (0..64).filter(|x| ![0, 2].contains(x)).collect();

        assert_eq!(ones, expected_result);
    }
}
