const KILOBYTE: f64 = 1024.0;
const MEGABYTE: f64 = KILOBYTE * 1024.0;
const GIGABYTE: f64 = MEGABYTE * 1024.0;
const TERABYTE: f64 = GIGABYTE * 1024.0;
const PETABYTE: f64 = TERABYTE * 1024.0;
const EXABYTE: f64 = PETABYTE * 1024.0;

pub fn nbytes_str(nbytes: u64, precision: Option<usize>) -> Result<String, String> {
    let nbytes = nbytes as f64;
    let precision = precision.unwrap_or(1);
    let formatted_size = match nbytes {
        n if n < KILOBYTE => format!("{:.1$} bytes", n, precision),
        n if n < MEGABYTE => format!("{:.1$} KB", n / KILOBYTE, precision),
        n if n < GIGABYTE => format!("{:.1$} MB", n / MEGABYTE, precision),
        n if n < TERABYTE => format!("{:.1$} GB", n / GIGABYTE, precision),
        n if n < PETABYTE => format!("{:.1$} TB", n / TERABYTE, precision),
        n if n < EXABYTE => format!("{:.1$} PB", n / PETABYTE, precision),
        n => format!("{:.1$} EB", n / EXABYTE, precision),
    };

    if nbytes >= 0.0 {
        Ok(formatted_size)
    } else {
        Err(format!("Invalid number of bytes: {}", nbytes))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_nbytes_str() {
        assert_eq!(
            super::nbytes_str(100, Option::from(1)).unwrap(),
            "100.0 bytes"
        );
        assert_eq!(
            super::nbytes_str(1000, Option::from(1)).unwrap(),
            "1000.0 bytes"
        );
        assert_eq!(super::nbytes_str(10000, Option::from(1)).unwrap(), "9.8 KB");
        assert_eq!(
            super::nbytes_str(100000, Option::from(1)).unwrap(),
            "97.7 KB"
        );
        assert_eq!(
            super::nbytes_str(1000000, Option::from(1)).unwrap(),
            "976.6 KB"
        );
        assert_eq!(
            super::nbytes_str(10_000_000, Option::from(1)).unwrap(),
            "9.5 MB"
        );
        assert_eq!(
            super::nbytes_str(100_000_000, Option::from(1)).unwrap(),
            "95.4 MB"
        );
        assert_eq!(
            super::nbytes_str(1000000000, Option::from(1)).unwrap(),
            "953.7 MB"
        );
        assert_eq!(
            super::nbytes_str(10000000000, Option::from(1)).unwrap(),
            "9.3 GB"
        );
        assert_eq!(
            super::nbytes_str(100000000000, Option::from(1)).unwrap(),
            "93.1 GB"
        );
        assert_eq!(
            super::nbytes_str(1000000000000, Option::from(1)).unwrap(),
            "931.3 GB"
        );
        assert_eq!(
            super::nbytes_str(10000000000000, Option::from(1)).unwrap(),
            "9.1 TB"
        );
        assert_eq!(
            super::nbytes_str(100000000000000, Option::from(1)).unwrap(),
            "90.9 TB"
        );
    }
}
