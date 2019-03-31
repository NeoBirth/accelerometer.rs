//! Collection of acceleration data samples with support for performing
//! various kinds of statistical analysis on them.

#[allow(unused_imports)]
use crate::f32ext::F32Ext;
use crate::vector::Vector;
use core::slice;
use generic_array::ArrayLength;
use heapless::Vec;

/// Ring buffer for accelerometer data samples
#[derive(Clone)]
pub struct Samples<V, L>
where
    V: Vector,
    L: ArrayLength<V>,
{
    /// Internal sample buffer
    buffer: Vec<V, L>,

    /// Position within our internal sample buffer
    position: usize,
}

impl<V, L> Samples<V, L>
where
    V: Vector,
    L: ArrayLength<V>,
{
    /// Create an empty set of samples
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Update the internal sample buffer with the given sample
    pub fn update(&mut self, sample: V) {
        if self.buffer.len() < L::to_usize() {
            // In this case, we haven't yet filled the internal buffer
            self.buffer.push(sample).unwrap();
        } else {
            self.buffer[self.position] = sample;
        }

        self.position = (self.position + 1) % L::to_usize();
    }

    /// Compute arithmetic mean of the components of an iterator over vectors,
    /// returning a vector consisting of the arithmetic means of its components.
    ///
    /// This method does not cull outliers.
    pub fn mean(&self) -> V {
        V::mean(self.buffer.iter().cloned())
    }

    /// Compute a "trimmed" mean which culls outliers before computing the mean.
    /// This should hopefully help reduce noise from the accelerometer data.
    pub fn trimmed_mean(&self) -> V {
        V::mean(self.iter_trimmed())
    }

    /// Iterate over the raw samples currently in the buffer.
    ///
    /// Iteration order is presently not necessarily correlated to recency.
    pub fn iter(&self) -> slice::Iter<V> {
        self.buffer.iter()
    }

    /// Create a "trimmed" iterator which skips over statistical outliers
    pub fn iter_trimmed(&self) -> IterTrimmed<V> {
        IterTrimmed::new(self)
    }
}

impl<V, L> AsRef<[V]> for Samples<V, L>
where
    V: Vector,
    L: ArrayLength<V>,
{
    fn as_ref(&self) -> &[V] {
        self.buffer.as_ref()
    }
}

impl<V, L> Default for Samples<V, L>
where
    V: Vector,
    L: ArrayLength<V>,
{
    fn default() -> Self {
        Self::new()
    }
}

/// A "trimmed" iterator over the sample buffer which skips outliers
pub struct IterTrimmed<'a, V: Vector> {
    /// Iterator over the underlying sample buffer
    samples: slice::Iter<'a, V>,

    /// Arithmetic mean the sample vectors as a vector
    mean: V,

    /// Standard deviation of the distance from the mean
    stddev: f32,
}

impl<'a, V> IterTrimmed<'a, V>
where
    V: Vector,
{
    /// Create a new trimmed iterator from a set of samples
    fn new<L>(samples: &'a Samples<V, L>) -> Self
    where
        L: ArrayLength<V>,
    {
        let mean = samples.mean();
        let mut sum = 0.0;

        for sample in samples.iter() {
            let n = sample.distance(mean);
            sum += n * n;
        }

        let variance = sum / (L::to_usize() as f32 - 1.0);
        let stddev = variance.sqrt();

        Self {
            samples: samples.iter(),
            mean,
            stddev,
        }
    }
}

impl<'a, V> Iterator for IterTrimmed<'a, V>
where
    V: Vector,
{
    type Item = V;

    fn next(&mut self) -> Option<V> {
        while let Some(sample) = self.samples.next() {
            // TODO(tarcieri): better method for finding outliers? (e.g. MAD, IQD)
            if (sample.distance(self.mean) / self.stddev) < 3.0 {
                return Some(*sample);
            }
        }

        None
    }
}
