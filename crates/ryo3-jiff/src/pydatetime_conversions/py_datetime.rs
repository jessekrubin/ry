use pyo3::prelude::*;
use pyo3::types::PyDateTime;
#[cfg(not(Py_LIMITED_API))]
use pyo3::types::PyTzInfoAccess;

use crate::JiffPyDateTime;
use crate::pydatetime_conversions::{pydatetime_to_datetime, pydatetime_to_zoned};

impl<'py> FromPyObject<'_, 'py> for JiffPyDateTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = ob.cast::<PyDateTime>()?;
        #[cfg(not(Py_LIMITED_API))]
        let has_tzinfo = dt.get_tzinfo().is_some();
        #[cfg(Py_LIMITED_API)]
        let has_tzinfo = !dt.getattr(pyo3::intern!(dt.py(), "tzinfo"))?.is_none();
        if has_tzinfo {
            pydatetime_to_zoned(&dt).map(Self::Aware)
        } else {
            pydatetime_to_datetime(&dt).map(Self::Naive)
        }
    }
}
