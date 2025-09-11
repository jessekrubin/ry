//! re-export `ryo3-*` crates
//!
//! Maybe figure out how to strip the `ryo3_` prefix from the crate names
//! and `use ryo3_thingy as thingy`?
//!
//! Also look MA(!), writing macros all on my own! Still crappy at it...

pub use ryo3_std;

macro_rules! ryo3_features_reexport {
    ($($feature:literal, $crate_name:ident),* $(,)?) => {
        $(
            #[cfg(feature = $feature)]
            pub use $crate_name;
        )*
    };
}
ryo3_features_reexport! {
    "brotli", ryo3_brotli,
    "bytes", ryo3_bytes,
    "bzip2", ryo3_bzip2,
    "dirs", ryo3_dirs,
    "flate2", ryo3_flate2,
    "fnv", ryo3_fnv,
    "globset", ryo3_globset,
    "heck", ryo3_heck,
    "jiff", ryo3_jiff,
    "jiter", ryo3_jiter,
    "memchr", ryo3_memchr,
    "regex", ryo3_regex,
    "reqwest", ryo3_reqwest,
    "same-file", ryo3_same_file,
    "shlex", ryo3_shlex,
    "size", ryo3_size,
    "sqlformat", ryo3_sqlformat,
    "tokio", ryo3_tokio,
    "unindent", ryo3_unindent,
    "url", ryo3_url,
    "walkdir", ryo3_walkdir,
    "which", ryo3_which,
    // "twox-hash", ryo3_twox_hash,
    "zstd", ryo3_zstd,
}
