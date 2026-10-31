use std::path::Path;
use std::process::Command;

use jiff::{Unit, Zoned};

const UNKNOWN: &str = "unknown";

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

/// stdout then stderr (msvc `cl.exe` prints its banner to stderr)
fn run(cmd: &mut Command) -> Option<String> {
    let output = cmd.output().ok()?;
    let mut s = String::from_utf8_lossy(&output.stdout).into_owned();
    s.push_str(&String::from_utf8_lossy(&output.stderr));
    Some(s)
}

fn first_line(s: &str) -> Option<String> {
    s.lines()
        .map(str::trim)
        .find(|l| !l.is_empty())
        .map(ToOwned::to_owned)
}

/// rustc version + llvm version via `rustc -vV`
fn rustc_info() -> (String, String) {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let out = run(Command::new(rustc).arg("-vV")).unwrap_or_default();
    let version = first_line(&out).unwrap_or_else(|| UNKNOWN.to_owned());
    let llvm = out
        .lines()
        .find_map(|l| l.strip_prefix("LLVM version:"))
        .map_or_else(|| UNKNOWN.to_owned(), |v| v.trim().to_owned());
    (version, llvm)
}

/// c compiler as resolved by the `cc` crate (same one c-deps use)
fn cc_info() -> String {
    let Ok(tool) = cc::Build::new().try_get_compiler() else {
        return UNKNOWN.to_owned();
    };
    if let Some(desc) = zig_wrapper_desc(tool.path()) {
        return desc;
    }
    let mut cmd = Command::new(tool.path());
    // tool env has the msvc INCLUDE/LIB/PATH setup
    cmd.envs(tool.get_envs());
    // msvc `cl.exe` has no `--version`; prints its banner w/ no args
    if !tool.is_like_msvc() {
        cmd.arg("--version");
    }
    run(&mut cmd)
        .and_then(|out| first_line(&out))
        .unwrap_or_else(|| UNKNOWN.to_owned())
}

/// cargo-zigbuild (`maturin --zig`) wrapper -> `zig cc (cargo-zigbuild 0.23.1)`
fn zig_wrapper_desc(path: &Path) -> Option<String> {
    let fname = path.file_name()?.to_str()?;
    let kind = if fname.starts_with("zigcxx") {
        "zig c++"
    } else if fname.starts_with("zigcc") {
        "zig cc"
    } else {
        return None;
    };
    // .../cargo-zigbuild/<version>/wrappers/...
    let zigbuild_version = path
        .components()
        .map(|c| c.as_os_str().to_string_lossy())
        .skip_while(|c| c != "cargo-zigbuild")
        .nth(1);
    Some(zigbuild_version.map_or_else(
        || kind.to_owned(),
        |v| format!("{kind} (cargo-zigbuild {v})"),
    ))
}

/// linker from `RUSTC_LINKER` (cargo `target.<triple>.linker`) or `-C linker=` in rustflags
fn linker_info(rustflags: &[&str]) -> String {
    let linker = std::env::var("RUSTC_LINKER").ok().or_else(|| {
        rustflags.windows(2).find_map(|w| {
            w[0].strip_prefix("-Clinker=")
                .or_else(|| w[1].strip_prefix("linker=").filter(|_| w[0] == "-C"))
                .map(ToOwned::to_owned)
        })
    });
    match linker {
        Some(l) => zig_wrapper_desc(Path::new(&l)).unwrap_or(l),
        None => "default".to_owned(),
    }
}

fn main() {
    pyo3_build_config::use_pyo3_cfgs();
    println!("cargo:rerun-if-env-changed=RY_GIT_SHA");
    println!("cargo:rerun-if-changed=python/ry/.git-sha");
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
        .or_else(|| std::fs::read_to_string("python/ry/.git-sha").ok())
        .map(|sha| sha.trim().to_owned())
        .filter(|sha| !sha.is_empty())
        .or_else(|| git_stdout(&["rev-parse", "HEAD"]))
        .unwrap_or_else(|| UNKNOWN.to_owned());
    println!("cargo:rustc-env=GIT_SHA={git_sha}");

    // toolchain
    let host = std::env::var("HOST").expect("HOST env var not found");
    println!("cargo:rustc-env=HOST={host}");
    let (rustc_version, llvm_version) = rustc_info();
    println!("cargo:rustc-env=RUSTC_VERSION={rustc_version}");
    println!("cargo:rustc-env=LLVM_VERSION={llvm_version}");
    println!("cargo:rustc-env=CC_INFO={}", cc_info());
    let encoded_rustflags = std::env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let rustflags: Vec<&str> = encoded_rustflags
        .split('\x1f')
        .filter(|f| !f.is_empty())
        .collect();
    println!("cargo:rustc-env=LINKER={}", linker_info(&rustflags));
    println!("cargo:rustc-env=RUSTFLAGS={}", rustflags.join(" "));
}
