use crate::JiffDate;
use pyo3::prelude::*;

// pub(crate) fn warn_truncated_leap_second(obj: &Bound<'_, PyAny>) {
//     let py = obj.py();
//     if let Err(e) = PyErr::warn(
//         py,
//         &py.get_type::<PyUserWarning>(),
//         ffi::c_str!("ignored leap-second, `datetime` does not support leap-seconds"),
//         0,
//     ) {
//         e.write_unraisable(py, Some(obj))
//     };
// }
pub(crate) struct DateArgs {
    pub(crate) year: i32,
    pub(crate) month: u8,
    pub(crate) day: u8,
}
impl TryFrom<&JiffDate> for DateArgs {
    type Error = PyErr;
    fn try_from(value: &JiffDate) -> Result<Self, Self::Error> {
        let year = i32::from(value.0.year());

        let month = value.0.month();
        let month_u8 = u8::try_from(month)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;

        let day = value.0.day();
        let day_u8 = u8::try_from(day)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
        Ok(Self {
            year,
            month: month_u8,
            day: day_u8,
        })
    }
}
