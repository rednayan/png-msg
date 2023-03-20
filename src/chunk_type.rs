use std::{convert::TryFrom, fmt::Display, str::FromStr};
use crate::Error;
// use crate::Result;
#[derive(Debug,Clone, PartialEq,Eq)]
pub struct ChunkType {
    bytes : [u8;4],
}

impl ChunkType { 
    fn bytes(&self) -> [u8;4] {
        return self.bytes;
    }
    
    fn is_valid(&self) -> bool {
        let valid_chars: bool = self.bytes
                                    .iter()
                                    .all(|&b: &u8| (b >= b'a' && b < b'z' || (b >= b'A' && b <= b'Z')));
        
        return valid_chars && self.is_reserved_bit_valid();
    }
    
    fn is_critical(&self) -> bool {
        return (self.bytes[0] & 0x20) != 0x20;
    }
    
    fn is_public(&self) -> bool {
        return (self.bytes[1] & 0x20) != 0x20;
    }
    
    fn is_reserved_bit_valid(&self) -> bool {
        return (self.bytes[2] & 0x20) != 0x20;
    }
    
    fn is_safe_to_copy(&self) -> bool {
        return (self.bytes[3] & 0x20) == 0x20;
    }
}

impl TryFrom<[u8;4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8;4]) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            bytes : value
        })
    }
}


impl FromStr for ChunkType {
    type Err  = Error;

    fn from_str(s: &str) -> Result<Self,Self::Err> {
        let bytes_arr: &[u8] = s.as_bytes();

        if bytes_arr.len() != 4 {
            return Err(Box::new(ChunkTypeError::ByteLengthError(bytes_arr.len())));
        }

        let valid_chars = bytes_arr.iter().all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')));

        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }

        let sized: [u8;4] = [bytes_arr[0],bytes_arr[1],bytes_arr[2],bytes_arr[3]];
        Ok(ChunkType::try_from(sized)?)
     }
}

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.bytes).map_err(|_| std::fmt::Error)?;
        write!(f,"{}",s)
    }
}


#[derive(Debug)]
pub enum ChunkTypeError {
    ByteLengthError(usize),
    InvalidCharacter
}

impl std::error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result  {
        match self {
            ChunkTypeError::ByteLengthError(actual) => write!(
                f,
                "Expected 4 bytes byt received {} when creating chunk type", actual
            ),
            ChunkTypeError::InvalidCharacter => {
                write!(f,"Input contains one or more invalid characters")
            }
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
















