pub fn coordinate_to_bit(coordinate: &str) -> u8 {
    let chars: Vec<char> = coordinate.chars().collect();

    if chars[0].is_ascii_uppercase() {
        panic!("Chess coordinates should be lowercase")
    }

    (chars[0].to_ascii_lowercase() as u8 - 97) * 8 + chars[1].to_digit(10).unwrap() as u8 - 1
}

#[cfg(test)]
mod test_utils {
    use super::*;

    #[test]
    fn test_coordinate_to_bit() {
        assert_eq!(coordinate_to_bit("a1"), 0);
        assert_eq!(coordinate_to_bit("h8"), 63);
    }
}
