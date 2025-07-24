use crate::types::{FlowUnits, HeadLossType};
use bindings as ffi;
use epanet_error::*;
use std::ffi::CString;

/// An EPANET Project wrapper
pub struct EPANET {
    ph: ffi::EN_Project,
}

impl EPANET {
    /// Creates a new EPANET project handle by calling the underlying C API.
    ///
    /// # Returns
    /// * `Ok(ffi::EN_Project)` - A valid project handle on success.
    /// * `Err(EPANETError)` - An error if the project could not be created.
    ///
    /// # Safety
    /// This function wraps an unsafe FFI call but is itself safe to use. The returned handle
    /// must be properly closed and deleted to avoid resource leaks.
    ///
    /// # Errors
    /// Return an `EPANETError` if the underlying C function fails.

    fn create_project_handle() -> Result<ffi::EN_Project> {
        let mut ph: ffi::EN_Project = std::ptr::null_mut();
        let result = unsafe { ffi::EN_createproject(&mut ph) };
        if result != 0 {
            Err(EPANETError::from(result))
        } else {
            Ok(ph)
        }
    }
    /// Creates a new EPANET instance by reading an input file.
    ///
    /// # Arguments
    /// * `inp_path` - Path to an existing EPANET-formatted input file.
    /// * `report_path` - Path to the report file to be created, or an empty string if not needed.
    /// * `out_path` - Path to the binary output file to be created, or an empty string if not needed.
    ///
    /// # Errors
    /// Returns an `EPANETError` if the creation or opening of the project fails.
    pub fn new(
        report_path: &str,
        out_path: &str,
        flow_units_type: FlowUnits,
        head_loss_type: HeadLossType,
    ) -> Result<Self> {
        // Step 1: Initialize the project handle
        let ph = Self::create_project_handle()?;

        // Step 2: Convert paths to C-compatible strings (panic on failure)
        let rpt = CString::new(report_path).expect("report_path contains null bytes");
        let out = CString::new(out_path).expect("out_path contains null bytes");

        // Step 3: Open the project
        let result = unsafe {
            ffi::EN_init(
                ph,
                rpt.as_ptr(),
                out.as_ptr(),
                flow_units_type as i32,
                head_loss_type as i32,
            )
        };
        if result != 0 {
            unsafe { ffi::EN_deleteproject(ph) }; // Clean up on failure
            return Err(EPANETError::from(result));
        }

        // Step 4: Return the EPANET instance
        Ok(Self { ph })
    }

    pub fn with_inp_file(inp_path: &str, report_path: &str, out_path: &str) -> Result<Self> {
        // Step 1: Initialize the project handle
        let ph = Self::create_project_handle()?;

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

    pub fn with_inp_file_allow_errors(
        inp_path: &str,
        report_path: &str,
        out_path: &str,
    ) -> Result<Self> {
        // Step 1: Initialize the project handle
        let ph = Self::create_project_handle()?;

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
}

unsafe impl Send for EPANET {}
unsafe impl Sync for EPANET {}

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
mod error_messages;
pub mod impls;
pub mod types;
