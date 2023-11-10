use pyo3::prelude::*;
use std::{
    fs::File,
    io::{stderr, stdout, Read, Write},
    process::{Command, Stdio},
    thread,
};

pub struct Done {
    pub command: String,
    pub returncode: i32,
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
}

// use std::io::{self, Read, Write};
// use std::process::{Command, Stdio};
// use std::thread;

#[pyfunction]
pub fn run() -> std::io::Result<(Vec<u8>, Vec<u8>)> {
    // let mut cmd: &mut Command = Command::new("python").arg("--version")
    //     .stdout(Stdio::piped())
    //     .stderr(Stdio::piped());
    // let mut child =  cmd
    //     .spawn()
    //     .expect("failed to execute child");
    // Create the Command object
    let mut cmd = Command::new("python");
    cmd.arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    // Now, execute the command
    let mut child = cmd.spawn().expect("failed to execute child");
    fn communicate(mut stream: impl Read) -> std::io::Result<Vec<u8>> {
        let mut output_vector = Vec::new();
        let mut buf = [0u8; 1024];

        loop {
            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }

            let buf = &buf[..num_read];
            output_vector.extend_from_slice(buf);
        }

        Ok(output_vector)
    }

    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");

    let thread_out = thread::spawn(move || communicate(child_out));
    let thread_err = thread::spawn(move || communicate(child_err));

    let stdout_vector = thread_out.join().unwrap()?;
    let stderr_vector = thread_err.join().unwrap()?;

    let ecode = child.wait().expect("failed to wait on child");
    assert!(ecode.success());

    // Return the captured stdout and stderr vectors
    // print the output
    println!(
        "stdout: {}",
        String::from_utf8(stdout_vector.clone()).unwrap()
    );
    println!(
        "stderr: {}",
        String::from_utf8(stderr_vector.clone()).unwrap()
    );
    Ok((stdout_vector, stderr_vector))
}

#[pyfunction]
pub fn run2() {
    let mut child = Command::new("python")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    fn communicate(
        mut stream: impl Read,
        output_vector: &mut Vec<u8>,
        mut output: impl Write,
    ) -> std::io::Result<()> {
        let mut buf = [0u8; 1024];
        loop {
            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }

            let buf = &buf[..num_read];
            output_vector.extend_from_slice(buf);
            output.write_all(buf)?;
        }

        Ok(())
    }

    let mut stdout_vector: Vec<u8> = Vec::new();
    let mut stderr_vector: Vec<u8> = Vec::new();

    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");

    let thread_out = thread::spawn(move || {
        communicate(child_out, &mut stdout_vector, stdout())
            .expect("error communicating with child stdout")
    });
    let thread_err = thread::spawn(move || {
        communicate(child_err, &mut stderr_vector, stderr())
            .expect("error communicating with child stderr")
    });

    thread_out.join().unwrap();
    thread_err.join().unwrap();

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());

    // print the output
    // println!("stdout: {}", String::from_utf8(stdout_vector).unwrap());
    // println!("stderr: {}", String::from_utf8(stderr_vector).unwrap());
    // At this point, stdout_vector and stderr_vector contain the output
}
#[pyfunction]
pub fn run1() {
    let mut child = Command::new("python")
        .arg("--version")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    fn communicate(
        mut stream: impl Read,
        filename: &'static str,
        mut output: impl Write,
    ) -> std::io::Result<()> {
        let mut file = File::create(filename)?;

        let mut buf = [0u8; 1024];
        loop {
            let num_read = stream.read(&mut buf)?;
            if num_read == 0 {
                break;
            }

            let buf = &buf[..num_read];
            file.write_all(buf)?;
            output.write_all(buf)?;
        }

        Ok(())
    }

    let child_out = std::mem::take(&mut child.stdout).expect("cannot attach to child stdout");
    let child_err = std::mem::take(&mut child.stderr).expect("cannot attach to child stderr");

    let thread_out = thread::spawn(move || {
        communicate(child_out, "stdout.txt", stdout())
            .expect("error communicating with child stdout")
    });
    let thread_err = thread::spawn(move || {
        communicate(child_err, "stderr.txt", stderr())
            .expect("error communicating with child stderr")
    });

    thread_out.join().unwrap();
    thread_err.join().unwrap();

    let ecode = child.wait().expect("failed to wait on child");

    assert!(ecode.success());
}
