use pyo3::pybacked::PyBackedStr;
use pyo3::{pyclass, pymethods, PyErr, PyRef, PyRefMut, PyResult};
use regex::{Regex};
use crate::py_captures::PyCaptures;

#[pyclass(name = "Match", frozen, module = "ryo3")]
#[derive(Debug)]
pub struct PyMatch {
    // #[allow(dead_code)]
    // mat: regex::Match<'static>pub(crate) ,
    // captures: regex::Captures<'static>,
    text: String,

    start: usize,
    end: usize,
}

#[pymethods]
impl PyMatch {
    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn as_str(&self) -> &str {
        self.text.as_str()
    }
    //
    // fn group(&self, i: usize) -> PyResult<String> {
    //     let msg = format!("not-implemented group({:?})", i);
    //     Err(
    //         PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //             msg
    //         )
    //     )
    // }
    //
    // fn groups(&self) -> PyResult<()> {
    //     let msg = format!("not-implemented groups({:?})");
    //     Err(
    //         PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //             msg
    //         )
    //     )
    // }
    //
    // fn groupdict(&self) -> PyResult<()> {
    //     let msg = format!("not-implemented groupdict({:?})");
    //     Err(
    //         PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //             msg
    //         )
    //     )
    // }
    //
    fn span(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    fn __str__(&self) -> String {
        format!("Match({:?})", self.text)
    }

    // fn __repr__(&self) -> String {
    //     format!("Match({:?})", self.mat.as_str())
    // }
}

#[pyclass(name = "Regex", frozen, module = "ryo3")]
#[derive(Clone, Debug)]
pub struct PyRegex(Regex);

#[pymethods]
impl PyRegex {
    #[new]
    fn new(pattern: &str) -> PyResult<Self> {
        Regex::new(pattern)
            .map(PyRegex)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn is_match(&self, text: &str) -> bool {
        self.0.is_match(text)
    }

    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Regex({:?})", self.0)
    }

    fn __eq__(&self, other: &PyRegex) -> bool {
        self.0.as_str() == other.0.as_str()
    }

    fn __ne__(&self, other: &PyRegex) -> bool {
        self.0.as_str() != other.0.as_str()
    }


    fn captures(&self, text: &str) -> PyResult<Option<PyCaptures>> {
        if let Some(caps) = self.0.captures(text) {
            // Extract all positional groups
            let mut groups = Vec::new();
            for g in 0..caps.len() {
                let sub = caps.get(g).map(|m| m.as_str().to_string());
                groups.push(sub);
            }

            // Extract named groups
            let mut named = std::collections::HashMap::new();
            self.0.capture_names().for_each(|name| {
                match name {
                    Some(name) => {
                        let sub = caps.name(name).map(|m| m.as_str().to_string());
                        named.insert(name.to_string(), sub);
                    }
                    None => {}
                }
                // named.insert(name.to_string(), None);
            });

            // for name in caps.names() {
            //     if let Some(m) = caps.name(name) {
            //         named.insert(name.to_string(), Some(m.as_str().to_string()));
            //     } else {
            //         named.insert(name.to_string(), None);
            //     }
            // }

            Ok(Some(PyCaptures { groups, named }))
        } else {
            Ok(None)
        }
    }
    // fn captures(&self, text: &str) -> PyResult<()> {
    //     let msg = format!("not-implemented Captures({:?})", self.0.captures(text));
    //     Err(PyErr::new::<pyo3::exceptions::PyNotImplementedError, _>(
    //         msg,
    //     ))
    // }

    fn find_iter(&self, text: PyBackedStr) -> PyResult<PyMatches> {
        // let hay_string = text.to_string();
        // let iter =reg.find_iter(&hay_string);
        Ok(PyMatches {
            re: self.clone(),
            text: text,
            pos: 0,
        })
    }
}

#[pyclass]
pub struct PyMatches {
    re: PyRegex,       // contains a `regex::Regex`
    text: PyBackedStr, // owned text
    pos: usize,        // where we left off in the text
}

#[pymethods]
impl PyMatches {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyMatch> {
        if let Some(m) = slf.re.0.find_at(&slf.text, slf.pos) {
            let start = m.start();
            let end = m.end();
            // Convert the slice to String immediately,
            // so we drop the immutable borrow before we mutate `slf`.
            let matched_str = slf.text[start..end].to_string();

            // Now we can safely mutate `slf`
            slf.pos = end;

            Some(PyMatch {
                text: matched_str,
                start,
                end,
            })
        } else {
            None
        }
    }
}
