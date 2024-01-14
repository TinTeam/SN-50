//! CommonError implementation and manipulation.
use std::result::Result as StdResult;

use thiserror::Error;

use crate::common::coord::Coord;
use crate::common::size::Size;

/// Common errors.
#[derive(Error, Debug)]
pub enum CommonError {
    /// Error to represent invalid coords.
    #[error("invalid coord ({coord:?}) for size ({size:?})")]
    InvalidCoord { coord: Coord, size: Size },
    /// Error to reprense invalid indexes.
    #[error("invalid index {index} for lenght {lenght}")]
    InvalidIndex { index: usize, lenght: usize },
}

impl CommonError {
    /// Creates a `InvalidCoord` error.
    pub fn new_invalid_coord(coord: Coord, size: Size) -> Self {
        Self::InvalidCoord { coord, size }
    }

    /// Creates a `InvalidIndex` error.
    pub fn new_invalid_index(index: usize, lenght: usize) -> Self {
        Self::InvalidIndex { index, lenght }
    }
}

pub type Result<T> = StdResult<T, CommonError>;

#[cfg(test)]
mod test_super {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_commonerror_new_invalid_index() {
        let index = 2usize;
        let lenght = 1usize;

        let error = CommonError::new_invalid_index(index, lenght);

        assert_matches!(
            error,
            CommonError::InvalidIndex { index: i, lenght: l } if i == index && l == lenght
        );
    }

    #[test]
    fn test_commonerror_new_invalid_coord() {
        let coord = Coord::new(2, 2);
        let size: Size = Size::new(1, 1);

        let error = CommonError::new_invalid_coord(coord, size);

        assert_matches!(
            error,
            CommonError::InvalidCoord { coord: c, size: s } if c == coord && s == size
        );
    }
}
