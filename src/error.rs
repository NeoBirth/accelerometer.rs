//! Accelerometer errors - generic over an inner "cause" type (intended to be
//! an underlying I2C or SPI error type, if applicable).

use core::fmt::{self, Debug, Display};

/// Accelerometer errors, generic around another error type `E` representing
/// an (optional) cause of this error.
#[derive(Clone, Debug)]
pub struct Error<E: Debug> {
    /// Kind of error which occurred
    kind: ErrorKind,

    /// Cause of the error (if applicable)
    cause: Option<E>,
}

impl<E> Error<E>
where
    E: Debug,
{
    /// Create a new error
    pub fn new(kind: ErrorKind) -> Self {
        Self { kind, cause: None }
    }

    /// Create a new error with a cause
    pub fn new_with_cause(kind: ErrorKind, cause: E) -> Self {
        Self {
            kind,
            cause: Some(cause),
        }
    }

    /// Get the kind of error which occurred
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Get the cause of the underlying error (if applicable)
    pub fn cause(&self) -> Option<&E> {
        self.cause.as_ref()
    }

    /// Convert this error into its underlying cause.
    ///
    /// Panics if the error does not have a cause.
    pub fn into_cause(self) -> E {
        self.cause
            .expect("into_cause called on an error with no cause")
    }
}

/// Kinds of accelerometer errors
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    /// Error in the underlying communications bus (e.g. I2C, SPI)
    Bus,

    /// Device invalid or other hardware error
    Device,

    /// Device is in an invalid mode to complete the requested operation
    Mode,

    /// Invalid parameter
    Param,
}

impl ErrorKind {
    /// Create an `Err(Error)` out of this `ErrorKind`
    pub fn err<E: Debug>(self) -> Result<(), Error<E>> {
        Err(Error::new(self))
    }
}

impl ErrorKind {
    /// Get a string describing the error
    pub fn description(self) -> &'static str {
        match self {
            ErrorKind::Device => "device error",
            ErrorKind::Bus => "bus error",
            ErrorKind::Mode => "invalid mode",
            ErrorKind::Param => "invalid parameter",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E> From<E> for Error<E>
where
    E: Debug,
{
    /// Create a new error from a cause, e.g. I2C or SPI I/O error
    fn from(cause: E) -> Error<E> {
        Self::new_with_cause(ErrorKind::Bus, cause)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
#[cfg(feature = "defmt")]
impl<E: Debug + defmt::Format> defmt::Format for Error<E> {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        // Implemented manually so that the docs.rs marker can be applied.
        defmt::write!(
            fmt,
            "Error {{ kind = {}, cause = {}  }}",
            self.kind,
            self.cause
        )
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "defmt")))]
#[cfg(feature = "defmt")]
impl defmt::Format for ErrorKind {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        // Implemented manually so that the docs.rs marker can be applied.
        match self {
            ErrorKind::Device => {
                defmt::write!(fmt, "Device")
            }
            ErrorKind::Bus => {
                defmt::write!(fmt, "Bus")
            }
            ErrorKind::Mode => {
                defmt::write!(fmt, "Mode")
            }
            ErrorKind::Param => {
                defmt::write!(fmt, "Param")
            }
        }
    }
}
