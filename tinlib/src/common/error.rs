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
    #[error("IO operation error")]
    Io(#[from] io::Error),
    #[error("UFT8 conversion error")]
    FromUtf8(#[from] FromUtf8Error),
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

/// Internal result.
pub type Result<T> = StdResult<T, Error>;

#[cfg(test)]
mod test_super {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_error_new_invalid_index() {
        let index = 2usize;
        let lenght = 1usize;

        let error = Error::new_invalid_index(index, lenght);

        assert_matches!(
            error,
            Error::InvalidIndex { index: i, lenght: l } if i == index && l == lenght
        );
    }

    #[test]
    fn test_error_new_invalid_coord() {
        let coord = Coord::new(2, 2);
        let size: Size = Size::new(1, 1);

        let error = Error::new_invalid_coord(coord, size);

        assert_matches!(
            error,
            Error::InvalidCoord { coord: c, size: s } if c == coord && s == size
        );
    }
}
