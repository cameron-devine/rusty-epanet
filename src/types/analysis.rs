use crate::bindings::*;
use crate::EPANET;
use enum_primitive::*;
use std::marker::PhantomData;

enum_from_primitive! {
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum InitHydOption {
    NoSave = EN_InitHydOption_EN_NOSAVE, // Don't save hydraulics; don't re-initialize flows
    Save = EN_InitHydOption_EN_SAVE, // Save hydraulics to file, don't re-initialize flows
    InitFlow = EN_InitHydOption_EN_INITFLOW, // Don't save hydraulics; re-initialize flows
    SaveAndInit = EN_InitHydOption_EN_SAVE_AND_INIT, // Save hydraulics; re-initialize flows
}}

struct Closed;
struct Initialized;
struct Running;
struct Solved;
pub struct HydraulicSolver<State = Closed> {
    pub ph: EPANET,
    pub next_step: f64,
    pub current_time: f64,
    state: PhantomData<State>,
}

impl HydraulicSolver<Closed> {
    pub fn solve(self) -> HydraulicSolver<Solved> {
        //EN_solveH
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Solved>,
        }
    }

    pub fn init(self) -> HydraulicSolver<Initialized> {
        //EN_openH
        //EN_initH
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Initialized>,
        }
    }
}

impl HydraulicSolver<Solved> {
    pub fn save(self) {}

    pub fn close(self) -> HydraulicSolver<Closed> {
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Closed>,
        }
    }
}
