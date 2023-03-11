use std::path::PathBuf;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-env-changed=NVBLAS_CONFIG_FILE");

    #[cfg(feature = "configfile")]
    configfile();

    println!("cargo:rustc-link-lib=dylib=nvblas");
}

#[allow(dead_code)]
fn configfile() {
    // If config file isn't set in env, then we'll include our own
    if env::var("NVBLAS_CONFIG_FILE").is_err() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let conf_path: PathBuf = format!("{}/{}", out_dir, "nvblas.conf").into();

        if !conf_path.try_exists().unwrap_or(false) {
            fs::copy("nvblas_template.conf", &conf_path).unwrap();
        }

        if conf_path.try_exists().unwrap_or(false) {
            println!(
                "cargo:rustc-env=NVBLAS_CONFIG_FILE={}",
                conf_path.to_str().unwrap()
            );
        }
    }
}
