extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let libv4l = pkg_config::probe_library("libv4l2").unwrap();

    // Path to directories of C header
    let include_dirs: Vec<PathBuf> = env::var("LIBCLANG_INCLUDE_PATH")
        .map(|path| vec![PathBuf::from(path)])
        .unwrap_or_default();

    let include_args: Vec<_> = include_dirs
        .iter()
        .chain(libv4l.include_paths.iter())
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();
    println!("cargo:warning={:?}", include_args);

    let target_args = vec!["-target".into(), env::var("TARGET").unwrap()];
    println!("cargo:warning={:?}", target_args);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&target_args)
        .clang_args(&include_args)
        .derive_debug(false)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
