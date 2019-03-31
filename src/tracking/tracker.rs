//! Device position tracker which uses a sliding widow of acceleration data
//! samples to help filter the signal from the noise.

use super::samples::Samples;
use crate::{accelerometer::Accelerometer, error::Error, vector::Vector};
use core::{fmt::Debug, marker::PhantomData};
use generic_array::{
    typenum::{U16, U32},
    ArrayLength,
};

/// Alias for a `Tracker` with a 16-sample buffer
pub type Tracker16<A, V, E> = Tracker<A, V, E, U16>;

/// Alias for a `Tracker` with a 32-sample buffer
pub type Tracker32<A, V, E> = Tracker<A, V, E, U32>;

/// Device position tracker which which filters noisy accelerometer data
/// using statistical methods
pub struct Tracker<A, V, E, L>
where
    A: Accelerometer<V, E>,
    V: Vector,
    E: Debug,
    L: ArrayLength<V>,
{
    /// The underlying accelerometer device
    accelerometer: A,

    /// Samples of accelerometer data
    samples: Samples<V, L>,

    /// Error type associated with the underlying accelerometer
    errors: PhantomData<E>,
}

impl<A, V, E, S> Tracker<A, V, E, S>
where
    A: Accelerometer<V, E>,
    V: Vector,
    E: Debug,
    S: ArrayLength<V>,
{
    /// Create a new device position tracker for the given accelerometer
    pub fn new(accelerometer: A) -> Self {
        Self {
            accelerometer,
            samples: Samples::new(),
            errors: PhantomData,
        }
    }

    /// Borrow the underlying accelerometer device
    pub fn accelerometer(&self) -> &A {
        &self.accelerometer
    }

    /// Consume `self` and return the underlying accelerometer
    pub fn into_accelerometer(self) -> A {
        self.accelerometer
    }

    /// Borrow the underlying buffer of `Samples`
    pub fn samples(&self) -> &Samples<V, S> {
        &self.samples
    }

    /// Read a sample from the underlying device and store it in the internal
    /// sample buffer
    pub fn update(&mut self) -> Result<V, Error<E>> {
        let sample = self.accelerometer.acceleration()?;
        self.samples.update(sample);
        Ok(sample)
    }

    /// Obtain a moving average (trimmed mean) of the current accelerometer data
    /// (after culling outliers).
    ///
    /// When called, this method will call `Tracker::update` to take a sample
    /// of accelerometer data first. To avoid this, call `Tracker::samples`
    /// and invoke the `mean()` function on the underlying buffer.
    pub fn mean_acceleration(&mut self) -> Result<V, Error<E>> {
        self.update()?;
        Ok(self.samples.trimmed_mean())
    }
}
