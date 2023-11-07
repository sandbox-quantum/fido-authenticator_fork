use std::env;
// use std::path::PathBuf;

fn main() {

    // Set the path to the directory containing the C library
    let current_dir = env::current_dir().unwrap();

    // Tell the build script to link against the C library
    println!("cargo:rustc-link-search=native={}", current_dir.display());
    println!("cargo:rustc-link-lib=static=pqc_lib2");
    println!("cargo:rustc-link-lib=static=kyber_clean");
}