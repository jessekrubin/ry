use std::io::{self, Read, Write, stderr, stdout};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread::{self};

use pyo3::prelude::*;
use pyo3::pyfunction;
use pyo3::types::PyTuple;
use tracing::warn;

use super::done::Done;
use super::pydone::PyDone;

// use serde::{Deserialize, Serialize};
// use tracing::instrument::WithSubscriber;

fn communicate_tee<W: Write + Send + 'static>(
    mut stream: impl Read,
    sender: &mpsc::Sender<Vec<u8>>,
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
fn communicate(
    mut stream: impl Read,
    sender: &mpsc::Sender<Vec<u8>>,
    buf_size: usize,
    collect: bool,
) -> io::Result<()> {
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

    Ok(())
}

#[pyfunction]
#[pyo3(signature = (* popenargs, tee = false, capture = true, timeout = None, check = false))]
pub fn run(
    popenargs: &Bound<'_, PyTuple>,
    tee: Option<bool>,
    capture: Option<bool>,
    timeout: Option<u64>,
    check: Option<bool>,
) -> PyResult<PyDone> {
    // warn that timeout and check are not implemented
    if timeout.is_some() {
        warn!("Warning: timeout is not implemented");
    }
    if check.is_some() {
        warn!("Warning: check is not implemented");
    }
    let popenargs = popenargs.extract::<Vec<String>>()?;
    let collect = capture.unwrap_or(true);
    // split the popenargs into the command and the args
    let Some(cmd_str) = popenargs.first() else {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "popenargs must have at least one element",
        ));
    };
    let cmd_args = popenargs[1..].to_vec();
    let mut binding = Command::new(cmd_str);
    let cmd = binding
        .args(cmd_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let (tx_out, rx_out) = mpsc::channel();
    let (tx_err, rx_err) = mpsc::channel();

    // Now, execute the command
    let mut child = cmd.spawn().expect("failed to execute child");

    // let timeout_duration = timeout.map(Duration::from_secs);
    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");
    let thread_out = if tee.unwrap_or(false) {
        thread::spawn(move || communicate_tee(child_out, &tx_out, stdout(), 4096, collect).unwrap())
    } else {
        thread::spawn(move || communicate(child_out, &tx_out, 4096, collect).unwrap())
    };

    // let thread_out = thread::spawn(move || communicate(child_out, tx_out).unwrap());
    let thread_err = if tee.unwrap_or(false) {
        thread::spawn(move || communicate_tee(child_err, &tx_err, stderr(), 4096, collect).unwrap())
    } else {
        thread::spawn(move || communicate(child_err, &tx_err, 4096, collect).unwrap())
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
        stderr_vector,
    );
    let pydone = PyDone::from(done);
    Ok(pydone)
}
