//! 3-Dimensional Vectors (X,Y,Z)

use super::Vector;
use crate::f32ext::F32Ext;
use core::ops::{Index, MulAssign};
use generic_array::{typenum::U3, GenericArray};

/// 3-Dimensional Vectors (X,Y,Z)
pub trait Vector3D: Vector {
    /// Instantiate from X, Y, and Z components
    fn new(x: Self::Component, y: Self::Component, z: Self::Component) -> Self;

    /// Compute pitch of a 3-axis vector normalized between `[0, 2)`
    fn pitch_norm(&self) -> f32 {
        let mut iter = self.iter();

        let x: f32 = iter.next().unwrap().into();
        let y: f32 = iter.next().unwrap().into();
        let z: f32 = iter.next().unwrap().into();

        let x_rad = x.radians(Self::MIN, Self::MAX);
        let y_rad = y.radians(Self::MIN, Self::MAX);
        let z_rad = z.radians(Self::MIN, Self::MAX);

        (-x_rad).atan2_norm((y_rad * y_rad + z_rad * z_rad).sqrt())
    }

    /// Compute pitch of a 3-axis vector in radians
    fn pitch_radians(&self) -> f32 {
        self.pitch_norm().radians_norm()
    }

    /// Compute roll of a 3-axis vector normalized between `[0, 2)`
    fn roll_norm(&self) -> f32 {
        let mut iter = self.iter();

        iter.next().unwrap();
        let y: f32 = iter.next().unwrap().into();
        let z: f32 = iter.next().unwrap().into();

        let y_rad = y.radians(Self::MIN, Self::MAX);
        let z_rad = z.radians(Self::MIN, Self::MAX);

        y_rad.atan2_norm(z_rad)
    }

    /// Compute norm of a 3-axis vector in radians
    fn roll_radians(&self) -> f32 {
        self.roll_norm().radians_norm()
    }
}

macro_rules! impl_3d_vector {
    ($vector:ident, $component:tt, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $vector {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,

            /// Z component
            pub z: $component,
        }

        impl Vector3D for $vector {
            /// Instantiate from X, Y, and Z components
            fn new(x: $component, y: $component, z: $component) -> Self {
                $vector { x, y, z }
            }
        }

        impl Vector for $vector {
            type Component = $component;
            type Axes = U3;

            const MIN: $component = core::$component::MIN;
            const MAX: $component = core::$component::MAX;

            fn from_iter<I>(into_iter: I) -> Self
            where
                I: IntoIterator<Item = Self::Component>
            {
                let mut iter = into_iter.into_iter();

                let x = iter.next().expect("no x-axis component in slice");
                let y = iter.next().expect("no y-axis component in slice");
                let z = iter.next().expect("no z-axis component in slice");
                debug_assert!(iter.next().is_none(), "too many items in 3-axis component slice");

                Self::new(x, y, z)
            }

            #[allow(trivial_numeric_casts)]
            fn from_floats(slice: &[f32]) -> Self {
                Self::from_iter(slice.iter().map(|float| *float as $component))
            }

            fn get(self, i: usize) -> Option<Self::Component> {
                if i <= 2 {
                    Some(self[i])
                } else {
                    None
                }
            }

            fn to_array(self) -> GenericArray<$component, U3> {
                arr![$component; self.x, self.y, self.z]
            }
        }

        impl From<($component, $component, $component)> for $vector {
            fn from(vector: ($component, $component, $component)) -> Self {
                $vector::new(vector.0, vector.1, vector.2)
            }
        }

        impl Index<usize> for $vector {
            type Output = $component;

            fn index(&self, i: usize) -> &$component {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("index out of range")
                }
            }
        }

        impl MulAssign<f32> for $vector {
            #[allow(trivial_numeric_casts)]
            fn mul_assign(&mut self, n: f32) {
                self.x = (f32::from(self.x) * n) as $component;
                self.y = (f32::from(self.y) * n) as $component;
                self.z = (f32::from(self.z) * n) as $component;
            }
        }
    }
}

impl_3d_vector!(I8x3, i8, "3-dimensional XYZ vector of `i8` values");
impl_3d_vector!(I16x3, i16, "3-dimensional XYZ vector of `i16` values");
impl_3d_vector!(U8x3, u8, "3-dimensional XYZ vector of `u8` values");
impl_3d_vector!(U16x3, u16, "3-dimensional XYZ vector of `u16` values");
impl_3d_vector!(F32x3, f32, "3-dimensional XYZ vector of `f32` values");

impl From<I8x3> for F32x3 {
    fn from(vector: I8x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<I16x3> for F32x3 {
    fn from(vector: I16x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<U8x3> for F32x3 {
    fn from(vector: U8x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}

impl From<U16x3> for F32x3 {
    fn from(vector: U16x3) -> F32x3 {
        F32x3::new(vector.x.into(), vector.y.into(), vector.z.into())
    }
}
