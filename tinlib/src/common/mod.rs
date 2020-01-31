//! Common utilities.
mod coord;
mod error;
mod size;

pub use crate::common::coord::{Coord, CoordEnumerate, CoordEnumerateMut, CoordIter};
pub use crate::common::error::{Error, Result};
pub use crate::common::size::Size;
