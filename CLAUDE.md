# CLAUDE.md - rusty-epanet

Safe Rust wrapper for the [EPANET 2.3](https://github.com/OpenWaterAnalytics/EPANET) C library for water distribution network modeling and simulation.

## Project Status

Early development (v0.1.0). The library provides safe wrappers for most EPANET C API functions, but the higher-level Rust-idiomatic abstractions (typestate solver, domain structs) are incomplete. See `TODO.md` for the full list.

## Build

```bash
cargo build    # compiles EPANET C lib via cmake, generates FFI bindings via bindgen
cargo test     # requires the built dynamic library on the library search path
```

**Prerequisites:** CMake, a C compiler, and `libclang` (for bindgen).

The build script (`build.rs`) does three things:
1. Compiles the EPANET C source from the `EPANET/` git submodule using CMake (Release mode).
2. Generates Rust FFI bindings from `wrapper.h` -> `$OUT_DIR/bindings.rs` using bindgen.
3. Parses `EPANET/src/errors.dat` to generate `$OUT_DIR/error_messages.rs` (error code -> static string match), included via `include!()` in `src/error_messages.rs`.

Links dynamically against `epanet2`.

## Architecture

```
src/
  lib.rs              # EPANET struct (owns EN_Project handle), Drop, Send, constructors
  bindings.rs         # include!() of bindgen output (aliased as `ffi` throughout)
  epanet_error.rs     # EPANETError, Result<T>, check_error(), check_error_with_context()
  error_messages.rs   # Generated: get_error_message(code) -> &'static str
  types/              # Enums, structs, and type definitions
    mod.rs            # ObjectType, CountType, ActionCodeType, MAX_ID_SIZE, MAX_MSG_SIZE
    analysis.rs       # HydraulicSolver typestate (Closed/Initialized/Running/Solved) - INCOMPLETE
    options.rs        # FlowUnits, HeadLossType, QualityType, TimeParameter, Option enums
    node.rs           # NodeType, NodeProperty enums
    link.rs           # LinkType, LinkProperty, PumpType, StatusType enums
    control.rs        # Control<'a> struct, ControlType enum
    curve.rs          # Curve<'a> struct, CurveType enum
    demand.rs         # DemandModel enum
    rule.rs           # Rule struct, RuleObject/RuleVariable/RuleOperator enums
  impls/              # impl EPANET blocks organized by domain
    mod.rs
    project.rs        # Title, count, comment, run_project, save_inp_file
    node.rs           # Node CRUD, property get/set, batch values
    link.rs           # Link CRUD, property get/set, vertices, pump/pipe specifics
    hydraulic.rs      # Hydraulic solver lifecycle (open/init/run/next/solve/save/close)
    quality.rs        # Water quality solver lifecycle
    control.rs        # Simple control CRUD with RAII pattern
    curve.rs          # Curve CRUD with RAII pattern
    options.rs        # Analysis options, flow units, time params, quality type
    demand.rs         # Demand model and demand management
    pattern.rs        # Time pattern CRUD
    report.rs         # Report generation, statistics, error lookup
    rule.rs           # Rule-based control CRUD
    test_utils/       # Test configuration and fixtures
```

## Core Design Patterns

### EPANET Struct
Single entry point. Owns the opaque `EN_Project` C handle. `Drop` calls `EN_close()` + `EN_deleteproject()`. Manually implements `Send` (but not `Sync` — the C library uses internal mutable state). All API methods are `impl EPANET` blocks in `impls/`.

### Error Handling
All C API calls return `i32` (0 = success). Two helpers convert these:
- `check_error(code)` -> `Result<()>`
- `check_error_with_context(code, msg)` -> `Result<()>` with debug context

`EPANETError` holds `code: i32`, `message: &'static str` (from generated match), optional `context: String`. Compared by code only (`PartialEq`). Helpers: `.is_warning()` (codes 1–99), `.is_error()` (codes ≥ 100).

### FFI Pattern
```rust
// Typical wrapper method:
pub fn some_operation(&self, ...) -> Result<T> {
    let mut out = MaybeUninit::uninit();  // or 0/0.0 for primitives
    let code = unsafe { ffi::EN_someFunction(self.ph, ..., out.as_mut_ptr()) };
    check_error_with_context(code, "context message")?;
    Ok(unsafe { out.assume_init() })
}
```

String returns use `Vec<c_char>` buffer sized to `MAX_MSG_SIZE` or `MAX_ID_SIZE`, then `CStr::from_ptr` -> `to_string_lossy`.

### Enum Pattern
All enums use `enum_primitive` crate with `enum_from_primitive!` macro for `FromPrimitive` (C int -> Rust enum). Enums are `#[repr(u32)]` matching bindgen constants. Cast to C with `as i32`.

### RAII Structs (Control, Curve)
Hold `&'a EPANET` reference + index + cached field values. Provide:
- `.update()` -> syncs modified fields back to C API
- `.delete(self)` -> consuming method, removes from C model

### Typestate Pattern (HydraulicSolver) - INCOMPLETE
State machine using `PhantomData<State>` with states: `Closed -> Initialized -> Running -> Solved`. Methods consume `self` and return the next state, making invalid transitions a compile error. Currently a skeleton without actual FFI calls wired up.

## Conventions

- **Module per domain:** Each EPANET subsystem gets its own file in `types/` (data) and `impls/` (methods).
- **1-based indexing:** All indices from the C API are 1-based. The Rust wrappers preserve this.
- **Doc comments:** Use `# Parameters`, `# Returns`, `# Errors`, `# Safety`, `# See Also` sections.
- **Testing:** Uses `rstest` with fixtures defined in `impls/test_utils/fixtures.rs`. Key fixtures: `ph` (loaded net1.inp), `ph_close` (empty project), `after_step` (post-solve), `ph_single_node` (empty + one node).
- **Error context:** Prefer `check_error_with_context` for user-facing operations; plain `check_error` for internal helpers.
- **No panics in wrappers:** `CString::new().unwrap()` is acceptable (null bytes in user IDs are a programming error). Enum `from_i32().unwrap()` is acceptable (C API returns known values).
- **`pub(crate)` for internals:** The `ph` field on `EPANET`, helper functions, and raw indices use restricted visibility.

## Testing

```bash
cargo test
```

Tests use `net1.inp` (standard EPANET example network) via rstest fixtures. The `approx_eq` helper in test_utils handles floating-point comparisons. Tests are co-located in each `impls/` module file.

## Dependencies

| Crate | Purpose |
|-------|---------|
| `enum_primitive` | `FromPrimitive` for C enum conversion |
| `rstest` (dev) | Fixture-based test framework |
| `strum`/`strum_macros` (dev) | Enum iteration in tests |
| `bindgen` (build) | C header -> Rust FFI bindings |
| `cmake` (build) | Build EPANET C library |
