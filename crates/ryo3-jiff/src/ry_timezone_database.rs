//! python shim for `TimeZoneDatabase`

use jiff::tz::TimeZoneDatabase;
use pyo3::prelude::*;
use ryo3_core::map_py_value_err;
use ryo3_core::repr::PyStrRepr;
use ryo3_macro_rules::{py_key_error, py_value_err};

use crate::RyTimeZone;

#[pyclass(name = "TimeZoneDatabase", frozen, immutable_type)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Debug, Clone)]
pub struct RyTimeZoneDatabase {
    db: TimeZoneDatabase,
    src: TzdbSource,
}

/// where/how the tz-db was created
#[derive(Debug, Clone)]
enum TzdbSource {
    Env,
    Bundled,
    Dir(String),
    Concatenated(String),
}

impl RyTimeZoneDatabase {
    fn new(db: TimeZoneDatabase, src: TzdbSource) -> Self {
        Self { db, src }
    }

    fn db(&self) -> &TimeZoneDatabase {
        &self.db
    }
}

impl std::fmt::Display for RyTimeZoneDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.src {
            TzdbSource::Env => f.write_str("TimeZoneDatabase()"),
            TzdbSource::Bundled => f.write_str("TimeZoneDatabase.bundled()"),
            TzdbSource::Dir(path) => {
                write!(f, "TimeZoneDatabase.from_dir({})", PyStrRepr::new(path))
            }
            TzdbSource::Concatenated(path) => write!(
                f,
                "TimeZoneDatabase.from_concatenated_path({})",
                PyStrRepr::new(path)
            ),
        }
    }
}

#[pymethods]
impl RyTimeZoneDatabase {
    #[new]
    fn py_new() -> Self {
        Self::from_env()
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    #[pyo3(signature = (name, err = false))]
    fn get(&self, name: &str, err: bool) -> PyResult<Option<RyTimeZone>> {
        let tz_res = self.db().get(name).map(RyTimeZone::from);
        match tz_res {
            Ok(tz) => Ok(Some(tz)),
            Err(e) => {
                if err {
                    py_value_err!("{e}")
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
            .map_err(|e| py_key_error!("{e}"))
    }

    fn __len__(&self) -> usize {
        self.db().available().count()
    }

    fn is_definitively_empty(&self) -> bool {
        self.db().is_definitively_empty()
    }

    #[staticmethod]
    fn bundled() -> Self {
        Self::new(TimeZoneDatabase::bundled(), TzdbSource::Bundled)
    }

    #[staticmethod]
    fn from_env() -> Self {
        Self::new(TimeZoneDatabase::from_env(), TzdbSource::Env)
    }

    #[staticmethod]
    fn from_dir(path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_dir(path)
            .map(|db| Self::new(db, TzdbSource::Dir(path.to_string())))
            .map_err(map_py_value_err)
    }

    #[staticmethod]
    fn from_concatenated_path(path: &str) -> PyResult<Self> {
        TimeZoneDatabase::from_concatenated_path(path)
            .map(|db| Self::new(db, TzdbSource::Concatenated(path.to_string())))
            .map_err(map_py_value_err)
    }
}
