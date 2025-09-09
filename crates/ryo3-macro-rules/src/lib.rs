mod not_implemented;
#[macro_use]
mod py_errs;

#[macro_export]
macro_rules! any_repr {
    ($obj:expr) => {{


        let typ = $obj.get_type();
        let name = typ
            .fully_qualified_name()
            .unwrap_or_else(|_| pyo3::types::PyString::new($obj.py(), "unknown"));
        match $obj.repr() {
            Ok(repr) => format!("{repr} ({name})"),
            Err(_) => name.to_string(),
        }
    }};
}
