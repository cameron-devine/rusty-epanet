use crate::bindings as ffi;
use ffi::EN_SizeLimits_EN_MAXMSG;
use std::error::Error;
use std::ffi::{c_char, CStr};
use std::fmt::{Display, Formatter};

/// EPANET Result type with EPANET specific errors
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
    code: i32,
    message: String,
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

/// Convert error code from C library into EPANETError
impl From<i32> for EPANETError {
    fn from(error: i32) -> Self {
        let out_errmsg: Vec<c_char> = vec![0; EN_SizeLimits_EN_MAXMSG as usize];
        match unsafe {
            ffi::EN_geterror(
                error,
                out_errmsg.as_ptr() as *mut i8,
                EN_SizeLimits_EN_MAXMSG as i32,
            )
        } {
            0 => EPANETError {
                code: error,
                message: unsafe {
                    CStr::from_ptr(out_errmsg.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_string()
                },
                context: None,
            },
            x => EPANETError {
                code: x,
                message: String::from("UNKNOWN ERROR"),
                context: None,
            },
        }
    }
}
