use cmake::Config;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let dst = Config::new("EPANET")
        .define("CMAKE_BUILD_TYPE", "Release")
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=dylib=epanet2");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Generate error messages
    let input_path = Path::new("EPANET/src/errors.dat");
    let output_path = Path::new("src/error_messages.rs");

    let input = File::open(input_path).expect("Failed to open errors.dat");
    let reader = BufReader::new(input);

    let mut output = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .expect("Failed to create error_messages.rs");

    writeln!(
        output,
        "pub fn get_error_message(code: i32) -> &'static str {{\n    match code {{"
    )
    .unwrap();

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(rest) = line.strip_prefix("DAT(") {
            let parts: Vec<&str> = rest.trim_end_matches(')').splitn(2, ',').collect();
            if parts.len() == 2 {
                let code = parts[0].trim();
                let msg = parts[1].trim().trim_matches('"');
                writeln!(output, "        {} => \"{}\",", code, msg).unwrap();
            }
        }
    }

    writeln!(output, "        _ => \"UNKNOWN ERROR\",\n    }}\n}}").unwrap();
}
