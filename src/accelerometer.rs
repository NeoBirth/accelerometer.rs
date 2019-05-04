//! Traits for reading acceleration measurements from accelerometers

use core::fmt::Debug;
use micromath::vector::Vector;

/// Accelerometers which measure acceleration vectors of type `V`
pub trait Accelerometer<V: Vector> {
    /// Error type
    type Error: Debug;

    /// Get acceleration vector reading from the accelerometer
    fn acceleration(&mut self) -> Result<V, Self::Error>;
}
