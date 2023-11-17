use std::io::{self,Read, Write, stdout, stderr};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::fs::File;
use std::process::{Stdio, Command};

use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::types::PyBytes;
use serde::{Deserialize, Serialize};
use tracing::instrument::WithSubscriber;

#[derive(Debug, Deserialize, Serialize)]
pub struct Done {
    pub args: Vec<String>,
    pub returncode: i32,

    #[serde(with = "serde_bytes")]
    pub stdout: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub stderr: Vec<u8>,
}

impl Done {
    pub fn new(
        args: Vec<String>,
        returncode: i32,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    ) -> Self {
        Self {
            args,
            returncode,
            stdout,
            stderr,
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct PyDone {
    done: Done,
}

// use std::io::{self, Read, Write};
// use std::process::{Command, Stdio};
// use std::thread;
// #[pyo3(signature = (*popenargs, input=None, capture_output=false, timeout=None, check=false))]
#[pymethods]
impl PyDone {
    #[new]
    fn new(
        args: Vec<String>,
        returncode: i32,
        stdout: Vec<u8>,
        stderr: Vec<u8>,
    ) -> Self {
        let d = Done::new(
            args,
            returncode,
            stdout,
            stderr,
        );
        Self {
            done: d,
        }
    }


    fn __repr__(&self) -> PyResult<String> {

        // format with escaped chars
        // let args_fmt = self.done.args.into_iter().map(
        //     |s| serde_json::to_string(&s).unwrap()
        // ).collect::<Vec<String>>().join(", ");
        //
        // let stdout_fmt = String::from_utf8(self.done.stdout.clone()).unwrap();
        // let stderr_fmt = String::from_utf8(self.done.stderr.clone()).unwrap();
        // let s = format!("Done(args=[{}], stdout: {}, stderr: {})",
        //                 args_fmt,
        //                 stdout_fmt,
        //                 stderr_fmt,
        // );
        let s = serde_json::to_string(&self.done).unwrap();
        Ok(s)
    }
    fn __str__(&self) -> PyResult<String> {
        self.__repr__()
    }

    #[getter]
    fn stdout<'py>(&'py self, py: Python<'py>) -> &'py PyBytes {
        PyBytes::new(py, &self.done.stdout)
    }
}

impl From<Done> for PyDone {
    fn from(done: Done) -> Self {
        Self {
            done,
        }
    }
}

// communicaTEE as in 'tee' the stuff to a "file"
fn communicate_tee<W: Write + Send + 'static>(
    mut stream: impl Read,
    sender: mpsc::Sender<Vec<u8>>,
    mut writer: W,
    buf_size: usize,
    collect: bool,
) -> io::Result<()> {
    let mut buf = vec![0; buf_size];

    loop {
        let num_read = stream.read(&mut buf)?;
        if num_read == 0 {
            break;
        }
        // Write to the writer
        writer.write_all(&buf[..num_read])?;
        if collect {
            // Send the data through the channel
            sender.send(buf[..num_read].to_vec()).unwrap();
        }
    }
    Ok(())
}

// Your existing function with modifications to use channels
fn communicate(mut stream: impl Read, sender: mpsc::Sender<Vec<u8>>, buf_size: usize, collect: bool) -> io::Result<()> {
    let mut buf = vec![0; buf_size];

    loop {
        let num_read = stream.read(&mut buf)?;
        if num_read == 0 {
            break;
        }

        // Send the data through the channel
        sender.send(buf[..num_read].to_vec()).unwrap();

        if collect {
            // Send the data through the channel
            sender.send(buf[..num_read].to_vec()).unwrap();
        }
    }
    //
    // let mut buf = [0; 1024];
    //
    // loop {
    //     let num_read = stream.read(&mut buf)?;
    //     if num_read == 0 {
    //         break;
    //     }
    //
    //     // Send the data through the channel
    //     sender.send(buf[..num_read].to_vec()).unwrap();
    // }
    Ok(())
}

// // Builder struct for configuration
// pub struct CommunicatorBuilder<W: Write + Send + 'static> {
//     writer: Option<W>,
//     buffer_size: Option<usize>,
// }
//
// impl<W: Write + Send + 'static> CommunicatorBuilder<W> {
//     // Constructor for the builder
//     pub fn new() -> Self {
//         CommunicatorBuilder {
//             writer: None,
//             buffer_size: None,
//         }
//     }
//
//     // Method to set the writer
//     pub fn writer(mut self, writer: W) -> Self {
//         self.writer = Some(writer);
//         self
//     }
//
//     // Method to set the buffer size
//     pub fn buffer_size(mut self, size: usize) -> Self {
//         self.buffer_size = Some(size);
//         self
//     }
//
//     pub fn new_with_writer( writer: W) -> Self {
//         CommunicatorBuilder {
//             writer: Some(writer),
//             buffer_size: None,
//         }
//     }
//
//     // Build method
//     pub fn build(self) -> Communicator<W> {
//         Communicator {
//             writer: self.writer,
//             buffer_size: self.buffer_size.unwrap_or(1024), // Default size
//         }
//     }
// }
//
// // Communicator struct
// pub struct Communicator<W: Write + Send + 'static> {
//     writer: Option<W>,
//     buffer_size: usize,
// }
//
// impl<W: Write + Send + 'static> Communicator<W> {
//     // Function to start communication
//     pub fn start(
//         &self,
//         mut stream: impl Read + Send + 'static,
//         sender: mpsc::Sender<Vec<u8>>,
//     ) -> JoinHandle<()> {
//         let buffer_size = self.buffer_size;
//         let mut writer = self.writer.clone();
//
//         thread::spawn(move || {
//             let mut buf = vec![0; buffer_size];
//             loop {
//                 let num_read = stream.read(&mut buf).expect("Read error");
//                 if num_read == 0 {
//                     break;
//                 }
//                 if let Some(ref mut w) = writer {
//                     w.write_all(&buf[..num_read]).expect("Write error");
//                 }
//                 sender.send(buf[..num_read].to_vec()).expect("Send error");
//             }
//         })
//     }
// }
//
#[pyfunction]
#[pyo3(signature = (
* py_args,
tee = false,
capture = true,
timeout = None,
check = false,

// popenargs,
// input,
// capture_output,
// timeout,
// check,
))]
pub fn run(py: Python,
           py_args: &PyTuple,
           tee: Option<bool>,
           capture: Option<bool>,
           timeout: Option<u64>,
           check: Option<bool>,

           // popenargs: Vec<String>,
           // input: Option<String>,
           // capture_output: bool,
           // timeout: Option<u64>,
           // check: bool,
) -> PyResult<
    PyDone
> {
    let popenargs = py_args.extract::<Vec<String>>()?;
    let collect = capture.unwrap_or(true);
    // split the popenargs into the command and the args
    let cmd_str = match popenargs.get(0) {
        Some(s) => s,
        None => {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "popenargs must have at least one element",
            ));
        }
    };
    let cmd_args = popenargs[1..].to_vec();
    let mut binding = Command::new(cmd_str);
    let mut cmd = binding.args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Now, execute the command
    let mut child = cmd.spawn().expect("failed to execute child");

    let (tx_out, rx_out) = mpsc::channel();
    let (tx_err, rx_err) = mpsc::channel();

    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");

    let thread_out = if tee.unwrap_or(false) {
        thread::spawn(move || communicate_tee(child_out, tx_out, stdout(), 4096, collect).unwrap())
    } else {
        thread::spawn(move || communicate(child_out, tx_out, 4096, collect).unwrap())
    };

    // let thread_out = thread::spawn(move || communicate(child_out, tx_out).unwrap());
    let thread_err =  if tee.unwrap_or(false) {
        thread::spawn(move || communicate_tee(child_err, tx_err, stderr(), 4096, collect).unwrap())
    } else {
        thread::spawn(move || communicate(child_err, tx_err, 4096, collect).unwrap())
    };

    let stdout_vector: Vec<u8> = rx_out.iter().flatten().collect();
    let stderr_vector: Vec<u8> = rx_err.iter().flatten().collect();

    thread_out.join().expect("Thread panicked");
    thread_err.join().expect("Thread panicked");

    let ecode = child.wait().expect("failed to wait on child");
    let done = Done::new(
        vec!["python".to_string(), "--version".to_string()],
        ecode.code().unwrap(),
        stdout_vector,
        stderr_vector
    );
    let pydone = PyDone::from(done);
    Ok(pydone)
}

