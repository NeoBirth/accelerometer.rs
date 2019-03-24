//! Generic embedded-friendly accelerometer support, including traits and types
//! for representing readings from 1, 2, or 3-axis accelerometers.
//!
//! Also includes optional device position tracking support with statistical
//! methods for smoothing out noisy accelerometer data.

#![no_std]
#![deny(
    warnings,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/accelerometer/0.1.0")]

#[macro_use]
extern crate generic_array;

mod accelerometer;
pub mod reading;

pub use crate::{accelerometer::*, reading::*};
