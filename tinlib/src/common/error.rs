//! Error implementation and manipulation.
use std::result::Result as StdResult;

use crate::common::coord::Coord;
use crate::common::size::Size;

/// Internal errors.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Error to reprense invalid indexes.
    InvalidIndex { index: usize, lenght: usize },
    /// Error to represent invalid coords.
    InvalidCoord { coord: Coord, size: Size },
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
