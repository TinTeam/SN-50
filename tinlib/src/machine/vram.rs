//! VRAM implementation and manipulation.
use crate::graphic::{Font, Palette};
use crate::machine::screen::Screen;

/// The machine VRAM representation.
pub struct VRAM {
    screen: Screen,
    palette: Palette,
    font: Font,
}

impl VRAM {
    /// Returns a screen reference.
    pub fn screen(&self) -> &Screen {
        &self.screen
    }

    /// Returns a mutable screen reference.
    pub fn screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }

    /// Returns a palette reference.
    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    /// Returns a mutable palette reference.
    pub fn palette_mut(&mut self) -> &mut Palette {
        &mut self.palette
    }

    /// Returns a font reference.
    pub fn font(&self) -> &Font {
        &self.font
    }

    /// Returns a mutable font reference.
    pub fn font_mut(&mut self) -> &mut Font {
        &mut self.font
    }
}

impl Default for VRAM {
    /// Creates a new VRAM.
    fn default() -> Self {
        Self {
            screen: Screen::default(),
            palette: Palette::default(),
            font: Font::default(),
        }
    }
}
