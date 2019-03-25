//! Traits for reading acceleration measurements from accelerometers

use crate::{error::Error, vector::Vector};
use core::fmt::Debug;

/// Accelerometers which measure acceleration vectors of type `V`
pub trait Accelerometer<V: Vector, E: Debug> {
    /// Get acceleration vector reading from the accelerometer
    fn acceleration(&mut self) -> Result<V, Error<E>>;
}
