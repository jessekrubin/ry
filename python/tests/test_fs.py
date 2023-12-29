import ry
import pytest

def test_read_string(tmp_path):
    p = tmp_path / "test.txt"
    p.write_text("hello")
    ry.cd(tmp_path)
    assert ry.read_text("test.txt") == "hello"

def test_read_string_invalid_utf8(tmp_path):
    p = tmp_path / "test.txt"
    p.write_bytes(b"\x80")
    ry.cd(tmp_path)
    with open("test.txt", "rb") as f:
        assert f.read() == b"\x80"
    # with python open and get error type
    with pytest.raises(UnicodeDecodeError):
        with open("test.txt", "r", encoding= "utf-8"
                  ) as f:
            f.read()
    with pytest.raises(UnicodeDecodeError):
        ry.read_text("test.txt")

def test_read_bytes(tmp_path):
    p = tmp_path / "test.txt"
    p.write_bytes(b"hello")
    ry.cd(tmp_path)
    assert ry.read_bytes("test.txt") == b"hello"

def test_read_file_missing(tmp_path):
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(
            str(p)
        )
    with pytest.raises(FileNotFoundError):
        ry.read_text(
            str(p)
        )

@pytest.mark.skip(reason="TODO: pathlike not implemented")
def test_read_file_missing_pathlike(tmp_path):
    p = tmp_path / "test.txt"
    ry.cd(tmp_path)
    with pytest.raises(FileNotFoundError):
        ry.read_bytes(
            p
        )
    with pytest.raises(FileNotFoundError):
        ry.read_text(
            p
        )

# use std::char::decode_utf16;
# use std::path::Path;
#
# use pyo3::exceptions::{PyFileNotFoundError, PyUnicodeDecodeError};
# use pyo3::prelude::*;
# use pyo3::types::{PyBytes, PyModule, PyString};
# use pyo3::{pyfunction, wrap_pyfunction, PyResult};
#
# pub mod fspath;
# #[pyfunction]
# pub fn read_vec_u8(py: Python<'_>, s: &str) -> PyResult<Vec<u8>> {
# let p = Path::new(s);
# let b = std::fs::read(p);
# match b {
#     Ok(b) => Ok(b),
# Err(e) => {
#     Err(
#         PyFileNotFoundError::new_err(format!("{}: {}", p.to_str().unwrap(),
#                                              format!("{}: {:?}", e.to_string(), p.to_str().unwrap())
# )))
# }
# }
# }
#
# #[pyfunction]
# pub fn read_bytes(py: Python<'_>, s: &PyString) -> PyResult<PyObject> {
# let bvec = read_vec_u8(py,
#                        s.to_str().unwrap(),
#                        )?;
# Ok(PyBytes::new(py, &bvec).into())
# // match bvec {
#               //     Ok(bvec) =>
# //     Err(e) => {
#                  //         Err(e)
#                  //         // let emsg = format!("{}: {:?}", e.to_string(), s.to_string());
# //         // let pye = PyFileNotFoundError::new_err(format!("read_bytes: {}", emsg));
# //         // panic!("{}", pye);
# //     }
# // }
# }
#
# #[pyfunction]
# pub fn read_text(py: Python<'_>, s: &PyString) -> PyResult<String> {
# let thingy = s.to_str().unwrap();
# let bvec = read_vec_u8(
#     py,
#     thingy,
# )?;
#
# // read_vec_u8(py, s).unwrap();
# // let s = String::from_utf8(bvec);
# let r = std::str::from_utf8(&*bvec);
#
# match r {
#     Ok(s) => Ok(s.to_string()),
# Err(e) => {
#     let decode_err = PyUnicodeDecodeError::new_utf8(
#     py, &*bvec, e,
# )
# .unwrap();
# Err(decode_err.into())
# }
# }
# }
#
# pub fn pymod(m: &PyModule) -> PyResult<()> {
#     m.add_function(wrap_pyfunction!(read_text, m)?)?;
# m.add_function(wrap_pyfunction!(read_bytes, m)?)?;
# m.add_class::<fspath::PyPath>()?;
#
# Ok(())
# }
