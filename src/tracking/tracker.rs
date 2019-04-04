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
pub struct Tracker<V, A, E, L>
where
    V: Vector,
    A: Accelerometer<V, E>,
    E: Debug,
    L: ArrayLength<V>,
{
    /// The underlying accelerometer device
    accelerometer: A,

    /// Raw samples of accelerometer data
    raw_samples: Samples<V, L>,

    /// Historical trimmed mean values used to compute the smoothed mean
    mean_samples: Samples<V, L>,

    /// Error type associated with the underlying accelerometer
    errors: PhantomData<E>,
}

impl<V, A, E, S> Tracker<V, A, E, S>
where
    V: Vector,
    A: Accelerometer<V, E>,
    E: Debug,
    S: ArrayLength<V>,
{
    /// Create a new device position tracker for the given accelerometer
    pub fn new(accelerometer: A) -> Self {
        Self {
            accelerometer,
            raw_samples: Samples::new(),
            mean_samples: Samples::new(),
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
        &self.raw_samples
    }

    /// Read a sample from the underlying device and store it in the internal
    /// sample buffer
    pub fn update(&mut self) -> Result<V, Error<E>> {
        let sample = self.accelerometer.acceleration()?;
        self.raw_samples.update(sample);
        self.mean_samples.update(self.raw_samples.mean());
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
        Ok(self.mean_samples.mean())
    }
}
