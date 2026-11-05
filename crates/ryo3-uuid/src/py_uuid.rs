#![doc = include_str!("../README.md")]
use std::sync::OnceLock;

use pyo3::BoundObject;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyString, PyTuple};
use ryo3_bytes::ExactReadableBuffer;
use ryo3_core::PyAsciiStr;
use ryo3_core::macros::{
    any_repr, py_type_err, py_type_error, py_value_err, py_value_error, pytodo,
};

static NODE_CACHE: OnceLock<u64> = OnceLock::new();

pub const RESERVED_NCS: &str = "reserved for NCS compatibility";
pub const RFC_4122: &str = "specified in RFC 4122";
pub const RESERVED_MICROSOFT: &str = "reserved for Microsoft compatibility";
pub const RESERVED_FUTURE: &str = "reserved for future definition";

#[pyclass(name = "UUID", frozen, immutable_type, weakref)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct PyUuid(uuid::Uuid);

impl PyUuid {
    #[must_use]
    pub fn inner(&self) -> &uuid::Uuid {
        &self.0
    }

    fn pystr_braced<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let mut buf = uuid::Uuid::encode_buffer();
        let s = self.0.braced().encode_lower(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn pystr_simple<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let mut buf = uuid::Uuid::encode_buffer();
        let s = self.0.simple().encode_lower(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn pystr_urn<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let mut buf = uuid::Uuid::encode_buffer();
        let s = self.0.urn().encode_lower(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn pystr_hyphenated<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let mut buf = uuid::Uuid::encode_buffer();
        let s = self.0.hyphenated().encode_lower(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn pystr_fmt<'py, const B: bool>(
        &self,
        py: Python<'py>,
        kind: UuidFormat<B>,
    ) -> Bound<'py, PyString> {
        match kind {
            UuidFormat::Hyphenated => self.pystr_hyphenated(py),
            UuidFormat::Simple => self.pystr_simple(py),
            UuidFormat::Urn => self.pystr_urn(py),
            UuidFormat::Braced => self.pystr_braced(py),
        }
    }

    /// Reseed rng
    ///
    /// This is called on unix-y systems (jurassic park) after fork and is
    /// registered when the module is registered w/ the below snippet:
    ///
    /// ```python
    /// os.register_at_fork(after_in_child=UUID._reseed_rng)
    /// ```
    ///
    /// (this caused a subtle bug in an old server w/ gunicorn I maintain)
    ///
    /// REFS:
    /// - uuid8: <https://github.com/python/cpython/blob/b5bffb9b4ce8f4515df1e0474c8f439e9f580d65/Lib/uuid.py#L940-L957>
    /// - random: <https://github.com/python/cpython/blob/b5bffb9b4ce8f4515df1e0474c8f439e9f580d65/Lib/random.py#L1012-L1013>
    /// - uuid4/uuid7: <https://github.com/python/cpython/blob/b5bffb9b4ce8f4515df1e0474c8f439e9f580d65/Lib/uuid.py#L118-L122>
    fn reseed_rng() -> PyResult<()> {
        rand::rng()
            .reseed()
            .map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))
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
            hex = None,
            bytes = None,
            bytes_le = None,
            fields = None,
            int = None,
            version = None,
        )
    )]
    fn py_new(
        hex: Option<&str>,
        bytes: Option<PyUuidBytes>,
        bytes_le: Option<PyUuidBytes>,
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
            (None, Some(bytes), None, None, None) => Ok(Self::from_pybytes(bytes)),
            (None, None, Some(bytes_le), None, None) => Ok(Self::from_bytes_le(bytes_le)),
            (None, None, None, Some(fields), None) => Self::from_fields(fields),
            (None, None, None, None, Some(int)) => Ok(Self::from_int(int)),
            // emsg taken from the python itself
            _ => py_type_err!(
                "one of the hex, bytes, bytes_le, fields, or int arguments must be given",
            ),
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
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn NAMESPACE_DNS() -> Self {
        Self(uuid::Uuid::NAMESPACE_DNS)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn NAMESPACE_URL() -> Self {
        Self(uuid::Uuid::NAMESPACE_URL)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn NAMESPACE_OID() -> Self {
        Self(uuid::Uuid::NAMESPACE_OID)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn NAMESPACE_X500() -> Self {
        Self(uuid::Uuid::NAMESPACE_X500)
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn NIL() -> Self {
        Self(uuid::Uuid::nil())
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn MAX() -> Self {
        Self(uuid::Uuid::from_u128(u128::MAX))
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn RESERVED_FUTURE() -> &'static str {
        RESERVED_FUTURE
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn RESERVED_NCS() -> &'static str {
        RESERVED_NCS
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn RFC_4122() -> &'static str {
        RFC_4122
    }

    #[classattr]
    #[expect(non_snake_case, reason = "python classattr")]
    pub(crate) fn RESERVED_MICROSOFT() -> &'static str {
        RESERVED_MICROSOFT
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, pyo3::types::PyType>, (Bound<'py, PyString>,)) {
        (py.get_type::<Self>(), (self.pystr_hyphenated(py),))
    }

    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    fn __hash__(&self) -> isize {
        #[cfg(target_pointer_width = "64")]
        const M: u128 = (1u128 << 61) - 1;

        #[cfg(target_pointer_width = "32")]
        const M: u128 = (1u128 << 31) - 1;
        (self.0.as_u128() % M) as isize
    }

    // not totally sure if this is needed...
    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "64")))]
    fn __hash__(&self) -> u64 {
        use std::hash::{DefaultHasher, Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    #[pyo3(name = "to_string")]
    fn py_to_string<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        self.pystr_hyphenated(py)
    }

    #[pyo3(
        name = "format",
        signature = (kind = UuidFormat::Hyphenated),
        text_signature = "(self, kind=\"hyphenated\")"
    )]
    fn py_format<'py>(&self, py: Python<'py>, kind: UuidFormat) -> Bound<'py, PyString> {
        self.pystr_fmt(py, kind)
    }

    #[pyo3(signature = (format_spec, /))]
    fn __format__<'py>(
        &self,
        py: Python<'py>,
        format_spec: UuidFormat<true>,
    ) -> Bound<'py, PyString> {
        self.pystr_fmt(py, format_spec)
    }

    fn __str__<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        self.pystr_hyphenated(py)
    }

    fn __repr__<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        // stack buf ~ aint no heap string
        let mut buf = [0u8; 44];
        buf[..6].copy_from_slice(b"UUID('");
        self.0.hyphenated().encode_lower(&mut buf[6..42]);
        buf[42..].copy_from_slice(b"')");
        let s = std::str::from_utf8(&buf).expect("wenodis: always ascii");
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn __int__(&self) -> u128 {
        self.0.as_u128()
    }

    // `other` is `&Bound<PyAny>` on PURPOSE
    //
    // WHY?
    //
    // A custom extractor will make a type error and add a note via
    // `add_note(...)` which is (in my napkin benchmarking) slower than
    // returning NotImplemented directly for non-UUID types.
    //
    // A quick bench w/ `timeit` of `uuid == None` showed a 10x perf hit for
    // using a custom extractor:
    // - custom extractor: ~300ns
    // - directly returning NotImplemented: ~29ns
    fn __richcmp__(
        &self,
        other: &Bound<'_, PyAny>,
        op: pyo3::basic::CompareOp,
    ) -> PyResult<ryo3_core::PyRichCmp> {
        use ryo3_core::PyCastExactOpt;
        let other = if let Some(u) = other.cast_exact_opt::<Self>() {
            u.get().0
        } else {
            match other.extract::<CPythonUuid>() {
                Ok(u) => u.0,
                Err(CPythonUuidError::NotAUuid) => return Ok(ryo3_core::PyRichCmp::NotImplemented),
                Err(CPythonUuidError::Py(e)) => return Err(e),
            }
        };
        // cmp as u128 (big-endian so same order as the bytes)
        // ALSO... its faster for some reason?
        let (a, b) = (self.0.as_u128(), other.as_u128());
        let b = match op {
            pyo3::basic::CompareOp::Eq => a == b,
            pyo3::basic::CompareOp::Ne => a != b,
            pyo3::basic::CompareOp::Lt => a < b,
            pyo3::basic::CompareOp::Le => a <= b,
            pyo3::basic::CompareOp::Gt => a > b,
            pyo3::basic::CompareOp::Ge => a >= b,
        };
        Ok(b.into())
    }

    fn equiv(&self, other: &Bound<'_, PyAny>) -> bool {
        use ryo3_core::{PyCastExactOpt, PyCastOpt, pystr_read_fast};
        let b = if let Some(u) = other.cast_exact_opt::<Self>() {
            Some(u.get().0)
        } else if let Some(s) = other.cast_opt::<PyString>() {
            pystr_read_fast(s.as_borrowed())
                .ok()
                .and_then(|s| uuid::Uuid::try_parse(s).ok())
        } else if let Some(b) = other.cast_opt::<pyo3::types::PyBytes>() {
            uuid::Uuid::from_slice(b.as_bytes()).ok()
        } else if let Some(i) = other.cast_opt::<pyo3::types::PyInt>() {
            i.extract::<u128>().ok().map(uuid::Uuid::from_u128)
        } else {
            other.extract::<CPythonUuid>().ok().map(Into::into)
        };
        b == Some(self.0)
    }

    #[getter]
    fn version(&self) -> usize {
        self.0.get_version_num()
    }

    #[getter]
    fn urn<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        self.pystr_urn(py)
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
    fn from_hex(hexstr: &str) -> PyResult<Self> {
        Self::from_str(hexstr)
    }

    #[staticmethod]
    #[pyo3(signature = (s, /))]
    fn from_str(s: &str) -> PyResult<Self> {
        uuid::Uuid::parse_str(s)
            .map(PyUuid)
            .map_err(|e| py_value_error!("invalid UUID string: {}", e))
    }

    #[staticmethod]
    fn from_int(i: u128) -> Self {
        Self::from(uuid::Uuid::from_bytes(i.to_be_bytes()))
    }

    #[staticmethod]
    #[pyo3(name = "from_bytes")]
    fn from_pybytes(b: PyUuidBytes) -> Self {
        Self(uuid::Uuid::from_bytes(b.0))
    }

    #[staticmethod]
    fn from_bytes_le(b: PyUuidBytes) -> Self {
        Self(uuid::Uuid::from_bytes_le(b.0))
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

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        pyo3::types::PyBytes::new(py, self.0.as_bytes())
    }

    #[getter]
    fn bytes<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        self.__bytes__(py)
    }

    #[getter]
    fn bytes_le<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyBytes> {
        let bytes = self.0.to_bytes_le();
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
    fn hex<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        self.pystr_simple(py)
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

    /// The 60-bit timestamp as a count of 100-nanosecond intervals since
    /// Gregorian epoch (1582-10-15 00:00:00) for versions 1 and 6, or the
    /// 48-bit timestamp in milliseconds since Unix epoch (1970-01-01 00:00:00)
    /// for version 7.
    #[getter]
    fn time(&self) -> u64 {
        self.py_time()
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
    #[pyo3(name = "getnode")]
    pub fn pydef_getnode(py: Python<'_>) -> PyResult<u64> {
        getnode(py)
    }

    // ========================================================================
    // CLASSATTR-STATIC FUNCS
    // allows the `UUID` class to behave almost like the Python `uuid` module
    // ========================================================================
    #[classattr]
    #[pyo3(name = "uuid1")]
    fn clsattr_uuid1(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid1, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid3")]
    fn clsattr_uuid3(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid3, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid4")]
    fn clsattr_uuid4(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid4, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid5")]
    fn clsattr_uuid5(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid5, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid6")]
    fn clsattr_uuid6(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid6, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid7")]
    fn clsattr_uuid7(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid7, py)?.into_any().unbind())
    }

    #[classattr]
    #[pyo3(name = "uuid8")]
    fn clsattr_uuid8(py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(wrap_pyfunction!(uuid8, py)?.into_any().unbind())
    }

    /// Reseed the rng
    #[staticmethod]
    #[pyo3(name = "_reseed_rng")]
    fn _py_reseed_rng() -> PyResult<()> {
        Self::reseed_rng()
    }

    #[staticmethod]
    fn from_any<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::{PyCastExactOpt, PyCastOpt};
        let py = value.py();
        if let Some(val) = value.cast_exact_opt::<Self>() {
            Ok(val.as_borrowed().into_bound())
        } else if let Ok(s) = value.extract::<&str>() {
            Self::from_str(s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(b) = value.extract::<[u8; 16]>() {
            Self::from_int(u128::from_be_bytes(b)).into_pyobject(py)
        } else if let Some(pybytes) = value.cast_opt::<pyo3::types::PyBytes>() {
            let s = String::from_utf8_lossy(pybytes.as_bytes());
            Self::from_str(&s).map(|dt| dt.into_pyobject(py))?
        } else if let Ok(v) = value.extract::<CPythonUuid>() {
            Self::from(v.0).into_pyobject(py)
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
    fn _pydantic_validate<'py>(value: &Bound<'py, PyAny>) -> PyResult<Bound<'py, Self>> {
        use ryo3_core::macros::py_value_error;
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

impl std::fmt::Display for PyUuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UUID('{}')", self.0)
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
#[pyfunction(signature = (node = None, clock_seq = None))]
#[expect(unused_variables)]
pub fn uuid1(node: Option<u64>, clock_seq: Option<u16>) -> PyResult<PyUuid> {
    pytodo!("UUID1 is not implemented yet");
}

#[pyfunction]
pub fn uuid3(namespace: &PyUuid, name: &Bound<'_, PyAny>) -> PyResult<PyUuid> {
    if let Ok(s) = name.extract::<&str>() {
        Ok(uuid::Uuid::new_v3(&namespace.0, s.as_bytes()).into())
    } else if let Ok(b) = name.extract::<&[u8]>() {
        Ok(uuid::Uuid::new_v3(&namespace.0, b).into())
    } else {
        py_type_err!("uuid3(): name must be str or bytes-like")
    }
}

#[pyfunction]
pub fn uuid4() -> PyResult<PyUuid> {
    Ok(PyUuid(uuid::Uuid::new_v4()))
}

#[pyfunction]
pub fn uuid5(namespace: &PyUuid, name: &Bound<'_, PyAny>) -> PyResult<PyUuid> {
    if let Ok(s) = name.extract::<&str>() {
        Ok(uuid::Uuid::new_v5(&namespace.0, s.as_bytes()).into())
    } else if let Ok(b) = name.extract::<&[u8]>() {
        Ok(uuid::Uuid::new_v5(&namespace.0, b).into())
    } else {
        py_type_err!("uuid5(): name must be str or bytes-like")
    }
}

/// Generate a UUID (RFC 9562, 5.6) from a sequence number and the current time.
///
/// This is an alternative to `uuid1()` to improve database locality.
///
/// When node is not specified, `getnode()` is used to obtain the hardware
/// address as a 48-bit positive integer. When a sequence number `clock_seq` is
/// not specified, a pseudo-random 14-bit positive integer is generated.
///
/// If node or `clock_seq` exceed their expected bit count, only their least
/// significant bits are kept.
///
/// # Panics
///
/// This function will panic if it fails to convert the node ID to a 6-byte, but
/// there is no-way-jose that could happen AFAICT.
#[pyfunction(signature = (node = None, clock_seq = None))]
pub fn uuid6(py: Python<'_>, node: Option<u64>, clock_seq: Option<u16>) -> PyResult<PyUuid> {
    let node = node.map_or_else(|| getnode(py), |n| Ok(n & 0xFFFF_FFFF_FFFF))?;
    let node_arr = node.to_be_bytes();
    let node_id: &[u8; 6] = &node_arr[2..8].try_into().expect("no-way-jose");
    if let Some(_clock_seq) = clock_seq {
        pytodo!("uuid6 with clock_seq is not implemented yet");
    }
    Ok(uuid::Uuid::now_v6(node_id).into())
}

#[pyfunction(signature = (timestamp = None))]
pub fn uuid7(timestamp: Option<u64>) -> PyResult<PyUuid> {
    match timestamp {
        Some(ms) => {
            // check ms < 2^60 bc UUIDv7 uses 60 bits for timestamp
            const MAX_MS: u64 = (1u64 << 60) - 1;
            if ms > MAX_MS {
                py_value_err!("timestamp too large")
            } else {
                let secs = ms / 1000;
                #[expect(clippy::cast_possible_truncation)]
                let nanos = ((ms % 1000) * 1_000_000) as u32;
                let ts = uuid::Timestamp::from_unix(uuid::NoContext, secs, nanos);
                Ok(uuid::Uuid::new_v7(ts).into())
            }
        }
        None => Ok(uuid::Uuid::now_v7().into()),
    }
}

#[pyfunction(signature = (a = None, b = None, c = None, *, buf = None))]
pub fn uuid8(
    a: Option<u64>,
    b: Option<u16>,
    c: Option<u64>,
    buf: Option<PyUuidBytes>,
) -> PyResult<PyUuid> {
    use rand::prelude::*;

    if let Some(bts) = buf {
        match (a, b, c) {
            (None, None, None) => {}
            _ => {
                return py_value_err!("uuid8(): pass either bytes=... or a/b/c, not both",);
            }
        }
        return Ok(PyUuid::from(uuid::Uuid::new_v8(bts.0)));
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

impl CPythonUuid {
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
        *UUID_TYPE_PTR.get_or_init(py, || Self::get_uuid_ob_pointer(py))
    }

    fn from_borrowed_pyany(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, CPythonUuidError> {
        let py = obj.py();
        let uuid_cls_ptr = Self::py_uuid_type_ptr(obj.py());
        let obj_ptr = obj.get_type().as_type_ptr() as usize;
        if obj_ptr == uuid_cls_ptr {
            let Some(uuid_int) = obj.getattr_opt(pyo3::intern!(py, "int"))? else {
                return Err(CPythonUuidError::NotAUuid);
            };
            let uuid_int: u128 = uuid_int.extract()?;
            Ok(Self(uuid::Uuid::from_u128(uuid_int)))
        } else {
            Err(CPythonUuidError::NotAUuid)
        }
    }
}

/// `CPythonUuid` extract error
///
/// `NotAUuid` is free-ish to create + drop, and the py `TypeError`
/// only gets built if it gets turned into a `PyErr`
#[derive(Debug)]
pub enum CPythonUuidError {
    NotAUuid,
    Py(PyErr),
}

impl From<PyErr> for CPythonUuidError {
    fn from(e: PyErr) -> Self {
        Self::Py(e)
    }
}

impl From<CPythonUuidError> for PyErr {
    fn from(e: CPythonUuidError) -> Self {
        match e {
            CPythonUuidError::NotAUuid => {
                py_type_error!("Expected a `uuid.UUID` instance.")
            }
            CPythonUuidError::Py(e) => e,
        }
    }
}

impl From<CPythonUuid> for uuid::Uuid {
    fn from(val: CPythonUuid) -> Self {
        val.0
    }
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
//             let uuid_int: u128 = obj.getattr(pyo3::intern!(py, "int"))?.extract()?;
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
    type Error = CPythonUuidError;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        Self::from_borrowed_pyany(obj)
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
        let uuid_schema = core_schema.call_method(pyo3::intern!(py, "uuid_schema"), (), None)?;
        let validation_fn = cls.getattr(interns::_pydantic_validate(py))?;
        let string_serialization_schema =
            core_schema.call_method(interns::to_string_ser_schema(py), (), None)?;
        let plain_validator_kwargs = pyo3::types::PyDict::new(py);
        plain_validator_kwargs.set_item(interns::json_schema_input_schema(py), &uuid_schema)?;
        plain_validator_kwargs
            .set_item(interns::serialization(py), &string_serialization_schema)?;
        core_schema.call_method(
            interns::no_info_plain_validator_function(py),
            (&validation_fn,),
            Some(&plain_validator_kwargs),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PyUuidBytes([u8; 16]);

impl<'a, 'py> FromPyObject<'a, 'py> for PyUuidBytes {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        if let Ok(b) = ob.extract::<ExactReadableBuffer<'a, 'py, 16>>() {
            Ok(Self(*b.as_array()))
        } else {
            py_type_err!("Expected a bytes-like object with length == 16")
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum UuidFormat<const ALLOW_EMPTY: bool = false> {
    #[default]
    Hyphenated,
    Simple,
    Urn,
    Braced,
}

impl<'a, 'py, const ALLOW_EMPTY: bool> FromPyObject<'a, 'py> for UuidFormat<ALLOW_EMPTY> {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> PyResult<Self> {
        const UUID_VALUE_ERROR_MSG: &str =
            "Invalid uuid format ('hyphenated', 'simple', 'urn', 'braced')";
        const UUID_TYPE_ERROR_MSG: &str = "Expected str ('hyphenated', 'simple', 'urn', 'braced')";
        // Copy pasta via this repl snippet to match stdlib
        // ```python
        // >>> import uuid
        // >>> uuid.uuid4().__format__("asdf")
        // Traceback (most recent call last):
        //   File "<stdin>", line 1, in <module>
        // TypeError: unsupported format string passed to UUID.__format__
        // ```

        const UUID_FORMAT_SPEC_ERROR_MSG: &str =
            "unsupported format string passed to UUID.__format__";
        use ryo3_core::{PyCastExactOpt, pystr_read_fast};
        if let Some(s) = ob.cast_exact_opt::<PyString>() {
            let s = pystr_read_fast(s)?;
            if ALLOW_EMPTY && s.is_empty() {
                return Ok(Self::Hyphenated);
            }
            match s {
                "hyphenated" => Ok(Self::Hyphenated),
                "simple" => Ok(Self::Simple),
                "urn" => Ok(Self::Urn),
                "braced" => Ok(Self::Braced),
                _ if ALLOW_EMPTY => py_type_err!(UUID_FORMAT_SPEC_ERROR_MSG),
                _ => py_value_err!(UUID_VALUE_ERROR_MSG),
            }
        } else {
            py_type_err!(UUID_TYPE_ERROR_MSG)
        }
    }
}
