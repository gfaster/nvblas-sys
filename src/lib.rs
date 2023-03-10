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
//! necessary to override the linkage, it can be [overridden][override] on a
//! per-machine or per-crate basis by using a [`config.toml` file][configtoml].
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
//! env::var("CARGO_MANIFEST_DIR").unwrap());
//! ```
//! Alternatively, you can set the environment variable `NVBLAS_CONFIG_FILE`
//! manually.
//!
//! Since [NVBLAS][nvblas] is a limited drop-in replacement to traditional CPU BLAS
//! implementations, nvblas-sys does not provide any declarations by default. if you
//! want to use any [level 2][lvl2] or [level 1][lvl1] functions, you'll need to
//! include `blas-sys` as a dependency.
//!
//! If you just need the [level 3][lvl3] operations, you can use the `standalone`
//! feature. The `standalone` feature includes just the level 3 function
//! declarations copied from [`blas-sys`][blassys] so you can use NVBLAS without
//! the need for [`blas-sys`][blassys] or any CPU BLAS implementation.
//!
//! [configtoml]: https://doc.rust-lang.org/cargo/reference/config.html
//! [override]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts
//! [nvblas]: https://docs.nvidia.com/cuda/nvblas/
//! [lvl3]: https://netlib.org/blas/#_level_3
//! [lvl2]: https://netlib.org/blas/#_level_2
//! [lvl1]: https://netlib.org/blas/#_level_1
//! [blassys]: https://crates.io/crates/blas-sys

#![no_std]

#[cfg(feature = "standalone")]
pub mod standalone;
