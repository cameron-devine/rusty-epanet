# Rusty EPANET

> **Status: Work in Progress**

Rusty EPANET provides safe Rust bindings to the [EPANET](https://github.com/OpenWaterAnalytics/EPANET) C library for working with water distribution network models. The EPANET source code is included as a git submodule and exposed through `bindgen`-generated Rust bindings. A high-level `EPANET` struct wraps the raw API to offer a safe interface that automatically cleans up resources when dropped.

## Implementation details

- The underlying C library lives in the `EPANET/` submodule.
- Bindings are generated at build time using [`bindgen`](https://rust-lang.github.io/rust-bindgen/).
- The `EPANET` struct owns an `EN_Project` handle and calls `EN_close` and `EN_deleteproject` in `Drop` for deterministic cleanup.
- `Send` and `Sync` are manually implemented to allow the project handle to be shared across threads.

## Installing

If you already cloned this repo without pulling the submodule you can run:

```
git submodule update --init --recursive
```

## Building and running tests

```
cargo build
cargo test
```

The tests are currently minimal and serve as a sanity check for the generated bindings.

## Examples

Create a new project:

```rust
use rusty_epanet::{EPANET, types::options::{FlowUnits, HeadLossType}};

fn main() -> rusty_epanet::Result<()> {
    let epanet = EPANET::new(
        "report.rpt",
        "output.bin",
        FlowUnits::GPM,
        HeadLossType::HazenWilliams,
    )?;
    // work with `epanet`...
    Ok(())
}
```

Open an existing `.inp` file:

```rust
use rusty_epanet::EPANET;

fn main() -> rusty_epanet::Result<()> {
    let epanet = EPANET::with_inp_file(
        "network.inp",
        "report.rpt",
        "output.bin",
    )?;
    // work with `epanet`...
    Ok(())
}
```

### Additional resources

Thanks to this guide for great guidance and information:
<https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a>

