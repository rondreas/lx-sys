extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::io::Write;

fn main() {
    // If this is missing, either set the environment variable or make a .cargo/config.toml with [env] variables,
    let include_path = env::var("LX_INCLUDE").expect("Missing Environment Variable: LX_INCLUDE");

    // We then attempt to create the wrapper.h file,
    let mut header = std::fs::File::create("wrapper.h").expect("Failed to create 'wrapper.h'");

    // Create the wrapper.h from the headers,
    let includes = std::fs::read_dir(&include_path).unwrap();  // This gets us an iterator with DirEntry objects in path,

    // TODO: More concise method for getting all .h files as string
    // For each DirEntry in includes, write_fmt the name for any headers found,
    for include in includes {
        let dir_entry = include.unwrap();
        let path = dir_entry.path();
        let filename = path.file_name().unwrap().to_str().expect("Failed to get path filename");
        if path.extension().unwrap() == "h" {
            if let Err(e) = writeln!(header, "#include \"{}\"", filename) {
                println!("macro writeln failed: {}", e.to_string());
            }
        }
    }

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(format!("--include-directory={}", include_path))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}