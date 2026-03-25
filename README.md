# Rusty EPANET

> **Status: Early Development (v0.2.1)**

Safe Rust bindings to the [EPANET 2.3](https://github.com/OpenWaterAnalytics/EPANET) C library for water distribution network modeling and simulation. The EPANET C library is compiled at build time via [`epanet-sys`](https://crates.io/crates/epanet-sys). A high-level `EPANET` struct wraps the raw C API with automatic resource cleanup, and domain structs (`Node`, `Link`, `Pattern`, etc.) provide an RAII layer for working with model objects.

## Prerequisites

- **CMake** and a **C compiler** (MSVC on Windows, GCC/Clang on Linux/macOS)
- **libclang** (for bindgen)
- **Rust** (stable)

## Installing

```toml
[dependencies]
epanet = "0.2"
```

## Building and Running Tests

```bash
cargo build    # epanet-sys compiles the EPANET C library via CMake
cargo test     # runs ~120 tests across unit, integration, and doc tests
```

By default, the EPANET C library is **statically linked** into your binary — no DLL/SO distribution needed.

## Linking

The crate defaults to **static linking** (`static-link` feature), which embeds the EPANET C library directly into your binary. This is the recommended approach — no shared libraries to distribute or locate at runtime.

To use **dynamic linking** instead (e.g., to swap EPANET versions without recompiling):

```bash
cargo build --features dynamic-link --no-default-features
```

Or in your `Cargo.toml`:

```toml
[dependencies]
epanet = { version = "0.3", default-features = false, features = ["dynamic-link"] }
```

With dynamic linking, you must ensure `epanet2.dll` (Windows), `libepanet2.so` (Linux), or `libepanet2.dylib` (macOS) is on the library search path at runtime.

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

The `EPANET::solver()` entry point returns a `Solver<HClosed>` whose type parameter encodes the current simulation state. Invalid call sequences (e.g. stepping before initializing) are caught by the Rust compiler.

### Hydraulic Analysis

```rust
use epanet::types::analysis::{InitHydOption, StepResult};

let ph = EPANET::with_inp_file("net1.inp", "", "")?;

// Option 1: One-shot solve
let solver = ph.solver().solve_h()?;
solver.save()?;

// Option 2: Step-by-step
let mut solver = ph.solver()
    .init_h(InitHydOption::Save)?
    .run_h()?;

loop {
    match solver.next_h()? {
        StepResult::Continue { current_time, next_step } => {
            // Read results mid-simulation via solver.project()
            let pressure = solver.project().get_node_value(1, NodeProperty::Pressure)?;
            println!("t={}: pressure={:.2}", current_time, pressure);
        }
        StepResult::Done { current_time } => {
            println!("Simulation complete at t={}", current_time);
            solver.close_h()?;
            break;
        }
    }
}
```

### Water Quality Analysis

Requires hydraulics to be solved first. Can be chained directly from a completed hydraulic solver:

```rust
// One-shot: solve hydraulics then quality
let hyd = ph.solver().solve_h()?;
hyd.save()?;
hyd.solve_q()?;

// Step-by-step quality after one-shot hydraulics
let mut qual = ph.solver()
    .solve_h()?
    .init_q(InitHydOption::Save)?
    .run_q()?;

loop {
    match qual.step_q()? {
        StepResult::Continue { .. } => {}
        StepResult::Done { .. } => { qual.close_q()?; break; }
    }
}
```

### Simultaneous Hydraulic + Quality

```rust
let mut solver = ph.solver()
    .init_h(InitHydOption::NoSave)?
    .init_q(InitHydOption::NoSave)?
    .run()?;

loop {
    match solver.next()? {
        StepResult::Continue { .. } => {}
        StepResult::Done { .. } => { solver.close()?; break; }
    }
}
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

`EPANET` implements `Send` but **not** `Sync`. Each project handle can be moved to another thread, but it cannot be shared concurrently via `&EPANET` because the underlying C library uses internal mutable state (e.g., shared message buffers, `strtok()`) that is not safe for concurrent access.

To share an `EPANET` instance across threads, wrap it in `Arc<Mutex<EPANET>>`:

```rust
use std::sync::{Arc, Mutex};

let ph = EPANET::with_inp_file("net1.inp", "", "")?;
let shared = Arc::new(Mutex::new(ph));

let shared_clone = Arc::clone(&shared);
std::thread::spawn(move || {
    let ph = shared_clone.lock().unwrap();
    let pressure = ph.get_node_value(1, NodeProperty::Pressure).unwrap();
    println!("Pressure: {:.2}", pressure);
});
```

Separate `EPANET` instances (different projects) can safely run on different threads without any synchronization.

## Architecture

```
src/
  lib.rs              # EPANET struct (owns EN_Project handle), Drop, Send, constructors
  bindings.rs         # re-exports from epanet-sys
  epanet_error.rs     # EPANETError, Result<T>, check_error()
  error_messages.rs   # Static error code -> &'static str lookup
  types/              # Enums, domain structs, and type definitions
    analysis.rs       # Unified typestate Solver<S> (HClosed → HRunning → HydDone → QRunning …)
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
| `epanet-sys` | Raw FFI bindings and EPANET C compilation |
| `num-traits` / `num-derive` | `FromPrimitive` for C enum conversion |
| `rstest` (dev) | Fixture-based test framework |
| `strum` / `strum_macros` (dev) | Enum iteration in tests |

### Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `static-link` | Yes | Statically link EPANET (self-contained binary) |
| `dynamic-link` | No | Dynamically link EPANET (requires shared library at runtime) |

## Additional Resources

- [EPANET 2.3 Programmer's Toolkit](https://wateranalytics.org/EPANET/_toolkit_page.html)
- [Using C Libraries in Rust](https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a)
