# `ryo3-jiff-core`

`jiff` <-> python conversions (new-type wrappers + `FromPyObject`/`IntoPyObject`
impls) used by `ryo3-jiff`, `ryo3-serde`, `ryo3-std` and friends.

No `#[pyclass]` structs live here.

```
src/
├── newts.rs     new-types (+ consts/Display)
├── fns.rs       raw conversion functions (the actual logic) + tz-db lookup
├── to_py.rs     IntoPyObject impls (jiff -> python)
└── from_py.rs   FromPyObject impls (python -> jiff)
```
