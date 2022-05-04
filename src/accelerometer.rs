//! Traits for reading acceleration measurements from accelerometers.

use micromath::vector::Component;

use crate::{
    error::Error,
    vector::{F32x3, Vector},
};
use core::fmt::Debug;

/// Accelerometer trait which provides g-normalized readings.
pub trait Accelerometer {
    /// Error type
    type Error: Debug;

    /// Get normalized ±g reading from the accelerometer.
    ///
    /// Ex. {0.0, 5.2, 0.0} - 5.2 g of acceleration in the y-axis
    ///
    /// Note: it is expected that you call this when you know
    /// data is ready and valid, ie in calling in response to an
    /// interrupt, polling far slower than your data rate, or if in
    /// a tight loop perhaps like
    /// ```ignore
    /// while !lis3dh.is_data_ready().unwrap() {}
    /// let dat = lis3dh.accel_raw().unwrap();
    /// ```
    /// Please comment this contract on your implementation to inform
    /// your users as well.
    fn accel_norm(&mut self) -> Result<F32x3, Error<Self::Error>>;

    /// Get sample rate of accelerometer data.
    ///
    /// Ex. 125.0 - sample rate of 125hz
    fn sample_rate(&mut self) -> Result<f32, Error<Self::Error>>;
}

/// Read raw acceleration vectors of type `V: Vector`.
///
/// This is intended to provide direct access to raw accelerometer data and
/// should use a vector type which best matches the raw accelerometer data.
pub trait RawAccelerometer<C: Component, V: Vector<C>>{
    /// Error type
    type Error: Debug;

    /// Get raw acceleration data from the accelerometer
    ///
    /// Note: it is expected that you call this when you know
    /// data is ready and valid, ie in calling in response to an
    /// interrupt, polling far slower than your data rate, or if in
    /// a tight loop perhaps like
    /// ```ignore
    /// while !lis3dh.is_data_ready().unwrap() {}
    /// let dat = lis3dh.accel_raw().unwrap();
    /// ```
    /// Please comment this contract on your implementation to inform
    /// your users as well.
    fn accel_raw(&mut self) -> Result<V, Error<Self::Error>>;
}
