//! Algebraic vector types generic over a number of axes and a component type,
//! useful for representing accelerometer data or values computed from it.

#[allow(unused_imports)]
use crate::f32ext::F32Ext;
use core::{
    fmt::Debug,
    ops::{Add, Div, Index, Mul, MulAssign, Sub},
};
use generic_array::{typenum::Unsigned, ArrayLength, GenericArray};

mod iter;
mod xy;
mod xyz;

pub use self::{iter::*, xy::*, xyz::*};

/// Vectors with numeric components
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

    /// Smallest value representable by a vector component
    const MIN: Self::Component;

    /// Largest value representable by a vector component
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
        /// Maximum number of axes we presently support
        // TODO(tarcieri): replace this with better use of `generic-array` or const generics
        // Won't someone please think of the n-dimensional hyperspace accelerometers?
        const MAX_AXES: usize = 3;

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
