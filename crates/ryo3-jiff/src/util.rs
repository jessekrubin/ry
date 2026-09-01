use pyo3::prelude::*;

// use ryo3_core::PyCastExactOpt;
// use ryo3_core::py_dict::KwargsIter;
// use ryo3_macro_rules::py_type_err;
use crate::functions::span;
use crate::span_units::{SpanUnit, SpanUnitsMask};

// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
// pub(crate) struct SpanKwargs1 {
//     years: i64,
//     months: i64,
//     weeks: i64,
//     days: i64,
//     hours: i64,
//     minutes: i64,
//     seconds: i64,
//     milliseconds: i64,
//     microseconds: i64,
//     nanoseconds: i64,
// }

// #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
// pub(crate) struct SpanKwargs2 {
//     years: i16,
//     months: i32,
//     weeks: i32,
//     days: i32,
//     hours: i32,
//     minutes: i64,
//     seconds: i64,
//     milliseconds: i64,
//     microseconds: i64,
//     nanoseconds: i64,
// }

// pub(crate) struct SpanKwargs3 {
//     years: Option<i16>,
//     months: Option<i32>,
//     weeks: Option<i32>,
//     days: Option<i32>,
//     hours: Option<i32>,
//     minutes: Option<i64>,
//     seconds: Option<i64>,
//     milliseconds: Option<i64>,
//     microseconds: Option<i64>,
//     nanoseconds: Option<i64>,
// }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SpanKwargs {
    units: SpanUnitsMask,

    years: i16,
    months: i32,
    weeks: i32,
    days: i32,
    hours: i32,
    minutes: i64,
    seconds: i64,
    milliseconds: i64,
    microseconds: i64,
    nanoseconds: i64,
}

// macro_rules! kw_builder {
//     ($($field:ident),*) => {
//         $(
//             pub(crate) fn $field(mut self, value: i64) -> Self {
//                 self.$field = value;
//                 self
//             }
//         )*
//     };
// }
macro_rules! kw_builder {
    ($(($field:ident, $unit:ident, $itype:ty)),* $(,)?) => {
        $(
            pub(crate) fn $field(mut self, value: $itype) -> Self {
                if value != 0 {
                    self.units.with_unit(SpanUnit::$unit);
                }
                self.$field = value;
                self
            }
        )*
    };
}

impl TryFrom<SpanKwargs> for jiff::Span {
    type Error = PyErr;

    fn try_from(value: SpanKwargs) -> Result<Self, Self::Error> {
        span(
            value.years.into(),
            value.months.into(),
            value.weeks.into(),
            value.days.into(),
            value.hours.into(),
            value.minutes,
            value.seconds,
            value.milliseconds,
            value.microseconds,
            value.nanoseconds,
        )
    }
}

// impl<'py> FromPyObject<'_, 'py> for SpanKwargs {
//     type Error = PyErr;

//     fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
//         if let Some(kw) = obj.cast_exact_opt::<pyo3::types::PyDict>() {
//             let kwiter = KwargsIter::new(kw);

//             let mut span_kwargs = SpanKwargs::default();
//             for (key, value) in kwiter {
//                 match key {
//                     "years" => span_kwargs.years = value.extract()?,
//                     "months" => span_kwargs.months = value.extract()?,
//                     "weeks" => span_kwargs.weeks = value.extract()?,
//                     "days" => span_kwargs.days = value.extract()?,
//                     "hours" => span_kwargs.hours = value.extract()?,
//                     "minutes" => span_kwargs.minutes = value.extract()?,
//                     "seconds" => span_kwargs.seconds = value.extract()?,
//                     "milliseconds" => span_kwargs.milliseconds = value.extract()?,
//                     "microseconds" => span_kwargs.microseconds = value.extract()?,
//                     "nanoseconds" => span_kwargs.nanoseconds = value.extract()?,
//                     _ => {
//                         return py_type_err!("unexpected keyword argument: {}", key);
//                     }
//                 }
//             }
//             Ok(span_kwargs)
//         } else {
//             py_type_err!("kwargs not a dictionary")
//         }
//     }
// }

impl SpanKwargs {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn has_units(&self) -> bool {
        self.units.count() > 0
    }
    // pub(crate) fn years(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Years);
    //     self.years = value as i16;
    //     self
    // }

    // pub(crate) fn months(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Months);
    //     self.months = value as i32;
    //     self
    // }

    // pub(crate) fn weeks(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Weeks);
    //     self.weeks = value as i32;
    //     self
    // }

    // pub(crate) fn days(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Days);
    //     self.days = value as i32;
    //     self
    // }

    // pub(crate) fn hours(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Hours);
    //     self.hours = value as i32;
    //     self
    // }

    // pub(crate) fn minutes(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Minutes);
    //     self.minutes = value;
    //     self
    // }

    // pub(crate) fn seconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Seconds);
    //     self.seconds = value;
    //     self

    // }

    // pub(crate) fn milliseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Milliseconds);
    //     self.milliseconds = value;
    //     self
    // }

    // pub(crate) fn microseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Microseconds);
    //     self.microseconds = value;
    //     self
    // }

    // pub(crate) fn nanoseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit2(SpanUnit::Nanoseconds);
    //     self.nanoseconds = value;
    //     self
    // }
    // pub(crate) fn years(mut self, value: i64) -> Self {
    //     if value != 0 {
    //         self.units.with_unit(SpanUnit::Years);
    //     }
    //     self.years = value as i16;
    //     self
    // }

    // pub (crate) fn months(mut self, value: i64) -> Self {
    //     if value != 0 {
    //         self.units.with_unit(SpanUnit::Months);
    //     }
    //     self.months = value as i32;
    //     self
    // }

    // pub(crate) fn weeks(mut self, value: i64) -> Self {
    //     if value != 0 {
    //         self.units.with_unit(SpanUnit::Weeks);
    //     }
    //     self.weeks = value as i32;
    //     self
    // }

    // pub(crate) fn days(mut self, value: i64) -> Self {

    // }

    // pub(crate) fn hours(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Hours);
    //     self.hours = value as i32;
    //     self
    // }

    // pub(crate) fn minutes(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Minutes);
    //     self.minutes = value;
    //     self
    // }

    // pub(crate) fn seconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Seconds);
    //     self.seconds = value;
    //     self
    // }

    // pub(crate) fn milliseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Milliseconds);
    //     self.milliseconds = value;
    //     self
    // }

    // pub(crate) fn microseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Microseconds);
    //     self.microseconds = value;
    //     self
    // }

    // pub(crate) fn nanoseconds(&mut self, value: i64) -> &mut Self {
    //     self.units.with_unit(SpanUnit::Nanoseconds);
    //     self.nanoseconds = value;
    //     self
    // }

    kw_builder!(
        (years, Years, i16),
        (months, Months, i32),
        (weeks, Weeks, i32),
        (days, Days, i32),
        (hours, Hours, i32),
        (minutes, Minutes, i64),
        (seconds, Seconds, i64),
        (milliseconds, Milliseconds, i64),
        (microseconds, Microseconds, i64),
        (nanoseconds, Nanoseconds, i64),
    );

    pub(crate) fn build(self) -> PyResult<jiff::Span> {
        jiff::Span::try_from(self)
    }

    pub(crate) fn is_zero(&self) -> bool {
        self.years == 0
            && self.months == 0
            && self.weeks == 0
            && self.days == 0
            && self.hours == 0
            && self.minutes == 0
            && self.seconds == 0
            && self.milliseconds == 0
            && self.microseconds == 0
            && self.nanoseconds == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // size of v1 kwargs
    // const SIZEOF_SPAN_KWARGS1: usize = std::mem::size_of::<SpanKwargs1>();
    // const SIZEOF_SPAN_KWARGS2: usize = std::mem::size_of::<SpanKwargs2>();
    // const SIZEOF_SPAN_KWARGS3: usize = std::mem::size_of::<SpanKwargs3>();
    const SIZEOF_SPAN_KWARGS4: usize = std::mem::size_of::<SpanKwargs>();
}
