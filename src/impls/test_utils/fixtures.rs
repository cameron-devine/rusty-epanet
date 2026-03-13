use crate::types::analysis::InitHydOption;
use crate::types::node::NodeType::Junction;
use crate::types::options::{FlowUnits, HeadLossType};
use crate::EPANET;
use rstest::fixture;
use std::sync::atomic::{AtomicU32, Ordering};

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

/// Returns a unique temp file path for test report output.
/// This avoids EPANET writing its banner/report to stdout.
pub fn temp_rpt_path() -> String {
    let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir();
    dir.join(format!("epanet_test_{id}.rpt"))
        .to_string_lossy()
        .into_owned()
}

pub fn approx_eq(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[fixture]
pub fn ph() -> EPANET {
    let rpt = temp_rpt_path();
    EPANET::with_inp_file("src/impls/test_utils/net1.inp", &rpt, "").expect("ERROR OPENING PROJECT")
}

#[fixture]
pub fn ph_close() -> EPANET {
    let rpt = temp_rpt_path();
    EPANET::new(&rpt, "", FlowUnits::Cfs, HeadLossType::HazenWilliams)
        .expect("ERROR CREATING PROJECT")
}

#[fixture]
pub fn after_step(ph: EPANET) -> EPANET {
    let t_stop = 10800;

    let mut result = ph.solve_h();
    assert!(result.is_ok());

    result = ph.open_q();
    assert!(result.is_ok());

    result = ph.init_q(InitHydOption::NoSave);
    assert!(result.is_ok());

    loop {
        let t = ph.run_q().expect("Failed to run quality simulation");
        let t_step = ph
            .step_q()
            .expect("Failed to step through quality simulation");
        // Intentionally silent — no println to avoid noisy test output
        if t_step <= 0 || t >= t_stop {
            break;
        }
    }
    ph
}

#[fixture]
pub fn ph_single_node(ph_close: EPANET) -> (EPANET, i32) {
    let node_id = ph_close.add_node("CUB_SCOUT_QUONSET_HUT", Junction).expect("Failed to add node");
    (ph_close, node_id)
}
