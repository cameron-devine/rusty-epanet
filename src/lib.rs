// The `as i32` casts on bindgen constants are required for cross-platform
// compatibility — some compilers emit u32, others i32.
// The too_many_arguments functions mirror the C API signatures directly.
#![allow(clippy::unnecessary_cast, clippy::too_many_arguments)]

pub mod types;
use bindings as ffi;
use epanet_error::*;
use std::cell::Cell;
use std::ffi::CString;
use std::os::raw::c_void;
use types::options::{FlowUnits, HeadLossType};
use types::report::ReportCallback;

/// An EPANET Project wrapper.
///
/// This struct owns the EPANET project handle and provides safe Rust wrappers
/// for all EPANET C API functions. When dropped, it automatically closes the
/// project and frees all associated resources.
///
/// # Thread Safety
///
/// `EPANET` implements `Send` but **not** `Sync`. Each project handle can be
/// moved to another thread, but it cannot be shared concurrently via `&EPANET`
/// because the underlying C library uses internal mutable state (e.g., shared
/// message buffers, `strtok()`) that is not safe for concurrent access.
///
/// To share an `EPANET` instance across threads, wrap it in `Arc<Mutex<EPANET>>`.
///
/// # Report Callback
///
/// An optional report callback can be registered via [`set_report_callback`](Self::set_report_callback)
/// to intercept report output instead of writing to a file. The callback is
/// automatically freed when the `EPANET` instance is dropped or when a new
/// callback is registered.
pub struct EPANET {
    /// The EPANET project handle (opaque pointer to C struct)
    pub(crate) ph: ffi::EN_Project,

    /// Raw pointer to the boxed report callback, if one is registered.
    ///
    /// This is stored as a raw pointer rather than `Option<Box<ReportCallback>>`
    /// because we need to pass it to the C API as `*mut c_void` user data.
    /// The pointer is created via `Box::into_raw` and must be freed via
    /// `Box::from_raw` when the callback is replaced or the struct is dropped.
    report_callback_ptr: Option<*mut c_void>,

    /// Whether the project has been closed by the C API (e.g. via `EN_runproject`).
    /// When true, `Drop` skips calling `EN_close` to avoid double-free.
    closed: Cell<bool>,
}

// Manual Debug implementation since *mut c_void doesn't implement Debug nicely
impl std::fmt::Debug for EPANET {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EPANET")
            .field("ph", &self.ph)
            .field(
                "report_callback_ptr",
                &self.report_callback_ptr.map(|p| format!("{:p}", p)),
            )
            .field("closed", &self.closed.get())
            .finish()
    }
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
    pub(crate) fn create_project_handle() -> Result<ffi::EN_Project> {
        let mut ph: ffi::EN_Project = std::ptr::null_mut();
        let result = unsafe { ffi::EN_createproject(&mut ph) };
        check_error(result)?;
        Ok(ph)
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
        if let Err(e) = check_error(result) {
            unsafe { ffi::EN_deleteproject(ph) }; // Clean up on failure
            return Err(e);
        }

        // Step 4: Return the EPANET instance
        Ok(Self {
            ph,
            report_callback_ptr: None,
            closed: Cell::new(false),
        })
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
        if let Err(e) = check_error(result) {
            unsafe { ffi::EN_deleteproject(ph) }; // Clean up on failure
            return Err(e);
        }

        // Step 4: Return the EPANET instance
        Ok(Self {
            ph,
            report_callback_ptr: None,
            closed: Cell::new(false),
        })
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

        // Step 3: Open the project, allowing warning codes (1-99) through
        let result = unsafe { ffi::EN_open(ph, inp.as_ptr(), rpt.as_ptr(), out.as_ptr()) };
        if let Err(e) = check_error_allow_warnings(result) {
            unsafe { ffi::EN_deleteproject(ph) }; // Clean up on failure
            return Err(e);
        }

        // Step 4: Return the EPANET instance
        Ok(Self {
            ph,
            report_callback_ptr: None,
            closed: Cell::new(false),
        })
    }
}

unsafe impl Send for EPANET {}

impl Drop for EPANET {
    fn drop(&mut self) {
        // Free the report callback if one is registered.
        // SAFETY: If report_callback_ptr is Some, it was created via Box::into_raw
        // in set_report_callback and has not been freed yet.
        if let Some(ptr) = self.report_callback_ptr.take() {
            unsafe {
                // First, unregister the callback from EPANET to prevent any further calls
                let _ = ffi::EN_setreportcallback(self.ph, None);
                let _ = ffi::EN_setreportcallbackuserdata(self.ph, std::ptr::null_mut());

                // Then free the boxed closure
                drop(Box::from_raw(ptr as *mut ReportCallback));
            }
        }

        // Close (if not already closed) and delete the EPANET project.
        // EN_close frees network data; calling it twice causes a double-free.
        // EN_deleteproject frees the project struct itself and is always safe.
        unsafe {
            if !self.closed.get() {
                ffi::EN_close(self.ph);
            }
            ffi::EN_deleteproject(self.ph);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::temp_rpt_path;

    #[test]
    fn epanet_is_send() {
        fn assert_send<T: Send>() {}
        assert_send::<EPANET>();
    }

    /// Verify that EPANET does not implement Sync.
    /// This is a documentation-level assertion — Rust stable doesn't support
    /// negative trait bounds, so we rely on the absence of `unsafe impl Sync`
    /// in lib.rs. If someone accidentally adds it, the strtok-based tests
    /// in CI (with --test-threads=1 removed) would catch the race conditions.
    #[test]
    fn epanet_is_not_sync() {}

    #[test]
    fn move_to_thread() {
        let rpt = temp_rpt_path();
        let ph = EPANET::new(&rpt, "", FlowUnits::Cfs, HeadLossType::HazenWilliams)
            .expect("Failed to create project");

        let handle = std::thread::spawn(move || {
            // Use the project on another thread
            let count = ph.get_count(types::CountType::NodeCount).unwrap();
            assert_eq!(count, 0);
        });
        handle.join().unwrap();
    }

    #[test]
    fn sequential_cross_thread_transfer() {
        let rpt = temp_rpt_path();
        let ph = EPANET::new(&rpt, "", FlowUnits::Cfs, HeadLossType::HazenWilliams)
            .expect("Failed to create project");

        // Move to thread 1
        let handle = std::thread::spawn(move || {
            let _ = ph.get_count(types::CountType::NodeCount).unwrap();
            ph // move back
        });
        let ph = handle.join().unwrap();

        // Move to thread 2
        let handle = std::thread::spawn(move || {
            let _ = ph.get_count(types::CountType::NodeCount).unwrap();
            ph
        });
        let _ph = handle.join().unwrap();
    }

    #[test]
    fn arc_mutex_shared_access() {
        use std::sync::{Arc, Mutex};

        let rpt = temp_rpt_path();
        let ph = EPANET::new(&rpt, "", FlowUnits::Cfs, HeadLossType::HazenWilliams)
            .expect("Failed to create project");
        let shared = Arc::new(Mutex::new(ph));

        let mut handles = vec![];
        for _ in 0..4 {
            let shared = Arc::clone(&shared);
            handles.push(std::thread::spawn(move || {
                let ph = shared.lock().unwrap();
                let count = ph.get_count(types::CountType::NodeCount).unwrap();
                assert_eq!(count, 0);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}

mod bindings;
pub mod epanet_error;
mod error_messages;
pub mod impls;

pub use impls::project::{run_project, run_project_with_callback};
