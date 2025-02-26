#![allow(clippy::needless_pass_by_value)]
use pyo3::types::{PyBytes, PyModule, PyModuleMethods};
use pyo3::{pyfunction, wrap_pyfunction, Bound, PyResult, Python};
use xxhash_rust::const_xxh3::xxh3_128_with_seed as const_xxh3_128_with_seed;
use xxhash_rust::const_xxh3::xxh3_64_with_seed as const_xxh3_64_with_seed;
use xxhash_rust::const_xxh32::xxh32 as const_xxh32;
use xxhash_rust::const_xxh64::xxh64 as const_xxh64;

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u32>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh32(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyResult<u32> {
    Ok(const_xxh32(b.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh32_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u32>) -> PyResult<String> {
    Ok(format!(
        "{:08x}",
        const_xxh32(b.as_ref(), seed.unwrap_or(0))
    ))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh64_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh64(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh64_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh64(b.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh64_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:016x}",
        const_xxh64(b.as_ref(), seed.unwrap_or(0))
    ))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_64_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh3_64_with_seed(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_64_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    Ok(const_xxh3_64_with_seed(b.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_64_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:016x}",
        const_xxh3_64_with_seed(b.as_ref(), seed.unwrap_or(0))
    ))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_128_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh3_128_with_seed(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_128_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    Ok(const_xxh3_128_with_seed(b.as_ref(), seed.unwrap_or(0)))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_128_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    Ok(format!(
        "{:032x}",
        const_xxh3_128_with_seed(b.as_ref(), seed.unwrap_or(0))
    ))
}

// =======
// ALIASES
// =======
#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh3_64_with_seed(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u64> {
    xxh3_64_intdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh3_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    xxh3_64_hexdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh128_digest(
    py: Python<'_>,
    b: ryo3_bytes::PyBytes,
    seed: Option<u64>,
) -> PyResult<Bound<'_, PyBytes>> {
    let v = const_xxh3_128_with_seed(b.as_ref(), seed.unwrap_or(0));
    Ok(PyBytes::new(py, &v.to_be_bytes()))
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh128_intdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<u128> {
    xxh3_128_intdigest(b, seed)
}

#[pyfunction]
#[pyo3(signature = (b, seed = None))]
pub fn xxh128_hexdigest(b: ryo3_bytes::PyBytes, seed: Option<u64>) -> PyResult<String> {
    xxh3_128_hexdigest(b, seed)
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
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
