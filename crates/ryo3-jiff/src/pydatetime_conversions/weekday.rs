use crate::JiffWeekday;
use pyo3::prelude::*;
use pyo3::types::PyInt;

impl<'py> IntoPyObject<'py> for JiffWeekday {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (&self).into_pyobject(py)
    }
}

impl<'py> IntoPyObject<'py> for &JiffWeekday {
    #[cfg(Py_LIMITED_API)]
    type Target = PyAny;
    #[cfg(not(Py_LIMITED_API))]
    type Target = PyInt;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    #[inline]
    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let num: u8 = match self.0 {
            jiff::civil::Weekday::Monday => 1,
            jiff::civil::Weekday::Tuesday => 2,
            jiff::civil::Weekday::Wednesday => 3,
            jiff::civil::Weekday::Thursday => 4,
            jiff::civil::Weekday::Friday => 5,
            jiff::civil::Weekday::Saturday => 6,
            jiff::civil::Weekday::Sunday => 7,
        };
        num.into_pyobject(py).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e} (weekday={num})"))
        })
    }
}
//
// impl FromPyObject<'_> for  JiffWeekday{
//     fn extract_bound(ob: &Bound<'_, PyAny>) -> PyResult<JiffWeekday> {
//         // extract as u8
//         let weekday = ob.extract::<u8>()?;
//         Ok(JiffWeekday(jiff::civil::Weekday::from(weekday)))
//         // #[cfg(not(Py_LIMITED_API))]
//         // {
//         //     let date = ob.downcast::<PyDate>()?;
//         //     py_date_to_jiff_date(date)
//         // }
//         // #[cfg(Py_LIMITED_API)]
//         // {
//         //     check_type(ob, &DatetimeTypes::get(ob.py()).date, "PyDate")?;
//         //     py_date_to_naive_date(ob)
//         // }
//     }
// }
