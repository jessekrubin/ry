#![doc = include_str!("../README.md")]
use pyo3::exceptions::{PyNotImplementedError, PyTypeError, PyValueError};
use pyo3::intern;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyTuple, PyType};
use ryo3_bytes::PyBytes;
use std::hash::{DefaultHasher, Hash, Hasher};

pub(crate) const RESERVED_NCS: &str = "reserved for NCS compatibility";
pub(crate) const RFC_4122: &str = "specified in RFC 4122";
pub(crate) const RESERVED_MICROSOFT: &str = "reserved for Microsoft compatibility";
pub(crate) const RESERVED_FUTURE: &str = "reserved for future definition";

// TODO: module name fix must be `ry.uuid` fix submodule name
#[pyclass(name = "UUID", frozen, weakref)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.uuid"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct PyUuid(uuid::Uuid);

impl PyUuid {
    #[must_use]
    pub fn get(&self) -> &uuid::Uuid {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PyUuid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        uuid::Uuid::deserialize(deserializer).map(Self::from)
    }
}

impl AsRef<uuid::Uuid> for PyUuid {
    fn as_ref(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl From<uuid::Uuid> for PyUuid {
    fn from(value: uuid::Uuid) -> Self {
        Self(value)
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
            (None, Some(bytes), None, None, None) => Self::from_pybytes(bytes),
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
            Ok(Self(b.into_uuid()))
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

    #[pyo3(name = "to_string")]
    fn py_to_string(&self) -> String {
        self.__str__()
    }

    #[allow(deprecated)]
    #[pyo3(
        warn(
            message = "obj.string() is deprecated, use `obj.to_string()` or `str(obj)` [remove in 0.0.60]",
            category = pyo3::exceptions::PyDeprecationWarning
      )
    )]
    fn string(&self) -> String {
        self.py_to_string()
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("UUID('{}')", self.py_to_string())
    }

    fn __int__(&self) -> u128 {
        self.0.as_u128()
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        if let Ok(rs_uuid) = other.cast::<Self>() {
            let other = rs_uuid.get();

            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
            }
        } else {
            let other = other.extract::<CPythonUuid>()?;
            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
            }
        }
    }

    #[getter]
    fn version(&self) -> usize {
        self.0.get_version_num()
    }

    #[getter]
    fn urn(&self) -> String {
        self.0.urn().to_string()
    }

    #[expect(clippy::wrong_self_convention)]
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
        Self::from(uuid::Uuid::from_bytes(int.to_be_bytes()))
    }

    #[staticmethod]
    #[pyo3(name = "from_bytes")]
    #[expect(clippy::needless_pass_by_value)]
    fn from_pybytes(bytes: PyBytes) -> PyResult<Self> {
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
        Ok(Self(uuid))
    }

    #[getter]
    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        let bytes = self.0.as_bytes().to_vec();
        pyo3::types::PyBytes::new(py, &bytes)
    }

    #[getter]
    fn bytes<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        self.__bytes__(py)
    }

    #[getter]
    fn bytes_le<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        let bytes = self.0.to_bytes_le().to_vec();
        pyo3::types::PyBytes::new(py, &bytes)
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
        u32::try_from(self.int().wrapping_shr(96))
            .expect("time_low is not a u32 - should not happen")
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

// ----------------------------------------------------------------------------
// python-uuid conversion fixed
// ----------------------------------------------------------------------------
// NOTE: As of today/now (2025-05-15) on Big-Endian system the uuid conversion
//       does not work as expected due to the usage of `.to_le()`

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CPythonUuid(pub(crate) uuid::Uuid);

impl From<CPythonUuid> for uuid::Uuid {
    fn from(val: CPythonUuid) -> Self {
        val.0
    }
}

fn get_uuid_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
    static UUID_CLS: PyOnceLock<Py<PyType>> = PyOnceLock::new();
    UUID_CLS.import(py, "uuid", "UUID")
}

impl FromPyObject<'_> for CPythonUuid {
    fn extract_bound(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        let py = obj.py();
        let uuid_cls = get_uuid_cls(py)?;

        if obj.is_instance(uuid_cls)? {
            let uuid_int: u128 = obj.getattr(intern!(py, "int"))?.extract()?;
            let bytes = uuid_int.to_be_bytes();
            Ok(Self(uuid::Uuid::from_bytes(bytes)))
        } else {
            Err(PyTypeError::new_err("Expected a `uuid.UUID` instance."))
        }
    }
}
