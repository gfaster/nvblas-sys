//! Standalone declarations for the functions supported by NVBLAS.
//!
//! If you only need [level 3][lvl3] BLAS routines, you can just link to NVBLAS
//! and not link to a traditional BLAS implementation.
//!
//! This module is a subset of [`blas-sys`][blassys]. If you need to use BLAS
//! routines not in [level 3][lvl3], include [`blas-sys`][blassys] instead of 
//! using the `standalone` feature.
//!
//! [blassys]: https://crates.io/crates/blas-sys
//! [lvl3]: https://netlib.org/blas/#_level_3
extern crate libc;

use libc::{c_char, c_double, c_float, c_int};

/// A complex number with 64-bit parts.
#[allow(bad_style)]
pub type c_double_complex = [libc::c_double; 2];

/// A complex number with 32-bit parts.
#[allow(bad_style)]
pub type c_float_complex = [libc::c_float; 2];

// Level 3
//
// http://www.netlib.org/blas/#_level_3
extern "C" {
    // Single
    pub fn sgemm_(
        transa: *const c_char,
        transb: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );
    pub fn ssymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );
    pub fn ssyrk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );
    pub fn ssyr2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *const c_float,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float,
        ldc: *const c_int,
    );
    pub fn strmm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *mut c_float,
        ldb: *const c_int,
    );
    pub fn strsm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float,
        a: *const c_float,
        lda: *const c_int,
        b: *mut c_float,
        ldb: *const c_int,
    );

    // Double
    pub fn dgemm_(
        transa: *const c_char,
        transb: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *const c_double,
        ldb: *const c_int,
        beta: *const c_double,
        c: *mut c_double,
        ldc: *const c_int,
    );
    pub fn dsymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *const c_double,
        ldb: *const c_int,
        beta: *const c_double,
        c: *mut c_double,
        ldc: *const c_int,
    );
    pub fn dsyrk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        beta: *const c_double,
        c: *mut c_double,
        ldc: *const c_int,
    );
    pub fn dsyr2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *const c_double,
        ldb: *const c_int,
        beta: *const c_double,
        c: *mut c_double,
        ldc: *const c_int,
    );
    pub fn dtrmm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *mut c_double,
        ldb: *const c_int,
    );
    pub fn dtrsm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double,
        a: *const c_double,
        lda: *const c_int,
        b: *mut c_double,
        ldb: *const c_int,
    );

    // Complex
    pub fn cgemm_(
        transa: *const c_char,
        transb: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *const c_float_complex,
        ldb: *const c_int,
        beta: *const c_float_complex,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn csymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *const c_float_complex,
        ldb: *const c_int,
        beta: *const c_float_complex,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn chemm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *const c_float_complex,
        ldb: *const c_int,
        beta: *const c_float_complex,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn csyrk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        beta: *const c_float_complex,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn cherk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float,
        a: *const c_float_complex,
        lda: *const c_int,
        beta: *const c_float,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn csyr2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *const c_float_complex,
        ldb: *const c_int,
        beta: *const c_float_complex,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn cher2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *const c_float_complex,
        ldb: *const c_int,
        beta: *const c_float,
        c: *mut c_float_complex,
        ldc: *const c_int,
    );
    pub fn ctrmm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *mut c_float_complex,
        ldb: *const c_int,
    );
    pub fn ctrsm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_float_complex,
        a: *const c_float_complex,
        lda: *const c_int,
        b: *mut c_float_complex,
        ldb: *const c_int,
    );

    // Double complex
    pub fn zgemm_(
        transa: *const c_char,
        transb: *const c_char,
        m: *const c_int,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *const c_double_complex,
        ldb: *const c_int,
        beta: *const c_double_complex,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zsymm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *const c_double_complex,
        ldb: *const c_int,
        beta: *const c_double_complex,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zhemm_(
        side: *const c_char,
        uplo: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *const c_double_complex,
        ldb: *const c_int,
        beta: *const c_double_complex,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zsyrk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        beta: *const c_double_complex,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zherk_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double,
        a: *const c_double_complex,
        lda: *const c_int,
        beta: *const c_double,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zsyr2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *const c_double_complex,
        ldb: *const c_int,
        beta: *const c_double_complex,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn zher2k_(
        uplo: *const c_char,
        trans: *const c_char,
        n: *const c_int,
        k: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *const c_double_complex,
        ldb: *const c_int,
        beta: *const c_double,
        c: *mut c_double_complex,
        ldc: *const c_int,
    );
    pub fn ztrmm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *mut c_double_complex,
        ldb: *const c_int,
    );
    pub fn ztrsm_(
        side: *const c_char,
        uplo: *const c_char,
        transa: *const c_char,
        diag: *const c_char,
        m: *const c_int,
        n: *const c_int,
        alpha: *const c_double_complex,
        a: *const c_double_complex,
        lda: *const c_int,
        b: *mut c_double_complex,
        ldb: *const c_int,
    );
}
