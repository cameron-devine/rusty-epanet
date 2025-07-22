use rstest::fixture;
use crate::EPANET;
use crate::types::{FlowUnits, HeadLossType, InitHydOption};

pub fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[fixture]
pub fn ph() -> EPANET {
    EPANET::with_inp_file("src/impls/test_utils/net1.inp", "", "").expect("ERROR OPENING PROJECT")
}

#[fixture]
pub fn ph_close() -> EPANET {
    EPANET::new("", "", FlowUnits::Cfs, HeadLossType::HazenWilliams).expect("ERROR CREATING PROJECT")
}

#[fixture]
pub fn after_step(mut ph: EPANET) -> EPANET {
    let t_stop = 10800;

    let mut result = ph.solve_h();
    assert!(result.is_ok());

    result = ph.open_q();
    assert!(result.is_ok());

    result = ph.init_q(InitHydOption::NoSave);
    assert!(result.is_ok());

    loop {
        let t = ph.run_q().expect("Failed to run quality simulation");
        let t_step = ph.step_q().expect("Failed to step through quality simulation");
        println!("Time: {}s, TStep: {}s", t, t_step);
        if t_step <= 0 || t >= t_stop {
            break;
        }
    }
    ph
}