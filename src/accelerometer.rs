//! Traits for reading acceleration measurements from accelerometers

use crate::reading::Reading;

/// Accelerometers capable of obtaining an acceleration reading of type `R`
pub trait Accelerometer<R: Reading, E> {
    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<R, E>;
}
