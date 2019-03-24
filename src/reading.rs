//! Accelerometer readings - axis-specific acceleration measurements

use core::fmt::Debug;
use generic_array::{
    typenum::{U2, U3},
    ArrayLength, GenericArray,
};

/// Accelerometer readings
pub trait Reading: Copy + Debug + Default + PartialEq + Sized + Send + Sync {
    /// Type representing measured acceleration for a particular axis
    type Component: Copy + Sized + Into<f32>;

    /// Number of axes
    type Axes: ArrayLength<Self::Component>;

    /// Obtain an array of the acceleration components for each of the axes
    fn to_array(&self) -> GenericArray<Self::Component, Self::Axes>;
}

macro_rules! impl_2axis_reading {
    ($reading:ident, $component:ty, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $reading {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,
        }

        impl $reading {
            /// Instantiate from X and Y components
            pub fn new(x: $component, y: $component) -> Self {
                $reading { x, y }
            }
        }

        impl Reading for $reading {
            type Component = $component;
            type Axes = U2;

            fn to_array(&self) -> GenericArray<$component, U2> {
                arr![$component; self.x, self.y]
            }
        }

        impl From<($component, $component)> for $reading {
            fn from(reading: ($component, $component)) -> Self {
                $reading::new(reading.0, reading.1)
            }
        }
    }
}

impl_2axis_reading!(I8x2, i8, "2-axis XY pair of `i8` values");
impl_2axis_reading!(I16x2, i16, "2-axis XY pair of `i16` values");
impl_2axis_reading!(U8x2, u8, "2-axis XY pair of `u8` values");
impl_2axis_reading!(U16x2, u16, "2-axis XY pair of `u16` values");
impl_2axis_reading!(F32x2, f32, "2-axis XY pair of `f32` values");

impl From<I8x2> for F32x2 {
    fn from(reading: I8x2) -> F32x2 {
        F32x2::new(reading.x.into(), reading.y.into())
    }
}

impl From<I16x2> for F32x2 {
    fn from(reading: I16x2) -> F32x2 {
        F32x2::new(reading.x.into(), reading.y.into())
    }
}

impl From<U8x2> for F32x2 {
    fn from(reading: U8x2) -> F32x2 {
        F32x2::new(reading.x.into(), reading.y.into())
    }
}

impl From<U16x2> for F32x2 {
    fn from(reading: U16x2) -> F32x2 {
        F32x2::new(reading.x.into(), reading.y.into())
    }
}

macro_rules! impl_3axis_reading {
    ($reading:ident, $component:ty, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $reading {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,

            /// Z component
            pub z: $component,
        }

        impl $reading {
            /// Instantiate from X, Y, and Z components
            pub fn new(x: $component, y: $component, z: $component) -> Self {
                $reading { x, y, z }
            }
        }

        impl Reading for $reading {
            type Component = $component;
            type Axes = U3;

            fn to_array(&self) -> GenericArray<$component, U3> {
                arr![$component; self.x, self.y, self.z]
            }
        }

        impl From<($component, $component, $component)> for $reading {
            fn from(reading: ($component, $component, $component)) -> Self {
                $reading::new(reading.0, reading.1, reading.2)
            }
        }
    }
}

impl_3axis_reading!(I8x3, i8, "3-axis XYZ triple of `i8` values");
impl_3axis_reading!(I16x3, i16, "3-axis XYZ triple of `i16` values");
impl_3axis_reading!(U8x3, u8, "3-axis XYZ triple of `u8` values");
impl_3axis_reading!(U16x3, u16, "3-axis XYZ triple of `u16` values");
impl_3axis_reading!(F32x3, f32, "3-axis XYZ triple of `f32` values");

impl From<I8x3> for F32x3 {
    fn from(reading: I8x3) -> F32x3 {
        F32x3::new(reading.x.into(), reading.y.into(), reading.z.into())
    }
}

impl From<I16x3> for F32x3 {
    fn from(reading: I16x3) -> F32x3 {
        F32x3::new(reading.x.into(), reading.y.into(), reading.z.into())
    }
}

impl From<U8x3> for F32x3 {
    fn from(reading: U8x3) -> F32x3 {
        F32x3::new(reading.x.into(), reading.y.into(), reading.z.into())
    }
}

impl From<U16x3> for F32x3 {
    fn from(reading: U16x3) -> F32x3 {
        F32x3::new(reading.x.into(), reading.y.into(), reading.z.into())
    }
}
