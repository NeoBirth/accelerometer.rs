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
//! Next, impl the [`Accelerometer`] trait (providing normalized readings) and/or
//! the [`RawAccelerometer`] trait (providing direct access to raw data) for
//! your driver (ideally the former, as it provides reuse across drivers).
//!
//! For [`RawAccelerometer`], you will need to choose a [`Vector`] type for
//! raw accelerometer data which best matches the output of your device.
//! This trait has a single method, [`RawAccelerometer::accel_raw`], which
//! returns a reading from the accelerometer or an error.
//!
//! See the [ADXL343 crate] for an example.
//!
//! [Accelerometer]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html
//! [Vector]: https://docs.rs/accelerometer/latest/accelerometer/trait.Vector.html
//! [acceleration]: https://docs.rs/accelerometer/latest/accelerometer/trait.Accelerometer.html#tymethod.acceleration
//! [ADXL343 crate]: https://github.com/NeoBirth/ADXL343.rs/blob/23664e0c765847708c8751c5d3cce76227c0cc76/src/lib.rs#L184

#![no_std]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/NeoBirth/accelerometer.rs/develop/img/cartesian-ferris.png",
    html_root_url = "https://docs.rs/accelerometer/0.12.0"
)]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

// Enables the `doc_cfg` feature when the `docsrs` configuration attribute is defined.
#![cfg_attr(docsrs, feature(doc_cfg))]

mod accelerometer;
pub mod error;
#[cfg_attr(docsrs, doc(cfg(feature = "orientation")))]
#[cfg(feature = "orientation")]
pub mod orientation;

pub use crate::{accelerometer::*, error::*};
pub use micromath::vector::{self, Vector};

#[cfg(feature = "orientation")]
pub use crate::orientation::*;
