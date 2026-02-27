use crate::bindings::*;
use crate::{ffi, EPANET};
use num_derive::FromPrimitive;
use std::marker::PhantomData;
use crate::epanet_error::*;

#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[repr(i32)]
pub enum InitHydOption {
    NoSave = EN_InitHydOption_EN_NOSAVE, // Don't save hydraulics; don't re-initialize flows
    Save = EN_InitHydOption_EN_SAVE, // Save hydraulics to file, don't re-initialize flows
    InitFlow = EN_InitHydOption_EN_INITFLOW, // Don't save hydraulics; re-initialize flows
    SaveAndInit = EN_InitHydOption_EN_SAVE_AND_INIT, // Save hydraulics; re-initialize flows
}

pub struct Closed;
pub struct Initialized;
pub struct Running;
pub struct Solved;

pub enum StepResult<'a> {
    Continue(HydraulicSolver<'a, Running>),
    Done(HydraulicSolver<'a, Solved>),
}

impl EPANET {
    /// Entry point — creates a solver in the Closed state
    pub fn hydraulic_solver(&self) -> HydraulicSolver<'_, Closed> {
        HydraulicSolver {
            project: self,
            state: PhantomData,
        }
    }
}

pub struct HydraulicSolver<'a,State = Closed> {
    project: &'a EPANET,
    state: PhantomData<State>,
}

impl<'a> HydraulicSolver<'a, Closed> {
    /// Full solve in one shot — no stepping needed
    pub fn solve(self) -> Result<HydraulicSolver<'a, Solved>> {
        // EN_solveH
        check_error(unsafe { ffi::EN_solveH(self.project.ph) })?;
        Ok(HydraulicSolver { project: self.project, state: PhantomData })
    }

    /// Open + init for manual stepping
    pub fn init(self, option: InitHydOption) -> Result<HydraulicSolver<'a, Initialized>> {
        check_error(unsafe { ffi::EN_openH(self.project.ph) })?;
        check_error(unsafe { ffi::EN_initH(self.project.ph, option as i32) })?;
        Ok(HydraulicSolver { project: self.project, state: PhantomData })
    }
}

impl<'a> HydraulicSolver<'a, Initialized> {
    /// Run the first timestep
    pub fn run(self) -> Result<(HydraulicSolver<'a, Running>, f64)> {
        let mut current_time: i32 = 0;
        check_error(unsafe { ffi::EN_runH(self.project.ph, &mut current_time) })?;
        let solver = HydraulicSolver { project: self.project, state: PhantomData };
        Ok((solver, current_time as f64))
    }
}

impl<'a> HydraulicSolver<'a, Running> {
    /// Advance to the next timestep.
    pub fn next(self) -> Result<StepResult<'a>> {
        let mut time_to_next: i32 = 0;
        check_error(unsafe { ffi::EN_nextH(self.project.ph, &mut time_to_next) })?;

        if time_to_next == 0 {
            // Simulation complete
            Ok(StepResult::Done(HydraulicSolver {
                project: self.project,
                state: PhantomData,
            }))
        } else {
            // Run the next timestep
            let mut current_time: i32 = 0;
            check_error(unsafe { ffi::EN_runH(self.project.ph, &mut current_time) })?;
            Ok(StepResult::Continue(HydraulicSolver {
                project: self.project,
                state: PhantomData,
            }))
        }
    }

    /// Access the project to read results at the current timestep
    pub fn project(&self) -> &EPANET {
        self.project
    }
}

impl<'a> HydraulicSolver<'a, Solved> {
    pub fn save(self) -> Result<HydraulicSolver<'a, Solved>> {
        check_error(unsafe { ffi::EN_saveH(self.project.ph) })?;
        Ok(self)
    }

    pub fn close(self) -> Result<()> {
        check_error(unsafe { ffi::EN_closeH(self.project.ph) })
    }

    pub fn project(&self) -> &EPANET {
        self.project
    }
}

/// Safety net — if someone drops the solver without calling close(),
/// make sure the C engine cleans up
impl<'a, State> Drop for HydraulicSolver<'a, State> {
    fn drop(&mut self) {
        // EN_closeH is safe to call even if not opened — it's a no-op.
        // We ignore the error because we're in Drop.
        unsafe { ffi::EN_closeH(self.project.ph); }
    }
}
