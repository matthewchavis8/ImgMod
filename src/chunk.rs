use std::{fmt::Display, io::Read, str::{from_utf8, Utf8Error}};

use crate::chunk_type::{self, ChunkType};
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

    pub fn as_bytes(&self) -> &Vec<u8> {
        &self.chunk_data
    }

    pub fn crc(&self) -> u32 {
        self.crc
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