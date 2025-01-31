use std::{
    fmt::Display,
    ops::{Add, Shl, Shr, Sub},
    str::FromStr,
    string::ParseError,
};

use serde_derive::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, Deserialize, Serialize)]
pub struct Square(u8);

impl Square {
    pub fn new(bit: u8) -> Self {
        Self(bit)
    }

    pub fn as_u8(&self) -> u8 {
        self.0
    }

    pub fn as_u16(&self) -> u16 {
        self.0 as u16
    }

    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn sub(&self, rhs: u8) -> Self {
        Self(self.0 - rhs)
    }

    pub fn add(&self, rhs: u8) -> Self {
        Self(self.0 + rhs)
    }

    pub fn get_rank(&self) -> u8 {
        self.0 / 8
    }

    pub fn get_file(&self) -> u8 {
        self.0 % 8
    }

    pub fn get_bit_index(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }
}

impl FromStr for Square {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let col = chars.next().unwrap().to_ascii_lowercase() as u8 - 97;
        let row = (chars.next().unwrap().to_digit(10).unwrap() - 1) * 8;

        Ok(Self(col + row as u8))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = (self.get_file() + 97) as char;
        let row = char::from_digit((self.get_rank() + 1) as u32, 10).unwrap();

        write!(f, "{}{}", file, row)?;

        Ok(())
    }
}

impl Add<u8> for Square {
    type Output = Self;

    fn add(self, rhs: u8) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<u8> for Square {
    type Output = Self;

    fn sub(self, rhs: u8) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Shl<u8> for Square {
    type Output = Self;

    fn shl(self, rhs: u8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<u8> for Square {
    type Output = Self;

    fn shr(self, rhs: u8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Shl<i8> for Square {
    type Output = Self;

    fn shl(self, rhs: i8) -> Self::Output {
        Self(self.0 << rhs)
    }
}

impl Shr<i8> for Square {
    type Output = Self;

    fn shr(self, rhs: i8) -> Self::Output {
        Self(self.0 >> rhs)
    }
}

impl Add<i8> for Square {
    type Output = Self;

    fn add(self, rhs: i8) -> Self::Output {
        let int = self.0 as i8;
        if rhs < 0 && rhs.abs() > int {
            panic!("Boomsies, tried to pass {} to square {}", rhs, self)
        }

        Self((int + rhs) as u8)
    }
}
