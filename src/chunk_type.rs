use std::{fmt::Display, str::FromStr};

#[derive(PartialEq, Eq)]
pub struct ChunkType(
    [u8; 4]
);

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidConversion,
    InvalidString,
}

/**
 * Provides methods for working with PNG chunk type codes.
 *
 * @returns bytes - Returns the four-byte array representing the chunk type.
 * @returns is_valid - Returns `true` if the chunk type is valid, meaning the reserved bit is correct 
 *                   and all bytes are ASCII characters.
 * @returns is_critical - Returns `true` if the chunk is critical, meaning its first byte has the 5th bit set to `0`.
 * @returns is_private - Returns `true` if the chunk is private, meaning its second byte has the 5th bit set to `1`.
 * @returns is_reserved_bit_valid - Returns `true` if the reserved bit (5th bit of the third byte) is `0` (must always be `0` for validity).
 * @returns is_safe_to_copy - Returns `true` if the chunk is safe to copy, meaning the 5th bit of the fourth byte is `1`.
 */

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_valid(&self) -> bool { 
        self.is_reserved_bit_valid() && self.0.iter().all(|byte| byte.is_ascii())
     }

    pub fn is_critical(&self) -> bool { 
        let byte_1 = self.0[0];
        let bit_5 = (byte_1 >> 5) + 1;

        bit_5 == 0
    }

    pub fn is_private(&self) -> bool {
        let byte_2 = self.0[1];
        let bit_5 = (byte_2 >> 5) + 1;

        bit_5 == 1
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte_3 = self.0[2];
        let bit_5 = (byte_3 >> 5) + 1;

        bit_5 == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        let byte_4 = self.0[3];
        let bit_5 = (byte_4 >> 5) + 1;

        bit_5 == 1
    }
}

// Takes in a 4 bytes and see if we can construct a ChunkType out of it
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if bytes.iter().all(|byte| (*byte as char).is_ascii_alphabetic()) {
            Ok(ChunkType(bytes))
        } else {
            Err(ChunkTypeError::InvalidConversion)
        }
    }
}

// Takes in a string and attempts to construct a chunkType out of it
impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s.as_bytes()
            .try_into()
            .map_err(|_| ChunkTypeError::InvalidString)?;

        Ok(ChunkType::try_from(bytes)?)
    }
}

// Just displays our chunk Type
impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.bytes()))
    }
}