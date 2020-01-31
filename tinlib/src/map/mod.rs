//! Map utilities.
use std::fmt;
use std::slice;

use crate::common::{Coord, CoordEnumerate, CoordEnumerateMut, CoordIter, Error, Result, Size};
use crate::graphic::{Color, Glyph};

/// Map width in Glyphs.
const MAP_WIDTH: usize = 320;
/// Map height in Glyphs.
const MAP_HEIGHT: usize = 192;

/// A Tile representation with a glyph and a color.
#[derive(Clone, Copy, PartialEq)]
pub struct Tile<'refs> {
    /// A reference to a Glyph.
    pub glyph: &'refs Glyph,
    /// A reference to a Color.
    pub color: &'refs Color,
}

impl<'refs> Tile<'refs> {
    /// Creates a new Tile with references to a Glyph and a Color.
    pub fn new(glyph: &'refs Glyph, color: &'refs Color) -> Self {
        Self { glyph, color }
    }
}

impl<'refs> fmt::Debug for Tile<'refs> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Tile")
            .field("glyph", self.glyph)
            .field("color", self.color)
            .finish()
    }
}

/// A iterator over all map tiles.
pub type MapTileIter<'iter, 'tile> = slice::Iter<'iter, Option<Tile<'tile>>>;
/// A mutable iterator over all map tiles.
pub type MapTileIterMut<'iter, 'tile> = slice::IterMut<'iter, Option<Tile<'tile>>>;
/// A enumeration iterator over all map tiles and their coords.
pub type MapTileEnumerate<'iter, 'tile> = CoordEnumerate<'iter, Option<Tile<'tile>>>;
/// A mutable enumeration iterator over all map tiles and their coords.
pub type MapTileEnumerateMut<'iter, 'tile> = CoordEnumerateMut<'iter, Option<Tile<'tile>>>;

/// A Map representation with 320x192 tiles.
pub struct Map<'tile> {
    /// Map's tiles.
    pub tiles: [Option<Tile<'tile>>; MAP_WIDTH * MAP_HEIGHT],
}

impl<'tile> Map<'tile> {
    /// Returns the width.
    pub fn width(&self) -> usize {
        MAP_WIDTH
    }

    /// Returns the height.
    pub fn height(&self) -> usize {
        MAP_HEIGHT
    }

    /// Returns the size.
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }

    /// Returns a tile.
    pub fn get_tile(&self, coord: Coord) -> Result<Option<Tile<'tile>>> {
        if !self.is_coord_valid(coord) {
            return Err(Error::new_invalid_coord(coord, self.size()));
        }

        let index = self.get_index(coord);
        Ok(self.tiles[index])
    }

    /// Sets a tile.
    pub fn set_tile(&mut self, coord: Coord, value: Tile<'tile>) -> Result<()> {
        if !self.is_coord_valid(coord) {
            return Err(Error::new_invalid_coord(coord, self.size()));
        }

        let index = self.get_index(coord);
        self.tiles[index] = Some(value);

        Ok(())
    }

    /// Returns an iterator over all map coords.
    pub fn coords(&self) -> CoordIter {
        CoordIter::new(self.size())
    }

    /// Returns an iterator over all map tiles.
    pub fn iter(&self) -> MapTileIter {
        self.tiles.iter()
    }

    /// Returns a mutable iterator over all map tiles.
    pub fn iter_mut<'iter>(&'iter mut self) -> MapTileIterMut<'iter, 'tile> {
        self.tiles.iter_mut()
    }

    /// Returns an enumerate iterator over all map coords and tiles.
    pub fn enumerate(&self) -> MapTileEnumerate {
        MapTileEnumerate::new(self.coords(), self.iter())
    }

    /// Returns a mutable enumerate iterator over all map coords and tiles.
    pub fn enumerate_mut<'iter>(&'iter mut self) -> MapTileEnumerateMut<'iter, 'tile> {
        MapTileEnumerateMut::new(self.coords(), self.iter_mut())
    }

    fn is_coord_valid(&self, coord: Coord) -> bool {
        coord.x < self.width() && coord.y < self.height()
    }

    fn get_index(&self, coord: Coord) -> usize {
        coord.x * self.width() + coord.y
    }
}

impl<'tile> Default for Map<'tile> {
    /// Creates a new empty Map.
    fn default() -> Self {
        Self {
            tiles: [None; MAP_WIDTH * MAP_HEIGHT],
        }
    }
}

impl<'tile> fmt::Debug for Map<'tile> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tiles: Vec<&Option<Tile<'tile>>> = self.tiles.iter().collect();

        f.debug_struct("Map").field("tiles", &tiles).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_default() {
        let glyph = Glyph::default();
        let color = Color::default();

        let tile = Tile::new(&glyph, &color);

        assert_eq!(tile.glyph, &glyph);
        assert_eq!(tile.color, &color);
    }

    #[test]
    fn test_tile_debug() {
        let glyph = Glyph::default();
        let color = Color::default();

        let tile = Tile::new(&glyph, &color);

        let expected = format!("Tile {{ glyph: {:?}, color: {:?} }}", &glyph, &color);
        let result = format!("{:?}", tile);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_new() {
        let map = Map::default();

        assert_eq!(map.tiles.len(), MAP_WIDTH * MAP_HEIGHT);
    }

    #[test]
    fn test_map_width_height_and_size() {
        let map = Map::default();

        assert_eq!(map.width(), MAP_WIDTH);
        assert_eq!(map.height(), MAP_HEIGHT);
        assert_eq!(map.size(), Size::new(MAP_WIDTH, MAP_HEIGHT));
    }

    #[test]
    fn test_map_get_tile() {
        let coord = Coord::new(1, 1);
        let map = Map::default();

        let result = map.get_tile(coord);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_map_get_tile_invalid_coord() {
        let coord = Coord::new(321, 1);
        let map = Map::default();

        let error = Error::new_invalid_coord(coord, map.size());
        let result = map.get_tile(coord);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_map_set_tile() {
        let glyph = Glyph::default();
        let color = Color::default();

        let coord = Coord::new(1, 1);
        let mut map = Map::default();
        let tile = Tile::new(&glyph, &color);

        let result = map.set_tile(coord, tile);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());

        let result = map.get_tile(coord);
        assert!(result.is_ok());
        let option = result.unwrap();
        assert!(option.is_some());
        assert_eq!(option.unwrap(), tile);
    }

    #[test]
    fn test_map_set_tile_invalid_coord() {
        let glyph = Glyph::default();
        let color = Color::default();

        let coord = Coord::new(321, 1);
        let mut map = Map::default();
        let tile = Tile::new(&glyph, &color);

        let error = Error::new_invalid_coord(coord, map.size());
        let result = map.set_tile(coord, tile);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), error);
    }

    #[test]
    fn test_map_coords() {
        let map = Map::default();

        let mut x = 0usize;
        let mut y = 0usize;
        for coord in map.coords() {
            assert_eq!(coord.x, x);
            assert_eq!(coord.y, y);

            y += 1;
            if y == map.width() {
                y = 0;
                x += 1;
            }
        }
    }

    #[test]
    fn test_map_iter() {
        let map = Map::default();

        for tile in map.iter() {
            assert!(tile.is_none());
        }
    }

    #[test]
    fn test_map_iter_mut() {
        let glyph = Glyph::default();
        let color = Color::default();

        let mut map = Map::default();
        let new_tile = Tile::new(&glyph, &color);

        for tile in map.iter_mut() {
            *tile = Some(new_tile);
        }

        for tile in map.iter() {
            assert!(tile.is_some());
            assert_eq!(tile.unwrap(), new_tile);
        }
    }

    #[test]
    fn test_map_enumerate() {
        let map = Map::default();
        let mut coorditer = map.coords();
        let mut pixeliter = map.iter();

        for (coord, tile) in map.enumerate() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert_eq!(tile, pixeliter.next().unwrap());
        }
    }

    #[test]
    fn test_map_enumerate_mut() {
        let glyph = Glyph::default();
        let color = Color::default();

        let mut map = Map::default();
        let mut coorditer = map.coords();
        let new_tile = Tile::new(&glyph, &color);

        for (coord, tile) in map.enumerate_mut() {
            assert_eq!(coord, coorditer.next().unwrap());
            assert!(tile.is_none());

            *tile = Some(new_tile);
        }

        for tile in map.iter() {
            assert!(tile.is_some());
            assert_eq!(tile.unwrap(), new_tile);
        }
    }
}
