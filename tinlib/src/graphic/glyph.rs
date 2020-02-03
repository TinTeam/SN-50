//! Glyph implementation and manipulation.
use std::fmt;
use std::slice;

use crate::common::{Coord, CoordEnumerate, CoordEnumerateMut, CoordIter, Error, Result, Size};

/// The Glyph width.
pub const GLYPH_WIDTH: usize = 8;
/// The Glyph height.
pub const GLYPH_HEIGHT: usize = 8;

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

/// A Glyph representation with 8x8 Pixels.
#[derive(Clone, Copy)]
pub struct Glyph {
    data: [GlyphPixel; GLYPH_WIDTH * GLYPH_HEIGHT],
}

impl Glyph {
    /// Returns the width.
    pub fn width(&self) -> usize {
        GLYPH_WIDTH
    }

    /// Returns the height.
    pub fn height(&self) -> usize {
        GLYPH_HEIGHT
    }

    /// Returns a Size.
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// Returns a pixel.
    pub fn get_pixel(&self, coord: Coord) -> Result<GlyphPixel> {
        if !self.is_coord_valid(coord) {
            return Err(Error::new_invalid_coord(coord, self.size()));
        }

        let index = self.get_index(coord);
        Ok(self.data[index])
    }

    /// Sets a pixel.
    pub fn set_pixel(&mut self, coord: Coord, value: GlyphPixel) -> Result<()> {
        if !self.is_coord_valid(coord) {
            return Err(Error::new_invalid_coord(coord, self.size()));
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
        coord.x < self.width() && coord.y < self.height()
    }

    fn get_index(&self, coord: Coord) -> usize {
        coord.x * self.width() + coord.y
    }
}

impl Default for Glyph {
    /// Creates a Glyph with all pixels black.
    fn default() -> Self {
        Self {
            data: [GlyphPixel::Empty; GLYPH_WIDTH * GLYPH_HEIGHT],
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
    fn test_glyph_width_height_and_size() {
        let glyph = Glyph::default();

        assert_eq!(glyph.width(), GLYPH_WIDTH);
        assert_eq!(glyph.height(), GLYPH_HEIGHT);
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
        let coord = Coord::new(9, 1);
        let glyph = Glyph::default();

        let result = glyph.get_pixel(coord);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            Error::InvalidCoord { coord: c, size: s } if c == coord && s == glyph.size()
        );
    }

    #[test]
    fn test_glyph_set_pixel() {
        let coord = Coord::new(1, 1);
        let mut glyph = Glyph::default();

        let result = glyph.set_pixel(coord, GlyphPixel::Solid);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());

        let result = glyph.get_pixel(coord);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), GlyphPixel::Solid);
    }

    #[test]
    fn test_glyph_set_pixel_invalid_coord() {
        let coord = Coord::new(9, 1);
        let mut glyph = Glyph::default();

        let result = glyph.set_pixel(coord, GlyphPixel::Solid);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            Error::InvalidCoord { coord: c, size: s } if c == coord && s == glyph.size()
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
            if y == glyph.width() {
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
