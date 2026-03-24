//! Unified typestate solver for EPANET hydraulic and water quality simulations.
//!
//! This module provides a single [`Solver<S>`] struct whose type parameter `S` encodes
//! every valid simulation path from the EPANET toolkit documentation, making invalid
//! sequences compile errors.
//!
//! # Valid Simulation Paths
//!
//! | Path | Steps |
//! |---|---|
//! | H one-shot | [`solve_h`](Solver::solve_h) |
//! | H step-by-step | [`init_h`] → [`run_h`] → loop{[`next_h`]} → [`close_h`] |
//! | H + Q simultaneous | [`init_h`] → [`init_q`] → [`run`] → loop{[`next`]} → [`close`] |
//! | Q after H | H (one-shot or step-by-step) → [`init_q`] → [`run_q`] → loop{[`step_q`]/[`next_q`]} → [`close_q`] |
//! | Q from file | [`solve_h`] → [`save_hyd_file`] / [`use_hyd_file`] → quality steps |
//!
//! [`init_h`]: Solver::init_h
//! [`run_h`]: Solver::run_h
//! [`next_h`]: Solver::next_h
//! [`close_h`]: Solver::close_h
//! [`init_q`]: Solver::init_q
//! [`run`]: Solver::run
//! [`next`]: Solver::next
//! [`close`]: Solver::close
//! [`run_q`]: Solver::run_q
//! [`step_q`]: Solver::step_q
//! [`next_q`]: Solver::next_q
//! [`close_q`]: Solver::close_q
//! [`save_hyd_file`]: Solver::save_hyd_file
//! [`use_hyd_file`]: Solver::use_hyd_file
//!
//! # Entry Point
//!
//! Use [`EPANET::solver`] to obtain a `Solver<HClosed>`.
//!
//! # Example: Hydraulic one-shot
//!
//! ```ignore
//! let solver = epanet.solver().solve_h()?;
//! solver.save()?;
//! ```
//!
//! # Example: Step-by-step hydraulics then quality
//!
//! ```ignore
//! use epanet::types::analysis::{InitHydOption, StepResult};
//!
//! let mut solver = epanet.solver().init_h(InitHydOption::Save)?.run_h()?;
//! loop {
//!     match solver.next_h()? {
//!         StepResult::Continue { .. } => {}
//!         StepResult::Done { .. } => break,
//!     }
//! }
//! let mut solver = solver.close_h()?.init_q(InitHydOption::Save)?.run_q()?;
//! loop {
//!     match solver.step_q()? {
//!         StepResult::Continue { .. } => {}
//!         StepResult::Done { .. } => { solver.close_q()?; break; }
//!     }
//! }
//! ```

use crate::bindings::*;
use crate::epanet_error::*;
use crate::{ffi, EPANET};
use num_derive::FromPrimitive;
use std::ffi::CString;
use std::marker::PhantomData;

// =============================================================================
// InitHydOption
// =============================================================================

/// Initialization options for hydraulic and quality analyses.
///
/// Controls whether results are saved to a file and whether flows are re-initialized.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum InitHydOption {
    /// Don't save hydraulics; don't re-initialize flows
    NoSave = EN_InitHydOption_EN_NOSAVE as i32,
    /// Save hydraulics to file, don't re-initialize flows
    Save = EN_InitHydOption_EN_SAVE as i32,
    /// Don't save hydraulics; re-initialize flows
    InitFlow = EN_InitHydOption_EN_INITFLOW as i32,
    /// Save hydraulics; re-initialize flows
    SaveAndInit = EN_InitHydOption_EN_SAVE_AND_INIT as i32,
}

// =============================================================================
// State marker types (9 total)
// =============================================================================

/// Marker: initial state — hydraulic engine not yet opened.
pub struct HClosed;

/// Marker: hydraulic engine opened and initialized, ready to run.
pub struct HInitialized;

/// Marker: hydraulic + quality both initialized for simultaneous simulation.
pub struct HQInitialized;

/// Marker: hydraulic solver stepping (step-by-step only).
pub struct HRunning;

/// Marker: simultaneous H+Q solver stepping.
pub struct HQRunning;

/// Marker: hydraulic solve complete (one-shot or closed after stepping).
pub struct HydDone;

/// Marker: hydraulics loaded from a saved binary file.
pub struct HydFileLoaded;

/// Marker: quality solver opened and initialized, ready to run.
pub struct QInitialized;

/// Marker: quality solver stepping (step-by-step).
pub struct QRunning;

// =============================================================================
// StepResult
// =============================================================================

/// Result of advancing a simulation step.
///
/// Returned by stepping methods to report the current simulation time and whether
/// more steps remain.
pub enum StepResult {
    /// Simulation has more steps to run.
    Continue {
        /// Current simulation time in seconds.
        current_time: i64,
        /// Time until the next step in seconds.
        next_step: i64,
    },
    /// Simulation has completed.
    Done {
        /// Final simulation time in seconds.
        current_time: i64,
    },
}

// =============================================================================
// Unified Solver struct
// =============================================================================

/// A unified typestate-based solver for EPANET hydraulic and water quality simulations.
///
/// The `S` type parameter tracks the current simulation state. Methods are only
/// available in the appropriate states, making invalid sequences compile errors.
///
/// # Entry Point
///
/// Use [`EPANET::solver`] to obtain a `Solver<HClosed>`.
///
/// # Drop Safety
///
/// If dropped without explicit close, the `Drop` impl automatically calls
/// `EN_closeH` and/or `EN_closeQ` as needed to prevent resource leaks.
pub struct Solver<'a, S = HClosed> {
    project: &'a EPANET,
    current_time: i64,
    /// True when `EN_openH` was called but `EN_closeH` has not yet been called.
    needs_close_h: bool,
    /// True when `EN_openQ` was called but `EN_closeQ` has not yet been called.
    needs_close_q: bool,
    state: PhantomData<S>,
}

impl<'a, S> Solver<'a, S> {
    /// Returns a reference to the underlying EPANET project.
    ///
    /// Use this to read simulation results at the current timestep.
    pub fn project(&self) -> &'a EPANET {
        self.project
    }
}

impl<'a, S> Drop for Solver<'a, S> {
    fn drop(&mut self) {
        unsafe {
            if self.needs_close_h {
                ffi::EN_closeH(self.project.ph);
            }
            if self.needs_close_q {
                ffi::EN_closeQ(self.project.ph);
            }
        }
    }
}

// =============================================================================
// EPANET entry point
// =============================================================================

impl EPANET {
    /// Creates a new solver in the initial [`HClosed`] state.
    ///
    /// This is the single entry point for all typestate-based hydraulic and water
    /// quality simulations.
    pub fn solver(&self) -> Solver<'_, HClosed> {
        Solver {
            project: self,
            current_time: 0,
            needs_close_h: false,
            needs_close_q: false,
            state: PhantomData,
        }
    }
}

// =============================================================================
// HClosed → {HydDone, HInitialized, HydFileLoaded}
// =============================================================================

impl<'a> Solver<'a, HClosed> {
    /// Runs a complete hydraulic simulation in one shot.
    ///
    /// Internally calls `EN_solveH`.
    pub fn solve_h(self) -> Result<Solver<'a, HydDone>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_solveH(project.ph) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: false,
            needs_close_q: false,
            state: PhantomData,
        })
    }

    /// Opens and initializes the hydraulic solver for step-by-step simulation.
    ///
    /// Calls `EN_openH` + `EN_initH`.
    ///
    /// # Parameters
    ///
    /// - `option`: Controls whether results are saved and whether flows are re-initialized.
    pub fn init_h(self, option: InitHydOption) -> Result<Solver<'a, HInitialized>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openH(project.ph) })?;
        check_error(unsafe { ffi::EN_initH(project.ph, option as i32) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: true,
            needs_close_q: false,
            state: PhantomData,
        })
    }

    /// Loads hydraulic results from a previously saved binary file.
    ///
    /// Calls `EN_usehydfile`. After loading, quality analysis can be run
    /// without re-running hydraulics.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the saved hydraulics file.
    pub fn use_hyd_file(self, path: &str) -> Result<Solver<'a, HydFileLoaded>> {
        let project = self.project;
        std::mem::forget(self);

        let c_path = CString::new(path).expect("path contains null bytes");
        check_error(unsafe { ffi::EN_usehydfile(project.ph, c_path.as_ptr()) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: false,
            needs_close_q: false,
            state: PhantomData,
        })
    }
}

// =============================================================================
// HInitialized → {HRunning, HQInitialized}
// =============================================================================

impl<'a> Solver<'a, HInitialized> {
    /// Runs the first hydraulic time step, transitioning to [`HRunning`].
    ///
    /// Calls `EN_runH`.
    pub fn run_h(self) -> Result<Solver<'a, HRunning>> {
        let project = self.project;
        let needs_close_h = self.needs_close_h;
        std::mem::forget(self);

        let mut current_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runH(project.ph, &mut current_time) })?;
        Ok(Solver {
            project,
            current_time: current_time as i64,
            needs_close_h,
            needs_close_q: false,
            state: PhantomData,
        })
    }

    /// Opens and initializes the quality solver for simultaneous H+Q simulation.
    ///
    /// Calls `EN_openQ` + `EN_initQ`. Use this before calling [`run`](Solver::run)
    /// to start the simultaneous loop.
    ///
    /// # Parameters
    ///
    /// - `option`: Controls whether quality results are saved.
    pub fn init_q(self, option: InitHydOption) -> Result<Solver<'a, HQInitialized>> {
        let project = self.project;
        let needs_close_h = self.needs_close_h;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openQ(project.ph) })?;
        check_error(unsafe { ffi::EN_initQ(project.ph, option as i32) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h,
            needs_close_q: true,
            state: PhantomData,
        })
    }
}

// =============================================================================
// HQInitialized → HQRunning
// =============================================================================

impl<'a> Solver<'a, HQInitialized> {
    /// Runs the first simultaneous H+Q time step, transitioning to [`HQRunning`].
    ///
    /// Calls `EN_runH` + `EN_runQ`.
    pub fn run(self) -> Result<Solver<'a, HQRunning>> {
        let project = self.project;
        let needs_close_h = self.needs_close_h;
        let needs_close_q = self.needs_close_q;
        std::mem::forget(self);

        let mut current_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runH(project.ph, &mut current_time) })?;
        let mut _q_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runQ(project.ph, &mut _q_time) })?;
        Ok(Solver {
            project,
            current_time: current_time as i64,
            needs_close_h,
            needs_close_q,
            state: PhantomData,
        })
    }
}

// =============================================================================
// HRunning → step, save, close_h → HydDone
// =============================================================================

impl<'a> Solver<'a, HRunning> {
    /// Advances to the next hydraulic time step.
    ///
    /// Calls `EN_nextH`. If more steps remain, also calls `EN_runH`.
    ///
    /// # Returns
    ///
    /// - [`StepResult::Continue`]: more steps remain; `next_step` is time to next event.
    /// - [`StepResult::Done`]: simulation complete; call [`close_h`](Self::close_h).
    #[allow(clippy::should_implement_trait)]
    pub fn next_h(&mut self) -> Result<StepResult> {
        let mut time_to_next: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_nextH(self.project.ph, &mut time_to_next) })?;

        if time_to_next == 0 {
            Ok(StepResult::Done {
                current_time: self.current_time,
            })
        } else {
            let mut current_time: std::os::raw::c_long = 0;
            check_error(unsafe { ffi::EN_runH(self.project.ph, &mut current_time) })?;
            self.current_time = current_time as i64;
            Ok(StepResult::Continue {
                current_time: current_time as i64,
                next_step: time_to_next as i64,
            })
        }
    }

    /// Saves hydraulic results to the binary output file.
    ///
    /// Calls `EN_saveH`.
    pub fn save(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_saveH(self.project.ph) })
    }

    /// Saves hydraulic results to a named binary file for later use.
    ///
    /// Calls `EN_savehydfile`. The saved file can be loaded with
    /// [`use_hyd_file`](Solver::use_hyd_file).
    ///
    /// # Parameters
    ///
    /// - `path`: Destination file path.
    pub fn save_hyd_file(&self, path: &str) -> Result<()> {
        let c_path = CString::new(path).expect("path contains null bytes");
        check_error(unsafe { ffi::EN_savehydfile(self.project.ph, c_path.as_ptr()) })
    }

    /// Closes the hydraulic solver and transitions to [`HydDone`].
    ///
    /// Calls `EN_closeH`. Returns a `Solver<HydDone>` so that quality analysis
    /// can be chained immediately.
    pub fn close_h(self) -> Result<Solver<'a, HydDone>> {
        let project = self.project;
        let needs_close_q = self.needs_close_q;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_closeH(project.ph) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: false,
            needs_close_q,
            state: PhantomData,
        })
    }
}

// =============================================================================
// HQRunning → step, close
// =============================================================================

impl<'a> Solver<'a, HQRunning> {
    /// Advances the simultaneous H+Q simulation by one step.
    ///
    /// Calls `EN_nextH` + `EN_nextQ`. If more steps remain, also calls
    /// `EN_runH` + `EN_runQ`.
    ///
    /// # Returns
    ///
    /// - [`StepResult::Continue`]: more steps remain; `next_step` is the minimum
    ///   of the H and Q times to next.
    /// - [`StepResult::Done`]: simulation complete; call [`close`](Self::close).
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<StepResult> {
        let mut time_to_next_h: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_nextH(self.project.ph, &mut time_to_next_h) })?;
        let mut time_to_next_q: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_nextQ(self.project.ph, &mut time_to_next_q) })?;

        if time_to_next_h == 0 || time_to_next_q == 0 {
            Ok(StepResult::Done {
                current_time: self.current_time,
            })
        } else {
            let mut current_time: std::os::raw::c_long = 0;
            check_error(unsafe { ffi::EN_runH(self.project.ph, &mut current_time) })?;
            let mut _q_time: std::os::raw::c_long = 0;
            check_error(unsafe { ffi::EN_runQ(self.project.ph, &mut _q_time) })?;
            self.current_time = current_time as i64;
            Ok(StepResult::Continue {
                current_time: current_time as i64,
                next_step: time_to_next_h.min(time_to_next_q) as i64,
            })
        }
    }

    /// Closes both the hydraulic and quality solvers.
    ///
    /// Calls `EN_closeH` + `EN_closeQ`.
    pub fn close(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_closeH(project.ph) })?;
        check_error(unsafe { ffi::EN_closeQ(project.ph) })
    }
}

// =============================================================================
// HydDone → save, save_hyd_file, solve_q, init_q → QInitialized
// =============================================================================

impl<'a> Solver<'a, HydDone> {
    /// Saves hydraulic results to the binary output file.
    ///
    /// Calls `EN_saveH`.
    pub fn save(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_saveH(self.project.ph) })
    }

    /// Saves hydraulic results to a named binary file for later use.
    ///
    /// Calls `EN_savehydfile`.
    ///
    /// # Parameters
    ///
    /// - `path`: Destination file path.
    pub fn save_hyd_file(&self, path: &str) -> Result<()> {
        let c_path = CString::new(path).expect("path contains null bytes");
        check_error(unsafe { ffi::EN_savehydfile(self.project.ph, c_path.as_ptr()) })
    }

    /// Runs a complete water quality simulation in one shot.
    ///
    /// Calls `EN_solveQ`. No explicit close needed — `EN_solveQ` handles cleanup.
    ///
    /// **Prerequisite:** Hydraulics must be solved and saved first.
    pub fn solve_q(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_solveQ(project.ph) })
    }

    /// Opens and initializes the quality solver for step-by-step simulation.
    ///
    /// Calls `EN_openQ` + `EN_initQ`.
    ///
    /// # Parameters
    ///
    /// - `option`: Controls whether quality results are saved.
    pub fn init_q(self, option: InitHydOption) -> Result<Solver<'a, QInitialized>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openQ(project.ph) })?;
        check_error(unsafe { ffi::EN_initQ(project.ph, option as i32) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: false,
            needs_close_q: true,
            state: PhantomData,
        })
    }
}

// =============================================================================
// HydFileLoaded → solve_q, init_q → QInitialized
// =============================================================================

impl<'a> Solver<'a, HydFileLoaded> {
    /// Runs a complete water quality simulation using the loaded hydraulic file.
    ///
    /// Calls `EN_solveQ`.
    pub fn solve_q(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_solveQ(project.ph) })
    }

    /// Opens and initializes the quality solver for step-by-step simulation.
    ///
    /// Calls `EN_openQ` + `EN_initQ`.
    ///
    /// # Parameters
    ///
    /// - `option`: Controls whether quality results are saved.
    pub fn init_q(self, option: InitHydOption) -> Result<Solver<'a, QInitialized>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openQ(project.ph) })?;
        check_error(unsafe { ffi::EN_initQ(project.ph, option as i32) })?;
        Ok(Solver {
            project,
            current_time: 0,
            needs_close_h: false,
            needs_close_q: true,
            state: PhantomData,
        })
    }
}

// =============================================================================
// QInitialized → QRunning
// =============================================================================

impl<'a> Solver<'a, QInitialized> {
    /// Runs the first quality time step, transitioning to [`QRunning`].
    ///
    /// Calls `EN_runQ`.
    pub fn run_q(self) -> Result<Solver<'a, QRunning>> {
        let project = self.project;
        let needs_close_q = self.needs_close_q;
        std::mem::forget(self);

        let mut current_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runQ(project.ph, &mut current_time) })?;
        Ok(Solver {
            project,
            current_time: current_time as i64,
            needs_close_h: false,
            needs_close_q,
            state: PhantomData,
        })
    }
}

// =============================================================================
// QRunning → step_q, next_q, close_q
// =============================================================================

impl<'a> Solver<'a, QRunning> {
    /// Advances the simulation by one water quality time step.
    ///
    /// Calls `EN_stepQ`, then `EN_runQ` to retrieve the current time.
    ///
    /// # Returns
    ///
    /// - [`StepResult::Continue`]: more steps remain; `next_step` is the time
    ///   remaining in the simulation.
    /// - [`StepResult::Done`]: simulation complete; call [`close_q`](Self::close_q).
    pub fn step_q(&mut self) -> Result<StepResult> {
        let mut time_left: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_stepQ(self.project.ph, &mut time_left) })?;

        let mut current_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runQ(self.project.ph, &mut current_time) })?;
        self.current_time = current_time as i64;

        if time_left == 0 {
            Ok(StepResult::Done {
                current_time: current_time as i64,
            })
        } else {
            Ok(StepResult::Continue {
                current_time: current_time as i64,
                next_step: time_left as i64,
            })
        }
    }

    /// Advances the simulation to the next reporting time step.
    ///
    /// Calls `EN_nextQ`. If more steps remain, also calls `EN_runQ`.
    ///
    /// # Returns
    ///
    /// - [`StepResult::Continue`]: more steps remain; `next_step` is the time step advanced.
    /// - [`StepResult::Done`]: simulation complete; call [`close_q`](Self::close_q).
    #[allow(clippy::should_implement_trait)]
    pub fn next_q(&mut self) -> Result<StepResult> {
        let mut time_step: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_nextQ(self.project.ph, &mut time_step) })?;

        if time_step == 0 {
            Ok(StepResult::Done {
                current_time: self.current_time,
            })
        } else {
            let mut current_time: std::os::raw::c_long = 0;
            check_error(unsafe { ffi::EN_runQ(self.project.ph, &mut current_time) })?;
            self.current_time = current_time as i64;
            Ok(StepResult::Continue {
                current_time: current_time as i64,
                next_step: time_step as i64,
            })
        }
    }

    /// Closes the quality solver and frees resources.
    ///
    /// Calls `EN_closeQ`.
    pub fn close_q(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_closeQ(project.ph) })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::impls::test_utils::fixtures::*;
    use rstest::rstest;

    // -------------------------------------------------------------------------
    // Hydraulic solver tests
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_hydraulic_solver_one_shot(ph: EPANET) {
        let solver = ph.solver().solve_h().expect("Failed to solve hydraulics");
        solver.save().expect("Failed to save hydraulics");
    }

    #[rstest]
    fn test_hydraulic_solver_step_by_step(ph: EPANET) {
        let mut solver = ph
            .solver()
            .init_h(InitHydOption::Save)
            .expect("Failed to init hydraulics")
            .run_h()
            .expect("Failed to run first hydraulic step");

        let mut steps = 0;
        loop {
            steps += 1;
            match solver.next_h().expect("Failed to step hydraulics") {
                StepResult::Continue {
                    current_time,
                    next_step,
                } => {
                    assert!(current_time >= 0);
                    assert!(next_step > 0);
                }
                StepResult::Done { current_time } => {
                    assert!(current_time >= 0);
                    solver.close_h().expect("Failed to close hydraulics");
                    break;
                }
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    #[rstest]
    fn test_hydraulic_drop_safety(ph: EPANET) {
        // Drop HRunning without calling close_h — Drop impl should clean up.
        let solver = ph
            .solver()
            .init_h(InitHydOption::NoSave)
            .expect("Failed to init")
            .run_h()
            .expect("Failed to run");
        drop(solver);
        // No crash = Drop worked correctly.
    }

    // -------------------------------------------------------------------------
    // Quality solver tests
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_quality_solver_one_shot(ph: EPANET) {
        let hyd = ph.solver().solve_h().expect("Failed to solve hydraulics");
        hyd.save().expect("Failed to save hydraulics");
        hyd.solve_q().expect("Failed to solve quality");
    }

    #[rstest]
    fn test_quality_solver_step_by_step(ph: EPANET) {
        let mut solver = ph
            .solver()
            .solve_h()
            .expect("Failed to solve hydraulics")
            .init_q(InitHydOption::Save)
            .expect("Failed to init quality")
            .run_q()
            .expect("Failed to run quality");

        let mut steps = 0;
        loop {
            steps += 1;

            // Verify project is accessible at each step.
            let _project = solver.project();

            match solver.step_q().expect("Failed to step quality") {
                StepResult::Continue { next_step, .. } => {
                    assert!(next_step > 0);
                }
                StepResult::Done { .. } => {
                    solver.close_q().expect("Failed to close quality");
                    break;
                }
            }

            if steps > 10000 {
                panic!("Too many steps — possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    #[rstest]
    fn test_quality_solver_next(ph: EPANET) {
        let mut solver = ph
            .solver()
            .solve_h()
            .expect("Failed to solve hydraulics")
            .init_q(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run_q()
            .expect("Failed to run quality");

        let mut steps = 0;
        loop {
            steps += 1;
            match solver.next_q().expect("Failed to next quality") {
                StepResult::Continue { next_step, .. } => {
                    assert!(next_step > 0);
                }
                StepResult::Done { .. } => {
                    solver.close_q().expect("Failed to close quality");
                    break;
                }
            }

            if steps > 1000 {
                panic!("Too many steps — possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    #[rstest]
    fn test_quality_solver_drop_safety(ph: EPANET) {
        let solver = ph
            .solver()
            .solve_h()
            .expect("Failed to solve hydraulics")
            .init_q(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run_q()
            .expect("Failed to run quality");

        // Drop without calling close_q — Drop impl should call EN_closeQ.
        drop(solver);
        // No crash = Drop worked correctly.
    }

    // -------------------------------------------------------------------------
    // Simultaneous H+Q test
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_simultaneous_hq(ph: EPANET) {
        let mut solver = ph
            .solver()
            .init_h(InitHydOption::NoSave)
            .expect("Failed to init hydraulics")
            .init_q(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run()
            .expect("Failed to run first H+Q step");

        let mut steps = 0;
        loop {
            steps += 1;
            match solver.next().expect("Failed to step H+Q") {
                StepResult::Continue {
                    current_time,
                    next_step,
                } => {
                    assert!(current_time >= 0);
                    assert!(next_step > 0);
                }
                StepResult::Done { .. } => {
                    solver.close().expect("Failed to close H+Q");
                    break;
                }
            }

            if steps > 10000 {
                panic!("Too many steps — possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    // -------------------------------------------------------------------------
    // Hydraulics file tests
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_use_hyd_file(ph: EPANET) {
        let hyd_path = {
            let dir = std::env::temp_dir();
            dir.join("epanet_test_solver.hyd")
                .to_string_lossy()
                .into_owned()
        };

        // Solve hydraulics and save to file.
        {
            let hyd = ph.solver().solve_h().expect("Failed to solve hydraulics");
            hyd.save_hyd_file(&hyd_path)
                .expect("Failed to save hydraulics file");
        } // hyd dropped here, releasing borrow of ph

        // Load file and run quality.
        ph.solver()
            .use_hyd_file(&hyd_path)
            .expect("Failed to load hydraulics file")
            .solve_q()
            .expect("Failed to solve quality from file");

        // Clean up.
        drop(ph);
        let _ = std::fs::remove_file(&hyd_path);
    }

    #[rstest]
    fn test_close_h_then_quality(ph: EPANET) {
        // Step-by-step hydraulics, then step-by-step quality.
        let mut hyd = ph
            .solver()
            .init_h(InitHydOption::Save)
            .expect("Failed to init hydraulics")
            .run_h()
            .expect("Failed to run hydraulics");

        loop {
            match hyd.next_h().expect("Failed to step hydraulics") {
                StepResult::Continue { .. } => {}
                StepResult::Done { .. } => break,
            }
        }

        let mut qual = hyd
            .close_h()
            .expect("Failed to close hydraulics")
            .init_q(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run_q()
            .expect("Failed to run quality");

        let mut steps = 0;
        loop {
            steps += 1;
            match qual.step_q().expect("Failed to step quality") {
                StepResult::Continue { .. } => {}
                StepResult::Done { .. } => {
                    qual.close_q().expect("Failed to close quality");
                    break;
                }
            }

            if steps > 10000 {
                panic!("Too many steps — possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }
}
