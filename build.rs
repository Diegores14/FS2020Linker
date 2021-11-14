extern crate bindgen;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-search=sim_connect_cpp/lib");
    println!("cargo:rustc-link-lib=static=SimConnect");

    let bindings = bindgen::Builder::default()
        .header("sim_connect_cpp/include/SimConnect.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
