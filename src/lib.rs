//! Generic embedded-friendly accelerometer support, including traits and types
//! for representing readings from 2 or 3-axis accelerometers.
//!
//! Also includes optional device position tracking support with statistical
//! methods for smoothing out noisy accelerometer data.
//!
//! ## Writing an accelerometer driver
//!
//! The `accelerometer` crate is intended to provide a "HAL" for accelerometers
//! which allows several accelerometer-consuming crates to leverage a common
//! API and set of vector types for consuming accelerometer data.
//!
//! Accelerometer drivers are intended to use `embedded-hal` I2C or SPI
//! interfaces. The first task of writing a driver is to choose one of these
//! and write a driver which is able to communicate with the accelerometer and
//! obtain data.
//!
//! Next, impl the [Accelerometer] trait for your driver. You will need to
//! choose a [Vector] type for representing accelerometer data which best
//! matches the output of your device. This trait has a single method,
//! [acceleration], which returns a reading from the accelerometer or an error.
//!
//! See the [ADXL343 crate] for an example.
//!
//! [Accelerometer]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html
//! [Vector]: https://docs.rs/accelerometer/latest/accelerometer/trait.Vector.html
//! [acceleration]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html#tymethod.acceleration
//! [ADXL343 crate]: https://github.com/NeoBirth/ADXL343.rs/blob/23664e0c765847708c8751c5d3cce76227c0cc76/src/lib.rs#L184

#![no_std]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/NeoBirth/accelerometer.rs/develop/img/cartesian-ferris.png",
    html_root_url = "https://docs.rs/accelerometer/0.5.1"
)]

mod accelerometer;
pub mod error;
#[cfg(feature = "orientation")]
pub mod orientation;

#[cfg(feature = "orientation")]
pub use crate::orientation::*;
pub use crate::{accelerometer::*, error::*};
pub use micromath::vector::{self, *};
