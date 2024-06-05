use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

const KILOBYTE: u64 = 1024;
const MEGABYTE: u64 = KILOBYTE * 1024;
const GIGABYTE: u64 = MEGABYTE * 1024;
const TERABYTE: u64 = GIGABYTE * 1024;
const PETABYTE: u64 = TERABYTE * 1024;
const EXABYTE: u64 = PETABYTE * 1024;

fn format_size(nbytes: u64, unit: &str, divisor: u64, precision: usize) -> String {
    let integer_part = nbytes / divisor;
    let fractional_part = (nbytes % divisor) * 10 / divisor;
    if precision == 0 {
        format!("{integer_part} {unit}")
    } else {
        format!("{integer_part}.{fractional_part:0precision$} {unit}")
    }
}

#[must_use]
pub fn nbytes_u64(nbytes: u64, precision: Option<usize>) -> String {
    let precision = precision.unwrap_or(1);
    match nbytes {
        n if n < KILOBYTE => {
            if n == 1 {
                "1 byte".to_string()
            } else {
                format!("{n} bytes")
            }
        }
        n if n < MEGABYTE => format_size(n, "KiB", KILOBYTE, precision),
        n if n < GIGABYTE => format_size(n, "MiB", MEGABYTE, precision),
        n if n < TERABYTE => format_size(n, "GiB", GIGABYTE, precision),
        n if n < PETABYTE => format_size(n, "TiB", TERABYTE, precision),
        n if n < EXABYTE => format_size(n, "PiB", PETABYTE, precision),
        n => format_size(n, "EiB", EXABYTE, precision),
    }
}

fn nbytes_i64(nbytes: i64, precision: Option<usize>) -> Result<String, String> {
    // abs value
    if nbytes < 0 {
        // abs it  and then convert to u64
        let ubytes_u64 = nbytes.unsigned_abs();
        Ok(format!("-{}", nbytes_u64(ubytes_u64, precision)))
    } else {
        // convert to u64
        let ubytes_u64 = nbytes.unsigned_abs();
        Ok(nbytes_u64(ubytes_u64, precision).to_string())
    }
}

// TODO: Fix to handle negative numbers
#[pyfunction]
#[pyo3(name = "fmt_nbytes")]
pub fn fmt_nbytes(nbytes: i64) -> PyResult<String> {
    let formatted_size = nbytes_i64(nbytes, Option::from(1))
        .map_err(PyErr::new::<pyo3::exceptions::PyValueError, _>)?;
    Ok(formatted_size)
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fmt_nbytes, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    #[test]
    fn test_nbytes_str() {
        let test_data: Vec<(u64, &str)> = vec![
            (100, "100 bytes"),
            (1000, "1000 bytes"),
            (10000, "9.7 KiB"),
            (100_000, "97.6 KiB"),
            (1_000_000, "976.5 KiB"),
            (10_000_000, "9.5 MiB"),
            (100_000_000, "95.3 MiB"),
            (1_000_000_000, "953.6 MiB"),
            (10_000_000_000, "9.3 GiB"),
            (100_000_000_000, "93.1 GiB"),
            (1_000_000_000_000, "931.3 GiB"),
            (10_000_000_000_000, "9.0 TiB"),
            (100_000_000_000_000, "90.9 TiB"),
        ];
        for (nbytes, expected) in test_data {
            assert_eq!(super::nbytes_u64(nbytes, Option::from(1)), expected);
        }
    }
}
