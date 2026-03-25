//! [`EPANET`](crate::EPANET) method implementations, organized by domain.
//!
//! Each submodule contains one or more `impl EPANET` blocks covering a specific
//! part of the EPANET C API. Methods are called directly on an [`EPANET`](crate::EPANET)
//! instance — there is no need to import these modules explicitly.
//!
//! | Submodule | API surface |
//! |---|---|
//! | [`project`] | Title, object counts, comments, tags, save input file, [`run_project`](project::run_project) |
//! | [`node`] | Node CRUD, property get/set, coordinates, junction/tank/reservoir data |
//! | [`link`] | Link CRUD, property get/set, vertices, pipe/pump/valve specifics |
//! | [`hydraulic`] | Hydraulic solver lifecycle: open / init / run / next / save / close |
//! | [`quality`] | Water quality solver lifecycle: open / init / run / step / next / close |
//! | [`control`] | Simple control CRUD |
//! | [`curve`] | Curve CRUD |
//! | [`demand`] | Demand model settings and demand category management |
//! | [`options`] | Flow units, head loss formula, time parameters, analysis options |
//! | [`pattern`] | Time pattern CRUD |
//! | [`report`] | Report generation, report file output, statistics, error lookup |
//! | [`rule`] | Rule-based control CRUD |
//! | [`collections`] | Bulk fetch: nodes, links, pipes, pumps, valves, patterns, curves, controls, rules |

pub mod collections;
pub mod control;
pub mod curve;
pub mod demand;
pub mod hydraulic;
pub mod link;
pub mod node;
pub mod options;
pub mod pattern;
pub mod project;
pub mod quality;
pub mod report;
pub mod rule;
pub mod test_utils;
