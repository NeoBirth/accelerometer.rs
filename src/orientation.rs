//! Orientation tracking for accelerometer-equipped devices.

mod tracker;

pub use self::tracker::Tracker;

/// Device orientation as computed from accelerometer data
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    /// Unable to determine the orientation from current data
    Unknown,

    /// Device is in portrait mode in whatever way is considered "up"
    PortraitUp,

    /// Device is in portrait mode in whatever way is considered "down"
    PortraitDown,

    /// Device is in landscape mode in whatever way is considered "up"
    LandscapeUp,

    /// Device is in landscape mode in whatever way is considered "down"
    LandscapeDown,

    /// Device is parallel to the ground, facing up
    FaceUp,

    /// Device is parallel to the ground, facing down
    FaceDown,
}

impl Orientation {
    /// Is this orientation considered to be flat?
    pub fn is_flat(self) -> bool {
        matches!(self, Orientation::FaceUp | Orientation::FaceDown)
    }

    /// Is the device in a landscape orientation?
    pub fn is_landscape(self) -> bool {
        matches!(self, Orientation::LandscapeUp | Orientation::LandscapeDown)
    }

    /// Is the device in a portrait orientation?
    pub fn is_portrait(self) -> bool {
        matches!(self, Orientation::PortraitUp | Orientation::PortraitDown)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
#[cfg(feature = "defmt")]
impl defmt::Format for Orientation {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        // Implemented manually so that the docs.rs marker can be applied.
        match self {
            Orientation::Unknown => {
                defmt::write!(fmt, "Unknown")
            }
            Orientation::PortraitUp => {
                defmt::write!(fmt, "PortraitUp")
            }
            Orientation::PortraitDown => {
                defmt::write!(fmt, "PortraitDown")
            }
            Orientation::LandscapeUp => {
                defmt::write!(fmt, "LandscapeUp")
            }
            Orientation::LandscapeDown => {
                defmt::write!(fmt, "LandscapeDown")
            }
            Orientation::FaceUp => {
                defmt::write!(fmt, "FaceUp")
            }
            Orientation::FaceDown => {
                defmt::write!(fmt, "FaceDown")
            }
        }
    }
}
