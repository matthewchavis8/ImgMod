use std::{fmt::{self, Display}, str::{from_utf8, Utf8Error}};
use std::str::FromStr;
use crate::png::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    chunk_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

#[derive(Debug)]
pub enum ChunkError {
    ConversionError,
}

pub fn u8_4_from_slice(bytes: &[u8]) -> [u8; 4] {
    bytes.try_into().expect("error converting")
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
impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Chunk {

        let crc_algorithm = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        let mut digest = crc_algorithm.digest();
        digest.update(&chunk_type.bytes());
        digest.update(&chunk_data);
        let crc = digest.finalize();

        Chunk { 
            chunk_length: chunk_data.len() as u32, 
            chunk_type: chunk_type, 
            chunk_data: chunk_data, 
            crc: crc, 
        }
    }

    pub fn length(&self) -> u32 {
        self.chunk_length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        self.chunk_data.as_slice()
    }

    pub fn data_as_string(&self) -> Result<String, Utf8Error> {
        let string = from_utf8(&self.chunk_data);

        match string {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            self.chunk_length.to_be_bytes().as_ref(),
            &self.chunk_type.bytes(),
            &self.chunk_data,
            self.crc.to_be_bytes().as_ref(),
        ]
        .concat()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn from_strings(chunk_type: &str, data: &str) -> Result<Chunk, ChunkError> {
        let chunk_type = ChunkType::from_str(chunk_type).map_err(|_| ChunkError::ConversionError)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }
    
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 12 {
            return Err(ChunkError::ConversionError)
        }

        let chunk_length = u32::from_be_bytes(u8_4_from_slice(&bytes[0..4])) as usize;
        let chunk_type = ChunkType::try_from(u8_4_from_slice(&bytes[4..8]))
            .map_err(|_| ChunkError::ConversionError)?;
        let chunk_data = bytes[8..8 + chunk_length].to_vec();
        
        let crc_start = 8 + chunk_length;
        let crc_end = crc_start + 4;

        let crc = u32::from_be_bytes(u8_4_from_slice(&bytes[crc_start..crc_end]));

        Ok(Chunk { 
            chunk_length: chunk_length as u32,
            chunk_type: chunk_type, 
            chunk_data: chunk_data,
            crc: crc,
        })

    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chunk\n{{\n")?;
        write!(
            f,
            "\tlength: {}, chunk_type: {}\n\tdata: {:?}\n\tcrc: {}\n}}",
            self.chunk_length, self.chunk_type, self.chunk_data, self.crc
        )
    }
}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ChunkError {}