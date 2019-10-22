extern crate bindgen;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search=../build/elements");

    // Only re-run the build script when memory.x is changed,
    // instead of when any part of the source code changes.
    println!("cargo:rerun-if-changed=memory.x");

    //does not work because generated bindings
    //do not contain #[link(name = "elements")]
    //potential solution in https://github.com/rust-lang/rust-bindgen/issues/1375
    let elements_header = "../elements/elements.h";
    println!("cargo:rerun-if-changed={}", elements_header);
    let bindings = bindgen::Builder::default()
        .header(elements_header)
        .ctypes_prefix("cty")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("elements.rs"))
        .expect("Couldn't write bindings!");

}
