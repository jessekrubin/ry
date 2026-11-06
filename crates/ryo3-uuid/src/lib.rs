#![doc = include_str!("../README.md")]
mod py_uuid;
pub use py_uuid::{
    CPythonUuid, CPythonUuidError, PyUuid, RESERVED_FUTURE, RESERVED_MICROSOFT, RESERVED_NCS,
    RFC_4122, getnode, uuid1, uuid3, uuid4, uuid5, uuid6, uuid7, uuid8,
};
use pyo3::prelude::*;
pub use uuid;

/// `os.register_at_fork(after_in_child=UUID._reseed_rng)` (once)
///
/// same as cpython's `random`: <https://github.com/python/cpython/blob/b5bffb9b4ce8f4515df1e0474c8f439e9f580d65/Lib/random.py#L1012-L1013>
#[cfg(unix)]
fn register_fork_reseed(py: Python<'_>) -> PyResult<()> {
    static ONCE: std::sync::Once = std::sync::Once::new();
    let mut res = Ok(());
    ONCE.call_once(|| {
        // no `register_at_fork` on windows
        if let Ok(register) = py
            .import("os")
            .and_then(|os| os.getattr("register_at_fork"))
        {
            res = (|| {
                let reseed = py.get_type::<PyUuid>().getattr("_reseed_rng")?;
                let kwargs = pyo3::types::PyDict::new(py);
                kwargs.set_item("after_in_child", reseed)?;
                register.call((), Some(&kwargs)).map(|_| ())
            })();
        }
    });
    res
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUuid>()?;
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid3, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid5, m)?)?;
    m.add_function(wrap_pyfunction!(uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(uuid8, m)?)?;

    #[cfg(unix)]
    register_fork_reseed(m.py())?;
    Ok(())
}
