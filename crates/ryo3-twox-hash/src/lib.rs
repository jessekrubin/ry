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
    m.add_class::<xxhash32::RyXxHash32>()?;
    #[cfg(feature = "xxhash64")]
    m.add_class::<xxhash64::RyXxHash64>()?;
    #[cfg(feature = "xxhash3_64")]
    m.add_class::<xxhash3_64::RyXxHash3_64>()?;
    #[cfg(feature = "xxhash3_128")]
    m.add_class::<xxhash3_128::RyXxHash3_128>()?;
    Ok(())
}
