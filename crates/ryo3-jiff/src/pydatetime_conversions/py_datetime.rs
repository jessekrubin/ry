use pyo3::prelude::*;
use pyo3::types::{PyDateTime, PyTzInfoAccess};

use crate::JiffPyDateTime;
use crate::pydatetime_conversions::{pydatetime_to_datetime, pydatetime_to_zoned};

impl<'py> FromPyObject<'_, 'py> for JiffPyDateTime {
    type Error = PyErr;
    fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        let dt = ob.cast::<PyDateTime>()?;
        if dt.get_tzinfo().is_some() {
            pydatetime_to_zoned(&dt).map(Self::Aware)
        } else {
            pydatetime_to_datetime(&dt).map(Self::Naive)
        }
    }
}
