#![doc = include_str!("../README.md")]
mod py_uuid;
use crate::py_uuid::{RESERVED_FUTURE, RESERVED_MICROSOFT, RESERVED_NCS, RFC_4122};
pub use py_uuid::{
    CPythonUuid, PyUuid, getnode, uuid1, uuid2, uuid3, uuid4, uuid5, uuid6, uuid7, uuid8,
};
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
pub use uuid;

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("NAMESPACE_DNS", PyUuid::NAMESPACE_DNS())?;
    m.add("NAMESPACE_OID", PyUuid::NAMESPACE_OID())?;
    m.add("NAMESPACE_URL", PyUuid::NAMESPACE_URL())?;
    m.add("NAMESPACE_X500", PyUuid::NAMESPACE_X500())?;
    m.add("RESERVED_FUTURE", RESERVED_FUTURE)?;
    m.add("RESERVED_MICROSOFT", RESERVED_MICROSOFT)?;
    m.add("RESERVED_NCS", RESERVED_NCS)?;
    m.add("RFC_4122", RFC_4122)?;
    m.add_class::<PyUuid>()?;
    m.add_function(wrap_pyfunction!(getnode, m)?)?;
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid2, m)?)?;
    m.add_function(wrap_pyfunction!(uuid3, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid5, m)?)?;
    m.add_function(wrap_pyfunction!(uuid6, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    m.add_function(wrap_pyfunction!(uuid8, m)?)?;
    Ok(())
}
