#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
#[cfg(feature = "xxhash32")]
pub mod xxhash32;
#[cfg(feature = "xxhash3_128")]
pub mod xxhash3_128;
#[cfg(feature = "xxhash3_64")]
pub mod xxhash3_64;
#[cfg(feature = "xxhash64")]
pub mod xxhash64;

// either xxhash3_128 or xxhash3_64 depending on which is enabled
#[cfg(any(feature = "xxhash3_128", feature = "xxhash3_64"))]
pub(crate) mod xxhash3_secret;

#[cfg_attr(
    not(any(
        feature = "xxhash32",
        feature = "xxhash64",
        feature = "xxhash3_64",
        feature = "xxhash3_128"
    )),
    expect(unused_variables)
)]
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "xxhash32")]
    m.add_class::<xxhash32::PyXxHash32>()?;
    #[cfg(feature = "xxhash64")]
    m.add_class::<xxhash64::PyXxHash64>()?;
    #[cfg(feature = "xxhash3_64")]
    m.add_class::<xxhash3_64::PyXxHash3_64>()?;
    #[cfg(feature = "xxhash3_128")]
    m.add_class::<xxhash3_128::PyXxHash3_128>()?;
    Ok(())
}

#[cfg_attr(
    not(any(
        feature = "xxhash32",
        feature = "xxhash64",
        feature = "xxhash3_64",
        feature = "xxhash3_128"
    )),
    expect(unused_variables)
)]
pub fn pysubmod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[cfg(feature = "xxhash32")]
    xxhash32::pymod_add(m)?;
    #[cfg(feature = "xxhash64")]
    xxhash64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_64")]
    xxhash3_64::pymod_add(m)?;
    #[cfg(feature = "xxhash3_128")]
    xxhash3_128::pymod_add(m)?;
    Ok(())
}
