//! Cartridge utilities.
mod chunk;
mod error;

pub use crate::cartridge::error::{CartridgeError, Result};

use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::cartridge::chunk::{Chunk, ChunkType};

/// The default cartridge file version.
const DEFAULT_CART_FILE_VERSION: u8 = 1;
/// The default name size.
const DEFAULT_NAME_SIZE: u8 = 64;
/// The default description size.
const DEFAULT_DESC_SIZE: u16 = 512;
/// The default author name size.
const DEFAULT_AUTHOR_SIZE: u8 = 64;
/// The default game version.
const DEFAULT_VERSION: u8 = 1;

/// The cartridge header.
#[derive(Debug, Clone, PartialEq)]
struct CartridgeHeader {
    pub cart_version: u8,
    pub name_size: u8,
    pub desc_size: u16,
    pub author_size: u8,
}

impl CartridgeHeader {
    /// Creates a CartridgeHeader from the data read from a Reader.
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<CartridgeHeader> {
        let cart_version = reader.read_u8()?; // TODO validate the version
        let name_size = reader.read_u8()?;
        let desc_size = reader.read_u16::<LittleEndian>()?;
        let author_size = reader.read_u8()?;

        Ok(CartridgeHeader {
            cart_version,
            name_size,
            desc_size,
            author_size,
        })
    }

    /// Saves the CartridgeHeader data into a Writer.
    pub fn save<W: Write>(&self, writer: &mut W) -> Result<()> {
        writer.write_u8(self.cart_version)?;
        writer.write_u8(self.name_size)?;
        writer.write_u16::<LittleEndian>(self.desc_size)?;
        writer.write_u8(self.author_size)?;

        Ok(())
    }
}

impl Default for CartridgeHeader {
    fn default() -> Self {
        Self {
            cart_version: DEFAULT_CART_FILE_VERSION,
            name_size: DEFAULT_NAME_SIZE,
            desc_size: DEFAULT_DESC_SIZE,
            author_size: DEFAULT_AUTHOR_SIZE,
        }
    }
}

/// The cartridge data.
#[derive(Debug, Clone, PartialEq)]
pub struct Cartridge {
    pub version: u8,
    pub name: String,
    pub desc: String,
    pub author: String,
    pub cover: Vec<u8>,
    pub font: Vec<u8>,
    pub palette: Vec<u8>,
    pub map: Vec<u8>,
    pub code: String,
}

impl Cartridge {
    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Cartridge> {
        let mut cart = Cartridge::default();
        let header = CartridgeHeader::from_reader(reader)?;

        cart.version = reader.read_u8()?;

        let mut name = vec![0u8; header.name_size as usize];
        reader.read_exact(&mut name)?;
        cart.name = String::from_utf8(name)?;

        let mut desc = vec![0u8; header.desc_size as usize];
        reader.read_exact(&mut desc)?;
        cart.desc = String::from_utf8(desc)?;

        let mut author = vec![0u8; header.author_size as usize];
        reader.read_exact(&mut author)?;
        cart.author = String::from_utf8(author)?;

        loop {
            let chunk = Chunk::from_reader(reader)?;

            match chunk.chunk_type() {
                ChunkType::End => {
                    break;
                }
                ChunkType::Cover => {
                    cart.cover = chunk.data().clone();
                }
                ChunkType::Code => {
                    cart.code = String::from_utf8(chunk.data().clone())?;
                }
                ChunkType::Font => {
                    cart.font = chunk.data().clone();
                }
                ChunkType::Palette => {
                    cart.palette = chunk.data().clone();
                }
                ChunkType::Map => {
                    cart.map = chunk.data().clone();
                }
            }
        }

        Ok(cart)
    }

    pub fn save<W: Write>(&self, writer: &mut W) -> Result<()> {
        let header = CartridgeHeader {
            name_size: self.name.len() as u8,
            desc_size: self.desc.len() as u16,
            author_size: self.author.len() as u8,
            ..Default::default()
        };
        header.save(writer)?;

        writer.write_u8(self.version)?;
        writer.write_all(self.name.as_bytes())?;
        writer.write_all(self.desc.as_bytes())?;
        writer.write_all(self.author.as_bytes())?;

        let chunks = vec![
            (self.cover.clone(), ChunkType::Cover),
            (self.code.as_bytes().to_vec(), ChunkType::Code),
            (self.font.clone(), ChunkType::Font),
            (self.palette.clone(), ChunkType::Palette),
            (self.map.clone(), ChunkType::Map),
        ];

        for (data, chunk_type) in chunks.into_iter().filter(|(d, _)| !d.is_empty()) {
            let chunk = Chunk::new(chunk_type, data);
            chunk.save(writer)?;
        }

        let chunk = Chunk::default();
        chunk.save(writer)?;

        Ok(())
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self {
            version: DEFAULT_VERSION,
            name: "".to_string(),
            desc: "".to_string(),
            author: "".to_string(),
            cover: vec![],
            font: vec![],
            palette: vec![],
            map: vec![],
            code: "".to_string(),
        }
    }
}

#[cfg(test)]
mod test_super {
    use std::io::Cursor;

    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_cartridgeheader_from_reader() {
        let mut reader = Cursor::new(vec![5, 32, 0, 1, 32]);
        let expected = CartridgeHeader {
            cart_version: 5,
            name_size: 32,
            desc_size: 256,
            author_size: 32,
        };

        let result = CartridgeHeader::from_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_cartridgeheader_from_reader_invalid_data() {
        let mut reader = Cursor::new(vec![5, 32, 0, 1]);

        let result = CartridgeHeader::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_cartridgeheader_save() {
        let header = CartridgeHeader {
            cart_version: 1,
            name_size: 64,
            desc_size: 512,
            author_size: 64,
        };
        let expected: Vec<u8> = vec![1, 64, 0, 2, 64];

        let mut writer = Cursor::new(vec![0u8; 5]);
        let result = header.save(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref(), &expected);
    }

    #[test]
    fn test_cartridgeheader_save_error() {
        let header = CartridgeHeader {
            cart_version: 1,
            name_size: 64,
            desc_size: 512,
            author_size: 64,
        };

        let mut buff = [0u8; 1];
        let mut writer = Cursor::new(&mut buff[0..]);
        let result = header.save(&mut writer);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_cartridgeheader_default() {
        let header = CartridgeHeader::default();
        assert_eq!(header.cart_version, DEFAULT_CART_FILE_VERSION);
        assert_eq!(header.name_size, DEFAULT_NAME_SIZE);
        assert_eq!(header.desc_size, DEFAULT_DESC_SIZE);
        assert_eq!(header.author_size, DEFAULT_AUTHOR_SIZE);
    }

    #[test]
    fn test_cartridge_from_reader() {
        let mut data = vec![
            // cart header
            1,  // cart version
            10, // name size
            11, 0, // desc size
            2, // author size
        ];

        // cart data
        data.extend_from_slice(&[
            11, // cart version
            116, 104, 105, 115, 105, 115, 110, 97, 109, 101, // name
            100, 101, 115, 99, 114, 105, 195, 167, 195, 163, 111, // desc
            109, 101, // author
        ]);

        // code chunk
        data.extend_from_slice(&[
            2, 6, 0, 0, 0, // header
            109, 97, 105, 110, 40, 41, // data
        ]);

        // map chunk
        data.extend_from_slice(&[
            5, 0, 0, 0, 0, // header
        ]);

        // font chunk
        data.extend_from_slice(&[
            3, 0, 64, 0, 0, // header
        ]);
        data.extend_from_slice(&[0; 16384]);

        // cover chunk
        data.extend_from_slice(&[
            1, 0, 0, 0, 0, // header
        ]);

        // palette chunk
        data.extend_from_slice(&[
            4, 12, 0, 0, 0, // header
            0, 0, 0, 255, 255, 255, 180, 180, 180, 90, 90, 90, // data
        ]);

        // end chunk
        data.extend_from_slice(&[
            0, 0, 0, 0, 0, // ignored
            1, 0, 0, 0, 0, // junk data
        ]);

        let mut reader = Cursor::new(data);
        let expected = Cartridge {
            version: 11,
            name: "thisisname".to_string(),
            desc: "descrição".to_string(),
            author: "me".to_string(),
            cover: vec![],
            font: vec![0; 16384],
            palette: vec![0, 0, 0, 255, 255, 255, 180, 180, 180, 90, 90, 90],
            map: vec![],
            code: "main()".to_string(),
        };

        let result = Cartridge::from_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_cartridge_from_reader_empty_data_and_chunks() {
        let mut reader = Cursor::new(vec![
            // header
            1, // cart version
            0, // name size
            0, 0, // desc size
            0, // author size
            // cart
            1, // version
            // end
            0, 0, 0, 0, 0,
        ]);
        let expected = Cartridge {
            version: 1,
            name: "".to_string(),
            desc: "".to_string(),
            author: "".to_string(),
            cover: vec![],
            font: vec![],
            palette: vec![],
            map: vec![],
            code: "".to_string(),
        };

        let result = Cartridge::from_reader(&mut reader);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_cartridge_from_reader_missing_data() {
        let mut reader = Cursor::new(vec![
            // header
            1, // cart version
            0, // name size
            0, 0, // desc size
            0, // author size
            // cart
            1, // version
        ]);

        let result = Cartridge::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_cartridge_from_reader_missing_end_chunk() {
        let mut reader = Cursor::new(vec![
            // header
            1, // cart version
            0, // name size
            0, 0, // desc size
            0, // author size
        ]);

        let result = Cartridge::from_reader(&mut reader);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_cartridge_save() {
        let cart = Cartridge {
            version: 11,
            name: "thisisname".to_string(),
            desc: "descrição".to_string(),
            author: "me".to_string(),
            cover: vec![],
            font: vec![0; 16384],
            palette: vec![0, 0, 0, 255, 255, 255, 180, 180, 180, 90, 90, 90],
            map: vec![],
            code: "main()".to_string(),
        };

        let mut expected = vec![
            // cart header
            1,  // cart version
            10, // name size
            11, 0, // desc size
            2, // author size
        ];

        // cart data
        expected.extend_from_slice(&[
            11, // cart version
            116, 104, 105, 115, 105, 115, 110, 97, 109, 101, // name
            100, 101, 115, 99, 114, 105, 195, 167, 195, 163, 111, // desc
            109, 101, // author
        ]);

        // code chunk
        expected.extend_from_slice(&[
            2, 6, 0, 0, 0, // header
            109, 97, 105, 110, 40, 41, // data
        ]);

        // font chunk
        expected.extend_from_slice(&[
            3, 0, 64, 0, 0, // header
        ]);
        expected.extend_from_slice(&[0; 16384]);

        // palette chunk
        expected.extend_from_slice(&[
            4, 12, 0, 0, 0, // header
            0, 0, 0, 255, 255, 255, 180, 180, 180, 90, 90, 90, // data
        ]);

        // end chunk
        expected.extend_from_slice(&[
            0, 0, 0, 0, 0, // ignored
        ]);

        let mut writer = Cursor::new(vec![0u8; expected.len()]);
        let result = cart.save(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref(), &expected);
    }

    #[test]
    fn test_cartridge_save_empty() {
        let cart = Cartridge::default();
        let expected: Vec<u8> = vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0];

        let mut writer = Cursor::new(vec![0u8; 5]);
        let result = cart.save(&mut writer);
        assert!(result.is_ok());
        assert_eq!(writer.get_ref(), &expected);
    }

    #[test]
    fn test_cartridge_save_error() {
        let cart = Cartridge::default();

        let mut buff = [0u8; 1];
        let mut writer = Cursor::new(&mut buff[0..]);
        let result = cart.save(&mut writer);
        assert!(result.is_err());
        assert_matches!(result.unwrap_err(), CartridgeError::Io(_));
    }

    #[test]
    fn test_cartridge_default() {
        let cart = Cartridge::default();
        assert_eq!(cart.version, DEFAULT_VERSION);
        assert_eq!(cart.name, "".to_string());
        assert_eq!(cart.desc, "".to_string());
        assert_eq!(cart.author, "".to_string());
        assert_eq!(cart.cover, vec![]);
        assert_eq!(cart.font, vec![]);
        assert_eq!(cart.palette, vec![]);
        assert_eq!(cart.map, vec![]);
        assert_eq!(cart.code, "".to_string());
    }
}
