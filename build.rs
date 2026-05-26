use std::process::Command;

use jiff::{Unit, Zoned};

fn git_stdout(args: &[&str]) -> Option<String> {
    let output = Command::new("git").args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn main() {
    pyo3_build_config::use_pyo3_cfgs();
    println!("cargo:rerun-if-env-changed=RY_GIT_SHA");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs");
    println!("cargo:rerun-if-changed=.git/packed-refs");

    // OPT_LEVEL is available directly
    let opt_level =
        std::env::var("OPT_LEVEL").expect("OPT_LEVEL env var not found which is SUPER strange!");
    println!("cargo:rustc-env=OPT_LEVEL={opt_level}");

    // env var build profile
    let profile =
        std::env::var("PROFILE").expect("PROFILE env var not found which is SUPER strange!");
    println!("cargo:rustc-env=PROFILE={profile}");

    let build_ts = Zoned::now()
        .round(Unit::Second)
        .expect("oh no, build time error");
    // build timestamp
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_ts}");
    // set the TARGET
    let target = std::env::var("TARGET").expect("TARGET env var not found");
    println!("cargo:rustc-env=TARGET={target}");

    let git_sha = std::env::var("RY_GIT_SHA")
        .ok()
        .filter(|sha| !sha.trim().is_empty())
        .or_else(|| git_stdout(&["rev-parse", "HEAD"]))
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=GIT_SHA={git_sha}");
}
