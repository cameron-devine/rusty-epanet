use std::marker::PhantomData;
use crate::EPANET;

struct Closed;
struct Initialized;
struct Running;
struct Solved;
pub struct HydraulicSolver<State = Closed> {
    pub ph: EPANET,
    pub next_step: f64,
    pub current_time: f64,
    state: PhantomData<State>
}

impl HydraulicSolver<Closed> {
    pub fn solve(self) -> HydraulicSolver<Solved> {
        //EN_solveH
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Solved>
        }
    }

    pub fn init(self) -> HydraulicSolver<Initialized> {
        //EN_openH
        //EN_initH
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Initialized>
        }
    }
}

impl HydraulicSolver<Solved> {
    pub fn save(self) {

    }

    pub fn close(self) -> HydraulicSolver<Closed>{
        HydraulicSolver {
            ph: self.ph,
            next_step: 0.0,
            current_time: 0.0,
            state: PhantomData::<Closed>
        }
    }
}