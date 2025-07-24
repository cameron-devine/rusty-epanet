//! Hydraulic Analysis-related API methods for EPANET.
//!
//! This module contains methods for opening, initializing, running, stepping, saving, and closing hydraulic analyses.

use crate::bindings as ffi;
use crate::epanet_error::*;
use crate::types::InitHydOption;
use crate::EPANET;
use std::mem::MaybeUninit;

/// ## Hydraulic Analysis APIs
impl EPANET {
    /// Opens the hydraulic solver for the EPANET project.
    ///
    /// This function prepares the hydraulic solver for analysis by calling the EPANET C API function `EN_openH`.
    /// It should be called before running hydraulic analyses using the `init_h`, `run_h`, and `next_h` sequence.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the solver was opened successfully.
    /// - `Err(EPANETError)` if an error occurred during opening.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_openH` with the project handle.
    ///
    /// # Safety
    /// This function uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid and initialized.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to open the hydraulic solver.
    ///
    /// # See Also
    /// - EN_openH (EPANET C API)
    pub fn open_h(&self) -> Result<()> {
        unsafe {
            match ffi::EN_openH(self.ph) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Initializes the network prior to running a hydraulic analysis.
    ///
    /// This function initializes tank levels, link status/settings, and the simulation clock.
    /// The `init_flag` controls whether flows are re-initialized and whether hydraulic results are saved to a temporary file.
    ///
    /// # Parameters
    /// - `init_flag`: The [`InitHydOption`] specifying initialization behavior.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if initialization succeeded.
    /// - `Err(EPANETError)` if an error occurred during initialization.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_initH` with the project handle and initialization flag.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    /// - The `init_flag` is a valid value.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if initialization fails.
    ///
    /// # See Also
    /// - EN_initH (EPANET C API)
    /// - [`InitHydOption`] for initialization options.
    pub fn init_h(&self, init_flag: InitHydOption) -> Result<()> {
        unsafe {
            match ffi::EN_initH(self.ph, init_flag as i32) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Uses a previously saved binary hydraulics file to supply the project's hydraulics.
    ///
    /// This function loads hydraulic results from a file, allowing reuse for water quality analyses or saving computation time.
    ///
    /// # Parameters
    /// - `file_name`: Path to the hydraulics file to load.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the file was loaded successfully.
    /// - `Err(EPANETError)` if an error occurred during loading.
    ///
    /// # Implementation Details
    /// - Converts the file name to a C-compatible string.
    /// - Calls the EPANET C API function `EN_usehydfile`.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    /// - The file name is a valid path.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the file cannot be loaded.
    ///
    /// # See Also
    /// - EN_usehydfile (EPANET C API)
    pub fn use_hydraulics_file(&self, file_name: &str) -> Result<()> {
        use std::ffi::CString;

        let c_file_name = CString::new(file_name).expect("file_name contains null bytes");
        unsafe {
            match ffi::EN_usehydfile(self.ph, c_file_name.as_ptr()) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Runs a complete hydraulic analysis for the EPANET project.
    ///
    /// This function generates a complete hydraulic solution for the project.
    /// Results can be used for reporting or as input to water quality analysis.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the analysis succeeded.
    /// - `Err(EPANETError)` if an error occurred during analysis.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_solveH` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the analysis fails.
    ///
    /// # See Also
    /// - EN_solveH (EPANET C API)
    pub fn solve_h(&self) -> Result<()> {
        unsafe {
            match ffi::EN_solveH(self.ph) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Computes a hydraulic solution for the current point in time.
    ///
    /// This function is used in a loop with `next_h` to run extended period hydraulic simulations.
    /// Returns the current simulation time in seconds.
    ///
    /// # Returns
    /// A [`Result<u64>`] which:
    /// - `Ok(u64)` contains the current simulation time in seconds.
    /// - `Err(EPANETError)` if an error occurred during computation.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_runH` and retrieves the current simulation time.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the computation fails.
    ///
    /// # See Also
    /// - EN_runH (EPANET C API)
    pub fn run_h(&self) -> Result<u64> {
        let mut out_current_time = MaybeUninit::uninit();
        unsafe {
            match ffi::EN_runH(self.ph, out_current_time.as_mut_ptr()) {
                0 => Ok(out_current_time.assume_init() as u64),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Advances the simulation to the next hydraulic event.
    ///
    /// This function is used in a loop with `run_h` to run extended period hydraulic simulations.
    /// Returns the time until the next event in seconds.
    ///
    /// # Returns
    /// A [`Result<u64>`] which:
    /// - `Ok(u64)` contains the time until the next event in seconds.
    /// - `Err(EPANETError)` if an error occurred during computation.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_nextH` and retrieves the next event time.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the computation fails.
    ///
    /// # See Also
    /// - EN_nextH (EPANET C API)
    pub fn next_h(&self) -> Result<u64> {
        let mut out_next_time = MaybeUninit::uninit();
        unsafe {
            match ffi::EN_nextH(self.ph, out_next_time.as_mut_ptr()) {
                0 => Ok(out_next_time.assume_init() as u64),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Transfers hydraulic results from the temporary hydraulics file to the binary output file.
    ///
    /// This function is used when only a hydraulic analysis is run and results at uniform reporting intervals
    /// need to be transferred to the output file.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the results were saved successfully.
    /// - `Err(EPANETError)` if an error occurred during saving.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_saveH` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if saving fails.
    ///
    /// # See Also
    /// - EN_saveH (EPANET C API)
    pub fn save_h(&self) -> Result<()> {
        unsafe {
            match ffi::EN_saveH(self.ph) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Saves the current hydraulics results to a binary file.
    ///
    /// This function saves the current set of hydraulics results to a file, either for post-processing
    /// or to be used at a later time by calling the `EN_usehydfile` function.
    /// The hydraulics file contains nodal demands and heads and link flows, status, and settings for all hydraulic time steps.
    ///
    /// # Parameters
    /// - `file_name`: The path to the file where hydraulics results will be saved.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the file was saved successfully.
    /// - `Err(EPANETError)` if an error occurred during saving.
    ///
    /// # Implementation Details
    /// - Converts the file name to a C-compatible string.
    /// - Calls the EPANET C API function `EN_savehydfile`.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    /// - The file name is a valid path.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if saving fails.
    ///
    /// # See Also
    /// - EN_savehydfile (EPANET C API)
    pub fn save_hydraulics_file(&self, file_name: &str) -> Result<()> {
        use std::ffi::CString;

        // todo: Should this be a std::path::PathBuf?
        let c_file_name = CString::new(file_name).expect("file_name contains null bytes");
        unsafe {
            match ffi::EN_savehydfile(self.ph, c_file_name.as_ptr()) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }

    /// Closes the hydraulic solver and frees all allocated memory.
    ///
    /// This function should be called after all hydraulics analyses have been made using the
    /// `EN_initH` - `EN_runH` - `EN_nextH` sequence of function calls.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the solver was closed successfully.
    /// - `Err(EPANETError)` if an error occurred during closing.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_closeH` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if closing fails.
    ///
    /// # See Also
    /// - EN_closeH (EPANET C API)
    pub fn close_h(&self) -> Result<()> {
        unsafe {
            match ffi::EN_closeH(self.ph) {
                0 => Ok(()),
                x => Err(EPANETError::from(x)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use rstest::rstest;
    use std::fs;

    #[rstest]
    fn test_solve_h(ph: EPANET) {
        let result = ph.solve_h();
        assert_eq!(result, Ok(()));
    }

    #[rstest]
    fn test_hyd_step(ph: EPANET) {
        let result = ph.open_h();
        assert_eq!(result, Ok(()));

        let init_result = ph.init_h(InitHydOption::NoSave);
        assert_eq!(init_result, Ok(()));

        loop {
            let run_result = ph.run_h();
            assert!(matches!(run_result, Ok(_)));

            let step_result = ph.next_h();
            assert!(matches!(step_result, Ok(_)));

            if step_result.unwrap() <= 0 {
                break;
            }
        }

        let close_result = ph.close_h();
        assert_eq!(close_result, Ok(()));
    }

    #[rstest]
    fn test_hydraulics_save(ph: EPANET) {
        let mut result = ph.solve_h();
        assert_eq!(result, Ok(()));

        result = ph.save_h();
        assert_eq!(result, Ok(()));

        let result = ph.report();
        assert_eq!(result, Ok(()));
    }

    #[rstest]
    fn test_hydraulics_save_file(ph: EPANET) {
        let mut result = ph.solve_h();
        assert_eq!(result, Ok(()));

        let hyd_file = std::path::Path::new("test_savefile.hyd");
        let _result = ph.save_hydraulics_file(hyd_file.to_str().unwrap());
        assert!(
            std::path::Path::new("test_savefile.hyd").exists(),
            "Hydraulics file was not created successfully"
        );

        result = ph.use_hydraulics_file(hyd_file.to_str().unwrap());
        assert_eq!(result, Ok(()));

        result = ph.solve_q();
        assert_eq!(result, Ok(()));

        // Clean up the created file after the test
        fs::remove_file(hyd_file).expect("Failed to remove the hydraulics file");
    }
}
