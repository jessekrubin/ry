use crate::JiffEraYear;
use jiff::civil::Era;
use pyo3::IntoPyObjectExt;
use pyo3::prelude::*;
use pyo3::types::PyTuple;

impl<'py> IntoPyObject<'py> for &JiffEraYear {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyTuple;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let (y, e) = self.0;
        let era_str_pyobj = match e {
            Era::BCE => crate::interns::bce(py),
            Era::CE => crate::interns::ce(py),
        };
        let year_py = y.into_py_any(py)?;
        let era_str = era_str_pyobj.into_py_any(py)?;

        let pyobjs_vec = vec![year_py, era_str];
        #[cfg(not(Py_LIMITED_API))]
        {
            PyTuple::new(py, pyobjs_vec)
        }
        #[cfg(Py_LIMITED_API)]
        {
            Ok(PyTuple::new(py, pyobjs_vec)?.into_any())
        }
    }
}

impl<'py> IntoPyObject<'py> for JiffEraYear {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyTuple;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}
