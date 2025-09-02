use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PyOpenOptions {
    append: bool,
    create: bool,
    create_new: bool,
    read: bool,
    truncate: bool,
    write: bool,
    // TODO: OpenOptionsExt for windows/unix
}

#[pymethods]
impl PyOpenOptions {
    #[new]
    #[pyo3(signature = (mode=None, *, append=false, create=true, create_new=false, read=true, truncate=true, write=false))]
    fn py_new(
        mode: Option<&str>,
        append: Option<bool>,
        create: Option<bool>,
        create_new: Option<bool>,
        read: Option<bool>,
        truncate: Option<bool>,
        write: Option<bool>,
    ) -> PyResult<Self> {
        if let Some(mode) = mode {
            if append.is_some()
                || create.is_some()
                || create_new.is_some()
                || read.is_some()
                || truncate.is_some()
                || write.is_some()
            {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Cannot specify both mode string and individual options",
                ));
            } else {
                todo!("Opening files with mode strings is not yet supported: {mode}");
            }
        } else {
            let opts = PyOpenOptions {
                append: append.unwrap_or(false),
                create: create.unwrap_or(true),
                create_new: create_new.unwrap_or(false),
                read: read.unwrap_or(true),
                truncate: truncate.unwrap_or(true),
                write: write.unwrap_or(false),
            };
            Ok(opts)
        }
    }

    fn __repr__(&self) -> String {
        format!("{self}")
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.append == other.append
            && self.create == other.create
            && self.create_new == other.create_new
            && self.read == other.read
            && self.truncate == other.truncate
            && self.write == other.write
    }
}

impl std::fmt::Display for PyOpenOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenOptions(")?;
        write!(f, "append={}", self.append)?;
        write!(f, ", create={}", self.create)?;
        write!(f, ", create_new={}", self.create_new)?;
        write!(f, ", read={}", self.read)?;
        write!(f, ", truncate={}", self.truncate)?;
        write!(f, ", write={})", self.write)
    }
}
