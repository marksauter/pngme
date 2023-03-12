#![allow(dead_code)]
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
        // self.0[0] & 0b00001000 != 0b00001000
    }

    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
        // self.0[1] & 0b00001000 != 0b00001000
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
        // self.0[2] & 0b00001000 != 0b00001000
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
        // self.0[3] & 0b00001000 == 0b00001000
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = crate::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if value.iter().all(|c| c.is_ascii_alphabetic()) {
            Ok(ChunkType(value))
        } else {
            Err("Invalid ChunkType".into())
        }
    }
}

impl FromStr for ChunkType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes().iter().all(|c| c.is_ascii_alphabetic()) {
            Ok(ChunkType(s.as_bytes().try_into()?))
        } else {
            Err("Invalid ChunkType".into())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.0) {
            Ok(c) => {
                f.write_fmt(format_args!("{}", c))?;
                Ok(())
            }
            Err(_e) => Err(std::fmt::Error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
