# TODO - rusty-epanet

## Missing C API Wrappers

Functions available in `bindings.rs` but not yet wrapped with safe Rust methods.

### Project / Network

- [ ] `EN_setcomment` - Set object comment (get exists, set missing)
- [ ] `EN_openX` - Open project with extended error reporting
- [ ] `EN_gettag` / `EN_settag` - Object tagging (commented out in project.rs, noted as missing from bindings - investigate)

### Node

- [ ] `EN_setjuncdata` - Set junction parameters (elevation, demand, demand pattern) in one call
- [ ] `EN_settankdata` - Set tank parameters (elevation, init level, min/max level, diameter, min volume, volume curve) in one call
- [ ] `EN_getcoord` / `EN_setcoord` - Node coordinate (x, y) get/set
- [ ] `EN_getnumdemands` - Get number of demand categories for a node (used internally in demand.rs but not public)

### Link

- [ ] `EN_addlink` - Create a new link (delete exists, add missing)

### Demand

- [ ] `EN_setdemandname` - Set demand category name (get exists, set missing)

### Pattern

- [ ] `EN_getpatternindex` - Get pattern index by ID
- [ ] `EN_loadpatternfile` - Load patterns from file (called in impls but may not be publicly exposed)

### Curve

- [ ] `EN_getcurvevalue` / `EN_setcurvevalue` - Individual point get/set (bulk get/set exists via `EN_getcurve`/`EN_setcurve`)

### Rule

- [ ] `EN_setpremise` - Set all properties of a premise
- [ ] `EN_setpremiseindex` - Set object index of a premise
- [ ] `EN_setpremisestatus` - Set status of a premise
- [ ] `EN_setpremisevalue` - Set value of a premise
- [ ] `EN_setthenaction` - Set a then-action clause
- [ ] `EN_setelseaction` - Set an else-action clause
- [ ] `EN_setrulepriority` - Set rule priority
- [ ] `EN_setruleenabled` - Enable/disable a rule (get exists, set missing)

### Report

- [ ] `EN_writeline` - Write a line to the report file
- [ ] `EN_setstatusreport` - Set level of hydraulic status reporting
- [ ] `EN_timetonextevent` - Get time until next event (exists in bindings, not wrapped)

## Incomplete Typestate / Trait Patterns

### HydraulicSolver (types/analysis.rs)

The typestate pattern is structurally defined but not wired to the C API.

- [ ] Wire `HydraulicSolver<Closed>::solve()` to actually call `EN_solveH`
- [ ] Wire `HydraulicSolver<Closed>::init()` to call `EN_openH` + `EN_initH`
- [ ] Add `HydraulicSolver<Initialized>::run()` -> `HydraulicSolver<Running>` calling `EN_runH`
- [ ] Add `HydraulicSolver<Running>::next()` -> `HydraulicSolver<Running>` or terminal, calling `EN_nextH`
- [ ] Wire `HydraulicSolver<Solved>::save()` to call `EN_saveH`
- [ ] Wire `HydraulicSolver<Solved>::close()` to call `EN_closeH`
- [ ] Return `Result<>` from all state transitions (currently infallible)
- [ ] Implement `Drop` for HydraulicSolver to call `EN_closeH` if not already closed
- [ ] Decide ownership model: solver currently takes ownership of `EPANET` (`pub ph: EPANET`) - should it borrow instead?

### QualitySolver (not started)

A matching typestate pattern for the water quality solver.

- [ ] Define `QualitySolver` with states: `Closed`, `Initialized`, `Running`, `Solved`
- [ ] `Closed::init()` -> `Initialized` via `EN_openQ` + `EN_initQ`
- [ ] `Closed::solve()` -> `Solved` via `EN_solveQ`
- [ ] `Initialized::run()` -> `Running` via `EN_runQ`
- [ ] `Running::next()` / `Running::step()` via `EN_nextQ` / `EN_stepQ`
- [ ] `Solved::close()` -> `Closed` via `EN_closeQ`

## Domain Structs

Currently, nodes and links are accessed purely through index-based C function calls. A more Rust-idiomatic approach would use structs similar to `Control<'a>` and `Curve<'a>`.

### Node Structs

- [ ] `Node<'a>` base struct with `&'a EPANET`, index, ID, and NodeType
- [ ] `Junction<'a>` - wraps Node with junction-specific fields (elevation, base demand, demand pattern, emitter coeff)
- [ ] `Tank<'a>` - wraps Node with tank-specific fields (init level, min/max level, diameter, min volume, volume curve)
- [ ] `Reservoir<'a>` - wraps Node with reservoir-specific fields (head, head pattern)
- [ ] `.update()` and `.delete(self)` methods matching the Control/Curve RAII pattern
- [ ] Builder or constructor that calls `EN_setjuncdata` / `EN_settankdata` for efficient initialization

### Link Structs

- [ ] `Link<'a>` base struct with `&'a EPANET`, index, ID, LinkType, and upstream/downstream node indices
- [ ] `Pipe<'a>` - wraps Link with pipe fields (length, diameter, roughness, minor loss, status)
- [ ] `Pump<'a>` - wraps Link with pump fields (pump type, head curve, speed, power, energy pattern)
- [ ] Valve subtypes (`PRV<'a>`, `PSV<'a>`, `PBV<'a>`, `FCV<'a>`, `TCV<'a>`, `GPV<'a>`) with valve-specific settings
- [ ] `.update()` and `.delete(self)` methods matching Control/Curve RAII pattern
- [ ] `EN_addlink` wrapper for constructors

### Pattern Struct

- [ ] `Pattern<'a>` - wraps EPANET ref + index + ID + cached multipliers
- [ ] `.update()` / `.delete(self)` RAII methods

### Demand Struct

- [ ] `Demand<'a>` - wraps node ref + demand category index + base demand + pattern + name

## Iterator / Collection Support

- [ ] `NodeIterator` - iterate over all nodes yielding `Node<'a>` (or typed variants)
- [ ] `LinkIterator` - iterate over all links yielding `Link<'a>` (or typed variants)
- [ ] `PatternIterator` - iterate over patterns
- [ ] `CurveIterator` - iterate over curves
- [ ] `ControlIterator` - iterate over controls
- [ ] `RuleIterator` - iterate over rules
- [ ] Consider `IntoIterator` impls on `EPANET` for ergonomic `for node in &project.nodes()` patterns

## API Ergonomics

- [ ] Make `add_link` public and wrap properly (currently no `EN_addlink` wrapper)
- [ ] `run_project` callback: provide a safe closure wrapper instead of `unsafe extern "C" fn`
- [ ] Consider `&str` -> index resolution helpers: many APIs take indices but users think in IDs
- [ ] `with_inp_file_allow_errors` currently has same implementation as `with_inp_file` - differentiate by handling warning-level error codes (codes 1-99)
- [ ] Expose `EN_getpatternindex` publicly (pattern module has add/delete/get by index but no index-by-ID lookup)

## Error Handling

- [ ] Distinguish warnings (codes 1-99) from errors (codes >= 100) in `EPANETError`
- [ ] Consider `EPANETWarning` type or `check_error` variant that allows warnings through
- [ ] Add `EPANETError::is_warning()` / `is_error()` helpers
- [ ] Standardize error checking pattern: some wrappers use `check_error`/`check_error_with_context`, others use manual `if result == 0` / `match` blocks - pick one and be consistent

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

- [ ] Replace `enum_primitive` (unmaintained, last release 2016) with `num_derive`/`num_traits` or `strum` (already a dev-dep)
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
- [ ] Investigate why `EN_gettag`/`EN_settag` are missing from bindgen output (may need wrapper.h update)
