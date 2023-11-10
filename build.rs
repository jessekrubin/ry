// use chrono::prelude::*;
use chrono::Local;

fn main() {
  pyo3_build_config::use_pyo3_cfgs();
  // if let Some(true) = version_check::supports_feature("no_coverage") {
  //     println!("cargo:rustc-cfg=has_no_coverage");
  // }
  // add the current profile as an env var
  println!(
      "cargo:rustc-env=PROFILE={}",
      std::env::var("PROFILE").unwrap()
  );

  // add timestamp as an env var
  println!(
      "cargo:rustc-env=BUILD_TIMESTAMP={}",
      Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
  );
}
