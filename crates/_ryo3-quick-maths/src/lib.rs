#![doc = include_str!("../README.md")]
use pyo3::prelude::*;

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
#[pyfunction]
#[must_use]
pub const fn quick_maths() -> i8 {
    // 2 + 2 that's 4, minus 1 that's 3, quick maths
    let mut qm = 2 + 2;
    debug_assert!(qm == 4, "2 plus 2 is 4");
    qm -= 1;
    debug_assert!(qm == 3, "minus one that's 3");
    qm
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(quick_maths, m)?)?;
    Ok(())
}
