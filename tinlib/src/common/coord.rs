//! Coord implementation and manipulation.
use std::slice;

use crate::common::size::Size;

/// A Coord representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    /// Creates a new Coord.
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl From<(usize, usize)> for Coord {
    fn from((x, y): (usize, usize)) -> Self {
        Self::new(x, y)
    }
}

impl From<[usize; 2]> for Coord {
    fn from(array: [usize; 2]) -> Self {
        Self::new(array[0], array[1])
    }
}

/// A iterator over all Coord limited by Size.
pub struct CoordIter {
    size: Size,
    coord: Coord,
}

impl CoordIter {
    // Creates a new CoordIter from a Size.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            coord: Coord::new(0, 0),
        }
    }
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.coord.x == self.size.height() {
            return None;
        }

        let result = self.coord;

        self.coord.y += 1;
        if self.coord.y == self.size.width() {
            self.coord.y = 0;
            self.coord.x += 1;
        }

        Some(result)
    }
}

/// A iterator over all Coord and their related itens, limited by Size.
pub struct CoordEnumerate<'iter, T: 'iter> {
    coords: CoordIter,
    iter: slice::Iter<'iter, T>,
}

impl<'iter, T> CoordEnumerate<'iter, T> {
    /// Creates a CoordEnumerate from a CoordIter and item Iter.
    pub fn new(coords: CoordIter, iter: slice::Iter<'iter, T>) -> Self {
        Self { coords, iter }
    }
}

impl<'iter, T> Iterator for CoordEnumerate<'iter, T> {
    type Item = (Coord, &'iter T);

    fn next(&mut self) -> Option<Self::Item> {
        self.coords
            .next()
            .and_then(|c| self.iter.next().map(|t| (c, t)))
    }
}

/// A mutable iterator over all Coord and their related itens, limited by Size.
pub struct CoordEnumerateMut<'iter, T: 'iter> {
    coords: CoordIter,
    iter: slice::IterMut<'iter, T>,
}

impl<'iter, T> CoordEnumerateMut<'iter, T> {
    /// Creates a CoordEnumerateMut from a CoordIter and item Iter.
    pub fn new(coords: CoordIter, iter: slice::IterMut<'iter, T>) -> Self {
        Self { coords, iter }
    }
}

impl<'iter, T> Iterator for CoordEnumerateMut<'iter, T> {
    type Item = (Coord, &'iter mut T);
    fn next(&mut self) -> Option<Self::Item> {
        self.coords
            .next()
            .and_then(|c| self.iter.next().map(|t| (c, t)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coord_new() {
        let coord = Coord::new(11, 27);

        assert_eq!(coord.x, 11);
        assert_eq!(coord.y, 27);
    }

    #[test]
    fn test_coord_from_tuple() {
        let tuple = (11usize, 27usize);
        let coord = Coord::from(tuple);

        assert_eq!(coord, Coord::new(11, 27));
    }

    #[test]
    fn test_coord_from_array() {
        let array = [11usize, 27usize];
        let coord = Coord::from(array);

        assert_eq!(coord, Coord::new(11, 27));
    }

    #[test]
    fn test_coorditer_new() {
        let size = Size::new(3, 2);
        let coord = Coord::new(0, 0);
        let iter = CoordIter::new(size);

        assert_eq!(iter.size, size);
        assert_eq!(iter.coord, coord);
    }

    #[test]
    fn test_coorditer_next() {
        let size = Size::new(3, 2);
        let mut iter = CoordIter::new(size);

        assert_eq!(iter.next(), Some(Coord::new(0, 0)));
        assert_eq!(iter.next(), Some(Coord::new(0, 1)));
        assert_eq!(iter.next(), Some(Coord::new(0, 2)));
        assert_eq!(iter.next(), Some(Coord::new(1, 0)));
        assert_eq!(iter.next(), Some(Coord::new(1, 1)));
        assert_eq!(iter.next(), Some(Coord::new(1, 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_coordenumerate_new_and_next() {
        let items = [1, 2, 3, 4, 5, 6];
        let size = Size::new(3, 2);
        let coorditer = CoordIter::new(size);
        let itemiter = items.iter();
        let mut enumerate = CoordEnumerate::new(coorditer, itemiter);

        assert_eq!(enumerate.next(), Some((Coord::new(0, 0), &1)));
        assert_eq!(enumerate.next(), Some((Coord::new(0, 1), &2)));
        assert_eq!(enumerate.next(), Some((Coord::new(0, 2), &3)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 0), &4)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 1), &5)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 2), &6)));
        assert_eq!(enumerate.next(), None);
    }

    #[test]
    fn test_coordenumeratemut_new_and_next() {
        let mut items = [1, 2, 3, 4, 5, 6];
        let size = Size::new(3, 2);
        let coorditer = CoordIter::new(size);
        let itemiter = items.iter_mut();
        let mut enumerate = CoordEnumerateMut::new(coorditer, itemiter);

        assert_eq!(enumerate.next(), Some((Coord::new(0, 0), &mut 1)));
        assert_eq!(enumerate.next(), Some((Coord::new(0, 1), &mut 2)));
        assert_eq!(enumerate.next(), Some((Coord::new(0, 2), &mut 3)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 0), &mut 4)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 1), &mut 5)));
        assert_eq!(enumerate.next(), Some((Coord::new(1, 2), &mut 6)));
        assert_eq!(enumerate.next(), None);
    }
}
