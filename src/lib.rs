//! # nvblas-sys
//!
//! This package provides linkage to and configuration for [NVBLAS][nvblas].
//!
//! ## Why should you use nvblas-sys
//! It's easy to link to a system library, especially when another package provides
//! bindings. However, anyone who includes your package down the line will be unable
//! to override it. Using nvblas-sys means exactly one package will link to
//! `libnvblas` and those who wish to override it can do so without causing problems
//! for dependents thanks to the use of the `links` manifest tag.
//!
//! ## Setup
//! Before anything else, [NVBLAS][nvblas] must be installed and exist in cargo
//! search path. On Linux machines, it should be named `libnvblas.so`. If it is
//! necessary to override the linkage, it can be [overridden][override] on a per-machine or
//! per-crate basis by using a [`config.toml` file][configtoml].
//!
//! Make sure to invoke the crate to make sure it runs:
//! ```rust
//! extern crate nvblas-sys;
//! ```
//!
//! nvblas-sys comes with a default configuration file, but it may not be may not be
//! acceptable for your usage. To use your own configuration file, include the
//! following line in your `build.rs` file:
//! ```rust
//! println!("cargo:rustc-env=NVBLAS_CONFIG_FILE={}/nvblas.conf",
//!           env::var("CARGO_MANIFEST_DIR").unwrap());
//! ```
//! Alternatively, you can set the environment variable `NVBLAS_CONFIG_FILE`
//! manually.
//!
//! Since [NVBLAS][nvblas] is a limited drop-in replacement to traditional CPU BLAS
//! implementations, nvblas-sys does not provide any declarations; you will also
//! need to include `blas-sys` as a dependency.
//!
//! [configtoml]: https://doc.rust-lang.org/cargo/reference/config.html
//! [override]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts
//! [nvblas]: https://docs.nvidia.com/cuda/nvblas/

#![no_std]
