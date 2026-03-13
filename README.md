# Rusty EPANET

> **Status: Early Development (v0.1.0)**

Safe Rust bindings to the [EPANET 2.2](https://github.com/USEPA/EPANET) C library for water distribution network modeling and simulation. The EPANET source is included as a git submodule, compiled via CMake at build time, and exposed through `bindgen`-generated FFI bindings. A high-level `EPANET` struct wraps the raw C API with automatic resource cleanup, and domain structs (`Node`, `Link`, `Pattern`, etc.) provide an RAII layer for working with model objects.

## Prerequisites

- **CMake** and a **C compiler** (MSVC on Windows, GCC/Clang on Linux/macOS)
- **libclang** (for bindgen)
- **Rust** (stable)

## Installing

Clone with the submodule:

```bash
git clone --recurse-submodules https://github.com/<your-org>/rusty-epanet.git
```

If you already cloned without the submodule:

```bash
git submodule update --init --recursive
```

## Building and Running Tests

```bash
cargo build    # compiles EPANET C lib via cmake, generates FFI bindings via bindgen
cargo test     # runs ~120 tests across unit, integration, and doc tests
```

## Quick Start

### Open an Existing Network

```rust
use epanet::EPANET;
use epanet::types::node::NodeProperty;

fn main() -> epanet::epanet_error::Result<()> {
    let ph = EPANET::with_inp_file("net1.inp", "", "")?;

    // Query a node using the C-style wrapper
    let idx = ph.get_node_index("11")?;
    let pressure = ph.get_node_value(idx, NodeProperty::Pressure)?;
    println!("Pressure at node 11: {:.2}", pressure);

    Ok(())
    // ph is dropped here: EN_close + EN_deleteproject are called automatically
}
```

### Build a Network from Scratch

```rust
use epanet::EPANET;
use epanet::types::node::Node;
use epanet::types::link::Link;
use epanet::types::options::{FlowUnits, HeadLossType, TimeParameter};

fn main() -> epanet::epanet_error::Result<()> {
    let ph = EPANET::new("", "", FlowUnits::Gpm, HeadLossType::HazenWilliams)?;

    // Add nodes using RAII structs
    let _reservoir = Node::new_reservoir(&ph, "R1", 100.0)?;
    let _j1 = Node::new_junction(&ph, "J1", 50.0, 100.0, "")?;
    let _j2 = Node::new_junction(&ph, "J2", 40.0, 50.0, "")?;

    // Add links
    let _pipe = Link::new_pipe(&ph, "P1", "J1", "J2", 1000.0, 12.0, 100.0, 0.0)?;
    let _pump = Link::new_pump(&ph, "PMP1", "R1", "J1", 75.0, 1.0, None)?;

    // Configure and solve
    ph.set_time_parameter(TimeParameter::Duration, 3600)?;
    ph.solve_h()?;

    // Save the model
    ph.save_inp_file("my_network.inp")?;
    Ok(())
}
```

## Two Levels of API

The library offers two ways to interact with every part of the EPANET model. You can freely mix both styles in the same program.

### C-Style Wrappers (Index-Based)

Every `EN_*` function in the C API has a corresponding method on the `EPANET` struct. These are thin wrappers that handle FFI string conversion and error checking, but otherwise mirror the C API exactly. You work with 1-based integer indices and property enums.

```rust
// Add a node, get its index back
let idx = ph.add_node("J3", NodeType::Junction)?;
ph.set_node_value(idx, NodeProperty::Elevation, 150.0)?;
ph.set_junction_data(idx, 150.0, 200.0, "")?;

// Query properties by index
let elev = ph.get_node_value(idx, NodeProperty::Elevation)?;

// Batch operations
let all_pressures = ph.get_node_values(NodeProperty::Pressure)?;

// Delete by index
ph.delete_node(idx, ActionCodeType::Unconditional)?;
```

This level is best when you need precise control, are porting existing C/Python EPANET code, or are doing bulk operations where the overhead of constructing domain structs isn't worthwhile.

### RAII Domain Structs (High-Level)

Domain structs (`Node`, `Link`, `Control`, `Curve`, `Pattern`, `Demand`, `Rule`) borrow the `EPANET` project and cache field values locally. They provide typed constructors, mutable access to fields, and an `update()`/`delete()` lifecycle.

```rust
use epanet::types::node::Node;

// Create a junction — immediately added to the C model
let mut node = Node::new_junction(&ph, "J3", 150.0, 200.0, "")?;

// Read typed data
let junc = node.as_junction().unwrap();
println!("Elevation: {}, Demand: {}", junc.elevation, junc.demand);

// Modify cached fields
if let Some(junc) = node.as_junction_mut() {
    junc.elevation = 175.0;
    junc.demand = 250.0;
}

// Push changes back to the C engine
node.update()?;

// Or retrieve an existing object from the model
let existing = ph.get_node("11")?;

// Consuming delete — removes from the C model and drops the struct
node.delete(ActionCodeType::Unconditional)?;
```

Each domain struct follows the same pattern:

| Struct | Constructors | Type-Specific Data |
|--------|-------------|-------------------|
| `Node` | `new_junction`, `new_reservoir`, `new_tank` | `JunctionData`, `ReservoirData`, `TankData` via `NodeKind` enum |
| `Link` | `new_pipe`, `new_pump`, `new_valve` | `PipeData`, `PumpData`, `ValveData` via `LinkKind` enum |
| `Control` | `new_lowlevel`, `new_hilevel`, `new_timer`, `new_timeofday` | `ControlType`, link/node indices, setting, level |
| `Curve` | `new_pump_curve`, `new_volume_curve`, `new_efficiency_curve`, `new_headloss_curve`, `new_generic_curve` | `CurveType`, points as `Vec<(f64, f64)>` |
| `Pattern` | `new` | ID, multipliers as `Vec<f64>` |
| `Demand` | `new` | Node index, base demand, pattern, name |
| `Rule` | `new` | Premises, then-actions, else-actions, priority |

### The `update()` / `delete()` Pattern

Domain structs are **snapshots** of the C engine state at the time they are created. Modifying their public fields changes only the Rust-side cache. You must call `.update()` to push changes back to the C engine.

```rust
let mut pipe = ph.get_link("P1")?;

// This only changes the Rust struct, NOT the C model
if let Some(data) = pipe.as_pipe_mut() {
    data.roughness = 120.0;
    data.diameter = 16.0;
}

// This writes the changes to the C model
pipe.update()?;

// Verify the round-trip
let pipe2 = ph.get_link("P1")?;
assert_eq!(pipe2.as_pipe().unwrap().roughness, 120.0);
```

`delete(self)` is a consuming method — it takes ownership of the struct, removes the object from the C model, and drops the struct so it can't be used afterward.

```rust
let curve = Curve::new_pump_curve(&ph, "C1", &[(0.0, 300.0), (150.0, 200.0)])?;
// ... use the curve ...
curve.delete()?;  // Removed from the model; `curve` is no longer accessible
```

### Collection Methods

Fetch all objects of a given type as a `Vec` of domain structs:

```rust
let nodes = ph.nodes()?;           // All nodes
let junctions = ph.junctions()?;   // Just junctions
let tanks = ph.tanks()?;           // Just tanks

let links = ph.links()?;           // All links
let pipes = ph.pipes()?;           // Just pipes
let pumps = ph.pumps()?;           // Just pumps
let valves = ph.valves()?;         // Just valves

let patterns = ph.patterns()?;
let curves = ph.curves()?;
let controls = ph.controls()?;
let rules = ph.rules()?;
```

## Solvers

### Hydraulic Analysis

The hydraulic solver uses a typestate pattern to enforce correct call ordering at compile time. Invalid transitions (e.g., calling `run()` before `init()`) are caught by the Rust compiler.

```rust
use epanet::types::analysis::{InitHydOption, StepResult};

let ph = EPANET::with_inp_file("net1.inp", "", "")?;

// Option 1: One-shot solve
ph.solve_h()?;

// Option 2: Typestate solver for step-by-step control
let solver = ph.hydraulic_solver()
    .init(InitHydOption::Save)?
    .run()?;

loop {
    match solver.next()? {
        StepResult::Continue { current_time, next_step } => {
            // Read results mid-simulation
            let pressure = ph.get_node_value(1, NodeProperty::Pressure)?;
            println!("t={}: pressure={:.2}", current_time, pressure);
        }
        StepResult::Done { current_time } => {
            println!("Simulation complete at t={}", current_time);
            solver.close()?;
            break;
        }
    }
}
```

### Water Quality Analysis

Follows the same pattern, but requires hydraulics to be solved first:

```rust
// Solve hydraulics first
ph.hydraulic_solver().solve()?.save()?;

// Then quality
ph.quality_solver().solve()?;
```

## Callbacks

### Report Callback

Instead of writing report output to a file, you can register a closure to intercept each line:

```rust
use std::sync::{Arc, Mutex};

let mut ph = EPANET::with_inp_file("net1.inp", "", "")?;

// Collect report lines in a thread-safe vector
let lines = Arc::new(Mutex::new(Vec::new()));
let lines_clone = Arc::clone(&lines);

ph.set_report_callback(Some(Box::new(move |line: &str| {
    lines_clone.lock().unwrap().push(line.to_string());
})))?;

// Any operation that generates report output will invoke the callback
ph.solve_h()?;

// Check what was captured
let captured = lines.lock().unwrap();
for line in captured.iter() {
    println!("{}", line);
}

// Remove the callback to revert to file-based reporting
ph.set_report_callback(None)?;
```

The callback is automatically freed when the `EPANET` instance is dropped or when a new callback is registered.

### Run Project with Progress Callback

For one-shot simulations, `run_project` and `run_project_with_callback` are standalone functions that create their own project handle, run the full simulation, and clean up:

```rust
use epanet::run_project_with_callback;

run_project_with_callback(
    "net1.inp",
    "report.rpt",
    "",
    |msg| println!("Progress: {}", msg),
)?;
```

These are standalone functions (not methods on `EPANET`) because `EN_runproject` internally opens and closes the project. See [Caveats](#en_runproject-and-project-lifecycle) below.

## Caveats

### 1-Based Indexing

All indices from the EPANET C API are **1-based**. The Rust wrappers preserve this convention. Index 0 is never valid.

```rust
let first_node = ph.get_node_by_index(1)?;  // First node, not zeroth
```

### `EN_runproject` and Project Lifecycle

The C function `EN_runproject` calls `EN_close` internally after the simulation completes, which frees all network data. Calling `EN_close` again (e.g., in `Drop`) would cause a double-free.

This library handles this in two ways:
1. `run_project()` and `run_project_with_callback()` are **standalone functions** that create and manage their own project handle, so they never conflict with an existing `EPANET` instance.
2. The `EPANET` struct tracks whether the project has been closed via an internal flag, and `Drop` skips `EN_close` if it has already been called.

### Domain Struct Lifetimes

All domain structs (`Node`, `Link`, `Control`, etc.) borrow the `EPANET` instance. This means you cannot drop or move the `EPANET` while any domain struct is alive:

```rust
let ph = EPANET::with_inp_file("net1.inp", "", "")?;
let node = ph.get_node("11")?;  // borrows ph

// ph cannot be dropped here because node holds a reference to it
println!("{}", node.id);

drop(node);  // now ph can be dropped
```

### Snapshot Semantics

Domain structs cache field values at construction time. If the C model changes after a struct is created (e.g., via direct C-wrapper calls or another struct's `update()`), the cached values become stale. Re-fetch the struct to get current values.

Live computed results (`pressure()`, `flow()`, `head_loss()`, etc.) always query the C engine directly and are never stale.

### Thread Safety

`EPANET` implements `Send` and `Sync`, but the underlying C library may use global state in some configurations. External synchronization (e.g., a `Mutex`) is recommended for concurrent access to the same project.

## Architecture

```
src/
  lib.rs              # EPANET struct (owns EN_Project handle), Drop, Send+Sync, constructors
  bindings.rs         # include!() of bindgen output
  epanet_error.rs     # EPANETError, Result<T>, check_error()
  error_messages.rs   # Generated: error code -> &'static str
  types/              # Enums, domain structs, and type definitions
    analysis.rs       # Typestate solvers (HydraulicSolver, QualitySolver)
    node.rs           # Node struct, NodeKind enum, JunctionData/TankData/ReservoirData
    link.rs           # Link struct, LinkKind enum, PipeData/PumpData/ValveData
    control.rs        # Control struct, ControlType enum
    curve.rs          # Curve struct, CurveType enum
    pattern.rs        # Pattern struct
    demand.rs         # Demand struct, DemandModel enum
    rule.rs           # Rule struct, rule enums
    options.rs        # FlowUnits, HeadLossType, QualityType, TimeParameter, Option enums
    report.rs         # ReportCallback type, trampoline function
  impls/              # impl EPANET blocks organized by domain
    project.rs        # Title, count, comment, save_inp_file, run_project (standalone)
    node.rs           # Node CRUD, property get/set, batch values
    link.rs           # Link CRUD, property get/set, vertices, pump/pipe specifics
    hydraulic.rs      # Hydraulic solver lifecycle
    quality.rs        # Water quality solver lifecycle
    options.rs        # Flow units, time params, quality type, analysis options
    control.rs        # Simple control CRUD
    curve.rs          # Curve CRUD
    pattern.rs        # Time pattern CRUD
    demand.rs         # Demand model and demand management
    report.rs         # Report generation, statistics, callbacks
    rule.rs           # Rule-based control CRUD
    collections.rs    # Bulk fetch methods (nodes(), links(), pipes(), etc.)
tests/
  integration.rs      # End-to-end: build network from scratch, solve, verify results
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| `num-traits` / `num-derive` | `FromPrimitive` for C enum conversion |
| `rstest` (dev) | Fixture-based test framework |
| `strum` / `strum_macros` (dev) | Enum iteration in tests |
| `bindgen` (build) | C header -> Rust FFI bindings |
| `cmake` (build) | Build EPANET C library |

## Additional Resources

- [EPANET 2.2 Programmer's Toolkit](https://wateranalytics.org/EPANET/_toolkit_page.html)
- [Using C Libraries in Rust](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a)
