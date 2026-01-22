use crate::{RyDate, RyDateTime, RySpan, RyTime, RyTimestamp, RyZoned};
use pyo3::prelude::*;
use ryo3_core::RyMutex;
use ryo3_macro_rules::py_value_err;

#[pyclass(name = "DateSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateSeries {
    start: jiff::civil::Date,
    period: jiff::Span,
    pub(crate) series: RyMutex<jiff::civil::DateSeries, false>,
}

impl TryFrom<(&RyDate, &RySpan)> for RyDateSeries {
    type Error = PyErr;

    fn try_from(value: (&RyDate, &RySpan)) -> Result<Self, Self::Error> {
        let (start, period) = value;
        if period.0.is_zero() {
            return py_value_err!("period cannot be zero");
        }
        let s = start.0.series(period.0);
        Ok(Self {
            start: start.0,
            period: period.0,
            series: RyMutex::new(s),
        })
    }
}

impl std::fmt::Debug for RyDateSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DateSeries(start={}, period={})",
            RyDate::from(self.start),
            RySpan::from(self.period)
        )
    }
}
#[pyclass(name = "DateTimeSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateTimeSeries {
    start: jiff::civil::DateTime,
    period: jiff::Span,
    pub(crate) series: RyMutex<jiff::civil::DateTimeSeries, false>,
}

impl TryFrom<(&RyDateTime, &RySpan)> for RyDateTimeSeries {
    type Error = PyErr;

    fn try_from(value: (&RyDateTime, &RySpan)) -> Result<Self, Self::Error> {
        let (start, period) = value;
        if period.0.is_zero() {
            return py_value_err!("period cannot be zero");
        }
        let s = start.0.series(period.0);
        Ok(Self {
            start: start.0,
            period: period.0,
            series: RyMutex::new(s),
        })
    }
}

impl std::fmt::Debug for RyDateTimeSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DateTimeSeries(start={}, period={})",
            RyDateTime::from(self.start),
            RySpan::from(self.period)
        )
    }
}

#[pyclass(name = "TimeSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimeSeries {
    start: jiff::civil::Time,
    period: jiff::Span,
    pub(crate) series: RyMutex<jiff::civil::TimeSeries, false>,
}

impl TryFrom<(&RyTime, &RySpan)> for RyTimeSeries {
    type Error = PyErr;

    fn try_from(value: (&RyTime, &RySpan)) -> Result<Self, Self::Error> {
        let (start, period) = value;
        if period.0.is_zero() {
            return py_value_err!("period cannot be zero");
        }
        let s = start.0.series(period.0);
        Ok(Self {
            start: start.0,
            period: period.0,
            series: RyMutex::new(s),
        })
    }
}

impl std::fmt::Debug for RyTimeSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimeSeries(start={}, period={})",
            RyTime::from(self.start),
            RySpan::from(self.period)
        )
    }
}

#[pyclass(name = "TimestampSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimestampSeries {
    start: jiff::Timestamp,
    period: jiff::Span,
    pub(crate) series: RyMutex<jiff::TimestampSeries, false>,
}

impl TryFrom<(&RyTimestamp, &RySpan)> for RyTimestampSeries {
    type Error = PyErr;

    fn try_from(value: (&RyTimestamp, &RySpan)) -> Result<Self, Self::Error> {
        let (start, period) = value;
        if period.0.is_zero() {
            return py_value_err!("period cannot be zero");
        }
        let s = start.0.series(period.0);
        Ok(Self {
            start: start.0,
            period: period.0,
            series: RyMutex::new(s),
        })
    }
}

impl std::fmt::Debug for RyTimestampSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TimestampSeries(start={}, period={})",
            RyTimestamp::from(self.start),
            RySpan::from(self.period)
        )
    }
}

#[pyclass(name = "ZonedSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyZonedSeries {
    start: jiff::Zoned,
    period: jiff::Span,
    pub(crate) series: RyMutex<jiff::ZonedSeries, false>,
}

impl TryFrom<(&RyZoned, &RySpan)> for RyZonedSeries {
    type Error = PyErr;

    fn try_from(value: (&RyZoned, &RySpan)) -> Result<Self, Self::Error> {
        let (start, period) = value;
        if period.0.is_zero() {
            return py_value_err!("period cannot be zero");
        }
        let s = start.0.series(period.0);
        Ok(Self {
            start: start.0.clone(),
            period: period.0,
            series: RyMutex::new(s),
        })
    }
}

macro_rules! impl_py_series_pymethods(
    ($ry_series:ty, $ry_item:ty, $jiff_series:ty, $jiff_item:ty) => {
        #[pymethods]
        impl $ry_series {
            #[new]
            fn py_new(start: &$ry_item, period: &RySpan) -> PyResult<Self> {
                Self::try_from((start, period))
            }

            fn __repr__(&self) -> String {
                format!("{self:?}")
            }

            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }

            fn __next__(&self) -> Option<$ry_item> {
                self.series.py_lock().next().map(<$ry_item>::from)
            }

            #[pyo3(signature = (n = 1))]
            fn take(&self, py: Python<'_>, n: usize) -> Vec<$ry_item> {
                py.detach(|| {
                    let mut s = self.series.py_lock();
                    s.by_ref().take(n).map(<$ry_item>::from).collect()
                })
            }

            fn take_until(&self, py: Python<'_>, value: &$ry_item) -> PyResult<Vec<$ry_item>> {
                // make sure it is sane...
                if !self.period.is_positive() {
                    if value.0 > self.start {
                        py_value_err!("cannot `take_until` for value greater than start w/ negative period")
                    } else {
                        py.detach(|| {
                            let mut s = self.series.py_lock();
                            let items = s.by_ref().take_while(|item| item >= &value.0)
                                .map(<$ry_item>::from)
                                .collect::<Vec<$ry_item>>();
                            Ok(items)
                        })
                    }
                } else if self.period.is_positive(){
                    if  value.0 < self.start {
                        py_value_err!("cannot `take_until` for value less than start w/ positive period")
                    } else {
                        py.detach(|| {
                            let mut s = self.series.py_lock();
                            let items = s.by_ref().take_while(|item| item <= &value.0)
                                .map(<$ry_item>::from)
                                .collect::<Vec<$ry_item>>();
                            Ok(items)
                        })
                    }
                } else{
                    unreachable!() // already checked cant be 0 no way jose
                }
            }

            fn collect(&self, py: Python<'_>) -> Vec<$ry_item> {
                py.detach(|| {
                    let mut s = self.series.py_lock();
                    s.by_ref()
                        .collect::<Vec<$jiff_item>>()
                        .into_iter()
                        .map(<$ry_item>::from)
                        .collect()
                })
            }
        }
    };
);

impl_py_series_pymethods!(RyZonedSeries, RyZoned, jiff::ZonedSeries, jiff::Zoned);
impl_py_series_pymethods!(
    RyTimestampSeries,
    RyTimestamp,
    jiff::TimestampSeries,
    jiff::Timestamp
);
impl_py_series_pymethods!(
    RyTimeSeries,
    RyTime,
    jiff::civil::TimeSeries,
    jiff::civil::Time
);
impl_py_series_pymethods!(
    RyDateTimeSeries,
    RyDateTime,
    jiff::civil::DateTimeSeries,
    jiff::civil::DateTime
);
impl_py_series_pymethods!(
    RyDateSeries,
    RyDate,
    jiff::civil::DateSeries,
    jiff::civil::Date
);

impl std::fmt::Debug for RyZonedSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ZonedSeries(start={}, period={})",
            RyZoned::from(self.start.clone()),
            RySpan::from(self.period)
        )
    }
}
