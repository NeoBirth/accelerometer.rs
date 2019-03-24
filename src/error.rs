//! Accelerometer errors - generic over an inner "cause" type (intended to be
//! an underlying I2C or SPI error type, if applicable)

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

    /// Create a new error from a cause, e.g. I2C or SPI I/O error
    pub fn from_cause(cause: E) -> Self {
        Self::new_with_cause(ErrorKind::Io, cause)
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
    /// Device invalid or other hardware error
    Device,

    /// I/O error
    Io,
}

impl ErrorKind {
    /// Get a string describing the error
    pub fn description(self) -> &'static str {
        match self {
            ErrorKind::Device => "device error",
            ErrorKind::Io => "I/O error",
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl<E> From<ErrorKind> for Error<E>
where
    E: Debug,
{
    fn from(kind: ErrorKind) -> Error<E> {
        Error::new(kind)
    }
}
