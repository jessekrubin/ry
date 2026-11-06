#![doc = include_str!("../README.md")]
mod py_ulid;
pub use py_ulid::PyUlid;
use pyo3::prelude::*;

/// `os.register_at_fork(after_in_child=ULID._reset_generator)` (once)
///
/// same as cpython's `random`: <https://github.com/python/cpython/blob/b5bffb9b4ce8f4515df1e0474c8f439e9f580d65/Lib/random.py#L1012-L1013>
#[cfg(unix)]
fn register_fork_reset(py: Python<'_>) -> PyResult<()> {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let mut res = Ok(());
    ONCE.call_once(|| {
        // no `register_at_fork` on windows
        if let Ok(register) = py
            .import("os")
            .and_then(|os| os.getattr("register_at_fork"))
        {
            res = (|| {
                let reset = py.get_type::<PyUlid>().getattr("_reset_generator")?;
                let kwargs = pyo3::types::PyDict::new(py);
                kwargs.set_item("after_in_child", reset)?;
                register.call((), Some(&kwargs)).map(|_| ())
            })();
        }
    });
    res
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUlid>()?;

    #[cfg(unix)]
    register_fork_reset(m.py())?;
    Ok(())
}
