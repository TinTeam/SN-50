//! CartridgeError implementation and manipulation.
use std::io;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::cartridge::chunk::ChunkType;

/// Cartridge errors.
#[derive(Error, Debug)]
pub enum CartridgeError {
    /// Error to represent invalid chunk types.
    #[error("invalid chunk type {0}")]
    InvalidChunkType(u8),
    /// Error to represent invalid chunk sizes.
    #[error("invalid chunk size {1} for type {0:?}, expected: {2:?}")]
    InvalidChunkSize(ChunkType, usize, Vec<usize>),
    /// Error to represent invalid chunk max sizes.
    #[error("invalid chunk size {1} for type {0:?}, max expected: {2}")]
    InvalidChunkMaxSize(ChunkType, usize, usize),
    /// Error to represent mismatched chunk sizes.
    #[error("mismatched chunk header size {1} and data sizes {2} for type {0:?}")]
    MismatchedChunkSizes(ChunkType, usize, usize),
    /// Error to wrap an invalid conversion to UTF8.
    #[error("UFT8 conversion error")]
    FromUtf8(#[from] FromUtf8Error),
    /// Error to wrap `io::Error`s from loading process.
    #[error("IO operation error")]
    Io(#[from] io::Error),
}

impl CartridgeError {
    /// Creates a `InvalidChunkType` error.
    pub fn new_invalid_chunk_type(chunk_type: u8) -> Self {
        Self::InvalidChunkType(chunk_type)
    }

    /// Creates a `InvalidChunkSize` error.
    pub fn new_invalid_chunk_size(
        chunk_type: ChunkType,
        value: usize,
        expected: Vec<usize>,
    ) -> Self {
        Self::InvalidChunkSize(chunk_type, value, expected)
    }

    /// Creates a `InvalidChunkMaxSize` error.
    pub fn new_invalid_chunk_max_size(
        chunk_type: ChunkType,
        value: usize,
        expected: usize,
    ) -> Self {
        Self::InvalidChunkMaxSize(chunk_type, value, expected)
    }

    /// Creates a `MismatchedChunkSizes` error.
    pub fn new_mismatched_chunk_sizes(
        chunk_type: ChunkType,
        header_size: usize,
        data_size: usize,
    ) -> Self {
        Self::MismatchedChunkSizes(chunk_type, header_size, data_size)
    }
}

pub type Result<T> = StdResult<T, CartridgeError>;

#[cfg(test)]
mod test_super {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_cartridgeerror_new_invalid_chunk_type() {
        let chunk_type = 99u8;

        let error = CartridgeError::new_invalid_chunk_type(chunk_type);

        assert_matches!(
            error,
            CartridgeError::InvalidChunkType(ct) if ct == chunk_type
        );
    }

    #[test]
    fn test_cartridgeerror_new_invalid_chunk_size() {
        let chunk_type = ChunkType::End;
        let value = 1usize;
        let expected = vec![0usize];

        let error = CartridgeError::new_invalid_chunk_size(chunk_type, value, expected.clone());

        assert_matches!(
            error,
            CartridgeError::InvalidChunkSize(ct, v, e) if ct == chunk_type && v == value && e == expected
        );
    }

    #[test]
    fn test_cartridgeerror_new_invalid_chunk_max_size() {
        let chunk_type = ChunkType::Code;
        let value = 140000usize;
        let expected = 131072usize;

        let error = CartridgeError::new_invalid_chunk_max_size(chunk_type, value, expected);

        assert_matches!(
            error,
            CartridgeError::InvalidChunkMaxSize(ct, v, e) if ct == chunk_type && v == value && e == expected
        );
    }

    #[test]
    fn test_cartridgeerror_new_mismatched_chunk_sizes() {
        let chunk_type = ChunkType::Code;
        let header_size = 10usize;
        let data_size = 15usize;

        let error = CartridgeError::new_mismatched_chunk_sizes(chunk_type, header_size, data_size);

        assert_matches!(
            error,
            CartridgeError::MismatchedChunkSizes(ct, h, d) if ct == chunk_type && h == header_size && d == data_size
        );
    }
}
