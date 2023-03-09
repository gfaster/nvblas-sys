# nvblas-sys

This package provides linkage to and configuration for [NVBLAS][nvblas].

## Setup
Before anything else, [NVBLAS][nvblas] must be installed and exist in cargo
search path. On linux machines, it should be named `libnvblas.so`. If it is
necessary to override the linkage, it can be [overriden][override] on a per-machine or
per-crate basis by using a [`config.toml` file][configtoml].

Make sure to invoke the crate to make sure it runs:
```rust
extern crate nvblas-sys;
```

nvblas-sys comes with a default configuration file, but it may not be may not be
acceptable for your usage. To use your own configuration file, include the
following line in your `build.rs` file:
```rust
println!("cargo:rustc-env=NVBLAS_CONFIG_FILE={}/nvblas.conf",
          env::var("CARGO_MANIFEST_DIR").unwrap());
```
Alternatively, you can set the environment variable `NVBLAS_CONFIG_FILE`
manually.

Since [NVBLAS][nvblas] is a limited drop-in replacement to traditional CPU BLAS
implementations, nvblas-sys does not provide any declarations; you will also
need to include `blas-sys` as a dependancy.

[configtoml]: https://doc.rust-lang.org/cargo/reference/config.html
[override]: https://doc.rust-lang.org/cargo/reference/build-scripts.html#overriding-build-scripts
[nvblas]: https://docs.nvidia.com/cuda/nvblas/
