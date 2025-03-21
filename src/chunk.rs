use crate::chunk_type::ChunkType;

pub struct Chunk {
    chunk_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

pub enum ChunkError {
    ConversionError,
}

pub fn u8_4_from_slice(bytes: &[u8]) -> [u8; 4] {
    bytes.try_into().expect("error converting")
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