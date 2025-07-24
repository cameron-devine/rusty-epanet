//! Water Quality Analysis related API methods for EPANET.
//!
//! This module contains methods for initializing, running, stepping, and closing water quality simulations.

use crate::epanet_error::*;
use crate::ffi;
use crate::types::InitHydOption;
use crate::EPANET;
use std::mem::MaybeUninit;

/// ## Water Quality Analysis APIs
impl EPANET {
    /// Closes the quality simulation.
    ///
    /// This function calls the EPANET library to close the water quality simulation.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the simulation is successfully closed.
    /// - `Err(EPANETError)` if an error occurs during closure.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_closeQ` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid and initialized.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if the EPANET library fails to close the quality simulation.
    ///
    /// # See Also
    /// - EN_closeQ (EPANET C API)
    pub fn close_q(&self) -> Result<()> {
        let result = unsafe { ffi::EN_closeQ(self.ph) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Initializes the quality simulation.
    ///
    /// This function initializes the water quality simulation in EPANET.
    ///
    /// # Parameters
    /// - `save_flag`: The [`InitHydOption`] specifying whether to save results.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the simulation is successfully initialized.
    /// - `Err(EPANETError)` if an error occurs during initialization.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_initQ` with the project handle and save flag.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    /// - The `save_flag` is a valid value.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if initialization fails.
    ///
    /// # See Also
    /// - EN_initQ (EPANET C API)
    pub fn init_q(&self, save_flag: InitHydOption) -> Result<()> {
        let result = unsafe { ffi::EN_initQ(self.ph, save_flag as i32) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Advances to the next quality time step.
    ///
    /// This function steps the simulation forward to the next water quality time step.
    ///
    /// # Returns
    /// A [`Result<u64>`] which:
    /// - `Ok(u64)` contains the time step advanced.
    /// - `Err(EPANETError)` if an error occurs.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_nextQ` and retrieves the next time step.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if stepping fails.
    ///
    /// # See Also
    /// - EN_nextQ (EPANET C API)
    pub fn next_q(&self) -> Result<u64> {
        let mut out_t_step = MaybeUninit::uninit();
        let result = unsafe { ffi::EN_nextQ(self.ph, out_t_step.as_mut_ptr()) };
        if result == 0 {
            Ok(unsafe { out_t_step.assume_init() as u64 })
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Opens the quality simulation.
    ///
    /// This function opens the water quality simulation in EPANET.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the simulation is successfully opened.
    /// - `Err(EPANETError)` if an error occurs.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_openQ` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if opening fails.
    ///
    /// # See Also
    /// - EN_openQ (EPANET C API)
    pub fn open_q(&self) -> Result<()> {
        let result = unsafe { ffi::EN_openQ(self.ph) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Runs the quality simulation for the current time step.
    ///
    /// This function runs the water quality simulation for the current time step and returns the current simulation time.
    ///
    /// # Returns
    /// A [`Result<u64>`] which:
    /// - `Ok(u64)` contains the current simulation time.
    /// - `Err(EPANETError)` if an error occurs.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_runQ` and retrieves the current simulation time.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if running fails.
    ///
    /// # See Also
    /// - EN_runQ (EPANET C API)
    pub fn run_q(&self) -> Result<u64> {
        let mut out_current_time = MaybeUninit::uninit();
        let result = unsafe { ffi::EN_runQ(self.ph, out_current_time.as_mut_ptr()) };
        if result == 0 {
            Ok(unsafe { out_current_time.assume_init() as u64 })
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Solves the entire quality simulation.
    ///
    /// This function solves the water quality simulation for the entire duration.
    ///
    /// # Returns
    /// A [`Result<()>`] which:
    /// - `Ok(())` if the simulation is successfully solved.
    /// - `Err(EPANETError)` if an error occurs.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_solveQ` with the project handle.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if solving fails.
    ///
    /// # See Also
    /// - EN_solveQ (EPANET C API)
    pub fn solve_q(&self) -> Result<()> {
        let result = unsafe { ffi::EN_solveQ(self.ph) };
        if result == 0 {
            Ok(())
        } else {
            Err(EPANETError::from(result))
        }
    }

    /// Steps through the quality simulation.
    ///
    /// This function advances the simulation by one step and returns the time left in the simulation.
    ///
    /// # Returns
    /// A [`Result<u64>`] which:
    /// - `Ok(u64)` contains the time left in the simulation.
    /// - `Err(EPANETError)` if an error occurs.
    ///
    /// # Implementation Details
    /// - Calls the EPANET C API function `EN_stepQ` and retrieves the time left.
    ///
    /// # Safety
    /// Uses `unsafe` code to interface with the EPANET C API. Assumes:
    /// - The EPANET project handle is valid.
    ///
    /// # Errors
    /// - Returns an [`EPANETError`] if stepping fails.
    ///
    /// # See Also
    /// - EN_stepQ (EPANET C API)
    pub fn step_q(&self) -> Result<u64> {
        let mut out_time_left = MaybeUninit::uninit();
        let result = unsafe { ffi::EN_stepQ(self.ph, out_time_left.as_mut_ptr()) };
        if result == 0 {
            Ok(unsafe { out_time_left.assume_init() as u64 })
        } else {
            Err(EPANETError::from(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::test_utils::fixtures::*;
    use crate::types::InitHydOption;
    use crate::EPANET;
    use rstest::rstest;

    #[rstest]
    pub fn test_solve_q(ph: EPANET) {
        let mut result = ph.solve_h();
        assert!(
            result.is_ok(),
            "Failed to solve hydraulic simulation: {:?}",
            result
        );

        result = ph.solve_q();
        assert!(
            result.is_ok(),
            "Failed to solve quality simulation: {:?}",
            result
        );

        result = ph.report();
        assert!(result.is_ok(), "Failed to report results: {:?}", result);
    }

    #[rstest]
    pub fn test_solve_q_step(ph: EPANET) {
        let t_stop = 10800; // 3 hours in seconds

        let mut result = ph.solve_h();
        assert!(
            result.is_ok(),
            "Failed to solve hydraulic simulation: {:?}",
            result
        );

        result = ph.open_q();
        assert!(
            result.is_ok(),
            "Failed to open quality simulation: {:?}",
            result
        );

        result = ph.init_q(InitHydOption::NoSave);
        assert!(
            result.is_ok(),
            "Failed to initialize quality simulation: {:?}",
            result
        );

        loop {
            let t_result = ph.run_q();
            assert!(
                t_result.is_ok(),
                "Failed to run quality analysis: {:?}",
                t_result
            );

            let t_step_result = ph.step_q();
            assert!(
                t_step_result.is_ok(),
                "Failed to step through quality analysis: {:?}",
                t_step_result
            );

            let t = t_result.unwrap();
            let t_step = t_step_result.unwrap();
            println!("Time: {}s, TStep: {}s", t, t_step);
            if t_step <= 0 || t >= t_stop {
                break;
            }
        }

        result = ph.close_q();
        assert!(
            result.is_ok(),
            "Failed to close quality simulation: {:?}",
            result
        );
    }

    #[rstest]
    pub fn test_progressive_step(ph: EPANET) {
        let mut result = ph.open_h();
        assert!(
            result.is_ok(),
            "Failed to open hydraulic simulation: {:?}",
            result
        );

        result = ph.init_h(InitHydOption::NoSave);
        assert!(
            result.is_ok(),
            "Failed to initialize hydraulic simulation: {:?}",
            result
        );

        result = ph.open_q();
        assert!(
            result.is_ok(),
            "Failed to open quality simulation: {:?}",
            result
        );

        result = ph.init_q(InitHydOption::NoSave);
        assert!(
            result.is_ok(),
            "Failed to initialize quality simulation: {:?}",
            result
        );

        result = ph.open_q();
        assert!(
            result.is_ok(),
            "Failed to open quality simulation: {:?}",
            result
        );

        result = ph.init_q(InitHydOption::NoSave);
        assert!(
            result.is_ok(),
            "Failed to initialize quality simulation: {:?}",
            result
        );

        loop {
            let mut run_result = ph.run_h();
            assert!(
                run_result.is_ok(),
                "Failed to run hydraulic simulation: {:?}",
                result
            );

            run_result = ph.run_q();
            assert!(
                run_result.is_ok(),
                "Failed to run quality simulation: {:?}",
                result
            );

            let tstep_h = ph.next_h();
            assert!(
                tstep_h.is_ok(),
                "Failed to step through hydraulic simulation: {:?}",
                tstep_h
            );

            let tstep_q = ph.next_q();
            assert!(
                tstep_q.is_ok(),
                "Failed to step through quality simulation: {:?}",
                tstep_q
            );

            if tstep_h.unwrap() <= 0 {
                break;
            }
        }

        result = ph.close_h();
        assert!(
            result.is_ok(),
            "Failed to close hydraulic simulation: {:?}",
            result
        );

        result = ph.close_q();
        assert!(
            result.is_ok(),
            "Failed to close quality simulation: {:?}",
            result
        );
    }
}
