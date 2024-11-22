use crate::bindings as ffi;
use core::fmt;
use ffi::EN_SizeLimits_EN_MAXMSG;
use std::ffi::{c_char, CStr};

/// EPANET Errors
#[derive(Debug, Clone)]
pub struct EPANETError {
    _code: i32,
    _message: String,
}

/// EPANET Result type with EPANET specific errors
pub type Result<T> = std::result::Result<T, EPANETError>;

/// Convert error code from C library into EPANETError
impl From<i32> for EPANETError {
    fn from(error: i32) -> Self {
        let out_errmsg: Vec<c_char> = vec![0; EN_SizeLimits_EN_MAXMSG as usize];
        unsafe {
            match ffi::EN_geterror(
                error,
                out_errmsg.as_ptr() as *mut i8,
                ffi::EN_SizeLimits_EN_MAXMSG as i32,
            ) {
                0 => EPANETError {
                    _code: error,
                    _message: CStr::from_ptr(out_errmsg.as_ptr())
                        .to_str()
                        .unwrap()
                        .to_string(),
                },
                x => EPANETError {
                    _code: x,
                    _message: String::from("UNKNOWN ERROR"),
                },
            }
        }
    }
}

/// Display the epanet error code
impl fmt::Display for EPANETError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EPANET Error Code {}: {}", self._code, self._message)
    }
}
