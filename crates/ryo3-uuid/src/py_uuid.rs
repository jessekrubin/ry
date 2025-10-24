#![doc = include_str!("../README.md")]
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::PyTuple;
use pyo3::{IntoPyObjectExt, intern};
use ryo3_bytes::PyBytes;
use ryo3_macro_rules::{any_repr, py_type_err, py_value_err, py_value_error, pytodo};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::OnceLock;

static NODE_CACHE: OnceLock<u64> = OnceLock::new();

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
        _ => py_value_err!("Invalid UUID version: {uuid}. Must be between 1 and 8."),
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
        if let Ok(rs_uuid) = other.cast_exact::<Self>() {
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
    fn from_pyuuid(ob: CPythonUuid) -> Self {
        Self::from(ob.0)
    }

    #[staticmethod]
    fn from_hex(hex: &str) -> PyResult<Self> {
        Self::from_str(hex)
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        uuid::Uuid::parse_str(s)
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
    /// The 60-bit timestamp as a count of 100-nanosecond intervals since
    /// Gregorian epoch (1582-10-15 00:00:00) for versions 1 and 6, or the
    /// 48-bit timestamp in milliseconds since Unix epoch (1970-01-01 00:00:00)
    /// for version 7.
    fn time(&self) -> u64 {
        self.py_time()
        // let version_rfc = (
        //     self.version(),
        //     matches!(self.0.get_variant(), uuid::Variant::RFC4122),
        // );
        // match version_rfc {
        //     (6, true) => {
        //         let high32 = (u64::from(self.time_low())) << 28;
        //         let mid16 = (u64::from(self.time_mid())) << 12;
        //         let low12 = u64::from(self.time_hi_version()) & 0x0fff;
        //         high32 | mid16 | low12
        //     }
        //     (7, true) => (self.0.as_u128() >> 80) as u64,
        //     // should be 1 and/or any other versions but idk if it is actually
        //     // implemented?
        //     _ => {
        //         let high = u64::from(self.time_hi_version()) & 0x0fff;
        //         let mid = u64::from(self.time_mid());
        //         high.wrapping_shl(48) | mid.wrapping_shl(32) | u64::from(self.time_low())
        //     }
        // }
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

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyAny>> {
        let py = value.py();
        if value.is_exact_instance_of::<Self>() {
            value.into_bound_py_any(py)
        } else if let Ok(s) = value.extract::<&str>() {
            Self::from_str(s).map(|dt| dt.into_bound_py_any(py).map(Bound::into_any))?
        } else if let Ok(b) = value.extract::<[u8; 16]>() {
            // let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_int(u128::from_be_bytes(b)).into_bound_py_any(py)
        } else if let Ok(pybytes) = value.cast::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_bound_py_any(py).map(Bound::into_any))?
        } else if let Ok(v) = value.extract::<CPythonUuid>() {
            Self::from(v.0).into_bound_py_any(py)
        } else {
            let valtype = any_repr!(value);
            py_type_err!("UUID conversion error: {valtype}")
        }
    }

    // ========================================================================
    // PYDANTIC
    // ========================================================================
    #[cfg(feature = "pydantic")]
    #[staticmethod]
    fn _pydantic_validate<'py>(
        value: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_macro_rules::py_value_error;
        Self::from_any(value).map_err(|e| py_value_error!("UUID validation error: {e}"))
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, ::pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }
}

#[cfg(not(Py_3_14))]
impl PyUuid {
    fn py_time(&self) -> u64 {
        let high = u64::from(self.time_hi_version()) & 0x0fff;
        let mid = u64::from(self.time_mid());
        high.wrapping_shl(48) | mid.wrapping_shl(32) | u64::from(self.time_low())
    }
}

#[cfg(Py_3_14)]
impl PyUuid {
    fn py_time(&self) -> u64 {
        let version_rfc = (
            self.version(),
            matches!(self.0.get_variant(), uuid::Variant::RFC4122),
        );
        match version_rfc {
            (6, true) => {
                (u64::from(self.time_low()) << 28)
                    | (u64::from(self.time_mid()) << 12)
                    | (u64::from(self.time_hi_version()) & 0x0fff)
            }
            (7, true) => (self.0.as_u128() >> 80) as u64,
            // should be 1 and/or any other versions but idk if it is actually
            // implemented?
            _ => {
                let high = u64::from(self.time_hi_version()) & 0x0fff;
                let mid = u64::from(self.time_mid());
                high.wrapping_shl(48) | mid.wrapping_shl(32) | u64::from(self.time_low())
            }
        }
    }
}

#[pyfunction(name = "getnode")]
pub fn getnode(py: Python<'_>) -> PyResult<u64> {
    if let Some(v) = NODE_CACHE.get() {
        return Ok(*v);
    }
    let node_py: u64 = py.import("uuid")?.getattr("getnode")?.call0()?.extract()?;
    if node_py > 0xFF_FFFF_FFFF_FFFF {
        py_value_err!("uuid.getnode() returned >48 bits")
    } else {
        let _ = NODE_CACHE.set(node_py);
        Ok(node_py)
    }
}

/// Generate a UUID from a host ID, sequence number, and the current time.
///
/// If node is not given, `getnode()` is used to obtain the hardware address.
/// If `clock_seq` is given, it is used as the sequence number; otherwise a
/// random 14-bit sequence number is chosen.
#[pyfunction(signature = (node=None, clock_seq=None))]
#[expect(unused_variables)]
pub fn uuid1(node: Option<u64>, clock_seq: Option<u16>) -> PyResult<PyUuid> {
    pytodo!("UUID1 is not implemented yet")
}

#[pyfunction]
pub fn uuid3() -> PyResult<PyUuid> {
    pytodo!("UUID3 is not implemented yet")
}

#[pyfunction]
pub fn uuid4() -> PyResult<PyUuid> {
    Ok(PyUuid(uuid::Uuid::new_v4()))
}

#[pyfunction]
pub fn uuid5() -> PyResult<PyUuid> {
    pytodo!("UUID5 is not implemented yet")
}

#[pyfunction]
pub fn uuid6() -> PyResult<PyUuid> {
    pytodo!("UUID6 is not implemented yet")
}

#[pyfunction]
#[must_use]
pub fn uuid7() -> PyUuid {
    PyUuid::from(uuid::Uuid::now_v7())
}

#[pyfunction(
    signature = (a = None, b = None, c = None, buf = None),
)]
pub fn uuid8(
    a: Option<u64>,
    b: Option<u16>,
    c: Option<u64>,
    buf: Option<PyBytes>,
) -> PyResult<PyUuid> {
    use rand::RngCore;

    if let Some(bts) = buf {
        match (a, b, c) {
            (None, None, None) => {}
            _ => {
                return py_value_err!("uuid8(): pass either bytes=... or a/b/c, not both",);
            }
        }
        // extract the bytes as [u8; 16]
        let slice: &[u8; 16] = bts
            .as_slice()
            .try_into()
            .map_err(|_| py_value_error!("uuid8(bytes=...): bytes must be exactly 16 bytes",))?;
        return Ok(PyUuid::from(uuid::Uuid::new_v8(*slice)));
    }

    let mut rng = rand::rng();

    let mut ubuf: [u8; 16] = [0; 16];
    let a48: u64 = a.unwrap_or_else(|| rng.next_u64()) & ((1u64 << 48) - 1);
    let b12: u16 = b.unwrap_or_else(|| (rng.next_u32() & 0xFFFF) as u16) & 0x0FFF;
    let c62: u64 = c.unwrap_or_else(|| rng.next_u64()) & ((1u64 << 62) - 1);

    // least significant 48 bits of a
    ubuf[0..6].copy_from_slice(&a48.to_be_bytes()[2..]);
    // least significant 12 bits of b
    ubuf[6..8].copy_from_slice(&b12.to_be_bytes());
    // least significant 62 bits of c
    ubuf[8..16].copy_from_slice(&c62.to_be_bytes());

    Ok(PyUuid::from(uuid::Uuid::new_v8(ubuf)))
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

fn get_uuid_ob_pointer(py: Python) -> usize {
    let uuid_mod = py.import("uuid").expect("uuid to be importable");
    // get a uuid how orjson does it...
    let uuid_ob = uuid_mod
        .getattr("NAMESPACE_DNS")
        .expect("uuid.NAMESPACE_DNS to be available");
    let uuid_type = uuid_ob.get_type();

    let uuid_type_ptr = uuid_type.as_type_ptr() as usize;
    // make sure we drop the reference
    drop(uuid_mod);
    drop(uuid_ob);
    drop(uuid_type);
    uuid_type_ptr
}

fn py_uuid_type_ptr(py: Python) -> usize {
    static UUID_TYPE_PTR: PyOnceLock<usize> = PyOnceLock::new();
    *UUID_TYPE_PTR.get_or_init(py, || get_uuid_ob_pointer(py))
}

// ORIG VERSION THAT USES INSTANCE CHECK
// ```
// fn get_uuid_cls(py: Python<'_>) -> PyResult<&Bound<'_, PyType>> {
//     static UUID_CLS: PyOnceLock<Py<PyType>> = PyOnceLock::new();
//     UUID_CLS.import(py, "uuid", "UUID")
// }
// impl FromPyObject<'_> for CPythonUuid {
//     fn extract_bound(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
//         let py = obj.py();
//         let uuid_cls = get_uuid_cls(py)?;

//         if obj.is_instance(uuid_cls)? {
//             let uuid_int: u128 = obj.getattr(intern!(py, "int"))?.extract()?;
//             let bytes = uuid_int.to_be_bytes();
//             Ok(Self(uuid::Uuid::from_bytes(bytes)))
//         } else {
//             Err(PyTypeError::new_err("Expected a `uuid.UUID` instance."))
//         }
//     }
// }
// ``````

// NEW VERSION THAT USES TYPE POINTER COMPARISON
impl<'py> FromPyObject<'_, 'py> for CPythonUuid {
    type Error = pyo3::PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        let py = obj.py();
        let uuid_cls_ptr = py_uuid_type_ptr(obj.py());
        let obj_ptr = obj.get_type().as_type_ptr() as usize;
        if obj_ptr == uuid_cls_ptr {
            let uuid_int: u128 = obj.getattr(intern!(py, "int"))?.extract()?;
            let bytes = uuid_int.to_be_bytes();
            Ok(Self(uuid::Uuid::from_bytes(bytes)))
        } else {
            py_type_err!("Expected a `uuid.UUID` instance.",)
        }
    }
}

#[cfg(feature = "pydantic")]
impl ryo3_pydantic::GetPydanticCoreSchemaCls for PyUuid {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::interns;

        let py = source.py();
        let core_schema = ryo3_pydantic::core_schema(py)?;
        let uuid_schema = core_schema.call_method(intern!(py, "uuid_schema"), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let args = PyTuple::new(py, vec![&validation_fn, &uuid_schema])?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let serialization_kwargs = pyo3::types::PyDict::new(py);
        serialization_kwargs.set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_wrap_validator_function(py),
            args,
            Some(&serialization_kwargs),
        )
    }
}
