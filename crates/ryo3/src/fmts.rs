use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{pyfunction, wrap_pyfunction, PyResult};

const KILOBYTE: f64 = 1024.0;
const MEGABYTE: f64 = KILOBYTE * 1024.0;
const GIGABYTE: f64 = MEGABYTE * 1024.0;
const TERABYTE: f64 = GIGABYTE * 1024.0;
const PETABYTE: f64 = TERABYTE * 1024.0;
const EXABYTE: f64 = PETABYTE * 1024.0;

pub fn nbytes_u64(nbytes: u64, precision: Option<usize>) -> Result<String, String> {
    let nbytes_f64 = nbytes as f64;
    let precision = precision.unwrap_or(1);
    let formatted_size = match nbytes_f64 {
        n if n < KILOBYTE => {
            if (n - 1.0).abs() < 0.001 {
                "1 byte".to_string()
            } else {
                format!("{n:.0} bytes")
            }
        }
        n if n < MEGABYTE => format!("{:.1$} KiB", n / KILOBYTE, precision),
        n if n < GIGABYTE => format!("{:.1$} MiB", n / MEGABYTE, precision),
        n if n < TERABYTE => format!("{:.1$} GiB", n / GIGABYTE, precision),
        n if n < PETABYTE => format!("{:.1$} TiB", n / TERABYTE, precision),
        n if n < EXABYTE => format!("{:.1$} PiB", n / PETABYTE, precision),
        n => format!("{:.1$} EiB", n / EXABYTE, precision),
    };

    if nbytes_f64 >= 0.0 {
        Ok(formatted_size)
    } else {
        Err(format!("Invalid number of bytes: {nbytes_f64}"))
    }
}

fn nbytes_i64(nbytes: i64, precision: Option<usize>) -> Result<String, String> {
    let nabs = if nbytes < 0 { -nbytes } else { nbytes };
    nbytes_u64(nabs as u64, precision)
}

// TODO: Fix to handle negative numbers
#[pyfunction]
#[pyo3(name = "fmt_nbytes")]
pub fn fmt_nbytes(nbytes: i64) -> PyResult<String> {
    Ok(nbytes_i64(nbytes, Option::from(1)).unwrap())
}

pub fn madd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fmt_nbytes, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_nbytes_str() {
        assert_eq!(
            super::nbytes_u64(100, Option::from(1)).unwrap(),
            "100 bytes"
        );
        assert_eq!(
            super::nbytes_u64(1000, Option::from(1)).unwrap(),
            "1000 bytes"
        );
        assert_eq!(
            super::nbytes_u64(10000, Option::from(1)).unwrap(),
            "9.8 KiB"
        );
        assert_eq!(
            super::nbytes_u64(100_000, Option::from(1)).unwrap(),
            "97.7 KiB"
        );
        assert_eq!(
            super::nbytes_u64(1_000_000, Option::from(1)).unwrap(),
            "976.6 KiB"
        );
        assert_eq!(
            super::nbytes_u64(10_000_000, Option::from(1)).unwrap(),
            "9.5 MiB"
        );
        assert_eq!(
            super::nbytes_u64(100_000_000, Option::from(1)).unwrap(),
            "95.4 MiB"
        );
        assert_eq!(
            super::nbytes_u64(1_000_000_000, Option::from(1)).unwrap(),
            "953.7 MiB"
        );
        assert_eq!(
            super::nbytes_u64(10_000_000_000, Option::from(1)).unwrap(),
            "9.3 GiB"
        );
        assert_eq!(
            super::nbytes_u64(100_000_000_000, Option::from(1)).unwrap(),
            "93.1 GiB"
        );
        assert_eq!(
            super::nbytes_u64(1_000_000_000_000, Option::from(1)).unwrap(),
            "931.3 GiB"
        );
        assert_eq!(
            super::nbytes_u64(10_000_000_000_000, Option::from(1)).unwrap(),
            "9.1 TiB"
        );
        assert_eq!(
            super::nbytes_u64(100_000_000_000_000, Option::from(1)).unwrap(),
            "90.9 TiB"
        );
    }
}
