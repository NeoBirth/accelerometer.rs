//! Device position tracker which uses a sliding widow of acceleration data
//! samples to help filter the signal from the noise.

use crate::{accelerometer::Accelerometer, orientation::Orientation, vector::Vector};
use core::marker::PhantomData;
use micromath::generic_array::typenum::U3;

// Spuriously triggers unused import warnings in cases std is linked
#[allow(unused_imports)]
use micromath::F32Ext;

/// Device position tracker which which filters noisy accelerometer data
/// using statistical methods
pub struct Tracker<A, V>
where
    A: Accelerometer<V>,
    V: Vector<Axes = U3>,
{
    /// The underlying accelerometer device
    accelerometer: A,

    /// Threshold at which acceleration due to gravity is registered
    threshold: f32,

    /// Last orientation type read from the accelerometer
    last_orientation: Orientation,

    /// Vector type
    vector: PhantomData<V>,
}

impl<A, V> Tracker<A, V>
where
    A: Accelerometer<V>,
    V: Vector<Axes = U3>,
{
    /// Create a new orientation tracker
    pub fn new(accelerometer: A, threshold: V::Component) -> Self {
        Self {
            accelerometer,
            threshold: threshold.into(),
            last_orientation: Orientation::Unknown,
            vector: PhantomData,
        }
    }

    /// Get an orientation reading
    pub fn orientation(&mut self) -> Result<Orientation, A::Error> {
        let accel = self.accelerometer.acceleration()?.to_array();
        let x: f32 = accel[0].into();
        let y: f32 = accel[1].into();
        let z: f32 = accel[2].into();

        let result = if x.abs() > self.threshold {
            // Landscape
            if x >= 0.0 {
                Orientation::LandscapeUp
            } else {
                Orientation::LandscapeDown
            }
        } else if y.abs() > self.threshold {
            // Portrait
            if y >= 0.0 {
                Orientation::PortraitUp
            } else {
                Orientation::PortraitDown
            }
        } else if z.abs() > self.threshold {
            // Flat
            if z >= 0.0 {
                Orientation::FaceUp
            } else {
                Orientation::FaceDown
            }
        } else {
            Orientation::Unknown
        };

        if result != Orientation::Unknown {
            self.last_orientation = result;
        }

        Ok(result)
    }

    /// Get the last known orientation reading for the device
    pub fn last_orientation(&self) -> Orientation {
        self.last_orientation
    }
}
