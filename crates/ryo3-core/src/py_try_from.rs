use pyo3::prelude::*;
pub trait PyTryFrom<T>: Sized {
    fn py_try_from(value: T) -> PyResult<Self>;
}
