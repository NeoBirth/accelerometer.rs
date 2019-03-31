//! Floating point operations which would ordinarily be provided by `std`
//! but aren't available in `no_std`.

#![allow(clippy::unreadable_literal)]

use core::f32::consts::PI;

/// `std`-like extension methods for math support
pub(crate) trait F32Ext: Sized {
    /// Compute absolute value
    fn abs(self) -> f32;

    /// Compute four quadrant arctangent normalized between `[0, 2)`
    fn atan2_norm(self, other: f32) -> f32;

    /// Compute four quadrant arctangent
    fn atan2(self, other: f32) -> f32;

    /// Compute `self` in radians given a min and max value
    ///
    /// Panics if `self` > `n`
    fn radians<N: Into<f32>>(self, min: N, max: N) -> f32;

    /// Assuming self is normalized between `[0, 2)`, compute radians
    fn radians_norm(self) -> f32;

    /// Compute square root
    fn sqrt(self) -> Self;
}

impl F32Ext for f32 {
    fn abs(self) -> f32 {
        if self < 0.0 {
            -self
        } else {
            self
        }
    }

    fn atan2_norm(self, other: f32) -> f32 {
        atan2_norm_approx(self, other)
    }

    fn atan2(self, other: f32) -> f32 {
        atan2_approx(self, other)
    }

    fn radians<N>(self, min: N, max: N) -> f32
    where
        N: Into<f32>,
    {
        let n_normalized = (self - min.into()) / max.into();
        debug_assert!(n_normalized <= 1.0);
        n_normalized * 2.0 * PI
    }

    fn radians_norm(self) -> f32 {
        PI * if self > 1.0 { self - 2.0 } else { self }
    }

    fn sqrt(self) -> Self {
        sqrt_approx(self)
    }
}

/// Computes an `atan2` approximation in radians (see below)
fn atan2_approx(y: f32, x: f32) -> f32 {
    atan2_norm_approx(y, x).radians_norm()
}

/// Approximates the four quadrant arctangent for a single-precision float.
/// Normalized to the `[0, 2)` range with a maximum error of `0.1620` degrees.
///
/// Method described at: <https://ieeexplore.ieee.org/document/6375931>
fn atan2_norm_approx(y: f32, x: f32) -> f32 {
    const SIGN_MASK: u32 = 0x80000000;
    const B: f32 = 0.596227;

    // Extract sign bits from floating point values
    let ux_s = SIGN_MASK & x.to_bits();
    let uy_s = SIGN_MASK & y.to_bits();

    // Determine quadrant offset
    let q = ((!ux_s & uy_s) >> 29 | ux_s >> 30) as f32;

    // Calculate arctangent in the first quadrant
    let bxy_a = (B * x * y).abs();
    let n = bxy_a + y * y;
    let atan_1q = n / (x * x + bxy_a + n);

    // Translate it to the proper quadrant
    let uatan_2q = (ux_s ^ uy_s) | atan_1q.to_bits();
    (q + f32::from_bits(uatan_2q)) / 2.0
}

/// Square root approximation function for a single-precision float.
/// Separated from `F32Ext` in order to make it easier to test.
///
/// Method described at: <https://bits.stephan-brumme.com/squareRoot.html>
fn sqrt_approx(n: f32) -> f32 {
    let mut n = n.to_bits();
    n += 127 << 23;
    n >>= 1;
    f32::from_bits(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atan2_approx_test() {
        let atan2_test_vectors = [
            (0.0, 1.0, 0.0),
            (0.0, -1.0, PI),
            (3.0, 2.0, (3.0f32 / 2.0).atan()),
            (2.0, -1.0, (2.0f32 / -1.0).atan() + PI),
            (-2.0, -1.0, (-2.0f32 / -1.0).atan() - PI),
        ];

        for (y, x, expected) in &atan2_test_vectors {
            let actual = atan2_approx(*y, *x);
            let delta = actual - expected;

            assert!(
                // 0.1620 degrees in radians
                delta <= 0.003,
                "delta {} too large: {} vs {}",
                delta,
                actual,
                expected
            );
        }
    }

    #[test]
    fn sqrt_approx_test() {
        let sqrt_test_vectors = [
            (1.0, 1.0),
            (2.0, 1.414),
            (3.0, 1.732),
            (4.0, 2.0),
            (5.0, 2.236),
            (10.0, 3.162),
            (100.0, 10.0),
            (250.0, 15.811),
            (500.0, 22.36),
            (1000.0, 31.622),
            (2500.0, 50.0),
            (5000.0, 70.710),
            (1000000.0, 1000.0),
            (2500000.0, 1581.138),
            (5000000.0, 2236.067),
            (10000000.0, 3162.277),
            (25000000.0, 5000.0),
            (50000000.0, 7071.067),
            (100000000.0, 10000.0),
        ];

        for (x, expected) in &sqrt_test_vectors {
            let sqrt_x = sqrt_approx(*x);
            let allowed_delta = x * 0.05;
            let actual_delta = sqrt_x - expected;

            assert!(
                actual_delta <= allowed_delta,
                "delta {} too large: {} vs {}",
                actual_delta,
                sqrt_x,
                expected
            );
        }
    }
}
