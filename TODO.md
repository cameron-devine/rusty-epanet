# TODO - rusty-epanet

## Missing C API Wrappers

Functions available in `bindings.rs` but not yet wrapped with safe Rust methods.

### Project / Network

- [x] `EN_setcomment` - Set object comment (get exists, set missing)
- [ ] `EN_openX` - Open project with extended error reporting
- [x] `EN_gettag` / `EN_settag` - Object tagging (commented out in project.rs, noted as missing from bindings - investigate)

### Node

- [x] `EN_setjuncdata` - Set junction parameters (elevation, demand, demand pattern) in one call
- [x] `EN_settankdata` - Set tank parameters (elevation, init level, min/max level, diameter, min volume, volume curve) in one call
- [x] `EN_getcoord` / `EN_setcoord` - Node coordinate (x, y) get/set
- [x] `EN_getnumdemands` - Get number of demand categories for a node (used internally in demand.rs but not public)

### Link

- [x] `EN_addlink` - Create a new link (delete exists, add missing)

### Demand

- [x] `EN_setdemandname` - Set demand category name (get exists, set missing)

### Pattern

- [x] `EN_getpatternindex` - Get pattern index by ID
- [x] `EN_loadpatternfile` - Load patterns from file (called in impls but may not be publicly exposed)

### Curve

- [x] `EN_getcurvevalue` / `EN_setcurvevalue` - Individual point get/set (bulk get/set exists via `EN_getcurve`/`EN_setcurve`)

### Rule

- [x] `EN_setpremise` - Set all properties of a premise
- [x] `EN_setpremiseindex` - Set object index of a premise
- [x] `EN_setpremisestatus` - Set status of a premise
- [x] `EN_setpremisevalue` - Set value of a premise
- [x] `EN_setthenaction` - Set a then-action clause
- [x] `EN_setelseaction` - Set an else-action clause
- [x] `EN_setrulepriority` - Set rule priority
- [x] `EN_setruleenabled` - Enable/disable a rule (get exists, set missing)

### Report

- [x] `EN_writeline` - Write a line to the report file
- [x] `EN_setstatusreport` - Set level of hydraulic status reporting
- [x] `EN_timetonextevent` - Get time until next event (exists in bindings, not wrapped)

## Typestate Solvers

The solvers should **borrow** `&'a EPANET` (not own it), so the project remains accessible for reading results mid-simulation via domain structs. The current `HydraulicSolver` owns the project and needs to be rewritten.

### HydraulicSolver (rewrite types/analysis.rs)

- [x] Change `HydraulicSolver` from owning `pub ph: EPANET` to borrowing `&'a EPANET`
- [x] Add `EPANET::hydraulic_solver(&self) -> HydraulicSolver<'_, Closed>` entry point
- [x] Wire `Closed::solve()` -> `Solved` via `EN_solveH` (return `Result`)
- [x] Wire `Closed::init(InitHydOption)` -> `Initialized` via `EN_openH` + `EN_initH` (return `Result`)
- [x] Add `Initialized::run()` -> `(Running, f64)` via `EN_runH` (returns current time)
- [x] Add `Running::next()` -> `Result<StepResult>` via `EN_nextH` + `EN_runH`
- [x] Define `StepResult<'a>` enum: `Continue(HydraulicSolver<Running>)` | `Done(HydraulicSolver<Solved>)` to encode simulation completion in the type system
- [x] Wire `Solved::save()` via `EN_saveH`
- [x] Wire `Solved::close()` via `EN_closeH`
- [x] Add `project(&self) -> &EPANET` accessor on `Running` and `Solved` states for reading results mid-simulation
- [x] Implement `Drop` for all states to call `EN_closeH` as safety net
- [x] All state transitions return `Result<>`

### QualitySolver (new, same pattern)

- [x] Define `QualitySolver<'a, State>` borrowing `&'a EPANET` with states: `Closed`, `Initialized`, `Running`, `Solved`
- [x] Add `EPANET::quality_solver(&self) -> QualitySolver<'_, Closed>` entry point
- [x] `Closed::solve()` -> `Solved` via `EN_solveQ`
- [x] `Closed::init()` -> `Initialized` via `EN_openQ` + `EN_initQ`
- [x] `Initialized::run()` -> `Running` via `EN_runQ`
- [x] `Running::next()` -> `StepResult` via `EN_nextQ` (or `EN_stepQ`)
- [x] `Solved::close()` -> drop via `EN_closeQ`
- [x] `Drop` impl calls `EN_closeQ`

## Domain Structs

Structs are **views** into the C engine state, not owners of data. Use enum-based type discrimination (not composition/inheritance). This replaces the current `Control<'a>` / `Curve<'a>` composition pattern.

### Design principles
- One struct per domain concept (`Node<'a>`, `Link<'a>`) with a `kind` enum for type-specific data
- Type-specific fields live in plain data structs (`PipeData`, `PumpData`, etc.) inside the enum
- Shared behavior (live result queries, `update()`, `delete()`) lives on the outer struct
- Convenience accessors: `as_pipe()`, `as_pump()`, `is_pipe()`, etc. return `Option<&Data>`
- `update()` pushes cached fields back to C; live results (`flow()`, `pressure()`) always query C directly

### Node

- [x] `Node<'a>` struct: `&'a EPANET`, index, id, `kind: NodeKind`
- [x] `NodeKind` enum: `Junction(JunctionData)`, `Tank(TankData)`, `Reservoir(ReservoirData)`
- [x] `JunctionData`: elevation, base_demand, demand_pattern, emitter_coeff, init_quality
- [x] `TankData`: elevation, init_level, min_level, max_level, diameter, min_volume, volume_curve
- [x] `ReservoirData`: total_head, head_pattern, init_quality
- [x] `Node::update()` - pushes cached fields back to C via `EN_setnodevalue` / `EN_setjuncdata` / `EN_settankdata`
- [x] `Node::delete(self, ActionCodeType)` - consuming delete via `EN_deletenode`
- [x] Live result methods: `pressure()`, `head()`, `demand()`, `quality()` - always query C
- [x] Convenience: `as_junction()`, `as_tank()`, `as_reservoir()`, `is_junction()`, etc.
- [x] `EPANET::get_node_by_index(i32) -> Result<Node>` constructor (fetches all fields from C)
- [x] `EPANET::get_node(id: &str) -> Result<Node>` constructor (resolves ID first)

### Link

- [x] `Link<'a>` struct: `&'a EPANET`, index, id, from_node, to_node, status, `kind: LinkKind`
- [x] `LinkKind` enum: `Pipe(PipeData)`, `CvPipe(PipeData)`, `Pump(PumpData)`, `Valve(ValveData)`
- [x] `PipeData`: length, diameter, roughness, minor_loss
- [x] `PumpData`: pump_type, power, speed, head_curve_index, efficiency_curve_index, energy_pattern_index, energy_cost
- [x] `ValveData`: valve_type (`ValveType` enum: Prv/Psv/Pbv/Fcv/Tcv/Gpv/Pcv), diameter, setting, curve_index
- [x] `Link::update()` - dispatches on `kind` to call `EN_setpipedata` / `EN_setlinkvalue` etc.
- [x] `Link::delete(self, ActionCodeType)` - consuming delete via `EN_deletelink`
- [x] Live result methods: `flow()`, `velocity()`, `head_loss()`, `quality()`
- [x] Convenience: `as_pipe()`, `as_pipe_mut()`, `as_pump()`, `as_pump_mut()`, `as_valve()`, `is_pipe()`, etc.
- [x] `EPANET::get_link_by_index(i32) -> Result<Link>` constructor
- [x] `EPANET::get_link(id: &str) -> Result<Link>` constructor

### Refactor Control and Curve to match

- [x] Refactor `Control<'a>` to use the same pattern (struct owns data, `update()`/`delete(self)` call FFI directly instead of delegating to `project.update_control()`)
- [x] Refactor `Curve<'a>` to match

### Pattern

- [x] `Pattern<'a>` struct: `&'a EPANET`, index, id, cached multipliers
- [x] `update()` / `delete(self)` methods

### Demand

- [x] `Demand<'a>` struct: `&'a EPANET`, node_index, demand_category_index, base_demand, pattern, name

## Collection / Iterator Support

- [x] `EPANET::nodes(&self) -> Result<Vec<Node>>` - fetch all nodes
- [x] `EPANET::links(&self) -> Result<Vec<Link>>` - fetch all links
- [x] `EPANET::pipes(&self) -> Result<Vec<Link>>` - filtered convenience (links where `is_pipe()`)
- [x] `EPANET::pumps(&self) -> Result<Vec<Link>>` - filtered convenience
- [x] `EPANET::valves(&self) -> Result<Vec<Link>>` - filtered convenience
- [x] `EPANET::junctions(&self) -> Result<Vec<Node>>` - filtered convenience
- [x] `EPANET::tanks(&self) -> Result<Vec<Node>>` - filtered convenience
- [x] `EPANET::patterns(&self) -> Result<Vec<Pattern>>` - fetch all patterns
- [x] `EPANET::curves(&self) -> Result<Vec<Curve>>` - fetch all curves
- [x] `EPANET::controls(&self) -> Result<Vec<Control>>` - fetch all controls
- [x] `EPANET::rules(&self) -> Result<Vec<Rule>>` - fetch all rules

## API Ergonomics

- [x] Make `add_link` public and wrap properly (currently no `EN_addlink` wrapper)
- [x] `run_project` callback: provide a safe closure wrapper instead of `unsafe extern "C" fn`
- [x] Consider `&str` -> index resolution helpers: many APIs take indices but users think in IDs
- [x] `with_inp_file_allow_errors` currently has same implementation as `with_inp_file` - differentiate by handling warning-level error codes (codes 1-99)
- [x] Expose `EN_getpatternindex` publicly (pattern module has add/delete/get by index but no index-by-ID lookup)

## Error Handling

- [ ] Distinguish warnings (codes 1-99) from errors (codes >= 100) in `EPANETError`
- [ ] Consider `EPANETWarning` type or `check_error` variant that allows warnings through
- [ ] Add `EPANETError::is_warning()` / `is_error()` helpers
- [x] Standardize error checking pattern: some wrappers use `check_error`/`check_error_with_context`, others use manual `if result == 0` / `match` blocks - pick one and be consistent

## Testing

- [ ] Add tests for link operations (currently no link test module)
- [ ] Add tests for pattern operations
- [ ] Add tests for demand operations
- [ ] Add tests for report operations
- [ ] Add tests for project title and comment operations
- [ ] Add integration test: build a network from scratch, solve, read results
- [ ] Consider property-based testing with `proptest` for enum round-trips
- [ ] Test thread safety claims (Send + Sync) - EPANET C library may not actually be thread-safe across projects

## Code Quality

- [x] Replace `enum_primitive` (unmaintained, last release 2016) with `num_derive`/`num_traits` or `strum` (already a dev-dep)
- [ ] Add `#[must_use]` to Result-returning methods
- [ ] Add `clippy` configuration and address any warnings
- [ ] Consider `thiserror` for `EPANETError` derive instead of manual `Display`/`Error` impls
- [ ] Add `#[non_exhaustive]` to public enums for future compatibility
- [ ] Audit `Send + Sync` safety: EPANET C library uses global state in some configurations

## Documentation

- [ ] Add crate-level docs in `lib.rs` with usage examples
- [ ] Add module-level docs for `types/` and `impls/`
- [ ] Ensure all public methods have doc comments (link.rs methods are mostly undocumented)
- [ ] Add `# Examples` sections to frequently-used methods
- [ ] Consider `doc = include_str!("../README.md")` for crate docs

## Build / CI

- [ ] Add CI workflow (GitHub Actions) for build + test on Linux/Windows/macOS
- [ ] Consider static linking option (`cargo:rustc-link-lib=static=epanet2`) to avoid DLL distribution
- [ ] Add cargo features for optional static vs dynamic linking
- [x] Investigate why `EN_gettag`/`EN_settag` are missing from bindgen output (may need wrapper.h update)
