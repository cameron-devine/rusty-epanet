//! Report-related API methods for EPANET.
//!
//! This module contains methods for generating, customizing, and managing
//! simulation reports in EPANET.
//!
//! # Report Callback
//!
//! In addition to file-based reporting, EPANET supports registering a callback
//! function to receive report output programmatically. This is useful for:
//!
//! - Capturing report output in memory for testing or analysis
//! - Filtering or transforming report output before display
//! - Integrating EPANET output with logging frameworks
//! - Building GUIs that display report output in real-time
//!
//! See [`EPANET::set_report_callback`] for details and examples.

use crate::epanet_error::*;
use crate::ffi;
use crate::types::options::{AnalysisStatistic, Event, StatusReport, TimestepEvent};
use crate::types::report::{report_callback_trampoline, ReportCallback};
use crate::types::{ObjectType, MAX_MSG_SIZE};
use crate::EPANET;
use num_traits::FromPrimitive;
use std::ffi::{c_char, CStr, CString};
use std::os::raw::c_void;

/// ## Report APIs
impl EPANET {
    /// Clears the contents of a project's report file.
    ///
    /// This function clears any report data that has been generated during the
    /// simulation. It is useful for resetting the report state before running
    /// a new analysis or generating a new report.
    ///
    /// # Returns
    /// - `Ok(())` if the report is successfully cleared.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::new()?;
    /// ph.clear_report()?;
    /// ```
    ///
    /// # See Also
    /// - [`report`](Self::report) - Generate a report
    /// - [`reset_report`](Self::reset_report) - Reset report options
    /// - EN_clearreport (EPANET C API)
    pub fn clear_report(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_clearreport(self.ph) })
    }

    /// Copies the current report to a specified file.
    ///
    /// This function writes the generated report to a new file at the specified path.
    /// The file will contain the full contents of the report as produced by EPANET.
    ///
    /// # Parameters
    /// - `file_name`: The path to the file where the report should be saved.
    ///
    /// # Returns
    /// - `Ok(())` if the report is successfully copied.
    /// - `Err(EPANETError)` if the operation fails (e.g., invalid path, write permission denied).
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    /// ph.solve_h()?;
    /// ph.report()?;
    /// ph.copy_report("backup_report.rpt")?;
    /// ```
    ///
    /// # See Also
    /// - [`report`](Self::report) - Generate a report first
    /// - EN_copyreport (EPANET C API)
    pub fn copy_report(&self, file_name: &str) -> Result<()> {
        let c_file_name = CString::new(file_name).expect("file_name contains null bytes");
        check_error(unsafe { ffi::EN_copyreport(self.ph, c_file_name.as_ptr()) })
    }

    /// Retrieves the error message for a given error code.
    ///
    /// This function fetches a human-readable error message from EPANET for the
    /// provided error code. Useful for debugging and displaying error information
    /// to users.
    ///
    /// # Parameters
    /// - `error_code`: The EPANET error code (typically from a failed operation).
    ///
    /// # Returns
    /// - `Ok(String)` containing the error message text.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::new()?;
    /// // Error code 204 = "Undefined node"
    /// let message = ph.get_error(204)?;
    /// assert!(message.contains("node"));
    /// ```
    ///
    /// # See Also
    /// - EN_geterror (EPANET C API)
    pub fn get_error(&self, error_code: i32) -> Result<String> {
        let mut error_message = [0 as c_char; MAX_MSG_SIZE as usize + 1];
        check_error(unsafe {
            ffi::EN_geterror(error_code, error_message.as_mut_ptr(), MAX_MSG_SIZE as i32)
        })?;
        let s = unsafe { CStr::from_ptr(error_message.as_ptr()) };
        Ok(s.to_string_lossy().into_owned())
    }

    /// Retrieves the order in which a node or link appears in an output file.
    ///
    /// This function returns the position index in the output file for a specific
    /// object type and network index. Useful for advanced querying of simulation
    /// results stored in binary output files.
    ///
    /// # Parameters
    /// - `object_type`: The type of object ([`ObjectType::Node`] or [`ObjectType::Link`]).
    /// - `object_index`: The 1-based index of the object in the network.
    ///
    /// # Returns
    /// - `Ok(i32)` containing the position index in the output file.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::{EPANET, types::ObjectType};
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "", "")?;
    /// let node_index = ph.get_node_index("Junction1")?;
    /// let result_index = ph.get_result_index(ObjectType::Node, node_index)?;
    /// ```
    ///
    /// # See Also
    /// - EN_getresultindex (EPANET C API)
    pub fn get_result_index(&self, object_type: ObjectType, object_index: i32) -> Result<i32> {
        let mut index: i32 = -1;
        check_error(unsafe {
            ffi::EN_getresultindex(self.ph, object_type as i32, object_index, &mut index)
        })?;
        Ok(index)
    }

    /// Retrieves a hydraulic simulation statistic.
    ///
    /// This function fetches various statistics from the most recent hydraulic
    /// analysis, such as the number of iterations, relative error, or mass balance.
    ///
    /// # Parameters
    /// - `stat_type`: The type of statistic to retrieve (see [`AnalysisStatistic`]).
    ///
    /// # Returns
    /// - `Ok(f64)` containing the statistic value.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Available Statistics
    /// - [`AnalysisStatistic::Iterations`] - Number of hydraulic iterations taken
    /// - [`AnalysisStatistic::RelativeError`] - Sum of link flow changes / sum of link flows
    /// - [`AnalysisStatistic::MaxHeadError`] - Largest head loss error for links
    /// - [`AnalysisStatistic::MaxFlowChange`] - Largest flow change in links
    /// - [`AnalysisStatistic::MassBalance`] - Cumulative water quality mass balance ratio
    /// - [`AnalysisStatistic::DeficientNodes`] - Number of pressure deficient nodes
    /// - [`AnalysisStatistic::DemandReduction`] - % demand reduction at pressure deficient nodes
    /// - [`AnalysisStatistic::LeakageLoss`] - % flow lost to system leakage
    ///
    /// # Example
    /// ```ignore
    /// use epanet::{EPANET, types::options::AnalysisStatistic};
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "", "")?;
    /// ph.solve_h()?;
    /// let iterations = ph.get_statistic(AnalysisStatistic::Iterations)?;
    /// let rel_error = ph.get_statistic(AnalysisStatistic::RelativeError)?;
    /// println!("Converged in {} iterations with error {:.6}", iterations, rel_error);
    /// ```
    ///
    /// # See Also
    /// - [`solve_h`](Self::solve_h) - Run hydraulic simulation first
    /// - EN_getstatistic (EPANET C API)
    pub fn get_statistic(&self, stat_type: AnalysisStatistic) -> Result<f64> {
        let mut value: f64 = 0.0;
        check_error(unsafe { ffi::EN_getstatistic(self.ph, stat_type as i32, &mut value) })?;
        Ok(value)
    }

    /// Retrieves the version number of the EPANET Toolkit.
    ///
    /// The version number is encoded as `major*10000 + minor*100 + patch`.
    /// For example, version 2.3.5 would be returned as 20305.
    ///
    /// # Returns
    /// - `Ok(i32)` containing the version number.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::new()?;
    /// let version = ph.get_version()?;
    /// let major = version / 10000;
    /// let minor = (version % 10000) / 100;
    /// let patch = version % 100;
    /// println!("EPANET version: {}.{}.{}", major, minor, patch);
    /// ```
    ///
    /// # See Also
    /// - EN_getversion (EPANET C API)
    pub fn get_version(&self) -> Result<i32> {
        let mut out_version = 0;
        check_error(unsafe { ffi::EN_getversion(&mut out_version) })?;
        Ok(out_version)
    }

    /// Writes simulation results to the report file.
    ///
    /// This function generates a formatted report of simulation results based on
    /// the current report settings. Both hydraulic and water quality analyses
    /// must be run and saved before calling this function.
    ///
    /// # Returns
    /// - `Ok(())` if the report is successfully generated.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Prerequisites
    /// - Hydraulic analysis must be completed (via [`solve_h`](Self::solve_h) or step-by-step)
    /// - Results must be saved (via [`save_h`](Self::save_h) if using step-by-step)
    /// - Water quality analysis should also be completed if quality reporting is desired
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    /// ph.solve_h()?;
    /// ph.solve_q()?;
    /// ph.report()?;
    /// // Results are now written to report.rpt
    /// ```
    ///
    /// # See Also
    /// - [`set_report`](Self::set_report) - Customize report format
    /// - [`copy_report`](Self::copy_report) - Copy report to another file
    /// - EN_report (EPANET C API)
    pub fn report(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_report(self.ph) })
    }

    /// Resets a project's report options to their default values.
    ///
    /// This function clears any report formatting commands that were issued via
    /// [`set_report`](Self::set_report), restoring default report behavior.
    ///
    /// # Returns
    /// - `Ok(())` if the report options are successfully reset.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    /// // Customize report
    /// ph.set_report("NODES ALL")?;
    /// ph.set_report("LINKS ALL")?;
    ///
    /// // Reset to defaults
    /// ph.reset_report()?;
    /// ```
    ///
    /// # See Also
    /// - [`set_report`](Self::set_report) - Set report options
    /// - [`clear_report`](Self::clear_report) - Clear report contents
    /// - EN_resetreport (EPANET C API)
    pub fn reset_report(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_resetreport(self.ph) })
    }

    /// Processes a reporting format command.
    ///
    /// This function allows customization of the report format by passing command
    /// strings similar to those used in the `[REPORT]` section of an EPANET input file.
    ///
    /// # Parameters
    /// - `format`: A report format command string.
    ///
    /// # Returns
    /// - `Ok(())` if the format command is successfully processed.
    /// - `Err(EPANETError)` if the command is invalid or fails.
    ///
    /// # Format Commands
    /// Common report format commands include:
    /// - `"NODES ALL"` or `"NODES NONE"` - Report all/no nodes
    /// - `"LINKS ALL"` or `"LINKS NONE"` - Report all/no links
    /// - `"NODES nodeID"` - Report specific node
    /// - `"LINKS linkID"` - Report specific link
    /// - `"ELEVATION YES"` - Include elevation in report
    /// - `"DEMAND YES"` - Include demand in report
    /// - `"HEAD YES"` - Include head in report
    /// - `"PRESSURE YES"` - Include pressure in report
    /// - `"QUALITY YES"` - Include quality in report
    /// - `"VELOCITY YES"` - Include velocity in report
    /// - `"HEADLOSS YES"` - Include headloss in report
    /// - `"STATUS YES"` - Include status in report
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    ///
    /// // Configure report to show all nodes with pressure and demand
    /// ph.set_report("NODES ALL")?;
    /// ph.set_report("PRESSURE YES")?;
    /// ph.set_report("DEMAND YES")?;
    ///
    /// // Show only specific links
    /// ph.set_report("LINKS NONE")?;
    /// ph.set_report("LINKS Pipe1")?;
    /// ph.set_report("LINKS Pipe2")?;
    ///
    /// ph.solve_h()?;
    /// ph.report()?;
    /// ```
    ///
    /// # See Also
    /// - [`reset_report`](Self::reset_report) - Reset to default report options
    /// - [`report`](Self::report) - Generate the report
    /// - EN_setreport (EPANET C API)
    pub fn set_report(&self, format: &str) -> Result<()> {
        let c_format = CString::new(format).expect("format contains null bytes");
        check_error(unsafe { ffi::EN_setreport(self.ph, c_format.as_ptr()) })
    }

    /// Sets the level of hydraulic status reporting.
    ///
    /// This function controls the amount of hydraulic status information written
    /// to the report file during a simulation.
    ///
    /// # Parameters
    /// - `level`: The status reporting level (see [`StatusReport`]).
    ///
    /// # Returns
    /// - `Ok(())` if the status level is successfully set.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Reporting Levels
    /// - [`StatusReport::NoReport`] - No status reporting
    /// - [`StatusReport::NormalReport`] - Normal reporting (changes in status)
    /// - [`StatusReport::FullReport`] - Full reporting (all status checks)
    ///
    /// # Note
    /// Full status reporting can significantly increase report file size and
    /// simulation time. It is recommended to disable status reporting
    /// (`StatusReport::NoReport`) for production simulations.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::{EPANET, types::options::StatusReport};
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    ///
    /// // Enable full status reporting for debugging
    /// ph.set_status_report(StatusReport::FullReport)?;
    /// ph.solve_h()?;
    ///
    /// // Disable for production
    /// ph.set_status_report(StatusReport::NoReport)?;
    /// ```
    ///
    /// # See Also
    /// - EN_setstatusreport (EPANET C API)
    pub fn set_status_report(&self, level: StatusReport) -> Result<()> {
        check_error(unsafe { ffi::EN_setstatusreport(self.ph, level as i32) })
    }

    /// Retrieves the time until the next event occurs during a simulation.
    ///
    /// This function queries EPANET for the type, duration, and element index
    /// of the next scheduled event in the simulation timeline. Useful for
    /// event-driven analysis and precise time-step control.
    ///
    /// # Returns
    /// - `Ok(Event)` containing:
    ///   - `event_type`: The type of next event (see [`TimestepEvent`])
    ///   - `duration`: Time in seconds until the event occurs
    ///   - `element_index`: Index of the element involved (if applicable)
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Event Types
    /// - [`TimestepEvent::StepReport`] - A reporting time step
    /// - [`TimestepEvent::StepHyd`] - A hydraulic time step
    /// - [`TimestepEvent::StepWq`] - A water quality time step
    /// - [`TimestepEvent::StepTankEvent`] - A tank becomes empty/full
    /// - [`TimestepEvent::StepControlEvent`] - A control action triggers
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "", "")?;
    /// ph.open_h()?;
    /// ph.init_h(epanet::types::analysis::InitHydOption::NoSave)?;
    ///
    /// loop {
    ///     let event = ph.time_to_next_event()?;
    ///     println!("Next event: {:?} in {} seconds", event.event_type, event.duration);
    ///
    ///     let t = ph.run_h()?;
    ///     let tstep = ph.next_h()?;
    ///     if tstep == 0 { break; }
    /// }
    /// ph.close_h()?;
    /// ```
    ///
    /// # See Also
    /// - EN_timetonextevent (EPANET C API)
    pub fn time_to_next_event(&self) -> Result<Event> {
        let mut event_type: i32 = 0;
        let mut duration: std::os::raw::c_long = 0;
        let mut element_index: i32 = 0;
        check_error(unsafe {
            ffi::EN_timetonextevent(self.ph, &mut event_type, &mut duration, &mut element_index)
        })?;
        Ok(Event {
            event_type: TimestepEvent::from_i32(event_type).unwrap(),
            duration: duration as u64,
            element_index,
        })
    }

    /// Writes a line of text to a project's report file.
    ///
    /// This function appends a line of text to the report generated by EPANET.
    /// Useful for adding custom notes, annotations, or section headers to the
    /// report output.
    ///
    /// # Parameters
    /// - `line`: The line of text to write to the report.
    ///
    /// # Returns
    /// - `Ok(())` if the line is successfully written.
    /// - `Err(EPANETError)` if the operation fails.
    ///
    /// # Example
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let ph = EPANET::with_inp_file("network.inp", "report.rpt", "")?;
    /// ph.solve_h()?;
    ///
    /// ph.write_line_to_report("")?;
    /// ph.write_line_to_report("==========================================")?;
    /// ph.write_line_to_report("  CUSTOM ANALYSIS NOTES")?;
    /// ph.write_line_to_report("==========================================")?;
    /// ph.write_line_to_report("")?;
    /// ph.write_line_to_report("Analysis performed on: 2024-01-15")?;
    /// ph.write_line_to_report("Scenario: Peak demand conditions")?;
    ///
    /// ph.report()?;
    /// ```
    ///
    /// # See Also
    /// - [`report`](Self::report) - Generate the report
    /// - [`set_report_callback`](Self::set_report_callback) - Intercept report output
    /// - EN_writeline (EPANET C API)
    pub fn write_line_to_report(&self, line: &str) -> Result<()> {
        let c_line = CString::new(line).expect("line contains null bytes");
        check_error(unsafe { ffi::EN_writeline(self.ph, c_line.as_ptr()) })
    }

    /// Sets a callback closure to receive report output instead of writing to a file.
    ///
    /// When a callback is registered, all output that would normally be written to the
    /// report file (via [`write_line_to_report`](Self::write_line_to_report) or internal
    /// EPANET operations) is instead passed to the callback closure. This includes output
    /// from [`report`](Self::report) and other report-generating functions.
    ///
    /// # Parameters
    ///
    /// - `callback`: A boxed closure that receives each line of report output as `&str`,
    ///   or `None` to unregister any existing callback and revert to file-based reporting.
    ///
    /// The closure must implement:
    /// - `FnMut(&str)`: Can be called multiple times and may mutate captured state
    /// - `Send`: Safe to send across thread boundaries
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the callback is successfully registered or unregistered.
    /// - `Err(EPANETError)` if the underlying EPANET C API call fails.
    ///
    /// # Lifetime Management
    ///
    /// The callback is automatically freed when:
    /// - A new callback is registered (replaces the previous one)
    /// - `None` is passed to unregister the callback
    /// - The `EPANET` instance is dropped
    ///
    /// # Thread Safety
    ///
    /// The callback may be invoked from the same thread that calls EPANET functions.
    /// If you need to share state with other threads, use synchronization primitives
    /// like [`Arc<Mutex<T>>`](std::sync::Mutex) or channels.
    ///
    /// # Panic Safety
    ///
    /// If the callback panics, the panic is caught internally to prevent undefined
    /// behavior from unwinding across the FFI boundary. The panic is silently discarded
    /// and the report line is skipped. Avoid panicking in callbacks if possible.
    ///
    /// # Example: Simple Logging
    ///
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let mut epanet = EPANET::with_inp_file("network.inp", "", "")?;
    ///
    /// // Log all report output to stdout
    /// epanet.set_report_callback(Some(Box::new(|line: &str| {
    ///     println!("[EPANET] {}", line);
    /// })))?;
    ///
    /// epanet.solve_h()?;
    /// epanet.write_line_to_report("Custom message")?;
    /// // Output: [EPANET] Custom message
    ///
    /// // Unregister callback
    /// epanet.set_report_callback(None)?;
    /// ```
    ///
    /// # Example: Collecting Lines in a Vector
    ///
    /// ```ignore
    /// use std::sync::{Arc, Mutex};
    /// use epanet::EPANET;
    ///
    /// let mut epanet = EPANET::with_inp_file("network.inp", "", "")?;
    ///
    /// // Collect lines in a thread-safe vector
    /// let lines = Arc::new(Mutex::new(Vec::new()));
    /// let lines_clone = Arc::clone(&lines);
    ///
    /// epanet.set_report_callback(Some(Box::new(move |line: &str| {
    ///     lines_clone.lock().unwrap().push(line.to_string());
    /// })))?;
    ///
    /// epanet.solve_h()?;
    /// epanet.write_line_to_report("Test line 1")?;
    /// epanet.write_line_to_report("Test line 2")?;
    ///
    /// // Check captured lines
    /// let captured = lines.lock().unwrap();
    /// assert_eq!(captured.len(), 2);
    /// assert_eq!(captured[0], "Test line 1");
    /// assert_eq!(captured[1], "Test line 2");
    /// ```
    ///
    /// # Example: Filtering Output
    ///
    /// ```ignore
    /// use epanet::EPANET;
    ///
    /// let mut epanet = EPANET::with_inp_file("network.inp", "", "")?;
    ///
    /// // Only log warnings and errors
    /// epanet.set_report_callback(Some(Box::new(|line: &str| {
    ///     if line.contains("WARNING") || line.contains("ERROR") {
    ///         eprintln!("{}", line);
    ///     }
    /// })))?;
    ///
    /// epanet.solve_h()?;
    /// epanet.report()?;
    /// ```
    ///
    /// # See Also
    ///
    /// - [`write_line_to_report`](Self::write_line_to_report) - Write custom lines
    /// - [`report`](Self::report) - Generate the full report
    /// - [`ReportCallback`](crate::types::report::ReportCallback) - The callback type alias
    /// - EN_setreportcallback (EPANET C API)
    /// - EN_setreportcallbackuserdata (EPANET C API)
    pub fn set_report_callback(&mut self, callback: Option<ReportCallback>) -> Result<()> {
        // Free any existing callback first to prevent memory leaks.
        // SAFETY: If report_callback_ptr is Some, it was created via Box::into_raw
        // in a previous call to this method and has not been freed yet.
        if let Some(old_ptr) = self.report_callback_ptr.take() {
            unsafe {
                drop(Box::from_raw(old_ptr as *mut ReportCallback));
            }
        }

        match callback {
            Some(cb) => {
                // Box the closure and convert to raw pointer for C API
                let boxed: Box<ReportCallback> = Box::new(cb);
                let raw_ptr = Box::into_raw(boxed) as *mut c_void;

                // Register user data first, then the trampoline function.
                // Order matters: user data must be set before the callback is invoked.
                check_error(unsafe { ffi::EN_setreportcallbackuserdata(self.ph, raw_ptr) })?;

                check_error(unsafe {
                    ffi::EN_setreportcallback(self.ph, Some(report_callback_trampoline))
                })?;

                // Store the raw pointer so we can free it later (in Drop or next call)
                self.report_callback_ptr = Some(raw_ptr);
            }
            None => {
                // Unregister: set callback to NULL first, then clear user data
                check_error(unsafe { ffi::EN_setreportcallback(self.ph, None) })?;

                check_error(unsafe {
                    ffi::EN_setreportcallbackuserdata(self.ph, std::ptr::null_mut())
                })?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use crate::types::options::StatusReport;
    use rstest::*;
    use std::sync::{Arc, Mutex};

    #[rstest]
    fn test_get_version(ph: EPANET) {
        let version = ph.get_version().unwrap();
        // Version should be at least 2.0.0 (20000)
        assert!(version >= 20000, "Version {} should be >= 20000", version);
        // Version encoding: major*10000 + minor*100 + patch
        let major = version / 10000;
        let minor = (version % 10000) / 100;
        let patch = version % 100;
        assert!(major >= 2, "Major version should be >= 2");
        println!("EPANET version: {}.{}.{}", major, minor, patch);
    }

    #[rstest]
    fn test_get_error_message(ph: EPANET) {
        // Test known error codes
        let msg_204 = ph.get_error(204).unwrap();
        assert!(!msg_204.is_empty(), "Error 204 message should not be empty");

        let msg_205 = ph.get_error(205).unwrap();
        assert!(!msg_205.is_empty(), "Error 205 message should not be empty");

        // Error code 0 - just verify the function works (EPANET may return empty string)
        let result = ph.get_error(0);
        assert!(result.is_ok(), "get_error(0) should succeed");
    }

    #[rstest]
    fn test_get_statistic_after_solve(ph: EPANET) {
        // Run hydraulic simulation first
        ph.solve_h().unwrap();

        // Get statistics
        let iterations = ph.get_statistic(AnalysisStatistic::Iterations).unwrap();
        assert!(iterations >= 0.0, "Iterations should be >= 0");

        let rel_error = ph.get_statistic(AnalysisStatistic::RelativeError).unwrap();
        assert!(rel_error >= 0.0, "Relative error should be >= 0");

        let max_head_error = ph.get_statistic(AnalysisStatistic::MaxHeadError).unwrap();
        assert!(max_head_error >= 0.0, "Max head error should be >= 0");
    }

    #[rstest]
    fn test_set_status_report(ph: EPANET) {
        // Test all status report levels
        ph.set_status_report(StatusReport::NoReport).unwrap();
        ph.set_status_report(StatusReport::NormalReport).unwrap();
        ph.set_status_report(StatusReport::FullReport).unwrap();

        // Set back to no report for cleaner output
        ph.set_status_report(StatusReport::NoReport).unwrap();
    }

    #[rstest]
    fn test_report_workflow(ph: EPANET) {
        // Run simulation
        ph.solve_h().unwrap();
        ph.solve_q().unwrap();

        // Configure report
        ph.set_report("NODES ALL").unwrap();
        ph.set_report("LINKS ALL").unwrap();

        // Write custom line
        ph.write_line_to_report("").unwrap();
        ph.write_line_to_report("Test Report Line").unwrap();

        // Generate report
        ph.report().unwrap();

        // Reset report options (note: clear_report requires a report file to be open)
        ph.reset_report().unwrap();
    }

    #[rstest]
    fn test_set_report_commands(ph: EPANET) {
        // Test various report format commands
        ph.set_report("NODES NONE").unwrap();
        ph.set_report("LINKS NONE").unwrap();
        ph.set_report("NODES ALL").unwrap();
        ph.set_report("LINKS ALL").unwrap();

        // Reset to defaults
        ph.reset_report().unwrap();
    }

    #[rstest]
    fn test_get_result_index(ph: EPANET) {
        // Get node index
        let node_index = ph.get_node_index("10").unwrap();
        let result_index = ph.get_result_index(ObjectType::Node, node_index);
        assert!(result_index.is_ok());

        // Get link index
        let link_index = ph.get_link_index("10").unwrap();
        let result_index = ph.get_result_index(ObjectType::Link, link_index);
        assert!(result_index.is_ok());
    }

    #[rstest]
    fn test_time_to_next_event(ph: EPANET) {
        use crate::types::analysis::InitHydOption;

        // Open and initialize hydraulics
        ph.open_h().unwrap();
        ph.init_h(InitHydOption::NoSave).unwrap();

        // Get the next event
        let event = ph.time_to_next_event().unwrap();

        // Event should have a valid type and reasonable duration
        assert!(event.duration <= 86400 * 7, "Duration should be <= 1 week in seconds");

        // Run a step to verify simulation is working
        let _t = ph.run_h().unwrap();

        ph.close_h().unwrap();
    }

    // ========================================================================
    // Report Callback Tests
    // ========================================================================

    #[rstest]
    fn test_set_report_callback_basic(mut ph: EPANET) {
        // Test that we can set a callback without errors
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        // Write a line - should be captured by callback
        ph.write_line_to_report("Test callback line").unwrap();

        // Verify the line was captured
        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0], "Test callback line");
    }

    #[rstest]
    fn test_set_report_callback_multiple_lines(mut ph: EPANET) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        // Write multiple lines
        ph.write_line_to_report("Line 1").unwrap();
        ph.write_line_to_report("Line 2").unwrap();
        ph.write_line_to_report("Line 3").unwrap();

        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 3);
        assert_eq!(captured[0], "Line 1");
        assert_eq!(captured[1], "Line 2");
        assert_eq!(captured[2], "Line 3");
    }

    #[rstest]
    fn test_set_report_callback_unregister(mut ph: EPANET) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        // Register callback
        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        ph.write_line_to_report("Before unregister").unwrap();

        // Unregister callback
        ph.set_report_callback(None).unwrap();

        // This line should NOT be captured (goes to file or is discarded)
        ph.write_line_to_report("After unregister").unwrap();

        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0], "Before unregister");
    }

    #[rstest]
    fn test_set_report_callback_replace(mut ph: EPANET) {
        let lines1 = Arc::new(Mutex::new(Vec::new()));
        let lines1_clone = Arc::clone(&lines1);

        let lines2 = Arc::new(Mutex::new(Vec::new()));
        let lines2_clone = Arc::clone(&lines2);

        // Register first callback
        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines1_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        ph.write_line_to_report("To callback 1").unwrap();

        // Replace with second callback
        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines2_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        ph.write_line_to_report("To callback 2").unwrap();

        // Verify each callback received only its lines
        let captured1 = lines1.lock().unwrap();
        let captured2 = lines2.lock().unwrap();

        assert_eq!(captured1.len(), 1);
        assert_eq!(captured1[0], "To callback 1");

        assert_eq!(captured2.len(), 1);
        assert_eq!(captured2[0], "To callback 2");
    }

    #[rstest]
    fn test_set_report_callback_empty_line(mut ph: EPANET) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        // Write empty lines (common for formatting)
        ph.write_line_to_report("").unwrap();
        ph.write_line_to_report("Non-empty").unwrap();
        ph.write_line_to_report("").unwrap();

        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 3);
        assert_eq!(captured[0], "");
        assert_eq!(captured[1], "Non-empty");
        assert_eq!(captured[2], "");
    }

    #[rstest]
    fn test_set_report_callback_special_characters(mut ph: EPANET) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        // Test various special characters
        ph.write_line_to_report("Tab:\tcharacter").unwrap();
        ph.write_line_to_report("Pipe: |").unwrap();
        ph.write_line_to_report("Unicode: café").unwrap();
        ph.write_line_to_report("Numbers: 123.456").unwrap();

        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 4);
        assert!(captured[0].contains('\t'));
        assert!(captured[1].contains('|'));
        assert!(captured[2].contains("café"));
        assert!(captured[3].contains("123.456"));
    }

    #[rstest]
    fn test_set_report_callback_with_solve(mut ph: EPANET) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let lines_clone = Arc::clone(&lines);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            lines_clone.lock().unwrap().push(line.to_string());
        })))
        .unwrap();

        // Enable status reporting so we get output during solve
        ph.set_status_report(StatusReport::NormalReport).unwrap();

        // Run simulation - this may generate report output
        ph.solve_h().unwrap();

        // Write a custom line
        ph.write_line_to_report("Custom line after solve").unwrap();

        let captured = lines.lock().unwrap();
        // At minimum we should have our custom line
        assert!(
            captured.iter().any(|l| l.contains("Custom line after solve")),
            "Custom line should be captured"
        );
    }

    #[rstest]
    fn test_set_report_callback_filtering(mut ph: EPANET) {
        // Test that callbacks can filter lines
        let errors_only = Arc::new(Mutex::new(Vec::new()));
        let errors_clone = Arc::clone(&errors_only);

        ph.set_report_callback(Some(Box::new(move |line: &str| {
            // Only capture lines containing "ERROR" or "WARNING"
            if line.contains("ERROR") || line.contains("WARNING") {
                errors_clone.lock().unwrap().push(line.to_string());
            }
        })))
        .unwrap();

        // Write various lines
        ph.write_line_to_report("Normal line").unwrap();
        ph.write_line_to_report("WARNING: Something happened")
            .unwrap();
        ph.write_line_to_report("Another normal line").unwrap();
        ph.write_line_to_report("ERROR: Bad thing occurred").unwrap();
        ph.write_line_to_report("All good").unwrap();

        let captured = errors_only.lock().unwrap();
        assert_eq!(captured.len(), 2);
        assert!(captured[0].contains("WARNING"));
        assert!(captured[1].contains("ERROR"));
    }

    #[rstest]
    fn test_set_report_callback_counter(mut ph: EPANET) {
        // Test that callback can maintain state via mutation
        let counter = Arc::new(Mutex::new(0usize));
        let counter_clone = Arc::clone(&counter);

        ph.set_report_callback(Some(Box::new(move |_line: &str| {
            *counter_clone.lock().unwrap() += 1;
        })))
        .unwrap();

        // Write 5 lines
        for i in 0..5 {
            ph.write_line_to_report(&format!("Line {}", i)).unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 5);
    }

    #[rstest]
    fn test_report_callback_dropped_with_epanet() {
        // Test that the callback is properly freed when EPANET is dropped
        // This test primarily checks that we don't leak memory or crash

        let lines = Arc::new(Mutex::new(Vec::new()));

        {
            let mut ph =
                EPANET::with_inp_file("src/impls/test_utils/net1.inp", "", "").unwrap();
            let lines_clone = Arc::clone(&lines);

            ph.set_report_callback(Some(Box::new(move |line: &str| {
                lines_clone.lock().unwrap().push(line.to_string());
            })))
            .unwrap();

            ph.write_line_to_report("Before drop").unwrap();

            // ph is dropped here, callback should be freed
        }

        // Verify the line was captured before drop
        let captured = lines.lock().unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(captured[0], "Before drop");
    }
}
