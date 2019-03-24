//! Traits for reading acceleration measurements from accelerometers

use crate::{error::Error, reading::Reading};
use core::fmt::Debug;

/// Accelerometers capable of obtaining an acceleration reading of type `R`
pub trait Accelerometer<R: Reading, E: Debug> {
    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<R, Error<E>>;
}
