use epanet::types::link::Link;
use epanet::types::node::{Node, NodeProperty};
use epanet::types::link::LinkProperty;
use epanet::types::options::{FlowUnits, HeadLossType, TimeParameter};
use epanet::types::CountType;
use epanet::EPANET;
use std::sync::atomic::{AtomicU32, Ordering};

static TEST_COUNTER: AtomicU32 = AtomicU32::new(0);

fn temp_rpt_path() -> String {
    let id = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    let dir = std::env::temp_dir();
    dir.join(format!("epanet_integration_{id}.rpt"))
        .to_string_lossy()
        .into_owned()
}

#[test]
fn test_build_solve_read_results() {
    // 1. Create empty project
    let rpt = temp_rpt_path();
    let ph = EPANET::new(&rpt, "", FlowUnits::Gpm, HeadLossType::HazenWilliams)
        .expect("Failed to create project");

    // 2. Add nodes
    let _r1 = Node::new_reservoir(&ph, "R1", 100.0).unwrap();
    let _j1 = Node::new_junction(&ph, "J1", 50.0, 100.0, "").unwrap();
    let _j2 = Node::new_junction(&ph, "J2", 40.0, 50.0, "").unwrap();

    // 3. Add links
    let _p1 = Link::new_pipe(&ph, "P1", "J1", "J2", 1000.0, 12.0, 100.0, 0.0).unwrap();
    let _pmp1 = Link::new_pump(&ph, "PMP1", "R1", "J1", 75.0, 1.0, None).unwrap();

    // 4. Set time parameters
    ph.set_time_parameter(TimeParameter::Duration, 3600).unwrap();
    ph.set_time_parameter(TimeParameter::HydStep, 300).unwrap();

    // 5. Solve hydraulics
    ph.solve_h().expect("Failed to solve hydraulics");

    // 6. Read pressures
    let j1_index = ph.get_node_index("J1").unwrap();
    let j2_index = ph.get_node_index("J2").unwrap();
    let p_j1 = ph.get_node_value(j1_index, NodeProperty::Pressure).unwrap();
    let p_j2 = ph.get_node_value(j2_index, NodeProperty::Pressure).unwrap();
    assert!(p_j1 > 0.0, "J1 pressure should be positive, got {}", p_j1);
    assert!(p_j2 > 0.0, "J2 pressure should be positive, got {}", p_j2);

    // 7. Read flow
    let p1_index = ph.get_link_index("P1").unwrap();
    let flow = ph.get_link_value(p1_index, LinkProperty::Flow).unwrap();
    assert!(flow > 0.0, "P1 flow should be positive, got {}", flow);

    // 8. Save and re-open
    let tmp_path = "integration_test.inp";
    ph.save_inp_file(tmp_path).unwrap();
    let rpt2 = temp_rpt_path();
    let ph2 = EPANET::with_inp_file(tmp_path, &rpt2, "").unwrap();
    assert_eq!(ph2.get_count(CountType::NodeCount).unwrap(), 3);

    // 9. Clean up
    let _ = std::fs::remove_file(tmp_path);
}
