use std::result::Result as StdResult;

pub mod cartridge;
pub mod common;
pub mod graphic;
pub mod machine;
pub mod map;

use thiserror::Error;

use crate::cartridge::CartridgeError;
use crate::common::CommonError;

/// Internal errors.
#[derive(Error, Debug)]
pub enum Error {
    /// Error to wrap internal Cartridge errors.
    #[error(transparent)]
    Cartridge(#[from] CartridgeError),
    /// Error to wrap internal Common errors.
    #[error(transparent)]
    Common(#[from] CommonError),
}

/// Internal result.
pub type Result<T> = StdResult<T, Error>;
