//! Algebraic vector types generic over a number of axes and a component type,
//! useful for representing accelerometer data or values computed from it.

#[allow(unused_imports)]
use crate::math::F32Ext;
use core::{
    fmt::Debug,
    ops::{Add, Div, Index, Mul, MulAssign, Sub},
};
use generic_array::{
    typenum::{Unsigned, U2, U3},
    ArrayLength, GenericArray,
};

/// Maximum number of axes we presently support
// TODO(tarcieri): replace this with better use of `generic-array` or const generics
// Won't someone please think of the n-dimensional hyperspace accelerometers?
const MAX_AXES: usize = 3;

/// Vectors of axis-specific component values
pub trait Vector:
    Copy + Debug + Default + Index<usize> + MulAssign<f32> + PartialEq + Sized + Send + Sync
{
    /// Type representing measured acceleration for a particular axis
    type Component: Copy
        + Default
        + Sized
        + Add<Output = Self::Component>
        + Sub<Output = Self::Component>
        + Mul<Output = Self::Component>
        + Div<Output = Self::Component>
        + Into<f32>;

    /// Number of axes
    type Axes: ArrayLength<Self::Component>;

    /// Largest value representable with a vector component
    const MAX: Self::Component;

    /// Instantiate a `Vector` from an iterator over `Self::Component` values.
    ///
    /// Panics of the iterator is not the correct length.
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self::Component>;

    /// Instantiate a vector from a slice of components.
    ///
    /// Panics if the slice is not the right size.
    fn from_slice(slice: &[Self::Component]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }

    /// Instantiate a vector from a slice of `f32`s.
    ///
    /// Panics if the slice is not the right size.
    fn from_floats(slice: &[f32]) -> Self;

    /// Compute arithmetic mean of the components of an iterator over vectors,
    /// returning a vector consisting of the arithmetic means of its components.
    fn mean<I>(vectors: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        // TODO(tarcieri): find a solution that doesn't need `MAX_AXES`
        debug_assert!(
            Self::Axes::to_usize() <= MAX_AXES,
            "maximum number of axes supported is 3"
        );

        let mut component_values = [0.0; MAX_AXES];

        for sample in vectors {
            for (axis, component) in sample.iter().enumerate() {
                component_values[axis] += component.into();
            }
        }

        for value in &mut component_values[..Self::Axes::to_usize()] {
            *value /= Self::Axes::to_usize() as f32;
        }

        Self::from_floats(&component_values[..Self::Axes::to_usize()])
    }

    /// Get the component value for a particular index
    fn get(self, index: usize) -> Option<Self::Component>;

    /// Iterate over the components of a vector
    fn iter(&self) -> Iter<Self> {
        Iter::new(self)
    }

    /// Compute the distance between two vectors
    fn distance(self, other: Self) -> f32 {
        let differences = self
            .iter()
            .zip(other.iter())
            .map(|(a, b)| a.into() - b.into());

        differences.map(|n| n * n).sum::<f32>().sqrt()
    }

    /// Compute the magnitude of a vector
    fn magnitude(self) -> f32 {
        self.iter()
            .map(|n| {
                let n = n.into();
                n * n
            })
            .sum::<f32>()
            .sqrt()
    }

    /// Obtain an array of the acceleration components for each of the axes
    fn to_array(self) -> GenericArray<Self::Component, Self::Axes>;
}

macro_rules! impl_2axis_vector {
    ($vector:ident, $component:tt, $doc:expr) => {
        #[doc=$doc]
        #[derive(Copy, Clone, Debug, Default, PartialEq)]
        pub struct $vector {
            /// X component
            pub x: $component,

            /// Y component
            pub y: $component,
        }

        impl $vector {
            /// Instantiate from X and Y components
            pub fn new(x: $component, y: $component) -> Self {
                $vector { x, y }
            }
        }

        impl Vector for $vector {
            type Component = $component;
            type Axes = U2;

            const MAX: $component = core::$component::MAX;

            fn from_iter<I>(into_iter: I) -> Self
            where
                I: IntoIterator<Item = Self::Component>
            {
                let mut iter = into_iter.into_iter();

                let x = iter.next().expect("no x-axis component in slice");
                let y = iter.next().expect("no y-axis component in slice");
                debug_assert!(iter.next().is_none(), "too many items in 2-axis component slice");

                Self::new(x, y)
            }

            #[allow(trivial_numeric_casts)]
            fn from_floats(slice: &[f32]) -> Self {
                Self::from_iter(slice.iter().map(|float| *float as $component))
            }

            fn get(self, i: usize) -> Option<Self::Component> {
                if i <= 1 {
                    Some(self[i])
                } else {
                    None
                }
            }

            fn to_array(self) -> GenericArray<$component, U2> {
                arr![$component; self.x, self.y]
            }
        }

        impl From<($component, $component)> for $vector {
            fn from(vector: ($component, $component)) -> Self {
                $vector::new(vector.0, vector.1)
            }
        }

        impl Index<usize> for $vector {
            type Output = $component;

            fn index(&self, i: usize) -> &$component {
                match i {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("index out of range")
                }
            }
        }

        impl MulAssign<f32> for $vector {
            #[allow(trivial_numeric_casts)]
            fn mul_assign(&mut self, n: f32) {
                self.x = (f32::from(self.x) * n) as $component;
                self.y = (f32::from(self.y) * n) as $component;
            }
        }
    }
}

impl_2axis_vector!(I8x2, i8, "2-axis XY vector of `i8` values");
impl_2axis_vector!(I16x2, i16, "2-axis XY vector of `i16` values");
impl_2axis_vector!(U8x2, u8, "2-axis XY vector of `u8` values");
impl_2axis_vector!(U16x2, u16, "2-axis XY vector of `u16` values");
impl_2axis_vector!(F32x2, f32, "2-axis XY vector of `f32` values");

impl From<I8x2> for F32x2 {
    fn from(vector: I8x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<I16x2> for F32x2 {
    fn from(vector: I16x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<U8x2> for F32x2 {
    fn from(vector: U8x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

impl From<U16x2> for F32x2 {
    fn from(vector: U16x2) -> F32x2 {
        F32x2::new(vector.x.into(), vector.y.into())
    }
}

macro_rules! impl_3axis_vector {
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

        impl $vector {
            /// Instantiate from X, Y, and Z components
            pub fn new(x: $component, y: $component, z: $component) -> Self {
                $vector { x, y, z }
            }
        }

        impl Vector for $vector {
            type Component = $component;
            type Axes = U3;

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

impl_3axis_vector!(I8x3, i8, "3-axis XYZ vector of `i8` values");
impl_3axis_vector!(I16x3, i16, "3-axis XYZ vector of `i16` values");
impl_3axis_vector!(U8x3, u8, "3-axis XYZ vector of `u8` values");
impl_3axis_vector!(U16x3, u16, "3-axis XYZ vector of `u16` values");
impl_3axis_vector!(F32x3, f32, "3-axis XYZ vector of `f32` values");

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

/// Iterator over the components of a vector
pub struct Iter<'a, V>
where
    V: Vector,
{
    /// Reference to the original vector
    vector: &'a V,

    /// Iteration position within the vector
    position: usize,
}

impl<'a, V> Iter<'a, V>
where
    V: Vector,
{
    /// Create a new iterator over the vector's components
    fn new(vector: &'a V) -> Self {
        Self {
            vector,
            position: 0,
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V>
where
    V: Vector,
{
    type Item = V::Component;

    fn next(&mut self) -> Option<V::Component> {
        let item = self.vector.get(self.position);
        self.position += 1;
        item
    }
}
