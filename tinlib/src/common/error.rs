//! Error implementation and manipulation.
use std::io;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;

use thiserror::Error;

use crate::common::coord::Coord;
use crate::common::size::Size;

/// Internal errors.
#[derive(Error, Debug)]
pub enum Error {
    /// Error to reprense invalid indexes.
    #[error("invalid index {index} for lenght {lenght}")]
    InvalidIndex { index: usize, lenght: usize },
    /// Error to represent invalid coords.
    #[error("invalid coord ({coord:?}) for size ({size:?})")]
    InvalidCoord { coord: Coord, size: Size },
    #[error("test")]
    Io(#[from] io::Error),
    #[error("test2")]
    Utf8(#[from] FromUtf8Error),
}

impl Error {
    /// Creates a `InvalidIndex` error.
    pub fn new_invalid_index(index: usize, lenght: usize) -> Self {
        Self::InvalidIndex { index, lenght }
    }

    /// Creates a `InvalidCoord` error.
    pub fn new_invalid_coord(coord: Coord, size: Size) -> Self {
        Self::InvalidCoord { coord, size }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        use Error::*;

        match (self, other) {
            (
                InvalidIndex {
                    index: i1,
                    lenght: l1,
                },
                InvalidIndex {
                    index: i2,
                    lenght: l2,
                },
            ) => i1 == i2 && l1 == l2,
            (
                InvalidCoord {
                    coord: c1,
                    size: s1,
                },
                InvalidCoord {
                    coord: c2,
                    size: s2,
                },
            ) => c1 == c2 && s1 == s2,
            (Io(_), Io(_)) => true,
            (Utf8(_), Utf8(_)) => true,
            _ => false,
        }
    }
}

/// Internal result.
pub type Result<T> = StdResult<T, Error>;

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_error_new_invalid_index() {
        let index = 2usize;
        let lenght = 1usize;

        let error = Error::new_invalid_index(index, lenght);
        let expected = Error::InvalidIndex { index, lenght };

        assert_eq!(error, expected);
    }

    #[test]
    fn test_error_new_invalid_coord() {
        let coord = Coord::new(2, 2);
        let size: Size = Size::new(1, 1);

        let error = Error::new_invalid_coord(coord, size);
        let expected = Error::InvalidCoord { coord, size };

        assert_eq!(error, expected);
    }
}
