use crate::chunk::Chunk;


pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>
}

pub enum PngError {
    InvalidSignature,
    InvalidChunk,
}
impl Png {
    const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

}

impl TryFrom<&[u8]> for Png {
    type Error = PngError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        
        if bytes.len() < 8  || !bytes.starts_with(&Png::STANDARD_HEADER) {
            return Err(PngError::InvalidSignature);
        }
        let mut chunks: Vec<Chunk> = Vec::new();
        let mut cursor = 8;

        while cursor < bytes.len() {
            let length = u32::from_be_bytes(bytes[cursor..cursor + 4].try_into().unwrap())
                as usize;
                
            let total_chunk_len = 4 + 4 + length + 4;

            let chunk_bytes = &bytes[cursor..cursor + total_chunk_len];
            let chunk = Chunk::try_from(chunk_bytes).unwrap();
            chunks.push(chunk);

            cursor += total_chunk_len;
        }

        Ok(Png{
            header: Png::STANDARD_HEADER,
            chunks: chunks,
        })
    }
}