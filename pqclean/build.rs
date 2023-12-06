use std::env;
// use std::path::PathBuf;

fn main() {
    let current_dir = env::current_dir().unwrap();

    // Cargo supports overriding the build script specified with a custom library,
    // however I didn't manage to make it work.
    // https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts

    let target = env::var("TARGET").unwrap();
    match target.as_str() {
        "thumbv8m.main-none-eabi" => {
            println!("cargo:rustc-link-search=native={}/arm/", current_dir.display());
            println!("cargo:rustc-link-lib=static=pqc_lib2");
            println!("cargo:rustc-link-lib=static=kyber_clean");
        }
        "aarch64-apple-darwin" => {
            println!("cargo:rustc-link-search=native={}/aarch64/", current_dir.display());
            println!("cargo:rustc-link-lib=static=dilithium3_clean");
        },
        _ => {
            panic!("unsupported target {}", target);
        }
    }
}
