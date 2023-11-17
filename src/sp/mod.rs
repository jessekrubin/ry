use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::sync::mpsc;
use tracing::instrument::WithSubscriber;

mod done;
mod pydone;
pub mod run;

// pub mod run;
// rexport run from ./run.rs
// pub use run::run;

// #[pyclass]
// #[derive(Debug)]
// pub struct PyDone {
//     done: Done,
// }
//
// // use std::io::{self, Read, Write};
// // use std::process::{Command, Stdio};
// // use std::thread;
// // #[pyo3(signature = (*popenargs, input=None, capture_output=false, timeout=None, check=false))]
// #[pymethods]
// impl PyDone {
//     #[new]
//     fn new(
//         args: Vec<String>,
//         returncode: i32,
//         stdout: Vec<u8>,
//         stderr: Vec<u8>,
//     ) -> Self {
//         let d = Done::new(
//             args,
//             returncode,
//             stdout,
//             stderr,
//         );
//         Self {
//             done: d,
//         }
//     }
//
//
//     fn __repr__(&self) -> PyResult<String> {
//
//         // format with escaped chars
//         // let args_fmt = self.done.args.into_iter().map(
//         //     |s| serde_json::to_string(&s).unwrap()
//         // ).collect::<Vec<String>>().join(", ");
//         //
//         // let stdout_fmt = String::from_utf8(self.done.stdout.clone()).unwrap();
//         // let stderr_fmt = String::from_utf8(self.done.stderr.clone()).unwrap();
//         // let s = format!("Done(args=[{}], stdout: {}, stderr: {})",
//         //                 args_fmt,
//         //                 stdout_fmt,
//         //                 stderr_fmt,
//         // );
//         let s = serde_json::to_string(&self.done).unwrap();
//         Ok(s)
//     }
//     fn __str__(&self) -> PyResult<String> {
//         self.__repr__()
//     }
//
//     #[getter]
//     fn stdout<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
//         PyBytes::new(py, &self.done.stdout)
//     }
// }

// communicaTEE as in 'tee' the stuff to a "file"
