use crate::{RyDate, RyDateTime, RyTime, RyTimestamp, RyZoned};
use pyo3::prelude::*;
use ryo3_core::PyMutex;

#[pyclass(name = "DateSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateSeries {
    pub(crate) series: PyMutex<jiff::civil::DateSeries, false>,
}

impl From<jiff::civil::DateSeries> for RyDateSeries {
    fn from(series: jiff::civil::DateSeries) -> Self {
        Self {
            series: PyMutex::new(series),
        }
    }
}

#[pymethods]
impl RyDateSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyDate> {
        self.series.py_lock().next().map(RyDate::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyDate> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref().take(n).map(RyDate::from).collect()
        })
    }

    fn collect(&self, py: Python<'_>) -> Vec<RyDate> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref()
                .collect::<Vec<_>>()
                .into_iter()
                .map(RyDate::from)
                .collect()
        })
    }
}

#[pyclass(name = "DateTimeSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyDateTimeSeries {
    pub(crate) series: PyMutex<jiff::civil::DateTimeSeries, false>,
}

impl From<jiff::civil::DateTimeSeries> for RyDateTimeSeries {
    fn from(series: jiff::civil::DateTimeSeries) -> Self {
        Self {
            series: PyMutex::new(series),
        }
    }
}

#[pymethods]
impl RyDateTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyDateTime> {
        self.series.py_lock().next().map(RyDateTime::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyDateTime> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref().take(n).map(RyDateTime::from).collect()
        })
    }

    fn collect(&self, py: Python<'_>) -> Vec<RyDateTime> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref()
                .collect::<Vec<_>>()
                .into_iter()
                .map(RyDateTime::from)
                .collect()
        })
    }
}

#[pyclass(name = "TimeSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimeSeries {
    pub(crate) series: PyMutex<jiff::civil::TimeSeries, false>,
}

impl From<jiff::civil::TimeSeries> for RyTimeSeries {
    fn from(series: jiff::civil::TimeSeries) -> Self {
        Self {
            series: PyMutex::new(series),
        }
    }
}

#[pymethods]
impl RyTimeSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyTime> {
        self.series.py_lock().next().map(RyTime::from)
    }

    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyTime> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref().take(n).map(RyTime::from).collect()
        })
    }

    fn collect(&self, py: Python<'_>) -> Vec<RyTime> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref()
                .collect::<Vec<_>>()
                .into_iter()
                .map(RyTime::from)
                .collect()
        })
    }
}

#[pyclass(name = "TimestampSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyTimestampSeries {
    pub(crate) series: PyMutex<jiff::TimestampSeries, false>,
}

impl From<jiff::TimestampSeries> for RyTimestampSeries {
    fn from(series: jiff::TimestampSeries) -> Self {
        Self {
            series: PyMutex::new(series),
        }
    }
}

#[pymethods]
impl RyTimestampSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyTimestamp> {
        self.series.py_lock().next().map(RyTimestamp::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyTimestamp> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref().take(n).map(RyTimestamp::from).collect()
        })
    }

    fn collect(&self, py: Python<'_>) -> Vec<RyTimestamp> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref()
                .collect::<Vec<_>>()
                .into_iter()
                .map(RyTimestamp::from)
                .collect()
        })
    }
}

#[pyclass(name = "ZonedSeries", frozen, immutable_type, skip_from_py_object)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
pub struct RyZonedSeries {
    pub(crate) series: PyMutex<jiff::ZonedSeries, false>,
}

impl From<jiff::ZonedSeries> for RyZonedSeries {
    fn from(series: jiff::ZonedSeries) -> Self {
        Self {
            series: PyMutex::new(series),
        }
    }
}

#[pymethods]
impl RyZonedSeries {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&self) -> Option<RyZoned> {
        self.series.py_lock().next().map(RyZoned::from)
    }

    #[pyo3(signature = (n = 1))]
    fn take(&self, py: Python<'_>, n: usize) -> Vec<RyZoned> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref().take(n).map(RyZoned::from).collect()
        })
    }

    fn collect(&self, py: Python<'_>) -> Vec<RyZoned> {
        py.detach(|| {
            let mut s = self.series.py_lock();
            s.by_ref()
                .collect::<Vec<_>>()
                .into_iter()
                .map(RyZoned::from)
                .collect()
        })
    }
}
