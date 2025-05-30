//! Palette implementation and manipulation.
use std::fmt;
use std::slice;

use crate::common::{CommonError, Result};
use crate::graphic::color::Color;

/// Default number of colors in a Palette.
const NUM_COLORS_IN_PALETTE: usize = 16;

/// A iterator over all palette colors.
pub type PaletteColorIter<'iter> = slice::Iter<'iter, Color>;
/// A mutable iterator over all palette colors.
pub type PaletteColorIterMut<'iter> = slice::IterMut<'iter, Color>;

/// A Palette representation with N colors.
#[derive(Clone)]
pub struct Palette {
    /// Palette's colors.
    colors: Vec<Color>,
}

impl Palette {
    /// Creates a new Palette.
    pub fn new(num_colors: usize) -> Self {
        Self {
            colors: vec![Color::default(); num_colors],
        }
    }

    /// Returns the lenght.
    pub fn lenght(&self) -> usize {
        self.colors.len()
    }

    /// Returns a color.
    pub fn get_color(&self, index: usize) -> Result<Color> {
        if !self.is_index_valid(index) {
            return Err(CommonError::new_invalid_index(index, self.lenght()));
        }

        Ok(self.colors[index])
    }

    /// Sets a color.
    pub fn set_color(&mut self, index: usize, color: Color) -> Result<()> {
        if !self.is_index_valid(index) {
            return Err(CommonError::new_invalid_index(index, self.lenght()));
        }

        self.colors[index] = color;

        Ok(())
    }

    /// Returns an iterator over all palette pixels.
    pub fn iter(&self) -> PaletteColorIter {
        self.colors.iter()
    }

    /// Returns a mutable iterator over all palette pixels.
    pub fn iter_mut(&mut self) -> PaletteColorIterMut {
        self.colors.iter_mut()
    }

    fn is_index_valid(&self, index: usize) -> bool {
        index < self.lenght()
    }
}

impl Default for Palette {
    /// Creates a Palette with all colors set to black.
    fn default() -> Self {
        Self {
            colors: vec![Color::default(); NUM_COLORS_IN_PALETTE],
        }
    }
}

impl fmt::Debug for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let data: Vec<&Color> = self.colors.iter().collect();

        f.debug_struct("Palette").field("colors", &data).finish()
    }
}

impl From<&[Color]> for Palette {
    fn from(colors: &[Color]) -> Self {
        Self {
            colors: colors.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_palette_default() {
        let palette = Palette::default();
        assert_eq!(palette.colors.len(), NUM_COLORS_IN_PALETTE);
    }

    #[test]
    fn test_palette_len() {
        let palette = Palette::default();
        assert_eq!(palette.lenght(), NUM_COLORS_IN_PALETTE);
    }

    #[test]
    fn test_palette_get_color() {
        let palette = Palette::default();
        let color = Color::default();

        let result = palette.get_color(0);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), color);
    }

    #[test]
    fn test_palette_get_color_invalid_index() {
        let palette = Palette::default();
        let index = 16usize;

        let result = palette.get_color(index);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidIndex { index: i, lenght: l } if i == index && l == palette.lenght()
        );
    }

    #[test]
    fn test_palette_set_color() {
        let mut palette = Palette::default();
        let color = Color::new(255, 255, 255);

        let result = palette.set_color(0, color);
        assert!(result.is_ok());

        let result = palette.get_color(0);
        assert_eq!(result.unwrap(), color);
    }

    #[test]
    fn test_palette_set_color_invalid_index() {
        let mut palette = Palette::default();
        let color = Color::new(255, 255, 255);
        let index = 16usize;

        let result = palette.set_color(16, color);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidIndex { index: i, lenght: l } if i == index && l == palette.lenght()
        );
    }

    #[test]
    fn test_palette_iter() {
        let palette = Palette::default();
        let default_color = Color::default();

        for color in palette.iter() {
            assert_eq!(color, &default_color);
        }
    }

    #[test]
    fn test_palette_iter_mut() {
        let mut palette = Palette::default();
        let new_color = Color::new(255, 255, 255);

        for color in palette.iter_mut() {
            *color = new_color;
        }

        for color in palette.iter() {
            assert_eq!(color, &new_color);
        }
    }

    #[test]
    fn test_palette_debug() {
        let palette = Palette::default();
        let data: Vec<&Color> = palette.colors.iter().collect();

        let expected = format!("Palette {{ colors: {:?} }}", data);
        let result = format!("{:?}", palette);

        assert_eq!(result, expected);
    }
}
