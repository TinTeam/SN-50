//! Screen implementation and manipulation.
use std::fmt;
use std::slice;

use crate::common::{
    CommonError, Coord, CoordEnumerate, CoordEnumerateMut, CoordIter, Result, Size,
};
use crate::graphic::Color;

/// Screen width in pixels.
const SCREEN_WIDTH: usize = 640;
/// Screen width in pixels.
const SCREEN_HEIGHT: usize = 384;

/// A screen pixel or color.
pub type ScreenPixel = Color;
/// A iterator over all screen pixels.
pub type ScreenPixelIter<'iter> = slice::Iter<'iter, ScreenPixel>;
/// A mutable iterator over all screen pixels.
pub type ScreenPixelIterMut<'iter> = slice::IterMut<'iter, ScreenPixel>;
/// A enumeration iterator over all screen pixels and their coords.
pub type ScreenPixelEnumerate<'iter> = CoordEnumerate<'iter, ScreenPixel>;
/// A mutable enumeration iterator over all screen pixels and their coords.
pub type ScreenPixelEnumerateMut<'iter> = CoordEnumerateMut<'iter, ScreenPixel>;

/// A Screen representation with 640x384 tiles.
pub struct Screen {
    pixels: [Color; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    /// Returns the width.
    pub fn width(&self) -> usize {
        SCREEN_WIDTH
    }

    /// Returns the height.
    pub fn height(&self) -> usize {
        SCREEN_HEIGHT
    }

    /// Returns the size.
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// Returns a pixel.
    pub fn get_pixel(&self, coord: Coord) -> Result<ScreenPixel> {
        if !self.is_coord_valid(coord) {
            return Err(CommonError::new_invalid_coord(coord, self.size()));
        }

        let index = self.get_index(coord);
        Ok(self.pixels[index])
    }

    /// Sets a pixels.
    pub fn set_pixel(&mut self, coord: Coord, pixel: ScreenPixel) -> Result<()> {
        if !self.is_coord_valid(coord) {
            return Err(CommonError::new_invalid_coord(coord, self.size()));
        }

        let index = self.get_index(coord);
        self.pixels[index] = pixel;

        Ok(())
    }

    /// Clears all pixels to black.
    pub fn clear(&mut self) {
        for pixel in self.pixels.iter_mut() {
            *pixel = ScreenPixel::default();
        }
    }

    /// Returns an iterator over all screen coords.
    pub fn coords(&self) -> CoordIter {
        CoordIter::new(self.size())
    }

    /// Returns an iterator over all screen pixels.
    pub fn iter(&self) -> ScreenPixelIter {
        self.pixels.iter()
    }

    /// Returns a mutable iterator over all screen pixels.
    pub fn iter_mut(&mut self) -> ScreenPixelIterMut {
        self.pixels.iter_mut()
    }

    /// Returns an enumerate iterator over all screen pixels and tiles.
    pub fn enumerate(&self) -> ScreenPixelEnumerate {
        ScreenPixelEnumerate::new(self.coords(), self.iter())
    }

    /// Returns a mutable enumerate iterator over all screen pixels and tiles.
    pub fn enumerate_mut(&mut self) -> ScreenPixelEnumerateMut {
        ScreenPixelEnumerateMut::new(self.coords(), self.iter_mut())
    }

    fn is_coord_valid(&self, coord: Coord) -> bool {
        coord.x < self.width() && coord.y < self.height()
    }

    fn get_index(&self, coord: Coord) -> usize {
        coord.x * self.width() + coord.y
    }
}

impl Default for Screen {
    /// Creates a new black Screen.
    fn default() -> Self {
        Self {
            pixels: [Color::default(); SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pixels: Vec<&ScreenPixel> = self.pixels.iter().collect();

        f.debug_struct("Screen").field("pixels", &pixels).finish()
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_screen_default() {
        let screen = Screen::default();
        let default_pixel = ScreenPixel::default();

        assert_eq!(screen.pixels.len(), SCREEN_WIDTH * SCREEN_HEIGHT);
        assert!(screen.pixels.iter().all(|p| *p == default_pixel));
    }

    #[test]
    fn test_screen_width_height_and_size() {
        let screen = Screen::default();

        assert_eq!(screen.width(), SCREEN_WIDTH);
        assert_eq!(screen.height(), SCREEN_HEIGHT);
        assert_eq!(screen.size(), Size::new(SCREEN_WIDTH, SCREEN_HEIGHT));
    }

    #[test]
    fn test_screen_get_pixel() {
        let screen = Screen::default();
        let coord = Coord::new(1, 1);
        let color = Color::default();

        let result = screen.get_pixel(coord);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), color);
    }

    #[test]
    fn test_screen_get_pixel_invalid_coord() {
        let screen = Screen::default();
        let coord = Coord::new(641, 1);

        let result = screen.get_pixel(coord);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidCoord { coord: c, size: s } if c == coord && s == screen.size()
        );
    }

    #[test]
    fn test_screen_set_pixel() {
        let mut screen = Screen::default();
        let coord = Coord::new(1, 1);
        let pixel = ScreenPixel::new(255, 255, 255);

        let result = screen.set_pixel(coord, pixel);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());

        let result = screen.get_pixel(coord);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), pixel);
    }

    #[test]
    fn test_screen_set_pixel_invalid_coord() {
        let mut screen = Screen::default();
        let coord = Coord::new(641, 1);
        let pixel = ScreenPixel::new(255, 255, 255);

        let result = screen.set_pixel(coord, pixel);
        assert!(result.is_err());
        assert_matches!(
            result.unwrap_err(),
            CommonError::InvalidCoord { coord: c, size: s } if c == coord && s == screen.size()
        );
    }

    #[test]
    fn test_screen_coords() {
        let screen = Screen::default();

        let mut x = 0usize;
        let mut y = 0usize;
        for coord in screen.coords() {
            assert_eq!(coord.x, x);
            assert_eq!(coord.y, y);

            y += 1;
            if y == screen.width() {
                y = 0;
                x += 1;
            }
        }
    }

    #[test]
    fn test_screen_iter() {
        let screen = Screen::default();
        let default_pixel = ScreenPixel::default();

        for pixel in screen.iter() {
            assert_eq!(pixel, &default_pixel);
        }
    }

    #[test]
    fn test_screen_iter_mut() {
        let mut screen = Screen::default();
        let new_pixel = ScreenPixel::new(255, 255, 255);

        for pixel in screen.iter_mut() {
            *pixel = new_pixel;
        }

        for pixel in screen.iter() {
            assert_eq!(pixel, &new_pixel);
        }
    }

    #[test]
    fn test_screen_enumerate() {
        let screen = Screen::default();
        let mut coorditer = screen.coords();
        let mut pixeliter = screen.iter();

        for (coord, pixel) in screen.enumerate() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert_eq!(pixel, pixeliter.next().unwrap());
        }
    }

    #[test]
    fn test_screen_enumerate_mut() {
        let mut screen = Screen::default();
        let mut coorditer = screen.coords();
        let old_pixel = ScreenPixel::default();
        let new_pixel = ScreenPixel::new(255, 255, 255);

        for (coord, pixel) in screen.enumerate_mut() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert_eq!(pixel, &old_pixel);

            *pixel = new_pixel;
        }

        for pixel in screen.iter() {
            assert_eq!(pixel, &new_pixel);
        }
    }

    #[test]
    fn test_screen_debug() {
        let screen = Screen::default();
        let data: Vec<&ScreenPixel> = screen.pixels.iter().collect();

        let expected = format!("Screen {{ pixels: {:?} }}", data);
        let result = format!("{:?}", screen);

        assert_eq!(result, expected);
    }
}
