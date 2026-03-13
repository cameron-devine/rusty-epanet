//! Analysis solver types using the typestate pattern.
//!
//! This module provides typestate-based solvers for hydraulic and water quality analyses.
//! The typestate pattern uses Rust's type system to enforce correct API usage at compile time,
//! making it impossible to call methods in the wrong order (e.g., stepping before initializing).
//!
//! # Solver Trait
//!
//! Both [`HydraulicSolver`] and [`QualitySolver`] implement the [`Solver`] trait, which provides
//! access to the underlying EPANET project via [`project()`](Solver::project). They share the
//! same state markers ([`Closed`], [`Initialized`], [`Running`], [`Solved`]) and the same
//! [`StepResult`] enum for stepping through simulations.
//!
//! # Hydraulic Solver
//!
//! The [`HydraulicSolver`] provides two modes of operation:
//!
//! 1. **One-shot solve**: Call [`solve()`](HydraulicSolver::solve) to run the entire simulation
//! 2. **Step-by-step**: Call [`init()`](HydraulicSolver::init) then loop with
//!    [`run()`](HydraulicSolver::run) and [`next()`](HydraulicSolver::next)
//!
//! # Quality Solver
//!
//! The [`QualitySolver`] requires hydraulics to be solved first and provides similar modes:
//!
//! 1. **One-shot solve**: Call [`solve()`](QualitySolver::solve) to run the entire simulation
//! 2. **Step-by-step**: Call [`init()`](QualitySolver::init) then loop with
//!    [`run()`](QualitySolver::run) and [`step()`](QualitySolver::step) or [`next()`](QualitySolver::next)
//!
//! # Example: Hydraulic Analysis
//!
//! ```ignore
//! use epanet::EPANET;
//! use epanet::types::analysis::{InitHydOption, StepResult};
//!
//! let epanet = EPANET::with_inp_file("network.inp", "", "")?;
//!
//! // One-shot solve
//! let solver = epanet.hydraulic_solver().solve()?;
//! solver.save()?;
//!
//! // Or step-by-step
//! let mut solver = epanet.hydraulic_solver().init(InitHydOption::Save)?.run()?;
//! loop {
//!     match solver.next()? {
//!         StepResult::Continue { current_time, next_step } => {
//!             println!("Time: {}, next step: {}", current_time, next_step);
//!         }
//!         StepResult::Done { current_time } => {
//!             println!("Done at time: {}", current_time);
//!             solver.close()?;
//!             break;
//!         }
//!     }
//! }
//! ```
//!
//! # Example: Quality Analysis
//!
//! ```ignore
//! use epanet::EPANET;
//! use epanet::types::analysis::{InitHydOption, StepResult};
//!
//! let epanet = EPANET::with_inp_file("network.inp", "", "")?;
//!
//! // Solve hydraulics first
//! epanet.hydraulic_solver().solve()?.save()?;
//!
//! // One-shot quality solve
//! epanet.quality_solver().solve()?;
//!
//! // Or step-by-step
//! let mut solver = epanet.quality_solver().init(InitHydOption::Save)?.run()?;
//! loop {
//!     match solver.next()? {
//!         StepResult::Continue { current_time, next_step } => {
//!             println!("Time: {}, next step: {}", current_time, next_step);
//!         }
//!         StepResult::Done { current_time } => {
//!             println!("Done at time: {}", current_time);
//!             solver.close()?;
//!             break;
//!         }
//!     }
//! }
//! ```

use crate::bindings::*;
use crate::epanet_error::*;
use crate::{ffi, EPANET};
use num_derive::FromPrimitive;
use std::marker::PhantomData;

/// Initialization options for hydraulic and quality analyses.
///
/// Controls whether results are saved to a file and whether flows are re-initialized.
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
// Solver States
// =============================================================================

/// Marker type for a closed solver.
pub struct Closed;

/// Marker type for an initialized solver (ready to run).
pub struct Initialized;

/// Marker type for a running solver (stepping through simulation).
pub struct Running;

/// Marker type for a solved solver (one-shot solve complete).
pub struct Solved;

// =============================================================================
// Solver Trait
// =============================================================================

/// Common interface for EPANET solvers.
///
/// Both [`HydraulicSolver`] and [`QualitySolver`] implement this trait, providing
/// access to the underlying EPANET project.
pub trait Solver<'a> {
    /// Returns a reference to the EPANET project.
    ///
    /// Use this to read simulation results at the current timestep.
    fn project(&self) -> &'a EPANET;
}

// =============================================================================
// StepResult
// =============================================================================

/// Result of advancing a simulation step.
///
/// Returned by stepping methods ([`HydraulicSolver::next`], [`QualitySolver::step`],
/// [`QualitySolver::next`]) to report the current simulation time and whether
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
// EPANET entry points
// =============================================================================

impl EPANET {
    /// Creates a new hydraulic solver in the Closed state.
    ///
    /// This is the entry point for hydraulic analysis using the typestate pattern.
    /// The returned solver enforces correct API usage at compile time.
    ///
    /// # See Also
    ///
    /// - [`HydraulicSolver`] for available operations
    /// - [`quality_solver`](Self::quality_solver) for water quality analysis
    pub fn hydraulic_solver(&self) -> HydraulicSolver<'_, Closed> {
        HydraulicSolver {
            project: self,
            current_time: 0,
            state: PhantomData,
        }
    }

    /// Creates a new quality solver in the Closed state.
    ///
    /// This is the entry point for water quality analysis using the typestate pattern.
    /// The returned solver enforces correct API usage at compile time.
    ///
    /// **Important:** Hydraulic analysis must be completed before running quality analysis.
    /// Either call [`HydraulicSolver::solve`] or complete the step-by-step hydraulic
    /// simulation first.
    ///
    /// # See Also
    ///
    /// - [`QualitySolver`] for available operations
    /// - [`hydraulic_solver`](Self::hydraulic_solver) for hydraulic analysis
    pub fn quality_solver(&self) -> QualitySolver<'_, Closed> {
        QualitySolver {
            project: self,
            current_time: 0,
            needs_close: false,
            state: PhantomData,
        }
    }
}

// =============================================================================
// Hydraulic Solver Implementation
// =============================================================================

/// A typestate-based hydraulic solver for EPANET simulations.
///
/// This struct uses the typestate pattern to enforce correct API usage at compile time.
/// The `State` type parameter tracks the current state of the solver, and methods are
/// only available in appropriate states.
///
/// # States
///
/// - [`Closed`]: Initial state. Can call [`solve()`](Self::solve) or [`init()`](Self::init).
/// - [`Initialized`]: Ready to run. Can call [`run()`](Self::run) to start stepping.
/// - [`Running`]: Actively stepping. Can call [`next()`](Self::next), [`save()`](Self::save),
///   and [`close()`](Self::close).
/// - [`Solved`]: One-shot solve complete. Can call [`save()`](Self::save).
///
/// # Drop Safety
///
/// If the solver is dropped without calling [`close()`](Self::close), the Drop
/// implementation will automatically close the hydraulic engine to prevent resource leaks.
pub struct HydraulicSolver<'a, State = Closed> {
    project: &'a EPANET,
    current_time: i64,
    state: PhantomData<State>,
}

impl<'a, State> Solver<'a> for HydraulicSolver<'a, State> {
    fn project(&self) -> &'a EPANET {
        self.project
    }
}

impl<'a> HydraulicSolver<'a, Closed> {
    /// Full solve in one shot — no stepping needed.
    ///
    /// Internally calls EN_openH, EN_initH, EN_runH, EN_nextH, and EN_closeH.
    pub fn solve(self) -> Result<HydraulicSolver<'a, Solved>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_solveH(project.ph) })?;
        Ok(HydraulicSolver {
            project,
            current_time: 0,
            state: PhantomData,
        })
    }

    /// Open + init for manual stepping.
    pub fn init(self, option: InitHydOption) -> Result<HydraulicSolver<'a, Initialized>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openH(project.ph) })?;
        check_error(unsafe { ffi::EN_initH(project.ph, option as i32) })?;
        Ok(HydraulicSolver {
            project,
            current_time: 0,
            state: PhantomData,
        })
    }
}

impl<'a> HydraulicSolver<'a, Initialized> {
    /// Run the first timestep, transitioning to Running state.
    pub fn run(self) -> Result<HydraulicSolver<'a, Running>> {
        let project = self.project;
        std::mem::forget(self);

        let mut current_time: i32 = 0;
        check_error(unsafe { ffi::EN_runH(project.ph, &mut current_time) })?;
        Ok(HydraulicSolver {
            project,
            current_time: current_time as i64,
            state: PhantomData,
        })
    }
}

impl<'a> HydraulicSolver<'a, Running> {
    /// Advance to the next timestep.
    ///
    /// Returns [`StepResult::Continue`] with the current time and next step duration,
    /// or [`StepResult::Done`] with the final time when the simulation is complete.
    pub fn next(&mut self) -> Result<StepResult> {
        let mut time_to_next: i32 = 0;
        check_error(unsafe { ffi::EN_nextH(self.project.ph, &mut time_to_next) })?;

        if time_to_next == 0 {
            Ok(StepResult::Done {
                current_time: self.current_time,
            })
        } else {
            let mut current_time: i32 = 0;
            check_error(unsafe { ffi::EN_runH(self.project.ph, &mut current_time) })?;
            self.current_time = current_time as i64;
            Ok(StepResult::Continue {
                current_time: current_time as i64,
                next_step: time_to_next as i64,
            })
        }
    }

    /// Saves hydraulic results to the output file.
    pub fn save(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_saveH(self.project.ph) })
    }

    /// Closes the hydraulic solver and frees resources.
    ///
    /// This consumes the solver and explicitly closes the hydraulic engine.
    /// If not called, the Drop implementation will close it automatically.
    pub fn close(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_closeH(project.ph) })
    }
}

impl<'a> HydraulicSolver<'a, Solved> {
    /// Saves hydraulic results to the output file.
    pub fn save(&self) -> Result<()> {
        check_error(unsafe { ffi::EN_saveH(self.project.ph) })
    }
}

/// Safety net — if someone drops the solver without calling close(),
/// make sure the C engine cleans up.
impl<'a, State> Drop for HydraulicSolver<'a, State> {
    fn drop(&mut self) {
        // EN_closeH is safe to call even if not opened — it's a no-op.
        unsafe {
            ffi::EN_closeH(self.project.ph);
        }
    }
}

// =============================================================================
// Quality Solver Implementation
// =============================================================================

/// A typestate-based water quality solver for EPANET simulations.
///
/// This struct uses the typestate pattern to enforce correct API usage at compile time.
/// The `State` type parameter tracks the current state of the solver, and methods are
/// only available in appropriate states.
///
/// **Important:** Hydraulic analysis must be completed before running quality analysis.
/// Use [`EPANET::hydraulic_solver`] to solve hydraulics first.
///
/// # States
///
/// - [`Closed`]: Initial state. Can call [`solve()`](Self::solve) or [`init()`](Self::init).
/// - [`Initialized`]: Ready to run. Can call [`run()`](Self::run) to start stepping.
/// - [`Running`]: Actively stepping. Can call [`step()`](Self::step), [`next()`](Self::next),
///   and [`close()`](Self::close).
/// - [`Solved`]: One-shot solve complete. No cleanup needed.
///
/// # Stepping Methods
///
/// The quality solver provides two methods for advancing the simulation:
///
/// - [`step()`](QualitySolver::step): Advances by one water quality time step.
/// - [`next()`](QualitySolver::next): Advances to the next reporting time step.
///
/// # Drop Safety
///
/// If the solver is dropped without calling [`close()`](Self::close), the Drop
/// implementation will automatically close the quality engine to prevent resource leaks.
pub struct QualitySolver<'a, State = Closed> {
    project: &'a EPANET,
    current_time: i64,
    /// Whether EN_closeQ needs to be called on drop.
    /// False for Closed (never opened) and Solved (EN_solveQ closes internally).
    /// True for Initialized, Running (step-by-step path).
    needs_close: bool,
    state: PhantomData<State>,
}

impl<'a, State> Solver<'a> for QualitySolver<'a, State> {
    fn project(&self) -> &'a EPANET {
        self.project
    }
}

impl<'a> QualitySolver<'a, Closed> {
    /// Runs the complete water quality simulation in one shot.
    ///
    /// Internally calls EN_openQ, EN_initQ, EN_runQ, EN_nextQ, and EN_closeQ.
    ///
    /// **Prerequisite:** Hydraulic analysis must be completed first.
    pub fn solve(self) -> Result<QualitySolver<'a, Solved>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_solveQ(project.ph) })?;
        Ok(QualitySolver {
            project,
            current_time: 0,
            needs_close: false,
            state: PhantomData,
        })
    }

    /// Opens and initializes the quality solver for step-by-step simulation.
    ///
    /// After calling this method, use [`run()`](QualitySolver::run) to start
    /// the simulation loop.
    ///
    /// **Prerequisite:** Hydraulic analysis must be completed first.
    ///
    /// # Parameters
    ///
    /// - `option`: Controls whether quality results are saved to the output file
    pub fn init(self, option: InitHydOption) -> Result<QualitySolver<'a, Initialized>> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_openQ(project.ph) })?;
        check_error(unsafe { ffi::EN_initQ(project.ph, option as i32) })?;
        Ok(QualitySolver {
            project,
            current_time: 0,
            needs_close: true,
            state: PhantomData,
        })
    }
}

impl<'a> QualitySolver<'a, Initialized> {
    /// Runs the quality simulation for the first time step, transitioning to Running state.
    pub fn run(self) -> Result<QualitySolver<'a, Running>> {
        let project = self.project;
        std::mem::forget(self);

        let mut current_time: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_runQ(project.ph, &mut current_time) })?;
        Ok(QualitySolver {
            project,
            current_time: current_time as i64,
            needs_close: true,
            state: PhantomData,
        })
    }
}

impl<'a> QualitySolver<'a, Running> {
    /// Advances the simulation by one water quality time step.
    ///
    /// This method uses `EN_stepQ` which advances by the water quality time step
    /// (typically smaller than the hydraulic time step for accuracy).
    /// The `next_step` field in [`StepResult::Continue`] contains the time remaining
    /// in the simulation.
    pub fn step(&mut self) -> Result<StepResult> {
        let mut time_left: std::os::raw::c_long = 0;
        check_error(unsafe { ffi::EN_stepQ(self.project.ph, &mut time_left) })?;

        // EN_runQ retrieves the current simulation time without advancing
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
    /// This method uses `EN_nextQ` which advances to the next time when
    /// results should be reported (typically at hydraulic time step intervals).
    pub fn next(&mut self) -> Result<StepResult> {
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
    /// This consumes the solver and explicitly closes the quality engine.
    /// If not called, the Drop implementation will close it automatically.
    pub fn close(self) -> Result<()> {
        let project = self.project;
        std::mem::forget(self);

        check_error(unsafe { ffi::EN_closeQ(project.ph) })
    }
}

/// Safety net — if someone drops the solver without calling close(),
/// make sure the C engine cleans up.
///
/// The `needs_close` field tracks whether EN_closeQ should be called:
/// - `false` for Closed (never opened) and Solved (EN_solveQ closes internally)
/// - `true` for Initialized and Running (step-by-step path)
impl<'a, State> Drop for QualitySolver<'a, State> {
    fn drop(&mut self) {
        if self.needs_close {
            unsafe {
                ffi::EN_closeQ(self.project.ph);
            }
        }
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
    // Hydraulic Solver Tests
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_hydraulic_solver_one_shot(ph: EPANET) {
        let solver = ph.hydraulic_solver();
        let solved = solver.solve().expect("Failed to solve hydraulics");
        solved.save().expect("Failed to save hydraulics");
    }

    #[rstest]
    fn test_hydraulic_solver_step_by_step(ph: EPANET) {
        let solver = ph.hydraulic_solver();
        let initialized = solver
            .init(InitHydOption::Save)
            .expect("Failed to init hydraulics");
        let mut running = initialized.run().expect("Failed to run hydraulics");

        let mut steps = 0;
        loop {
            steps += 1;

            match running.next().expect("Failed to step hydraulics") {
                StepResult::Continue {
                    current_time,
                    next_step,
                } => {
                    assert!(current_time >= 0, "Time should be non-negative");
                    assert!(next_step > 0, "Next step should be positive while continuing");
                }
                StepResult::Done { current_time } => {
                    assert!(current_time >= 0, "Final time should be non-negative");
                    running.close().expect("Failed to close hydraulics");
                    break;
                }
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    // -------------------------------------------------------------------------
    // Quality Solver Tests
    // -------------------------------------------------------------------------

    #[rstest]
    fn test_quality_solver_one_shot(ph: EPANET) {
        // Solve hydraulics first
        ph.hydraulic_solver()
            .solve()
            .expect("Failed to solve hydraulics")
            .save()
            .expect("Failed to save hydraulics");

        // One-shot quality solve — no close() needed
        let solved = ph.quality_solver().solve().expect("Failed to solve quality");

        // We can still access the project via the Solver trait
        let _project = Solver::project(&solved);
    }

    #[rstest]
    fn test_quality_solver_step_by_step(ph: EPANET) {
        // Solve hydraulics first
        ph.hydraulic_solver()
            .solve()
            .expect("Failed to solve hydraulics")
            .save()
            .expect("Failed to save hydraulics");

        // Step-by-step quality solve using step()
        let mut running = ph
            .quality_solver()
            .init(InitHydOption::Save)
            .expect("Failed to init quality")
            .run()
            .expect("Failed to run quality");

        let mut steps = 0;
        loop {
            steps += 1;

            // Access project via Solver trait to verify we can read results
            let _project = Solver::project(&running);

            match running.step().expect("Failed to step quality") {
                StepResult::Continue {
                    current_time: _,
                    next_step,
                } => {
                    assert!(next_step > 0, "Time left should be positive while continuing");
                }
                StepResult::Done { .. } => {
                    running.close().expect("Failed to close quality");
                    break;
                }
            }

            // Safety limit to prevent infinite loops in tests
            if steps > 10000 {
                panic!("Too many steps, possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    #[rstest]
    fn test_quality_solver_next(ph: EPANET) {
        // Solve hydraulics first
        ph.hydraulic_solver()
            .solve()
            .expect("Failed to solve hydraulics")
            .save()
            .expect("Failed to save hydraulics");

        // Step-by-step quality solve using next()
        let mut running = ph
            .quality_solver()
            .init(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run()
            .expect("Failed to run quality");

        let mut steps = 0;
        loop {
            steps += 1;

            match running.next().expect("Failed to next quality") {
                StepResult::Continue {
                    current_time: _,
                    next_step,
                } => {
                    assert!(next_step > 0, "Time step should be positive while continuing");
                }
                StepResult::Done { .. } => {
                    running.close().expect("Failed to close quality");
                    break;
                }
            }

            // Safety limit
            if steps > 1000 {
                panic!("Too many steps, possible infinite loop");
            }
        }

        assert!(steps > 1, "Should have taken multiple steps");
    }

    #[rstest]
    fn test_quality_solver_drop_safety(ph: EPANET) {
        // Solve hydraulics first
        ph.hydraulic_solver()
            .solve()
            .expect("Failed to solve hydraulics")
            .save()
            .expect("Failed to save hydraulics");

        // Create solver but don't close it - should be cleaned up by Drop
        let running = ph
            .quality_solver()
            .init(InitHydOption::NoSave)
            .expect("Failed to init quality")
            .run()
            .expect("Failed to run quality");

        // Just drop it without calling close() explicitly
        drop(running);

        // If we get here without crashing, Drop worked correctly
    }
}
