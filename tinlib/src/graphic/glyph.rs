//! Glyph implementation and manipulation.
use std::fmt;
use std::slice;

use crate::common::{
    CommonError, Coord, CoordEnumerate, CoordEnumerateMut, CoordIter, Result, Size,
};

/// The default Glyph width.
pub const GLYPH_WIDTH: usize = 16;
/// The default Glyph height.
pub const GLYPH_HEIGHT: usize = 16;

/// A Glyph pixel representation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlyphPixel {
    /// An empty or transparent pixel.
    Empty,
    /// An solid pixel.
    Solid,
}

/// A iterator over all glyph pìxels.
pub type GlyphPixelIter<'iter> = slice::Iter<'iter, GlyphPixel>;
/// A mutable iterator over all glyph pìxels.
pub type GlyphPixelIterMut<'iter> = slice::IterMut<'iter, GlyphPixel>;
/// A enumeration iterator over all glyph pixels and their coords.
pub type GlyphPixelEnumerate<'iter> = CoordEnumerate<'iter, GlyphPixel>;
/// A mutable enumeration iterator over all glyph pixels and their coords.
pub type GlyphPixelEnumerateMut<'iter> = CoordEnumerateMut<'iter, GlyphPixel>;

/// A Glyph representation with NxM Pixels.
#[derive(Clone)]
pub struct Glyph {
    size: Size,
    data: Vec<GlyphPixel>,
}

impl Glyph {
    /// Creates a new Glyph.
    pub fn new(size: Size) -> Self {
        Self {
            size,
            data: vec![GlyphPixel::Empty; size.width() * size.height()],
        }
    }

    /// Returns a Size.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Returns a pixel.
    pub fn get_pixel(&self, coord: Coord) -> Result<GlyphPixel> {
        if !self.is_coord_valid(coord) {
            return Err(CommonError::new_invalid_coord(coord, self.size));
        }

        let index = self.get_index(coord);
        Ok(self.data[index])
    }

    /// Sets a pixel.
    pub fn set_pixel(&mut self, coord: Coord, value: GlyphPixel) -> Result<()> {
        if !self.is_coord_valid(coord) {
            return Err(CommonError::new_invalid_coord(coord, self.size));
        }

        let index = self.get_index(coord);
        self.data[index] = value;

        Ok(())
    }

    /// Returns a iterator over the glyph's coords.
    pub fn coords(&self) -> CoordIter {
        CoordIter::new(self.size())
    }

    /// Returns an iterator over all Glyph pixels.
    pub fn iter(&self) -> GlyphPixelIter {
        self.data.iter()
    }

    /// Returns a mutable iterator over all Glyph pixels.
    pub fn iter_mut(&mut self) -> GlyphPixelIterMut {
        self.data.iter_mut()
    }

    /// Returns an enumerate iterator over glyph's coords and pixels.
    pub fn enumerate(&self) -> GlyphPixelEnumerate {
        GlyphPixelEnumerate::new(self.coords(), self.iter())
    }

    /// Returns a mutable enumerate iterator over glyph's coords and pixels.
    pub fn enumerate_mut(&mut self) -> GlyphPixelEnumerateMut {
        GlyphPixelEnumerateMut::new(self.coords(), self.iter_mut())
    }

    fn is_coord_valid(&self, coord: Coord) -> bool {
        coord.x < self.size.width() && coord.y < self.size.height()
    }

    fn get_index(&self, coord: Coord) -> usize {
        coord.x * self.size.width() + coord.y
    }
}

impl Default for Glyph {
    /// Creates a Glyph with all pixels black.
    fn default() -> Self {
        Self {
            size: Size::new(GLYPH_WIDTH, GLYPH_HEIGHT),
            data: vec![GlyphPixel::Empty; GLYPH_WIDTH * GLYPH_HEIGHT],
        }
    }
}

impl PartialEq for Glyph {
    fn eq(&self, other: &Self) -> bool {
        self.data.iter().zip(other.data.iter()).all(|(a, b)| a == b)
    }
}

impl fmt::Debug for Glyph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: Vec<&GlyphPixel> = self.data.iter().collect();

        f.debug_struct("Glyph").field("data", &data).finish()
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_glyph_default() {
        let glyph = Glyph::default();

        assert_eq!(glyph.data.len(), GLYPH_WIDTH * GLYPH_HEIGHT);
        for pixel in glyph.data.iter() {
            assert_eq!(*pixel, GlyphPixel::Empty);
        }
    }

    #[test]
    fn test_glyph_size() {
        let glyph = Glyph::default();
        assert_eq!(glyph.size(), Size::new(GLYPH_WIDTH, GLYPH_HEIGHT));
    }

    #[test]
    fn test_glyph_get_pixel() {
        let coord = Coord::new(1, 1);
        let glyph = Glyph::default();

        let result = glyph.get_pixel(coord);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GlyphPixel::Empty);
    }

    #[test]
    fn test_glyph_get_pixel_invalid_coord() {
        let coord = Coord::new(17, 1);
        let glyph = Glyph::default();

        let result = glyph.get_pixel(coord);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidCoord { coord: c, size: s } if c == coord && s == glyph.size()
        );
    }

    #[test]
    fn test_glyph_set_pixel() {
        let coord = Coord::new(1, 1);
        let mut glyph = Glyph::default();

        let result = glyph.set_pixel(coord, GlyphPixel::Solid);
        assert!(result.is_ok());

        let result = glyph.get_pixel(coord);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GlyphPixel::Solid);
    }

    #[test]
    fn test_glyph_set_pixel_invalid_coord() {
        let coord = Coord::new(17, 1);
        let mut glyph = Glyph::default();

        let result = glyph.set_pixel(coord, GlyphPixel::Solid);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidCoord { coord: c, size: s } if c == coord && s == glyph.size()
        );
    }

    #[test]
    fn test_glyph_coords() {
        let glyph = Glyph::default();

        let mut x = 0usize;
        let mut y = 0usize;
        for coord in glyph.coords() {
            assert_eq!(coord.x, x);
            assert_eq!(coord.y, y);

            y += 1;
            if y == glyph.size().width() {
                y = 0;
                x += 1;
            }
        }
    }

    #[test]
    fn test_glyph_iter() {
        let glyph = Glyph::default();
        let default_pixel = GlyphPixel::Empty;

        for pixel in glyph.iter() {
            assert_eq!(pixel, &default_pixel);
        }
    }

    #[test]
    fn test_glyph_iter_mut() {
        let mut glyph = Glyph::default();
        let new_pixel = GlyphPixel::Solid;

        for pixel in glyph.iter_mut() {
            *pixel = new_pixel;
        }

        for pixel in glyph.iter() {
            assert_eq!(pixel, &new_pixel);
        }
    }

    #[test]
    fn test_glyph_enumerate() {
        let glyph = Glyph::default();
        let mut coorditer = glyph.coords();
        let mut pixeliter = glyph.iter();

        for (coord, pixel) in glyph.enumerate() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert_eq!(pixel, pixeliter.next().unwrap());
        }
    }

    #[test]
    fn test_glyph_enumerate_mut() {
        let mut glyph = Glyph::default();
        let mut coorditer = glyph.coords();
        let old_pixel = GlyphPixel::Empty;
        let new_pixel = GlyphPixel::Solid;

        for (coord, pixel) in glyph.enumerate_mut() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert_eq!(pixel, &old_pixel);

            *pixel = new_pixel;
        }

        for pixel in glyph.iter() {
            assert_eq!(pixel, &new_pixel);
        }
    }

    #[test]
    fn test_glyph_partialeq() {
        let glyph_1 = Glyph::default();
        let mut glyph_2 = Glyph::default();

        assert_eq!(glyph_1, glyph_2);

        glyph_2.data[0] = GlyphPixel::Solid;
        assert_ne!(glyph_1, glyph_2);
    }

    #[test]
    fn test_glyph_debug() {
        let glyph = Glyph::default();
        let data: Vec<&GlyphPixel> = glyph.data.iter().collect();

        let expected = format!("Glyph {{ data: {:?} }}", data);
        let result = format!("{:?}", glyph);

        assert_eq!(result, expected);
    }
}
