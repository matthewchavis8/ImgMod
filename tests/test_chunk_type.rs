#[allow(dead_code)]

/**
 * Unit tests for the `ChunkType` struct.
 *
 * This module contains tests to verify the correctness of `ChunkType`'s methods, ensuring that
 * chunk type construction and property checks work as expected.
 *
 * Tests:
 *
 * - `test_construct_chunk_type` - Checks if `try_from(bytes)` correctly constructs a `ChunkType`.
 * - `test_construct_from_string` - Verifies that a `ChunkType` can be correctly constructed from a string.
 * - `test_is_critical` - Checks if a chunk is correctly identified as critical.
 * - `test_is_private` - Checks if a chunk is correctly identified as private.
 * - `test_is_reserved` - Ensures that the reserved bit validation works correctly.
 * - `test_is_safe_to_copy` - Checks if a chunk is correctly identified as safe to copy.
 * - `test_is_valid` - Ensures that a chunk is considered valid only if it meets all required conditions.
 */

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use imgmod::png::chunk_type::ChunkType;

    #[test]
    fn test_construct_chunk_type() {
        let expected = [77, 97, 116, 116];
        let bytes: [u8; 4] = [77, 97, 116, 116];

        let actual = ChunkType::try_from(bytes).unwrap();
        
        assert_eq!(expected, actual.bytes());
    }
    
    #[test]
    fn test_construct_from_string() {
        let expected: [u8; 4] = [77, 97, 116, 116];
        let actual = ChunkType::from_str("Matt").unwrap();

        assert_eq!(expected, actual.bytes());
    }
    
    #[test]
    fn test_is_critical() {
        let chunk = ChunkType::from_str("Matt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    fn test_is_private() {
        let chunk = ChunkType::from_str("Matt").unwrap();
        assert!(!chunk.is_private());
    }

    #[test]
    fn test_is_reserved() {
        let chunk = ChunkType::from_str("Matt").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    fn test_is_safe_to_copy() {
        let chunk = ChunkType::from_str("Matt").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    fn test_is_valid() {
        let chunk = ChunkType::from_str("Matt").unwrap();
        assert!(!chunk.is_valid());
    }
}