use pyo3::prelude::*;
use pyo3::{Bound, PyResult, intern};

/// Macro to generate constant bindings for Python
macro_rules! zstd_pymod_register_constants {
    ($m:ident, $py:ident, $($name:ident),*) => {
        $(
            $m.add(intern!($py, stringify!($name)), zstd_safe::$name)?;
        )*
    };
}

/// Adds all Zstd constants and functions to the Python module
pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = m.py();
    let __zstd_version__ = zstd_safe::version_string();
    m.add(intern!(py, "__zstd_version__"), __zstd_version__)?;
    zstd_pymod_register_constants!(
        m,
        py,
        BLOCKSIZELOG_MAX,
        BLOCKSIZE_MAX,
        CLEVEL_DEFAULT,
        CONTENTSIZE_ERROR,
        CONTENTSIZE_UNKNOWN,
        MAGIC_DICTIONARY,
        MAGICNUMBER,
        MAGIC_SKIPPABLE_MASK,
        MAGIC_SKIPPABLE_START,
        VERSION_MAJOR,
        VERSION_MINOR,
        VERSION_NUMBER,
        VERSION_RELEASE
    );
    Ok(())
}
