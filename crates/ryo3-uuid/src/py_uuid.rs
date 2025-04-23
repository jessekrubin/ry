#![doc = include_str!("../README.md")]
use pyo3::exceptions::{PyNotImplementedError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyNotImplemented, PyTuple};
use ryo3_bytes::PyBytes;

#[pyclass(name = "UUID", module = "ry.ryo3", frozen)]
pub struct PyUuid(pub(crate) uuid::Uuid);

impl From<uuid::Uuid> for PyUuid {
    fn from(value: uuid::Uuid) -> Self {
        PyUuid(value)
    }
}

fn get_version(uuid: u8) -> PyResult<uuid::Version> {
    match uuid {
        1 => Ok(uuid::Version::Mac),
        2 => Ok(uuid::Version::Dce),
        3 => Ok(uuid::Version::Md5),
        4 => Ok(uuid::Version::Random),
        5 => Ok(uuid::Version::Sha1),
        6 => Ok(uuid::Version::SortMac),
        7 => Ok(uuid::Version::SortRand),
        8 => Ok(uuid::Version::Custom),
        _ => Err(PyValueError::new_err(format!(
            "Invalid UUID version: {}. Must be between 1 and 8.",
            uuid
        ))),
    }
}

#[pymethods]
impl PyUuid {
    #[new]
    #[pyo3(
        signature = (
            hex=None,
            bytes=None,
            bytes_le=None,
            fields=None,
            int=None,
            version=None
        )
    )]
    fn py_new(
        hex: Option<&str>,
        bytes: Option<PyBytes>,
        bytes_le: Option<PyBytes>,
        fields: Option<&Bound<PyTuple>>,
        int: Option<u128>,
        version: Option<u8>,
    ) -> PyResult<Self> {
        // get the version
        let _version = match version {
            Some(v) => get_version(v).map(|v| Some(v))?,
            None => None,
        };

        match (hex, bytes, bytes_le, fields, int) {
            (Some(hex), None, None, None, None) => Self::from_hex(hex),
            (None, Some(bytes), None, None, None) => Self::from_bytes(bytes),
            (None, None, Some(bytes_le), None, None) => Self::from_bytes_le(bytes_le),
            (None, None, None, Some(fields), None) => Self::from_fields(fields),
            (None, None, None, None, Some(int)) => Ok(Self::from_int(int)),
            _ => Err(PyValueError::new_err(
                "Only one of hex or bytes or fields or int can be provided.",
            )),
        }

        // let uuid = uuid::Uuid::parse_str(hex).map_err(|e| PyValueError::new_err(e.to_string()))?;
        // Ok(PyUuid(uuid))
    }

    fn string(&self) -> String {
        self.0.to_string()
    }

    fn __str__(&self) -> String {
        self.string()
    }

    fn __repr__(&self) -> String {
        format!("UUID('{}')", self.string())
    }

    fn __int__(&self) -> u128 {
        self.0.as_u128()
    }

    fn __richcmp__(&self, other: PyRef<PyUuid>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        match op {
            pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
            pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
            pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
            pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
            pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
            pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
        }
    }

    #[getter]
    fn version(&self) -> Option<u8> {
        if let Some(v) = self.0.get_version() {
            match v {
                uuid::Version::Mac => Some(1),
                uuid::Version::Dce => Some(2),
                uuid::Version::Md5 => Some(3),
                uuid::Version::Random => Some(4),
                uuid::Version::Sha1 => Some(5),
                uuid::Version::SortMac => Some(6),
                uuid::Version::SortRand => Some(7),
                uuid::Version::Custom => Some(8),
                _ => None,
            }
        } else {
            None
        }
    }

    #[getter]
    fn urn(&self) -> PyResult<String> {
        Ok(self.0.urn().to_string())
    }

    fn to_py(&self) -> uuid::Uuid {
        self.0
    }

    // static/class methods
    #[staticmethod]
    fn from_hex(hex: &str) -> PyResult<Self> {
        uuid::Uuid::parse_str(hex)
            .map(|uuid| PyUuid(uuid))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn from_int(int: u128) -> Self {
        Self::from(uuid::Uuid::from_u128(int))
    }

    #[staticmethod]
    fn from_bytes(bytes: PyBytes) -> PyResult<Self> {
        uuid::Uuid::from_slice(bytes.as_ref())
            .map(|uuid| PyUuid(uuid))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn from_bytes_le(bytes: PyBytes) -> PyResult<Self> {
        uuid::Uuid::from_slice_le(bytes.as_ref())
            .map(|uuid| PyUuid(uuid))
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // Field

    // Meaning

    // UUID.time_low
    // The first 32 bits of the UUID.

    // UUID.time_mid
    // The next 16 bits of the UUID.

    // UUID.time_hi_version
    // The next 16 bits of the UUID.

    // UUID.clock_seq_hi_variant
    // The next 8 bits of the UUID.

    // UUID.clock_seq_low
    // The next 8 bits of the UUID.

    // UUID.node
    // The last 48 bits of the UUID.

    // UUID.time
    // The 60-bit timestamp.

    // UUID.clock_seq
    // The 14-bit sequence number.

    #[staticmethod]
    fn from_fields<'py>(fields: &Bound<'py, PyTuple>) -> PyResult<Self> {
        let fields = fields.extract::<(u32, u16, u16, u8, u8, u64)>()?;
        let time_low = u128::from(fields.0) << 96;
        let time_mid = u128::from(fields.1) << 80;
        let time_hi_version = u128::from(fields.2) << 64;
        let clock_seq_hi_variant = u128::from(fields.3) << 56;
        let clock_seq_low = u128::from(fields.4) << 48;
        let node = u128::from(fields.5);
        let uuid =
            time_low | time_mid | time_hi_version | clock_seq_hi_variant | clock_seq_low | node;
        let uuid = uuid::Uuid::from_u128(uuid);
        Ok(PyUuid(uuid))
    }

    fn to_fields(&self) -> PyResult<(u32, u16, u16, u8, u8, u64)> {
        let uuid = self.0;
        let time_low = uuid.as_u128() >> 96;
        let time_mid = (uuid.as_u128() >> 80) & 0xFFFF;
        let time_hi_version = (uuid.as_u128() >> 64) & 0xFFFF;
        let clock_seq_hi_variant = (uuid.as_u128() >> 56) & 0xFF;
        let clock_seq_low = (uuid.as_u128() >> 48) & 0xFF;
        let node = uuid.as_u128() & 0xFFFFFFFFFFFF;
        Ok((
            time_low as u32,
            time_mid as u16,
            time_hi_version as u16,
            clock_seq_hi_variant as u8,
            clock_seq_low as u8,
            node as u64,
        ))
    }

    #[getter]
    fn node(&self) -> PyResult<u64> {
        let n = self.0.as_u128() & 0xFFFFFFFFFFFF;
        Ok(n as u64)
    }

    #[getter]
    fn time_low(&self) -> u32 {
        u32::try_from((self.0.as_u128() >> 96) & 0xFFFFFFFF)
            .expect("time_low is not a u32 - should not happen")
    }

    #[getter]
    fn time_mid(&self) -> u16 {
        u16::try_from((self.0.as_u128() >> 80) & 0xFFFF)
            .expect("time_mid is not a u16 - should not happen")
    }

    #[getter]
    fn time_hi_version(&self) -> u16 {
        u16::try_from((self.0.as_u128() >> 64) & 0xFFFF)
            .expect("time_hi_version is not a u16 - should not happen")
    }

    #[getter]
    fn clock_seq_hi_variant(&self) -> u8 {
        u8::try_from((self.0.as_u128() >> 56) & 0xFF)
            .expect("clock_seq_hi_variant is not a u8 - should not happen")
    }

    #[getter]
    fn clock_seq_low(&self) -> u8 {
        u8::try_from((self.0.as_u128() >> 48) & 0xFF)
            .expect("clock_seq_low is not a u8 - should not happen")
    }
}

#[pyfunction]
pub fn uuid4() -> PyResult<PyUuid> {
    let u = uuid::Uuid::new_v4();
    Ok(PyUuid(u))
}
