use pyo3::prelude::*;

/// Macro to generate constant bindings for Python
macro_rules! zstd_pymod_register_constants {
    ($m:ident, $($name:ident),*) => {
        $(
            $m.add(stringify!($name), zstd_safe::$name)?;
        )*
    };
}

/// Adds all Zstd constants and functions to the Python module
pub(crate) fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    let __zstd_version__ = zstd_safe::version_string();
    m.add("__zstd_version__", __zstd_version__)?;
    zstd_pymod_register_constants!(
        m,
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
