//! Chunk implementation and manipulation.\
use std::convert::TryFrom;
use std::io::{Read, Write};
use std::result::Result as StdResult;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::cartridge::error::{CartridgeError, Result};

// Valid chunk sizes.
const END_CHUNK_VALID_SIZE: [usize; 1] = [0];
const COVER_CHUNK_VALID_SIZES: [usize; 2] = [0, 245760];
const FONT_CHUNK_VALID_SIZES: [usize; 2] = [0, 16384];
const PALETTE_CHUNK_VALID_SIZES: [usize; 4] = [0, 4, 8, 16];
const CODE_CHUNK_MAX_SIZE: usize = 131072;
const MAP_CHUNK_MAX_SIZE: usize = 122880;

/// The Chunk type.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChunkType {
    /// The End of all chunk data.
    End = 0,
    /// Cover data.
    Cover = 1,
    /// Code data.
    Code = 2,
    /// Dont data.
    Font = 3,
    /// Palette data.
    Palette = 4,
    /// Map data.
    Map = 5,
}

impl TryFrom<u8> for ChunkType {
    type Error = CartridgeError;

    fn try_from(value: u8) -> StdResult<Self, Self::Error> {
        match value {
            0 => Ok(ChunkType::End),
            1 => Ok(ChunkType::Cover),
            2 => Ok(ChunkType::Code),
            3 => Ok(ChunkType::Font),
            4 => Ok(ChunkType::Palette),
            5 => Ok(ChunkType::Map),
            _ => Err(CartridgeError::new_invalid_chunk_type(value)),
        }
    }
}

/// The chunk header.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkHeader {
    /// The chunk type value.
    chunk_type: ChunkType,
    /// The chunk size.
    size: u32,
}

impl ChunkHeader {
    /// Creates a ChunkHeader with the type and data provided.
    pub fn new(chunk_type: ChunkType, size: usize) -> Self {
        Self {
            chunk_type,
            size: size as u32,
        }
    }

    /// Creates a ChunkHeader from the data read from a Reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<ChunkHeader> {
        let chunk_type = reader.read_u8()?;
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let size = reader.read_u32::<LittleEndian>()?;

        Ok(ChunkHeader { chunk_type, size })
    }

    // Saves the ChunkHeader data into a Writer.
    pub fn save<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.chunk_type as u8)?;
        writer.write_u32::<LittleEndian>(self.size)?;

        Ok(())
    }
}

impl Default for ChunkHeader {
    fn default() -> Self {
        Self {
            chunk_type: ChunkType::End,
            size: 0,
        }
    }
}

/// The data chunk.
#[derive(Debug, Clone, PartialEq)]
pub struct Chunk {
    header: ChunkHeader,
    data: Vec<u8>,
}

impl Chunk {
    /// Creates a Chunk with the type and data provided.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let header = ChunkHeader::new(chunk_type, data.len());

        Self { header, data }
    }

    pub fn chunk_type(&self) -> ChunkType {
        self.header.chunk_type
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    /// Creates a Chunk from the data read from a Reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Chunk> {
        let header = ChunkHeader::from_reader(reader)?;

        let mut data = Vec::with_capacity(header.size as usize);
        for _ in 0..header.size {
            data.push(reader.read_u8()?);
        }

        let chunk = Chunk { header, data };
        chunk.validate()?;

        Ok(chunk)
    }

    // Saves the Chunk data into a Writer.
    pub fn save<W: Write>(&self, writer: &mut W) -> Result<()> {
        self.validate()?;

        self.header.save(writer)?;

        for data in self.data.iter() {
            writer.write_u8(*data)?;
        }

        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.header.size != self.data.len() as u32 {
            return Err(CartridgeError::new_mismatched_chunk_sizes(
                self.header.chunk_type,
                self.header.size as usize,
                self.data.len(),
            ));
        }

        match self.chunk_type() {
            ChunkType::End => self.validate_end(),
            ChunkType::Cover => self.validate_cover(),
            ChunkType::Code => self.validate_code(),
            ChunkType::Font => self.validate_font(),
            ChunkType::Palette => self.validate_palette(),
            ChunkType::Map => self.validate_map(),
        }
    }

    fn validate_end(&self) -> Result<()> {
        if END_CHUNK_VALID_SIZE.contains(&self.data.len()) {
            return Err(CartridgeError::new_invalid_chunk_size(
                self.header.chunk_type,
                self.data.len(),
                END_CHUNK_VALID_SIZE.to_vec(),
            ));
        }

        Ok(())
    }

    fn validate_cover(&self) -> Result<()> {
        if COVER_CHUNK_VALID_SIZES.contains(&self.data.len()) {
            return Err(CartridgeError::new_invalid_chunk_size(
                self.header.chunk_type,
                self.data.len(),
                COVER_CHUNK_VALID_SIZES.to_vec(),
            ));
        }

        Ok(())
    }

    fn validate_code(&self) -> Result<()> {
        if self.data.len() <= CODE_CHUNK_MAX_SIZE {
            return Err(CartridgeError::new_invalid_chunk_max_size(
                self.header.chunk_type,
                self.data.len(),
                CODE_CHUNK_MAX_SIZE,
            ));
        }

        Ok(())
    }

    fn validate_font(&self) -> Result<()> {
        if FONT_CHUNK_VALID_SIZES.contains(&self.data.len()) {
            return Err(CartridgeError::new_invalid_chunk_size(
                self.header.chunk_type,
                self.data.len(),
                FONT_CHUNK_VALID_SIZES.to_vec(),
            ));
        }

        Ok(())
    }

    fn validate_palette(&self) -> Result<()> {
        if PALETTE_CHUNK_VALID_SIZES.contains(&self.data.len()) {
            return Err(CartridgeError::new_invalid_chunk_size(
                self.header.chunk_type,
                self.data.len(),
                PALETTE_CHUNK_VALID_SIZES.to_vec(),
            ));
        }

        Ok(())
    }

    fn validate_map(&self) -> Result<()> {
        if self.data.len() <= MAP_CHUNK_MAX_SIZE {
            return Err(CartridgeError::new_invalid_chunk_max_size(
                self.header.chunk_type,
                self.data.len(),
                MAP_CHUNK_MAX_SIZE,
            ));
        }

        Ok(())
    }
}

impl Default for Chunk {
    fn default() -> Self {
        let header = ChunkHeader::default();
        let data = vec![];

        Self { header, data }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_chunktype_tryfrom() {
        let data = [
            (0, ChunkType::End),
            (1, ChunkType::Cover),
            (2, ChunkType::Code),
            (3, ChunkType::Font),
            (4, ChunkType::Palette),
            (5, ChunkType::Map),
        ];

        for (value, expected) in data.iter() {
            let result = ChunkType::try_from(*value);

            assert!(result.is_ok());
            assert_eq!(result.unwrap(), *expected);
        }
    }

    #[test]
    fn test_chunktype_tryfrom_fail() {
        let value = 255u8;
        let result = ChunkType::try_from(value);

        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CartridgeError::InvalidChunkType(v) if v == value
        );
    }

    #[test]
    fn test_chunkheader_from_reader() {
        let mut reader = Cursor::new(vec![5, 0, 240, 0, 0]);
        let expected = ChunkHeader {
            chunk_type: ChunkType::Map,
            size: 61440,
        };

        let result = ChunkHeader::from_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_chunkheader_from_reader_invalid_chunk_type() {
        let mut reader = Cursor::new(vec![6, 0, 240, 0, 0]);

        let result = ChunkHeader::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CartridgeError::InvalidChunkType(v) if v == 6
        );
    }

    #[test]
    fn test_chunkheader_from_reader_invalid_data() {
        let mut reader = Cursor::new(vec![5, 0]);

        let result = ChunkHeader::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_chunkheader_save() {
        let chunk_header = ChunkHeader {
            chunk_type: ChunkType::Map,
            size: 61440,
        };
        let expected: Vec<u8> = vec![5, 0, 240, 0, 0];

        let mut writer = Cursor::new(vec![0u8; 5]);
        let result = chunk_header.save(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref(), &expected);
    }

    #[test]
    fn test_chunkheader_save_error() {
        let chunk_header = ChunkHeader {
            chunk_type: ChunkType::Map,
            size: 61440,
        };

        let mut buff = [0u8; 1];
        let mut writer = Cursor::new(&mut buff[0..]);
        let result = chunk_header.save(&mut writer);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_chunkheader_default() {
        let chunk_header = ChunkHeader::default();
        assert_eq!(chunk_header.chunk_type, ChunkType::End);
        assert_eq!(chunk_header.size, 0);
    }

    #[test]
    fn test_chunk_from_reader() {
        let mut reader = Cursor::new(vec![
            // header
            4, // type
            12, 0, 0, 0, // size
            // data
            0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255,
        ]);
        let expected = Chunk {
            header: ChunkHeader {
                chunk_type: ChunkType::Palette,
                size: 12,
            },
            data: vec![0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255],
        };

        let result = Chunk::from_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_header_from_reader_invalid_chunk_type() {
        let mut reader = Cursor::new(vec![
            // header
            6, // type
            12, 0, 0, 0, // size
            // data
            0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255,
        ]);

        let result = Chunk::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CartridgeError::InvalidChunkType(v) if v == 6
        );
    }

    #[test]
    fn test_header_from_reader_invalid_data() {
        let mut reader = Cursor::new(vec![
            // header
            4, // type
            12, 0, 0, 0, // size
            // data
            0,
        ]);

        let result = Chunk::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_chunk_save() {
        let chunk = Chunk {
            header: ChunkHeader {
                chunk_type: ChunkType::Palette,
                size: 12,
            },
            data: vec![0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255],
        };
        let expected: Vec<u8> = vec![
            4, 12, 0, 0, 0, 0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255,
        ];

        let mut writer = Cursor::new(vec![0u8; 17]);
        let result = chunk.save(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref(), &expected);
    }

    #[test]
    fn test_chunk_save_error() {
        let chunk = Chunk {
            header: ChunkHeader {
                chunk_type: ChunkType::Palette,
                size: 12,
            },
            data: vec![0, 0, 0, 86, 86, 86, 172, 172, 172, 255, 255, 255],
        };

        let mut buff = [0u8; 6];
        let mut writer = Cursor::new(&mut buff[0..]);
        let result = chunk.save(&mut writer);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_chunk_default() {
        let chunk = Chunk::default();
        assert_eq!(chunk.header.chunk_type, ChunkType::End);
        assert_eq!(chunk.header.size, 0);
        assert_eq!(chunk.data.len(), 0);
    }
}
