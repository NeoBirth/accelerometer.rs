//! Floating point operations which would ordinarily be provided by `std`
//! but aren't available in `no_std`.

use core::mem;

/// `std`-like extension methods for math support
pub(crate) trait F32Ext: Sized {
    fn sqrt(self) -> Self;
}

impl F32Ext for f32 {
    /// Square root approximation
    fn sqrt(self) -> Self {
        sqrt_approx(self)
    }
}

/// Square root approximation function for a single-precision float.
/// Separated from `F32Ext` in order to make it easier to test.
fn sqrt_approx(n: f32) -> f32 {
    #[allow(unsafe_code)]
    let mut n: u32 = unsafe { mem::transmute(n) };
    n += 127 << 23;
    n >>= 1;
    f32::from_bits(n)
}

#[cfg(test)]
mod tests {
    use super::sqrt_approx;

    /// Square root test vectors
    const SQRT_TEST_VECTORS: &[(f32, f32)] = &[
        (1.0, 1.0),
        (2.0, 1.414),
        (3.0, 1.732),
        (4.0, 2.0),
        (5.0, 2.236),
        (6.0, 2.449),
        (7.0, 2.645),
        (8.0, 2.828),
        (9.0, 3.0),
        (10.0, 3.162),
        (100.0, 10.0),
        (250.0, 15.811),
        (500.0, 22.36),
        (1000.0, 31.622),
        (2500.0, 50.0),
        (5000.0, 70.710),
        (10000.0, 100.0),
        (25000.0, 158.113),
        (50000.0, 223.606),
        (100000.0, 316.227),
        (250000.0, 500.0),
        (500000.0, 707.106),
        (1000000.0, 1000.0),
        (2500000.0, 1581.138),
        (5000000.0, 2236.067),
        (10000000.0, 3162.277),
        (25000000.0, 5000.0),
        (50000000.0, 7071.067),
        (100000000.0, 10000.0),
    ];

    #[test]
    fn sqrt_approx_test() {
        for (n, n_sqrt) in SQRT_TEST_VECTORS {
            let allowed_delta = n * 0.05;
            let actual_delta = sqrt_approx(*n) - n_sqrt;
            assert!(
                actual_delta <= allowed_delta,
                "delta {} too large: {} vs {}",
                actual_delta,
                sqrt_approx(*n),
                n_sqrt
            );
        }
    }
}
