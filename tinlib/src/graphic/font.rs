//! Font implementation and manipulation.
use std::fmt;
use std::slice;

use crate::common::{CommonError, Result};
use crate::graphic::glyph::Glyph;

/// Number of Glyphs in a Font.
const GLYPHS_IN_FONT: usize = 256;

/// A iterator over all font glyphs.
pub type FontGlyphIter<'iter> = slice::Iter<'iter, Glyph>;
/// A mutable iterator over all font glyphs.
pub type FontGlyphIterMut<'iter> = slice::IterMut<'iter, Glyph>;

/// A Font representation with 256 Glyphs.
#[derive(Clone, Copy)]
pub struct Font {
    /// Font's glyphs.
    pub glyphs: [Glyph; GLYPHS_IN_FONT],
}

impl Font {
    /// Returns the lenght.
    pub fn lenght(&self) -> usize {
        GLYPHS_IN_FONT
    }

    /// Returns a glyph.
    pub fn get_glyph(&self, index: usize) -> Result<Glyph> {
        if !self.is_index_valid(index) {
            return Err(CommonError::new_invalid_index(index, self.lenght()));
        }

        Ok(self.glyphs[index])
    }

    /// Sets a glyph.
    pub fn set_glyph(&mut self, index: usize, glyph: Glyph) -> Result<()> {
        if !self.is_index_valid(index) {
            return Err(CommonError::new_invalid_index(index, self.lenght()));
        }

        self.glyphs[index] = glyph;

        Ok(())
    }

    /// Returns an iterator over all font glyphs.
    pub fn iter(&self) -> FontGlyphIter {
        self.glyphs.iter()
    }

    /// Returns a mutable iterator over all font glyphs.
    pub fn iter_mut(&mut self) -> FontGlyphIterMut {
        self.glyphs.iter_mut()
    }

    fn is_index_valid(&self, index: usize) -> bool {
        index < self.lenght()
    }
}

impl Default for Font {
    /// Creates a Font with default empty glyphs.
    fn default() -> Self {
        Self {
            glyphs: [Glyph::default(); GLYPHS_IN_FONT],
        }
    }
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: Vec<&Glyph> = self.glyphs.iter().collect();

        f.debug_struct("Font").field("data", &data).finish()
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::common::Coord;
    use crate::graphic::glyph::GlyphPixel;

    use super::*;

    #[test]
    fn test_font_default() {
        let font = Font::default();
        assert_eq!(font.glyphs.len(), GLYPHS_IN_FONT);
    }

    #[test]
    fn test_font_len() {
        let font = Font::default();
        assert_eq!(font.lenght(), GLYPHS_IN_FONT);
    }

    #[test]
    fn test_font_get_glyph() {
        let font = Font::default();
        let glyph = Glyph::default();

        let result = font.get_glyph(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), glyph);
    }

    #[test]
    fn test_font_get_glyph_invalid_index() {
        let font = Font::default();
        let index = 256usize;

        let result = font.get_glyph(index);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidIndex { index: i, lenght: l } if i == index && l == font.lenght()
        );
    }

    #[test]
    fn test_font_set_glyph() {
        let mut font = Font::default();

        let coord = Coord::new(0, 0);
        let mut new_glyph = Glyph::default();
        new_glyph.set_pixel(coord, GlyphPixel::Solid).unwrap();

        let result = font.set_glyph(0, new_glyph);
        assert!(result.is_ok());

        let result = font.get_glyph(0);
        assert_eq!(result.unwrap(), new_glyph);
    }

    #[test]
    fn test_font_set_glyph_invalid_index() {
        let mut font = Font::default();
        let glyph = Glyph::default();
        let index = 256usize;

        let result = font.set_glyph(index, glyph);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidIndex { index: i, lenght: l } if i == index && l == font.lenght()
        );
    }

    #[test]
    fn test_font_iter() {
        let font = Font::default();
        let default_glyph = Glyph::default();

        for color in font.iter() {
            assert_eq!(color, &default_glyph);
        }
    }

    #[test]
    fn test_font_iter_mut() {
        let mut font = Font::default();

        let coord = Coord::new(0, 0);
        let mut new_glyph = Glyph::default();
        new_glyph.set_pixel(coord, GlyphPixel::Solid).unwrap();

        for glyph in font.iter_mut() {
            *glyph = new_glyph;
        }

        for glyph in font.iter() {
            assert_eq!(glyph, &new_glyph);
        }
    }

    #[test]
    fn test_font_debug() {
        let font = Font::default();
        let data: Vec<&Glyph> = font.glyphs.iter().collect();

        let expected = format!("Font {{ data: {:?} }}", data);
        let result = format!("{:?}", font);

        assert_eq!(result, expected);
    }
}
