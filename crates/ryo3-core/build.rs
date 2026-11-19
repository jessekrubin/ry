use pyo3_build_config::{PythonAbiKind, PythonImplementation};

fn main() {
    pyo3_build_config::use_pyo3_cfgs();

    // inline int reading ~ like orjson only for 64-bit cpython (non-abi3) where
    // the `PyLongObject` layout + 30-bit digits are a given
    println!("cargo::rustc-check-cfg=cfg(ryo3_pylong_layout)");
    let cfg = pyo3_build_config::get();
    if cfg.implementation() == PythonImplementation::CPython
        && matches!(cfg.target_abi().kind(), PythonAbiKind::VersionSpecific(_))
        && cfg.pointer_width() == Some(64)
    {
        println!("cargo::rustc-cfg=ryo3_pylong_layout");
    }
}
