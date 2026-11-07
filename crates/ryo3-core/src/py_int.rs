//! int/number helpers
use pyo3::prelude::*;
use pyo3::types::PyInt;

/// `PyNumber_Check`: has `__index__`/`__int__`/`__float__` (or is complex)
///
/// Fast check if `ob.extract::<u64>` et al would fail
#[inline]
#[must_use]
pub fn pynumber_check(ob: Borrowed<'_, '_, PyAny>) -> bool {
    ob.is_exact_instance_of::<PyInt>() || {
        #[expect(unsafe_code, reason = "valid object + attached")]
        unsafe {
            pyo3::ffi::PyNumber_Check(ob.as_ptr()) != 0
        }
    }
}
