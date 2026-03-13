//! Report callback types and utilities.
//!
//! This module provides safe Rust abstractions for EPANET's report callback functionality.
//! Instead of writing report output to a file, users can register a callback closure that
//! receives each line of report output as it's generated.
//!
//! # Overview
//!
//! The EPANET C API provides `EN_setreportcallback` and `EN_setreportcallbackuserdata` functions
//! that allow intercepting report output. This module wraps those in a safe, idiomatic Rust API
//! using the **trampoline pattern**:
//!
//! 1. User provides a Rust closure implementing [`FnMut(&str)`]
//! 2. The closure is boxed and stored as raw user data via `EN_setreportcallbackuserdata`
//! 3. A static `extern "C"` trampoline function is registered via `EN_setreportcallback`
//! 4. When EPANET calls the trampoline, it converts the C string and invokes the Rust closure
//!
//! # Safety
//!
//! The trampoline function includes several safety measures:
//! - Null pointer checks for both user data and the line string
//! - Panic catching via [`std::panic::catch_unwind`] to prevent unwinding across FFI boundaries
//! - Graceful handling of invalid UTF-8 in the C string
//!
//! # Example
//!
//! ```ignore
//! use std::sync::{Arc, Mutex};
//! use epanet::EPANET;
//!
//! let mut epanet = EPANET::with_inp_file("network.inp", "", "")?;
//!
//! // Collect report lines in a thread-safe vector
//! let lines = Arc::new(Mutex::new(Vec::new()));
//! let lines_clone = Arc::clone(&lines);
//!
//! epanet.set_report_callback(Some(Box::new(move |line: &str| {
//!     lines_clone.lock().unwrap().push(line.to_string());
//! })))?;
//!
//! // Generate report output
//! epanet.solve_h()?;
//! epanet.write_line_to_report("Custom message")?;
//!
//! // Check captured lines
//! let captured = lines.lock().unwrap();
//! assert!(captured.iter().any(|l| l.contains("Custom message")));
//!
//! // Remove callback (reverts to file-based reporting)
//! epanet.set_report_callback(None)?;
//! ```

use std::ffi::CStr;
use std::os::raw::{c_char, c_void};
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Type alias for the report callback closure.
///
/// The callback receives each line of report output as a `&str`. The closure must be:
/// - `FnMut`: Can be called multiple times and may mutate captured state
/// - `Send`: Safe to send across thread boundaries (required since [`EPANET`] is `Send + Sync`)
///
/// # Examples
///
/// Simple logging callback:
/// ```ignore
/// let callback: ReportCallback = Box::new(|line: &str| {
///     println!("[EPANET] {}", line);
/// });
/// ```
///
/// Collecting lines with shared state:
/// ```ignore
/// use std::sync::{Arc, Mutex};
///
/// let lines = Arc::new(Mutex::new(Vec::new()));
/// let lines_clone = Arc::clone(&lines);
///
/// let callback: ReportCallback = Box::new(move |line: &str| {
///     lines_clone.lock().unwrap().push(line.to_string());
/// });
/// ```
///
/// Filtering and processing:
/// ```ignore
/// let callback: ReportCallback = Box::new(|line: &str| {
///     if line.contains("WARNING") || line.contains("ERROR") {
///         eprintln!("{}", line);
///     }
/// });
/// ```
pub type ReportCallback = Box<dyn FnMut(&str) + Send>;

/// The trampoline function that bridges C callbacks to Rust closures.
///
/// This function is registered with EPANET via `EN_setreportcallback`. When EPANET generates
/// report output, it calls this function with:
/// - `user_data`: A raw pointer to the boxed [`ReportCallback`]
/// - `_project_handle`: The EPANET project handle (unused in the Rust wrapper)
/// - `line`: A null-terminated C string containing the report line
///
/// # Safety
///
/// This function is `unsafe extern "C"` and is called from C code. The following invariants
/// must be maintained by the caller (i.e., the [`EPANET::set_report_callback`] implementation):
///
/// 1. `user_data` must be a valid pointer created via `Box::into_raw(Box::new(callback))`
/// 2. `user_data` must remain valid for the entire duration the callback is registered
/// 3. `user_data` must be freed via `Box::from_raw` when the callback is unregistered
///
/// # Panic Safety
///
/// This function catches any panics from the Rust closure using [`catch_unwind`] to prevent
/// unwinding across the FFI boundary, which would be undefined behavior. If the closure panics,
/// the panic is silently caught and the function returns normally.
///
/// # Error Handling
///
/// - If `user_data` is null, the function returns immediately without calling the closure
/// - If `line` is null, the function returns immediately without calling the closure
/// - If the C string contains invalid UTF-8, the function returns without calling the closure
///
/// [`EPANET::set_report_callback`]: crate::EPANET::set_report_callback
pub unsafe extern "C" fn report_callback_trampoline(
    user_data: *mut c_void,
    _project_handle: *mut c_void,
    line: *const c_char,
) {
    // Safety check: ensure pointers are valid
    if user_data.is_null() || line.is_null() {
        return;
    }

    // Catch panics to prevent unwinding across FFI boundary (undefined behavior)
    // AssertUnwindSafe is needed because raw pointers are not UnwindSafe
    let _ = catch_unwind(AssertUnwindSafe(|| {
        // Convert C string to Rust &str
        // SAFETY: We checked that `line` is not null above. The C API guarantees
        // that the string is null-terminated.
        let c_str = CStr::from_ptr(line);

        // Try to convert to UTF-8. If it fails, skip this line rather than panicking.
        let line_str = match c_str.to_str() {
            Ok(s) => s,
            Err(_) => return, // Invalid UTF-8, skip silently
        };

        // Cast user_data back to our boxed closure and call it
        // SAFETY: We trust that user_data was created via Box::into_raw in set_report_callback
        // and has not been freed yet. We borrow (not consume) the closure.
        let callback = &mut *(user_data as *mut ReportCallback);
        callback(line_str);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_report_callback_type_alias() {
        // Verify that ReportCallback can be created with a simple closure
        // Using Arc<Mutex<>> because ReportCallback requires Send + 'static
        let called = Arc::new(Mutex::new(false));
        let called_clone = Arc::clone(&called);

        let mut callback: ReportCallback = Box::new(move |_line: &str| {
            *called_clone.lock().unwrap() = true;
        });

        // Call the callback
        callback("test line");
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_report_callback_with_capture() {
        // Verify that ReportCallback can capture and mutate state
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        let mut callback: ReportCallback = Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        });

        callback("line 1");
        callback("line 2");
        callback("line 3");

        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 3);
        assert_eq!(captured[0], "line 1");
        assert_eq!(captured[1], "line 2");
        assert_eq!(captured[2], "line 3");
    }

    #[test]
    fn test_trampoline_null_user_data() {
        // Verify that trampoline handles null user_data safely
        let line = std::ffi::CString::new("test").unwrap();
        unsafe {
            // Should not panic or crash
            report_callback_trampoline(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                line.as_ptr(),
            );
        }
    }

    #[test]
    fn test_trampoline_null_line() {
        // Verify that trampoline handles null line safely
        let callback: ReportCallback = Box::new(|_line: &str| {
            panic!("Should not be called");
        });
        let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;

        unsafe {
            // Should not panic or crash
            report_callback_trampoline(user_data, std::ptr::null_mut(), std::ptr::null());

            // Clean up
            drop(Box::from_raw(user_data as *mut ReportCallback));
        }
    }

    #[test]
    fn test_trampoline_valid_call() {
        // Verify that trampoline correctly invokes the callback
        use std::sync::{Arc, Mutex};

        let received = Arc::new(Mutex::new(String::new()));
        let received_clone = Arc::clone(&received);

        let callback: ReportCallback = Box::new(move |line: &str| {
            *received_clone.lock().unwrap() = line.to_string();
        });
        let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;

        let line = std::ffi::CString::new("Hello from EPANET").unwrap();

        unsafe {
            report_callback_trampoline(user_data, std::ptr::null_mut(), line.as_ptr());

            // Clean up
            drop(Box::from_raw(user_data as *mut ReportCallback));
        }

        assert_eq!(*received.lock().unwrap(), "Hello from EPANET");
    }

    #[test]
    fn test_trampoline_panic_safety() {
        // Verify that trampoline catches panics from the callback
        let callback: ReportCallback = Box::new(|_line: &str| {
            panic!("This panic should be caught!");
        });
        let user_data = Box::into_raw(Box::new(callback)) as *mut c_void;

        let line = std::ffi::CString::new("trigger panic").unwrap();

        unsafe {
            // Should not panic - the panic is caught internally
            report_callback_trampoline(user_data, std::ptr::null_mut(), line.as_ptr());

            // Clean up
            drop(Box::from_raw(user_data as *mut ReportCallback));
        }
        // If we reach here, the test passes (panic was caught)
    }
}
