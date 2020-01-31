//! RAM implementation and manipulation.
use crate::machine::code::Code;
use crate::machine::input::Input;
use crate::map::Map;

/// The machine RAM representation.
pub struct RAM<'map> {
    code: Code,
    map: Map<'map>,
    input: Input,
}

impl<'map> RAM<'map> {
    /// Returns a code reference.
    pub fn code(&self) -> &Code {
        &self.code
    }

    /// Returns a mutable code reference.
    pub fn code_mut(&mut self) -> &mut Code {
        &mut self.code
    }

    /// Returns a map reference.
    pub fn map(&self) -> &Map {
        &self.map
    }

    /// Returns a mutable map reference.
    pub fn map_mut(&mut self) -> &mut Map<'map> {
        &mut self.map
    }

    /// Returns an input reference.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// Returns a mutable input reference.
    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }
}

impl<'map> Default for RAM<'map> {
    // Creates a new RAM.
    fn default() -> Self {
        Self {
            code: Code::default(),
            map: Map::default(),
            input: Input::default(),
        }
    }
}
