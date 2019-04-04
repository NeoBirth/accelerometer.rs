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
    html_logo_url = "https://raw.githubusercontent.com/NeoBirth/accelerometer.rs/master/img/cartesian-ferris.png",
    html_root_url = "https://docs.rs/accelerometer/0.4.0"
)]

#[macro_use]
extern crate generic_array;

mod accelerometer;
pub mod error;
mod f32ext;
#[cfg(feature = "orientation")]
pub mod orientation;
pub mod vector;

#[cfg(feature = "orientation")]
pub use crate::orientation::*;
pub use crate::{accelerometer::*, error::*, vector::*};
