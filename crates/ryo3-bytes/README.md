# `ryo3-bytes`

**NOTE:** This builds on the `pyo3-bytes` crate to expose extra Python‐style
bytes methods. Extending the `pyo3-bytes` crate is done with pyo3's
`multiple-pymethods` feature; this can and does cause longer compile times. To
avoid the compile time increase, there is an identical version under
`src/ryo3_bytes.rs`

Ideally these would be upstreamed to the `pyo3-bytes` crate at somepoint!

`bytes`:

- [crates.io](https://crates.io/crates/bytes)
- [docs.rs](https://docs.rs/bytes)
