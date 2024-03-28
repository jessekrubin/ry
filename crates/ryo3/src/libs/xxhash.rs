use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::{wrap_pyfunction, PyResult, Python};
use xxhash_rust::const_xxh3::xxh3_128_with_seed as const_xxh3_128_with_seed;
use xxhash_rust::const_xxh3::xxh3_64_with_seed as const_xxh3_64_with_seed;
use xxhash_rust::const_xxh32::xxh32 as const_xxh32;
use xxhash_rust::const_xxh64::xxh64 as const_xxh64;

// use xxhash_rust::const_xxh64::xxh64 as const_xxh64;

#[pyfunction]
fn xxh32_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u32>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh32(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
fn xxh32_intdigest(b: &[u8], seed: Option<u32>) -> PyResult<u32> {
    Ok(const_xxh32(b, seed.unwrap_or(0)))
}

#[pyfunction]
fn xxh32_hexdigest(b: &[u8], seed: Option<u32>) -> PyResult<String> {
    Ok(format!("{:x}", const_xxh32(b, seed.unwrap_or(0))))
}

#[pyfunction]
fn xxh64_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh64(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
fn xxh64_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh64(b, seed.unwrap_or(0)))
}

#[pyfunction]
fn xxh64_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!("{:x}", const_xxh64(b, seed.unwrap_or(0))))
}

#[pyfunction]
fn xxh3_64_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh3_64_with_seed(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
fn xxh3_64_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh3_64_with_seed(b, seed.unwrap_or(0)))
}

#[pyfunction]
fn xxh3_64_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:x}",
        const_xxh3_64_with_seed(b, seed.unwrap_or(0))
    ))
}

#[pyfunction]
fn xxh3_128_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    let v = const_xxh3_128_with_seed(b, seed.unwrap_or(0));
    Ok(PyBytes::new_bound(py, &v.to_be_bytes()))
}

#[pyfunction]
fn xxh3_128_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u128> {
    Ok(const_xxh3_128_with_seed(b, seed.unwrap_or(0)))
}

#[pyfunction]
fn xxh3_128_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:x}",
        const_xxh3_128_with_seed(b, seed.unwrap_or(0))
    ))
}

// =======
// ALIASES
// =======
#[pyfunction]
fn xxh3_digest<'a>(py: Python<'a>, b: &'a [u8], seed: Option<u64>) -> PyResult<Bound<'a, PyBytes>> {
    xxh3_64_digest(py, b, seed)
}

#[pyfunction]
fn xxh3_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u64> {
    xxh3_64_intdigest(b, seed)
}

#[pyfunction]
fn xxh3_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    xxh3_64_hexdigest(b, seed)
}

#[pyfunction]
fn xxh128_digest<'a>(
    py: Python<'a>,
    b: &'a [u8],
    seed: Option<u64>,
) -> PyResult<Bound<'a, PyBytes>> {
    xxh3_128_digest(py, b, seed)
}

#[pyfunction]
fn xxh128_intdigest(b: &[u8], seed: Option<u64>) -> PyResult<u128> {
    xxh3_128_intdigest(b, seed)
}

#[pyfunction]
fn xxh128_hexdigest(b: &[u8], seed: Option<u64>) -> PyResult<String> {
    xxh3_128_hexdigest(b, seed)
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(xxh32_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh32_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh64_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_64_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_64_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh3_128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_128_hexdigest, m)?)?;

    // aliases
    m.add_function(wrap_pyfunction!(xxh3_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh3_hexdigest, m)?)?;

    m.add_function(wrap_pyfunction!(xxh128_digest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_intdigest, m)?)?;
    m.add_function(wrap_pyfunction!(xxh128_hexdigest, m)?)?;

    Ok(())
}
