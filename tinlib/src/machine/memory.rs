//! Memory implementation and manipulation.
use crate::machine::ram::RAM;
use crate::machine::vram::VRAM;

/// The machine Memory representation.
pub struct Memory<'ram> {
    ram: RAM<'ram>,
    vram: VRAM,
}

impl<'ram> Memory<'ram> {
    /// Returns a ram reference.
    pub fn ram(&self) -> &RAM {
        &self.ram
    }

    /// Returns a mutable ram reference.
    pub fn ram_mut(&mut self) -> &mut RAM<'ram> {
        &mut self.ram
    }

    /// Returns a vram reference.
    pub fn vram(&self) -> &VRAM {
        &self.vram
    }

    /// Returns a mutable ram reference.
    pub fn vram_mut(&mut self) -> &mut VRAM {
        &mut self.vram
    }
}

impl Default for Memory<'_> {
    /// Creates a new Memory.
    fn default() -> Self {
        Self {
            ram: RAM::default(),
            vram: VRAM::default(),
        }
    }
}
