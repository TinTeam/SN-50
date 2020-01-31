//! Size implementation and manipulation.

/// A Size implementation with `usize` dimensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Size {
    width: usize,
    height: usize,
}

impl Size {
    /// Creates a new Size.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Returns the width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl From<(usize, usize)> for Size {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl From<[usize; 2]> for Size {
    fn from(array: [usize; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_new() {
        let size = Size::new(80, 48);

        assert_eq!(size.width, 80);
        assert_eq!(size.height, 48);
    }

    #[test]
    fn test_size_width_and_height() {
        let size = Size::new(80, 48);

        assert_eq!(size.width(), 80);
        assert_eq!(size.height(), 48);
    }

    #[test]
    fn test_size_from_tuple() {
        let tuple = (80usize, 48usize);
        let size = Size::from(tuple);

        assert_eq!(size, Size::new(80, 48));
    }

    #[test]
    fn test_size_from_array() {
        let array = [80usize, 48usize];
        let size = Size::from(array);

        assert_eq!(size, Size::new(80, 48));
    }
}
