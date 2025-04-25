#![doc = include_str!("../README.md")]

use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use ryo3_bytes::PyBytes;
use std::hash::{DefaultHasher, Hash, Hasher};
pub(crate) const RESERVED_NCS: &str = "reserved for NCS compatibility";
pub(crate) const RFC_4122: &str = "specified in RFC 4122";
pub(crate) const RESERVED_MICROSOFT: &str = "reserved for Microsoft compatibility";
pub(crate) const RESERVED_FUTURE: &str = "reserved for future definition";
#[pyclass(name = "UUID", module = "ry.uuid", frozen)]
pub struct PyUuid(pub uuid::Uuid);

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
            "Invalid UUID version: {uuid}. Must be between 1 and 8."
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
        let version = match version {
            Some(v) => get_version(v).map(Some)?,
            None => None,
        };

        let py_uuid = match (hex, bytes, bytes_le, fields, int) {
            (Some(hex), None, None, None, None) => Self::from_hex(hex),
            (None, Some(bytes), None, None, None) => Self::from_bytes(bytes),
            (None, None, Some(bytes_le), None, None) => Self::from_bytes_le(bytes_le),
            (None, None, None, Some(fields), None) => Self::from_fields(fields),
            (None, None, None, None, Some(int)) => Ok(Self::from_int(int)),
            _ => Err(PyTypeError::new_err(
                // taken from the python itself
                "one of the hex, bytes, bytes_le, fields, or int arguments must be given",
            )),
        }?;

        if let Some(v) = version {
            let mut b = uuid::Builder::from_u128(py_uuid.0.as_u128());
            b.set_version(v);
            Ok(PyUuid(b.into_uuid()))
        } else {
            Ok(py_uuid)
        }
    }
    #[classattr]
    #[expect(non_snake_case)]
    pub(crate) fn NAMESPACE_DNS() -> Self {
        Self(uuid::Uuid::NAMESPACE_DNS)
    }

    #[classattr]
    #[expect(non_snake_case)]
    pub(crate) fn NAMESPACE_URL() -> Self {
        Self(uuid::Uuid::NAMESPACE_URL)
    }

    #[classattr]
    #[expect(non_snake_case)]
    pub(crate) fn NAMESPACE_OID() -> Self {
        Self(uuid::Uuid::NAMESPACE_OID)
    }

    #[classattr]
    #[expect(non_snake_case)]
    pub(crate) fn NAMESPACE_X500() -> Self {
        Self(uuid::Uuid::NAMESPACE_X500)
    }

    fn __getnewargs__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, vec![self.0.hyphenated().to_string()])
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
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

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        if let Ok(rs_uuid) = other.downcast::<PyUuid>() {
            let other = rs_uuid.get();

            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
            }
            // return self.compare(rs_uuid, op);
        } else {
            let other = other.extract::<uuid::Uuid>()?;

            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other),
            }
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
    fn urn(&self) -> String {
        self.0.urn().to_string()
    }

    fn to_py(&self) -> uuid::Uuid {
        self.0
    }

    // static/class methods
    #[staticmethod]
    fn from_hex(hex: &str) -> PyResult<Self> {
        uuid::Uuid::parse_str(hex)
            .map(PyUuid)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn from_int(int: u128) -> Self {
        Self::from(uuid::Uuid::from_u128(int))
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_bytes(bytes: PyBytes) -> PyResult<Self> {
        uuid::Uuid::from_slice(bytes.as_ref())
            .map(PyUuid)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    #[expect(clippy::needless_pass_by_value)]
    fn from_bytes_le(bytes: PyBytes) -> PyResult<Self> {
        uuid::Uuid::from_slice_le(bytes.as_ref())
            .map(PyUuid)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    // | Field                      | Meaning                          |
    // |----------------------------|----------------------------------|
    // | `UUID.time_low`            | The first 32 bits of the UUID.   |
    // | `UUID.time_mid`            | The next 16 bits of the UUID.    |
    // | `UUID.time_hi_version`     | The next 16 bits of the UUID.    |
    // | `UUID.clock_seq_hi_variant`| The next 8 bits of the UUID.     |
    // | `UUID.clock_seq_low`       | The next 8 bits of the UUID.     |
    // | `UUID.node`                | The last 48 bits of the UUID.    |
    // | `UUID.time`                | The 60-bit timestamp.            |
    // | `UUID.clock_seq`           | The 14-bit sequence number.      |
    #[staticmethod]
    fn from_fields(fields: &Bound<'_, PyTuple>) -> PyResult<Self> {
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

    #[getter]
    fn __bytes__(&self) -> PyBytes {
        let bytes = self.0.as_bytes().to_vec();
        PyBytes::from(bytes)
    }

    #[getter]
    fn bytes(&self) -> PyBytes {
        self.__bytes__()
    }

    #[getter]
    fn bytes_le(&self) -> PyBytes {
        let bytes = self.0.to_bytes_le().to_vec();
        PyBytes::from(bytes)
    }

    #[getter]
    fn fields(&self) -> (u32, u16, u16, u8, u8, u64) {
        let uuid = self.0;
        let time_low = uuid.as_u128() >> 96;
        let time_mid = (uuid.as_u128() >> 80) & 0xFFFF;
        let time_hi_version = (uuid.as_u128() >> 64) & 0xFFFF;
        let clock_seq_hi_variant = (uuid.as_u128() >> 56) & 0xFF;
        let clock_seq_low = (uuid.as_u128() >> 48) & 0xFF;
        let node = uuid.as_u128() & 0xFFFF_FFFF_FFFF;
        (
            time_low as u32,
            time_mid as u16,
            time_hi_version as u16,
            clock_seq_hi_variant as u8,
            clock_seq_low as u8,
            node as u64,
        )
    }

    #[getter]
    fn node(&self) -> u64 {
        (self.0.as_u128() & 0xFFFF_FFFF_FFFF) as u64
    }

    #[getter]
    fn hex(&self) -> String {
        self.0.simple().to_string()
    }

    #[getter]
    fn int(&self) -> u128 {
        self.0.as_u128()
    }

    #[getter]
    fn time_low(&self) -> u32 {
        self.int().wrapping_shr(96) as u32
    }

    #[getter]
    fn time_mid(&self) -> u16 {
        ((self.int().wrapping_shr(80)) & 0xffff) as u16
    }

    #[getter]
    fn time_hi_version(&self) -> u16 {
        ((self.int().wrapping_shr(64)) & 0xffff) as u16
    }

    #[getter]
    fn time(&self) -> u64 {
        let high = u64::from(self.time_hi_version()) & 0x0fff;
        let mid = u64::from(self.time_mid());
        high.wrapping_shl(48) | mid.wrapping_shl(32) | u64::from(self.time_low())
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

    #[getter]
    fn clock_seq(&self) -> u16 {
        let high = u16::from(self.clock_seq_hi_variant()) & 0x3f;
        high.wrapping_shl(8) | u16::from(self.clock_seq_low())
    }

    #[getter]
    fn variant(&self) -> &str {
        match self.0.get_variant() {
            uuid::Variant::NCS => RESERVED_NCS,
            uuid::Variant::RFC4122 => RFC_4122,
            uuid::Variant::Microsoft => RESERVED_MICROSOFT,
            uuid::Variant::Future => RESERVED_FUTURE,
            _ => RESERVED_FUTURE,
        }
    }
    #[getter]
    fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

#[pyfunction(name = "getnode")]
pub fn getnode() -> PyResult<u64> {
    Err(PyNotImplementedError::new_err("not implemented"))
}

#[pyfunction]
pub fn uuid1() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID1 is not implemented yet",
    ))
}
#[pyfunction]
pub fn uuid2() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID2 is not implemented yet",
    ))
}

#[pyfunction]
pub fn uuid3() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID3 is not implemented yet",
    ))
}

#[pyfunction]
pub fn uuid4() -> PyResult<PyUuid> {
    let u = uuid::Uuid::new_v4();
    Ok(PyUuid(u))
}

#[pyfunction]
pub fn uuid5() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID5 is not implemented yet",
    ))
}

#[pyfunction]
pub fn uuid6() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID6 is not implemented yet",
    ))
}

#[pyfunction]
pub fn uuid7() -> PyResult<PyUuid> {
    Err(PyNotImplementedError::new_err(
        "UUID7 is not implemented yet",
    ))
}

#[pyfunction]
#[expect(clippy::needless_pass_by_value)]
pub fn uuid8(b: PyBytes) -> PyResult<PyUuid> {
    uuid::Bytes::try_from(b.as_slice())
        .map(|b| PyUuid::from(uuid::Uuid::new_v8(b)))
        .map_err(|_| PyValueError::new_err("UUID8 must be 16 bytes long"))
}
