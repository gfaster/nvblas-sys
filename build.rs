use std::env;

fn main() {
    println!("cargo:rustc-env=NVBLAS_CONFIG_FILE={}/nvblas.conf", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rustc-link-lib=dylib=nvblas");
}
