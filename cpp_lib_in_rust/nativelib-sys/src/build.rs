use std::{env, path::PathBuf};

fn main(){
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rustc-link-search=c/");
    println!("cargo:rustc-link-lib=native");

    cc::Build::new().cpp(false).file("c/wrapper.h").compile("native");

    let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
    .header("c/wrapper.h")
    // Tell cargo to invalidate the built crate whenever any of the
    // included header files changed.
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    // use core for no_std
    .ctypes_prefix("cty")
    .use_core()
    // Finish the builder and generate the bindings.
    .generate()
    // Unwrap the Result and panic on failure.
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}