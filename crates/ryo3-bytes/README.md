# `ryo3-bytes`

**NOTE:** This builds on the `pyo3-bytes` crate to expose extra Python-style
bytes methods. Extending the `pyo3-bytes` crate is done with pyo3's
`multiple-pymethods` feature; this can and does cause longer compile times. To
avoid the compile time increase, there is an identical version under
`src/ryo3_bytes.rs`

**NOTE-UPDATE (2026-06-09):** side by side maintenance of the `pyo3-bytes` fork
and the "ry-only" version is not being done anymore (as recent developments have
made it possible for me to "not care") so the `multiple-pymethods` feature is no
more...

Ideally these would be upstreamed to the `pyo3-bytes` crate at somepoint!

`bytes`:

- [crates.io](https://crates.io/crates/bytes)
- [docs.rs](https://docs.rs/bytes)
