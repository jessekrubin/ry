//! cpython struct-layout tricks + raw ffi fast paths (private)
//!
//! the unsafe guts behind `PyIntRead` + `pystr_read_fast*`; the public apis
//! live in `py_int.rs` / `py_str.rs`
pub(crate) mod pylong;
pub(crate) mod pyunicode;
