use ::which as which_rs;
use pyo3::prelude::*;
use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

#[pyfunction]
pub fn which(cmd: &str, path: Option<&str>) -> PyResult<Option<std::path::PathBuf>> {
    match path {
        Some(p) => {
            let which_res = which_rs::which_in(cmd, Some(p), env::current_dir().unwrap());
            let res = match which_res {
                Ok(p) => Ok(Some(PathBuf::from(p))),
                Err(_e) => {
                    // println!("which_rs::which_in({:?}, {:?}) -> {:?}", cmd, p, e);
                    Ok(None)
                }
            };
            res.into()
        }
        None => {
            let r = which_rs::which(cmd);
            let res = match r {
                Ok(p) => Ok(Some(p)),
                Err(_e) => {
                    // println!("which_rs::which({:?}) -> {:?}", cmd, e);
                    Ok(None)
                }
            };
            res.into()
        }
    }
}

#[pyfunction]
pub fn which_all(cmd: &str, path: Option<&str>) -> PyResult<Vec<String>> {
    let search_path: Option<OsString> = match path {
        Some(p) => Some(OsString::from(p)),
        None => env::var_os("PATH"),
    };
    let which_iter = which_rs::which_in_all(cmd, search_path, env::current_dir().unwrap()).unwrap();
    let which_vec = which_iter
        .into_iter()
        .map(|p| p.to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    Ok(which_vec)
}

#[pyfunction]
pub fn whicha(cmd: &str, path: Option<&str>) -> PyResult<Vec<String>> {
    which_all(cmd, path)
}

pub fn madd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(self::which, m)?)?;
    m.add_function(wrap_pyfunction!(self::which_all, m)?)?;
    m.add_function(wrap_pyfunction!(self::whicha, m)?)?;
    Ok(())
}
