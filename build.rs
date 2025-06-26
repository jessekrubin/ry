use jiff::{Unit, Zoned};

fn main() {
    pyo3_build_config::use_pyo3_cfgs();

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
