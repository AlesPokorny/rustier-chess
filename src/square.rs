use std::{fmt::Display, str::FromStr, string::ParseError};

pub struct Aaa;

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
        write!(f, "{}", self.0)
    }
}
