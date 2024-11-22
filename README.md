# Rusty EPANET

This is a safe API wrapper for [EPANET](https://github.com/OpenWaterAnalytics/EPANET) in Rust. The EPANET source code
is pulled as a git submodule. `bindgen` is used to expose the C library to rust and then generate rust bindings.
This generates a lot of `unsafe` code so to prevent users of this library from having to wrap everything in unsafe
blocks (```unsafe{}```) there is a `EPANET` wrapper struct that wraps all of the C API calls into safe code.

Thanks to this guide for great guidance and information: 
https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a

### Installing
If you already cloned this repo without pulling the submodule you can run `git submodule update --init --recursive.`

### Building and running tests

The tests are very minimal and just used as a sanity check to make sure the bindings generated properly.

```
cargo build
cargo test
```