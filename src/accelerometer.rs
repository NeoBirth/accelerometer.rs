//! Traits for reading acceleration measurements from accelerometers

use core::fmt::Debug;
use micromath::vector::{F32x3, Vector};

/// Accelerometers which measure acceleration vectors of type `V`
pub trait Accelerometer<V: Vector> {
    /// Error type
    type Error: Debug;

    /// Get raw acceleration data from the accelerometer
    fn accel_raw(&mut self) -> Result<V, Self::Error>;

    /// Get normalized acceleration data from the acclerometer
    /// Ex {0.0, 5.2, 0.0} - 5.2 G's of acceleration in the y-axis
    fn accel_norm(&mut self) -> Result<F32x3, Self::Error>;
}
