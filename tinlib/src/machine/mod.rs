//! Machine utilities.
mod code;
mod input;
mod memory;
mod ram;
mod screen;
mod vram;

pub use crate::machine::code::Code;
pub use crate::machine::input::Input;
pub use crate::machine::memory::Memory;
pub use crate::machine::ram::RAM;
pub use crate::machine::screen::{
    Screen, ScreenPixel, ScreenPixelEnumerate, ScreenPixelEnumerateMut, ScreenPixelIter,
    ScreenPixelIterMut,
};
pub use crate::machine::vram::VRAM;

/// Machine states.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MachineState {
    /// When the machine was just created and no cart was loaded yet.
    Created,
    /// When a cart was loaded but the machine is not running it yet.
    Loaded,
    /// When the machine is running the loaded cart.
    Started,
    /// When the cart execution is paused.
    Paused,
}

/// The machine representation.
pub struct Machine<'mem> {
    state: MachineState,
    #[allow(dead_code)]
    memory: Memory<'mem>,
}

impl Machine<'_> {
    /// Returns the current state.
    pub fn state(&self) -> MachineState {
        self.state
    }

    pub fn load_cartridge(&mut self) {}

    pub fn start(&mut self) {}

    pub fn pause(&mut self) {}

    pub fn stop(&mut self) {}
}

impl Default for Machine<'_> {
    /// Creates a new Machine in the `Created` state.
    fn default() -> Self {
        Self {
            state: MachineState::Created,
            memory: Memory::default(),
        }
    }
}
