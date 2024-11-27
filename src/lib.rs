use bindings as ffi;
use std::ffi::{c_int, CString};
use std::mem::MaybeUninit;

use crate::types::CountType;
use epanet_error::*;

/// An EPANET Project wrapper
pub struct EPANET {
    ph: ffi::EN_Project,
}

impl EPANET {
    /// Creates a new EPANET instance by reading an input file.
    ///
    /// # Arguments
    /// * `inp_path` - Path to an existing EPANET-formatted input file.
    /// * `report_path` - Path to the report file to be created, or an empty string if not needed.
    /// * `out_path` - Path to the binary output file to be created, or an empty string if not needed.
    ///
    /// # Errors
    /// Returns an `EPANETError` if the creation or opening of the project fails.
    pub fn new(inp_path: &str, report_path: &str, out_path: &str) -> Result<Self> {
        // Step 1: Initialize the project handle
        let mut ph = MaybeUninit::<*mut ffi::Project>::uninit();
        let result = unsafe { ffi::EN_createproject(ph.as_mut_ptr()) };
        if result != 0 {
            return Err(EPANETError::from(result));
        }
        let ph = unsafe { ph.assume_init() };

        // Step 2: Convert paths to C-compatible strings (panic on failure)
        let inp = CString::new(inp_path).expect("inp_path contains null bytes");
        let rpt = CString::new(report_path).expect("report_path contains null bytes");
        let out = CString::new(out_path).expect("out_path contains null bytes");

        // Step 3: Open the project
        let result = unsafe { ffi::EN_open(ph, inp.as_ptr(), rpt.as_ptr(), out.as_ptr()) };
        if result != 0 {
            unsafe { ffi::EN_deleteproject(ph) }; // Clean up on failure
            return Err(EPANETError::from(result));
        }

        // Step 4: Return the EPANET instance
        Ok(Self { ph })
    }

    /// Retrieves the number of objects of a given type.
    pub fn get_count(&mut self, count_type: CountType) -> Result<i32> {
        let mut count: MaybeUninit<c_int> = MaybeUninit::uninit();
        unsafe {
            match ffi::EN_getcount(self.ph, count_type as i32, count.as_mut_ptr()) {
                0 => Ok(count.assume_init()),
                x => Err(EPANETError::from(x)),
            }
        }
    }
}

impl Drop for EPANET {
    fn drop(&mut self) {
        unsafe {
            ffi::EN_close(self.ph);
            ffi::EN_deleteproject(self.ph);
        }
    }
}

#[cfg(test)]
mod tests {}

mod bindings;
pub mod epanet_error;
pub mod impls;
pub mod types;
