//! Generic embedded-friendly accelerometer support, including traits and types
//! for representing readings from 2 or 3-axis accelerometers.
//!
//! Also includes optional device position tracking support with statistical
//! methods for smoothing out noisy accelerometer data.

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
