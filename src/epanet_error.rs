use crate::error_messages::get_error_message;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// EPANET Result type with EPANET-specific errors
pub type Result<T> = std::result::Result<T, EPANETError>;

/// Represents errors returned by the EPANET library.
///
/// EPANET errors consist of a numeric error code, a descriptive message, and an optional
/// context string that provides additional information about the error's origin or usage.
///
/// # Fields
/// * `_code` - The numeric error code returned by the EPANET library.
/// * `_message` - A human-readable description of the error associated with the error code.
/// * `_context` - Optional additional context about the error, such as the operation or parameters
///                that caused it.
#[derive(Debug, Clone)]
pub struct EPANETError {
    pub code: i32,
    message: &'static str,
    context: Option<String>,
}

impl Display for EPANETError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.context.is_some() {
            return write!(f, "{} - {:?}", self.message, self.context.as_ref().unwrap());
        }
        write!(f, "{}", self.message)
    }
}

impl PartialEq for EPANETError {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}

impl Error for EPANETError {}

impl EPANETError {
    /// Adds context to the `EPANETError`, returning a new error with the context included.
    ///
    /// # Arguments
    /// * `context` - A string providing additional information about the error.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

/// Convert error code from the C library into EPANETError
impl From<i32> for EPANETError {
    fn from(error: i32) -> Self {
        EPANETError {
            code: error,
            message: get_error_message(error),
            context: None,
        }
    }
}

/// Convenience helper to convert an EPANET error code into a [`Result`].
///
/// The C API uses `0` to indicate success for nearly every function. This helper
/// centralizes that check and translates non-zero codes into [`EPANETError`].
pub(crate) fn check_error(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(EPANETError::from(code))
    }
}

/// Variant of [`check_error`] that attaches additional context to failures.
pub(crate) fn check_error_with_context(
    code: i32,
    context: impl Into<String>,
) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(EPANETError::from(code).with_context(context))
    }
}
