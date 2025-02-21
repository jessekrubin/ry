use jiff::{Unit, Zoned};

fn main() {
    // env var build profile
    let profile =
        std::env::var("PROFILE").expect("PROFILE env var not found which is SUPER strange!");
    println!("cargo:rustc-env=PROFILE={profile}");

    let build_ts = Zoned::now()
        .round(Unit::Second)
        .expect("oh no, build time error");
    // build timestamp
    println!("cargo:rustc-env=BUILD_TIMESTAMP={build_ts}");
}
