use jiff::{Unit, Zoned};

fn main() {
    pyo3_build_config::use_pyo3_cfgs();

    // OPT_LEVEL is available directly
    let opt_level =
        std::env::var("OPT_LEVEL").expect("OPT_LEVEL env var not found which is SUPER strange!");
    println!("cargo:rustc-env=OPT_LEVEL={opt_level}");

    // print every env var for debugging
    for (key, value) in std::env::vars() {
        println!("cargo:warning=env var: {key}={value}");
        eprintln!("cargo:warning=env var: {key}={value}");
    }

    if let Ok(lto) = std::env::var("CARGO_PROFILE_RELEASE_LTO") {
        println!("cargo:rustc-env=LTO={lto}");
    } else {
        println!("cargo:rustc-env=LTO=none");
    }
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
}
