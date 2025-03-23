use crate::chunk::Chunk;
use std::fmt::Display;

pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>
}

#[derive(Debug)]
pub enum PngError {
    InvalidSignature,
    InvalidChunk,
}
impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Png {
        Png {
            header: Png::STANDARD_HEADER,
            chunks: chunks
        }
    }
    
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    pub fn remove_first_chunk(&mut self, chunk_type: &str) -> Result<Chunk, PngError> {
        if let Some(idx) = self.chunks
            .iter()
            .position(|c| c.chunk_type().to_string() == chunk_type) {
            Ok(self.chunks.remove(idx))
        } else {
            Err(PngError::InvalidChunk)
        }
    } 

    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks.iter().find(|c| c.chunk_type().to_string() == chunk_type)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        [
            Png::STANDARD_HEADER.to_vec(),
            self.chunks()
                .iter()
                .flat_map(|chunk| chunk.as_bytes())
                .collect(),
        ]
        .concat()
    }
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

impl Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "header: {:?}\nchunks:\n", Png::STANDARD_HEADER)?;
        write!(f, "[\n")?;
        for chunk in self.chunks.iter() {
            write!(f, "{}\n", chunk)?;
        }
        write!(f, "]")?;

        Ok(())
    }
}
