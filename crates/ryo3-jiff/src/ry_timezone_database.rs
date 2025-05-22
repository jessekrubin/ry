//! python shim for `TimeZoneDatabase`
use crate::RyTimeZone;
use jiff::tz::TimeZoneDatabase;
use pyo3::prelude::*;
use pyo3::types::PyType;
#[pyclass(name = "TimeZoneDatabase", module = "ry.ryo3", frozen)]
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
        RyTimeZoneDatabase::from(TimeZoneDatabase::from_env())
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.db())
    }

    #[pyo3(signature = (name, err = false))]
    pub fn get(&self, name: &str, err: bool) -> PyResult<Option<RyTimeZone>> {
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

    pub fn available(&self) -> Vec<String> {
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

    #[classmethod]
    fn bundled(_cls: &Bound<'_, PyType>) -> Self {
        RyTimeZoneDatabase::from(TimeZoneDatabase::bundled())
    }

    #[classmethod]
    fn from_env(_cls: &Bound<'_, PyType>) -> Self {
        RyTimeZoneDatabase::from(TimeZoneDatabase::from_env())
    }

    #[classmethod]
    fn from_dir(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_dir(path)
            .map(RyTimeZoneDatabase::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    #[classmethod]
    fn from_concatenated_path(_cls: &Bound<'_, PyType>, path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_concatenated_path(path)
            .map(RyTimeZoneDatabase::from)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
}

impl From<TimeZoneDatabase> for RyTimeZoneDatabase {
    fn from(db: TimeZoneDatabase) -> Self {
        RyTimeZoneDatabase { inner: Some(db) }
    }
}
