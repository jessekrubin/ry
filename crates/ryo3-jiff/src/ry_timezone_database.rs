//! python shim for `TimeZoneDatabase`
use crate::{RyTimeZone, errors::map_py_value_err};
use jiff::tz::TimeZoneDatabase;
use pyo3::prelude::*;
#[pyclass(name = "TimeZoneDatabase", frozen)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct RyTimeZoneDatabase {
    inner: Option<TimeZoneDatabase>,
}

impl RyTimeZoneDatabase {
    fn db(&self) -> &TimeZoneDatabase {
        if let Some(db) = &self.inner {
            db
        } else {
            jiff::tz::db()
        }
    }
}

#[pymethods]
impl RyTimeZoneDatabase {
    #[new]
    fn py_new() -> Self {
        Self::from(TimeZoneDatabase::from_env())
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.db())
    }

    #[pyo3(signature = (name, err = false))]
    fn get(&self, name: &str, err: bool) -> PyResult<Option<RyTimeZone>> {
        let tz_res = self.db().get(name).map(RyTimeZone::from);
        match tz_res {
            Ok(tz) => Ok(Some(tz)),
            Err(e) => {
                if err {
                    Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        e.to_string(),
                    ))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn available(&self) -> Vec<String> {
        self.db()
            .available()
            .map(|tz_name| tz_name.to_string())
            .collect()
    }

    fn __getitem__(&self, name: &str) -> PyResult<RyTimeZone> {
        self.db()
            .get(name)
            .map(RyTimeZone::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyKeyError, _>(e.to_string()))
    }

    fn __len__(&self) -> usize {
        self.db().available().count()
    }

    fn is_definitively_empty(&self) -> bool {
        self.db().is_definitively_empty()
    }

    #[staticmethod]
    fn bundled() -> Self {
        Self::from(TimeZoneDatabase::bundled())
    }

    #[staticmethod]
    fn from_env() -> Self {
        Self::from(TimeZoneDatabase::from_env())
    }

    #[staticmethod]
    fn from_dir(path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_dir(path)
            .map(Self::from)
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_concatenated_path(path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_concatenated_path(path)
            .map(Self::from)
            .map_err(map_py_value_err)
    }
}

impl From<TimeZoneDatabase> for RyTimeZoneDatabase {
    fn from(db: TimeZoneDatabase) -> Self {
        Self { inner: Some(db) }
    }
}
