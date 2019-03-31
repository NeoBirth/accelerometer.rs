//! Device position tracking which uses statistical methods to filter noisy
//! accelerometer data (moving average computed from a trimmed mean with
//! outliers culled).

mod orientation;
mod samples;
mod tracker;

pub use self::{orientation::*, samples::Samples, tracker::*};
