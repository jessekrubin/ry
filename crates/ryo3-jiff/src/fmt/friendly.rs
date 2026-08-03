pub(crate) mod printer {
    use jiff::fmt::friendly::Designator;
    use pyo3::prelude::*;
    use ryo3_macro_rules::{py_type_err, py_value_err};

    #[derive(Clone, Copy, Debug)]
    pub(crate) struct PyDesignator(Designator);

    impl Default for PyDesignator {
        fn default() -> Self {
            Self::COMPACT
        }
    }

    impl PyDesignator {
        const VERBOSE: Self = Self::new(Designator::Verbose);
        const SHORT: Self = Self::new(Designator::Short);
        const COMPACT: Self = Self::new(Designator::Compact);
        const HUMAN_TIME: Self = Self::new(Designator::HumanTime);

        pub(crate) const fn new(designator: Designator) -> Self {
            Self(designator)
        }

        fn into_inner(self) -> Designator {
            self.0
        }
    }

    impl From<PyDesignator> for Designator {
        fn from(value: PyDesignator) -> Self {
            value.into_inner()
        }
    }

    const JIFF_FMT_FRIENDLY_DESIGNATOR: &str =
        "'human'/'human-time', 'short', 'compact', or 'verbose'";
    impl<'py> FromPyObject<'_, 'py> for PyDesignator {
        type Error = PyErr;
        fn extract(ob: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
            if let Ok(s) = ob.extract::<&str>() {
                match s {
                    "human-time" | "human" => Ok(Self::HUMAN_TIME),
                    "short" => Ok(Self::SHORT),
                    "compact" => Ok(Self::COMPACT),
                    "verbose" => Ok(Self::VERBOSE),
                    _ => py_value_err!(
                        "Invalid designator: {s} (options: {JIFF_FMT_FRIENDLY_DESIGNATOR})"
                    ),
                }
            } else {
                py_type_err!(
                    "Invalid type for designator, expected a string (options: {JIFF_FMT_FRIENDLY_DESIGNATOR})"
                )
            }
        }
    }
}
