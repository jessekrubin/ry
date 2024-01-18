use dirs;
use pyo3::exceptions::PyFileNotFoundError;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

use crate::fs::fspath::PathLike;

#[pyfunction]
pub fn home() -> String {
    dirs::home_dir().unwrap().to_str().unwrap().to_string()
}

#[pyfunction]
pub fn pwd() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[pyfunction]
pub fn cd(
    // py: Python<'_>,
    p: PathLike,
) -> PyResult<()> {
    let r = std::env::set_current_dir(p.as_ref());
    match r {
        Ok(_) => Ok(()),
        Err(e) => {
            let p_string = p.to_string();
            let emsg = format!("{}: {:?}", e.to_string(), p_string);
            let pye = PyFileNotFoundError::new_err(format!("cd: {}", emsg));
            // pye.set_filename("cd");
            // pye.set_lineno(1);
            // pye.set_colno(1);
            // pye.set_function("cd");
            // pye.set_traceback(py, vec![]);

            Err(pye)
        }
    }
}

pub fn madd(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pwd, m)?)?;
    m.add_function(wrap_pyfunction!(cd, m)?)?;
    m.add_function(wrap_pyfunction!(home, m)?)?;
    Ok(())
}
