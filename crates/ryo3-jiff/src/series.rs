use crate::{RyDate, RyDateTime, RyTime, RyTimestamp};
use parking_lot::Mutex;
use pyo3::prelude::*;

macro_rules! define_py_jiff_series {
    (
        py_name: $py_name:literal,
        ry_name: $ry_name:ident,
        series_type: $jiff_series_type:ty,
        jiff_item_type: $jiff_item_type:ty,
        ry_item_type: $ry_item_type:ty
    ) => {
        #[pyclass(name = $py_name, frozen)]
        #[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
        pub struct $ry_name {
            pub(crate) series: Mutex<$jiff_series_type>,
        }

        impl From<$jiff_series_type> for $ry_name {
            fn from(series: $jiff_series_type) -> Self {
                Self {
                    series: Mutex::new(series),
                }
            }
        }

        #[pymethods]
        impl $ry_name {
            fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
                slf
            }

            fn __next__(&self) -> Option<$ry_item_type> {
                self.series.lock().next().map(Into::into)
            }

            #[pyo3(signature = (n = 1))]
            fn take(&self, n: usize) -> Vec<$ry_item_type> {
                let mut s = self.series.lock();
                s.by_ref().take(n).map(Into::into).collect()
            }
        }
    };
}

define_py_jiff_series!(
    py_name: "DateSeries",
    ry_name: RyDateSeries,
    series_type: jiff::civil::DateSeries,
    jiff_item_type: jiff::civil::Date,
    ry_item_type: RyDate
    // "DateSeries",
    // RyDateSeries,
    // jiff::civil::DateSeries,
    // jiff::civil::Date,
    // RyDate
);
// #[pyclass(name = "DateSeries", frozen)]
// #[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
// pub struct RyDateSeries {
//     pub(crate) series: Mutex<jiff::civil::DateSeries>,
// }

// impl From<jiff::civil::DateSeries> for RyDateSeries {
//     fn from(series: jiff::civil::DateSeries) -> Self {
//         Self {
//             series: Mutex::new(series),
//         }
//     }
// }

// #[pymethods]
// impl RyDateSeries {
//     fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
//         slf
//     }

//     fn __next__(&self) -> Option<RyDate> {
//         self.series.lock().next().map(RyDate::from)
//     }

//     #[pyo3(signature = (n = 1))]
//     fn take(&self, n: usize) -> Vec<RyDate> {
//         let mut s = self.series.lock();
//         s.by_ref().take(n).map(RyDate::from).collect()
//     }
// }

#[pyclass(name = "DateTimeSeries", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateTimeSeries {
    pub(crate) series: Mutex<jiff::civil::DateTimeSeries>,
}

impl From<jiff::civil::DateTimeSeries> for RyDateTimeSeries {
    fn from(series: jiff::civil::DateTimeSeries) -> Self {
        Self {
            series: Mutex::new(series),
        }
    }
}

#[pymethods]
impl RyDateTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyDateTime> {
        self.series.lock().next().map(RyDateTime::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> Vec<RyDateTime> {
        let mut s = self.series.lock();
        s.by_ref().take(n).map(RyDateTime::from).collect()
    }
}

#[pyclass(name = "TimeSeries", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimeSeries {
    pub(crate) series: Mutex<jiff::civil::TimeSeries>,
}

impl From<jiff::civil::TimeSeries> for RyTimeSeries {
    fn from(series: jiff::civil::TimeSeries) -> Self {
        Self {
            series: Mutex::new(series),
        }
    }
}

#[pymethods]
impl RyTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyTime> {
        self.series.lock().next().map(RyTime::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> Vec<RyTime> {
        let mut s = self.series.lock();
        s.by_ref().take(n).map(RyTime::from).collect()
    }
}

#[pyclass(name = "TimestampSeries", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimestampSeries {
    pub(crate) series: Mutex<jiff::TimestampSeries>,
}

impl From<jiff::TimestampSeries> for RyTimestampSeries {
    fn from(series: jiff::TimestampSeries) -> Self {
        Self {
            series: Mutex::new(series),
        }
    }
}

#[pymethods]
impl RyTimestampSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyTimestamp> {
        self.series.lock().next().map(RyTimestamp::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, n: usize) -> Vec<RyTimestamp> {
        let mut s = self.series.lock();
        s.by_ref().take(n).map(RyTimestamp::from).collect()
    }
}
