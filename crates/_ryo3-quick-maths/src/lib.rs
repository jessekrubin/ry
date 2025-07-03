#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};

/// 2 + 2 that's 4, minus 1 that's 3, quick maths
///
/// Based on the Big Shaq's thesis "Man's Not Hot"
///
/// Ref: [https://youtu.be/3M_5oYU-IsU?t=60](https://youtu.be/3M_5oYU-IsU?t=60)
///
/// This is a very computationally expensive operation, but it may be the
/// fastest implementation of `quick_maths` in the world.
///
/// # Example
/// ```
/// # use ryo3_quick_maths::quick_maths;
/// let result = ryo3_quick_maths::quick_maths();
/// assert_eq!(result, 3);
/// ```
///
#[pyfunction]
#[must_use]
pub fn quick_maths() -> i32 {
    // 2 + 2 that's 4, minus 1 that's 3, quick maths
    let mut qm = 2 + 2;
    qm -= 1;
    qm
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quick_maths, m)?)?;
    Ok(())
}
