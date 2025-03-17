use std::
{
    str::FromStr,
    fmt::Display
};


#[allow(unused_variables)]
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType (
    [u8; 4]
);

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidCharacter,
    InvalidLength,
    DisplayError,
}

impl ChunkType {
    
    // Returns the bytes in our tuple struict
    pub fn bytes(&self) -> [u8; 4]  {
        self.0
    }

    // Checks if our chunk type is valid
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() && self.0.iter().all(|b| b.is_ascii())
    }

    pub fn is_critical(&self) -> bool {
        let byte_1 = self.0[0];
        let bit_5 = (byte_1 >> 5) & 1;

        bit_5 == 0
    }

    // Checks the 5th bit of byte 2 if it is public or private
    pub fn is_public(&self) -> bool {
        let byte_2 = self.0[1];
        let bit_5 = (byte_2 >> 5) & 1;

        bit_5 == 0
    }

    // Checks the 5th bit of byte 3 if it is valid or not
    pub fn is_reserved_bit_valid(&self) -> bool {
        let byte_3 = self.0[2];
        let bit_5 = (byte_3 >> 5) & 1;
        
        println!("{}", bit_5);
        bit_5 == 0
    }

    // Checks the 5th bit of byte 4 to see if it is safe to copy
    pub fn is_safe_to_copy(&self) -> bool {
        let byte_4 = self.0[3];
        let bit_5 = (byte_4 >> 5) & 1;

        bit_5 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;

    
    /**
     * takes in a chunk and checks if the chunk is valid or not
     * Returns a Result<Self, ChunkTypeError>
     * if it is valid meaning all alphanumberic characters we will return Ok()
     * else not valid so return an Error
     * 
     * @param bytes takes in a chunk
     * @ return Result<self, Self::Errror>
     */
    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if bytes.iter().all(|byte| (*byte as char).is_ascii_alphabetic()) {
            Ok(ChunkType(bytes))
        } else {
            Err(ChunkTypeError::InvalidCharacter)
        }
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;

    /**
     * Takes in a string slice and checks if it is 4 in length.
     * then attemp to try and construct a Chunk from it
     * 
     * @param str string slice representing a chunk
     */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        if s.len() != 4 { 
            return Err(ChunkTypeError::InvalidLength) 
        }

        let bytes: [u8; 4] = s.as_bytes().try_into().unwrap();

        ChunkType::try_from(bytes)
    }
}

// Displays the [ChunkType] using its string representation of bytes
impl Display for ChunkType {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match str::from_utf8(&self.bytes()) {
            Ok(string) => write!(f, "{}", string),
            Err(_) => write!(f, "Invalid UTF-8")
        }
    }
}
