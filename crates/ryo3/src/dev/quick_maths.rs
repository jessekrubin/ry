//! Quick maths - template module
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{wrap_pyfunction, PyResult};

/// 2 + 2 that's 4, minus 1 that's 3, quick maths
///
/// Based on the Big Shaq's thesis "Man's Not Hot"
///
/// Ref: https://youtu.be/3M_5oYU-IsU?t=60
///
/// # Example
/// ```
/// # use ryo3::dev::quick_maths;
/// let result = quick_maths::quick_maths();
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

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quick_maths, m)?)?;
    Ok(())
}
