use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyInt, PyString};
use ryo3_core::macros::{
    py_overflow_error, py_runtime_error, py_type_err, py_value_err, py_value_error,
};
use ryo3_core::{PyAsciiStr, pystr_read_fast};
use ryo3_uuid::{CPythonUuid, RyUuid};
use ulid::Ulid;
use uuid::Uuid;

static ULID_GENERATOR: OnceLock<Mutex<ulid::Generator>> = OnceLock::new();

fn generator() -> &'static Mutex<ulid::Generator> {
    ULID_GENERATOR.get_or_init(|| Mutex::new(ulid::Generator::new()))
}

fn gen_new() -> PyResult<Ulid> {
    generator()
        .lock()
        .map_err(|_| py_runtime_error!("ulid-generator lock-error"))?
        .generate()
        .map_err(|_| py_overflow_error!("ulid-generator overflow"))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[pyclass(name = "ULID", frozen, weakref, eq, ord)]
#[cfg_attr(any(not(Py_LIMITED_API), Py_3_14), pyo3(immutable_type))]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ryo3"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct RyUlid(Ulid);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RyUlid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ulid::deserialize(deserializer).map(RyUlid)
    }
}

struct PyUlidDecodeError(::ulid::DecodeError);

impl From<PyUlidDecodeError> for PyErr {
    fn from(error: PyUlidDecodeError) -> Self {
        match error.0 {
            ulid::DecodeError::InvalidLength => py_value_error!("Invalid ULID string length"),
            ulid::DecodeError::InvalidChar => py_value_error!("Invalid character in ULID string"),
        }
    }
}

impl RyUlid {
    #[must_use]
    pub const fn new(ulid: Ulid) -> Self {
        Self(ulid)
    }

    const fn to_u128(self) -> u128 {
        self.0.0
    }

    fn hex2bytes(hex: &str) -> PyResult<[u8; 16]> {
        if hex.len() != 32 {
            return py_value_err!("Hex string must be exactly 32 characters long");
        }
        Uuid::try_parse(hex)
            .map(Uuid::into_bytes)
            .map_err(|_| py_value_error!("Invalid hex character"))
    }

    fn parse_str_u128(s: &str) -> Option<u128> {
        match s.len() {
            ulid::ULID_LEN => Ulid::from_string(s).ok().map(|u| u.0),
            32 | 36 => Uuid::try_parse(s).ok().map(|u| u.as_u128()),
            _ => None,
        }
    }

    /// 26 char crockford string
    fn pystr<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        // stack buf, no heap string
        let mut buf = [0u8; ulid::ULID_LEN];
        let s = self.0.array_to_str(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    const fn from_stringc(encoded: &str) -> Result<Self, PyUlidDecodeError> {
        match Ulid::from_string(encoded) {
            Ok(ulid) => Ok(Self(ulid)),
            Err(e) => Err(PyUlidDecodeError(e)),
        }
    }

    pub fn py_from_string(encoded: &str) -> PyResult<Self> {
        Self::from_stringc(encoded).map_err(Into::into)
    }

    const fn from_u128(value: u128) -> Self {
        Self(Ulid(value))
    }

    /// Reset the (monotonic) ulid generator
    ///
    /// This is called on unix-y systems after fork and is registered when the
    /// module is registered w/ the below snippet:
    ///
    /// ```python
    /// os.register_at_fork(after_in_child=ULID._reset_generator)
    /// ```
    ///
    /// w/o it the child inherits the parent's generator state and hands out
    /// the same "next" ulid as the parent (see `UUID._reseed_rng` for the
    /// rng side of things). `try_lock` bc a lock held by a thread that didn't
    /// survive the fork would deadlock here.
    fn reset_generator() {
        if let Some(g) = ULID_GENERATOR.get()
            && let Ok(mut g) = g.try_lock()
        {
            *g = ulid::Generator::new();
        }
    }
}

#[pymethods]
impl RyUlid {
    /// The number of bits in a Ulid's time portion
    #[classattr]
    pub const TIME_BITS: u8 = 48;

    /// The number of bits in a Ulid's random portion
    #[classattr]
    pub const RAND_BITS: u8 = 80;

    /// The 'max Ulid'.
    ///
    /// The max Ulid is special form of Ulid that is specified to have
    /// all 128 bits set to one.
    #[classattr]
    pub const MAX: Self = Self::new(Ulid::max());

    /// The 'nil Ulid'.
    #[classattr]
    pub const NIL: Self = Self::new(Ulid::nil());

    #[new]
    #[pyo3(signature = (value = None))]
    pub fn py_new(value: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
        use ryo3_core::PyCastOpt;
        if let Some(value) = value {
            if let Some(s) = value.cast_opt::<PyString>() {
                Self::from_str(pystr_read_fast(s.as_borrowed())?)
            } else if let Some(b) = value.cast_opt::<PyBytes>() {
                let b: [u8; 16] = b
                    .as_bytes()
                    .try_into()
                    .map_err(|_| py_value_error!("ULID must be exactly 16 bytes long"))?;
                Ok(Self::from_bytes(b))
            } else {
                py_type_err!("Expected a ULID string (26 or 32 characters) or bytes (16 bytes)")
            }
        } else {
            let ulid = gen_new()?;
            Ok(Self(ulid))
        }
    }

    fn __str__<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        self.pystr(py)
    }

    fn __repr__<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        // stack buf ~ `ULID('` + 26 + `')`
        let mut buf = [0u8; 34];
        buf[..6].copy_from_slice(b"ULID('");
        let mut ubuf = [0u8; ulid::ULID_LEN];
        buf[6..32].copy_from_slice(self.0.array_to_str(&mut ubuf).as_bytes());
        buf[32..].copy_from_slice(b"')");
        let s = std::str::from_utf8(&buf).expect("wenodis: always ascii");
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    fn __int__(&self) -> u128 {
        self.to_u128()
    }

    fn __hash__(&self) -> isize {
        // python's int hash modulus (`sys.hash_info.modulus`)
        const M: u128 = if isize::BITS >= 64 {
            (1 << 61) - 1
        } else {
            (1 << 31) - 1
        };
        (self.to_u128() % M) as isize
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> (Bound<'py, pyo3::types::PyType>, (Bound<'py, PyString>,)) {
        (py.get_type::<Self>(), (self.pystr(py),))
    }

    #[pyo3(signature = (other, /))]
    fn equiv(&self, other: &Bound<'_, PyAny>) -> bool {
        use ryo3_core::{PyCastExactOpt, PyCastOpt};
        let a = self.to_u128();
        let b = if let Some(u) = other.cast_exact_opt::<Self>() {
            Some(u.get().to_u128())
        } else if let Some(i) = other.cast_opt::<PyInt>() {
            i.extract::<u128>().ok()
        } else if let Some(s) = other.cast_opt::<PyString>() {
            pystr_read_fast(s.as_borrowed())
                .ok()
                .and_then(Self::parse_str_u128)
        } else if let Some(pybytes) = other.cast_opt::<PyBytes>() {
            <[u8; 16]>::try_from(pybytes.as_bytes())
                .ok()
                .map(u128::from_be_bytes)
        } else if let Some(u) = other.cast_exact_opt::<RyUuid>() {
            Some(u.get().inner().as_u128())
        } else {
            other
                .extract::<CPythonUuid>()
                .ok()
                .map(|u| Uuid::from(u).as_u128())
        };
        b == Some(a)
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.to_bytes())
    }

    /// Reset the ulid generator
    #[staticmethod]
    #[pyo3(name = "_reset_generator")]
    #[expect(clippy::used_underscore_items, reason = "python-private-method")]
    fn _py_reset_generator() {
        Self::reset_generator();
    }

    #[staticmethod]
    fn from_bytes(b: [u8; 16]) -> Self {
        let ulid = Ulid::from_bytes(b);
        Self(ulid)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        self.__bytes__(py)
    }

    #[staticmethod]
    fn from_hex(hexstr: &str) -> PyResult<Self> {
        let b = Self::hex2bytes(hexstr)?;
        let ul = Ulid::from_bytes(b);
        Ok(Self(ul))
    }

    #[staticmethod]
    fn from_int(i: u128) -> Self {
        Self::from_u128(i)
    }

    #[staticmethod]
    #[pyo3(signature = (s, /))]
    fn from_string(s: &str) -> PyResult<Self> {
        Self::from_str(s)
    }

    #[staticmethod]
    #[pyo3(signature = (s, /))]
    fn from_str(s: &str) -> PyResult<Self> {
        if s.len() == 26 {
            Self::py_from_string(s)
        } else if s.len() == 32 {
            Self::from_hex(s)
        } else {
            py_value_err!("ULID string must be either 26 or 32 characters long",)
        }
    }

    #[staticmethod]
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn from_timestamp_seconds(value: f64) -> PyResult<Self> {
        if value < 0.0 {
            py_value_err!("Timestamp cannot be negative")
        } else {
            let millis = (value * 1000.0) as u64;
            let dt = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(millis);
            Ok(Self(Ulid::from_datetime(dt)))
        }
    }

    #[staticmethod]
    fn from_timestamp_milliseconds(value: u64) -> PyResult<Self> {
        let dt = SystemTime::UNIX_EPOCH.checked_add(std::time::Duration::from_millis(value));
        let dt = dt.ok_or_else(|| {
            py_overflow_error!("Timestamp exceeds the maximum value for SystemTime")
        })?;
        Ok(Self(Ulid::from_datetime(dt)))
    }

    #[staticmethod]
    fn from_timestamp(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pyint) = value.cast::<pyo3::types::PyInt>() {
            let i = pyint
                .extract::<u64>()
                .map_err(|_| py_overflow_error!("value exceeds u64::MAX"))?;
            Self::from_timestamp_milliseconds(i)
        } else if let Ok(pyfloat) = value.cast::<pyo3::types::PyFloat>() {
            let f = pyfloat.extract::<f64>()?;
            Self::from_timestamp_seconds(f)
        } else {
            py_type_err!("Expected a float (seconds) or int (ms) for timestamp")
        }
    }

    #[staticmethod]
    fn from_datetime(value: SystemTime) -> Self {
        Self::from(Ulid::from_datetime(value))
    }

    #[staticmethod]
    fn from_uuid(uu: UuidLike) -> Self {
        let uu = uu.0;
        let ul = Ulid::from_bytes(*uu.as_bytes());
        Self(ul)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_uuid(&self) -> RyUuid {
        let b = self.0.to_bytes();
        ryo3_uuid::RyUuid::from(Uuid::from_bytes(b))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_uuid4(&self) -> RyUuid {
        let mut b = uuid::Builder::from_u128(self.to_u128());
        b.set_version(uuid::Version::Random);
        RyUuid::from(b.into_uuid())
    }

    #[staticmethod]
    fn parse(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        use ryo3_core::{PyCastExactOpt, PyCastOpt};
        if let Some(rs_ulid) = value.cast_exact_opt::<Self>() {
            Ok(*rs_ulid.get())
        } else if let Some(pyint) = value.cast_opt::<PyInt>() {
            let i = pyint.extract::<u128>()?;
            if let Ok(smaller_int) = u64::try_from(i) {
                let ulid = Ulid::from_datetime(
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(smaller_int),
                );
                return Ok(Self(ulid));
            }
            // if the integer is too damn big, treat it as a ulid int (u128)
            Ok(Self::from_int(i))
        } else if let Some(s) = value.cast_opt::<PyString>() {
            let cs = pystr_read_fast(s.as_borrowed())?;
            // uuid string length
            match cs.len() {
                36 => {
                    let uu = Uuid::parse_str(cs)
                        .map_err(|e| py_value_error!("Invalid UUID string: {e}"))?;
                    let ul = Ulid::from_bytes(*uu.as_bytes());
                    Ok(Self(ul))
                }
                26 => Self::py_from_string(cs),
                32 => Self::from_hex(cs),
                _ => py_value_err!(
                    "Cannot parse ULID from string of length {} (expected 26, 32, or 36)",
                    cs.len()
                ),
            }
        }
        // has to go through `isinstance` apparatus
        else if value.is_instance_of::<pyo3::types::PyFloat>() {
            let f = value.extract::<f64>()?;
            Self::from_timestamp_seconds(f)
        } else if let Some(pybytes) = value.cast_opt::<PyBytes>() {
            let b = pybytes.extract::<[u8; 16]>()?;
            Ok(Self::from_bytes(b))
        } else if let Some(py_uuid) = value.cast_exact_opt::<RyUuid>() {
            Ok(Self::from_uuid(UuidLike(*py_uuid.get().inner())))
        } else if let Ok(c_uuid) = value.extract::<CPythonUuid>() {
            Ok(Self::from_uuid(UuidLike(c_uuid.into())))
        } else if let Ok(dt) = value.extract::<SystemTime>() {
            Ok(Self::from_datetime(dt))
        } else {
            let other_type = value.get_type();
            let other_type_name = other_type
                .name()
                .map_or_else(|_| String::from("Unknown"), |e| e.as_borrowed().to_string());
            py_type_err!("Cannot parse ULID from type {other_type_name}")
        }
    }

    // -----------------------------------------------------------------------
    // PROPERTIES
    // -----------------------------------------------------------------------

    /// Return python-bytes of the ULID.
    #[getter]
    fn bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        self.__bytes__(py)
    }

    #[getter]
    fn datetime(&self) -> SystemTime {
        self.0.datetime()
    }

    #[getter]
    fn hex<'py>(&self, py: Python<'py>) -> Bound<'py, PyString> {
        let mut buf = Uuid::encode_buffer();
        let s = Uuid::from_u128(self.to_u128())
            .simple()
            .encode_upper(&mut buf);
        let Ok(s) = PyAsciiStr::new(s).into_pyobject(py);
        s
    }

    #[getter]
    fn milliseconds(&self) -> u64 {
        self.0.timestamp_ms()
    }

    #[getter]
    #[expect(
        clippy::cast_precision_loss,
        reason = "wenodis: 48-bit timestamp is ALWAYS ok as f64"
    )]
    fn timestamp(&self) -> f64 {
        self.0.timestamp_ms() as f64 / 1000.0
    }

    /// This is a hideous function but I struggled through this to try to figure
    /// out how to do pydantic schema validators which I hope to do for jiff
    /// soon... (as-of: 2025-05-29)
    #[cfg(feature = "pydantic")]
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        source: &Bound<'py, pyo3::types::PyAny>,
        handler: &Bound<'py, pyo3::types::PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::GetPydanticCoreSchemaCls;
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    #[pyo3(name = "_pydantic_validate")]
    fn py_pydantic_validate<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        value: &Bound<'py, pyo3::types::PyAny>,
        handler: &Bound<'py, pyo3::types::PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3::IntoPyObjectExt;
        let py = value.py();
        let ulid = if let Ok(pyint) = value.cast::<pyo3::types::PyInt>() {
            cls.call_method1(pyo3::intern!(py, "from_int"), (pyint,))
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            cls.call_method1(pyo3::intern!(py, "from_str"), (pystr,))
        } else if let Ok(pyulid) = value.cast_exact::<Self>() {
            pyulid.into_bound_py_any(py)
        } else if let Ok(pybytes) = value.cast::<PyBytes>() {
            cls.call_method1(pyo3::intern!(py, "from_bytes"), (pybytes,))
        } else {
            py_type_err!("Unrecognized format for ULID")
        }?;
        handler.call1((ulid,))
    }

    #[cfg(feature = "pydantic")]
    #[classmethod]
    #[pyo3(name = "_pydantic_validate_strict")]
    fn py_pydantic_validate_strict<'py>(
        cls: &Bound<'py, pyo3::types::PyType>,
        value: &Bound<'py, pyo3::types::PyAny>,
        handler: &Bound<'py, pyo3::types::PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use pyo3::IntoPyObjectExt;
        let py = value.py();
        let ulid = if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            cls.call_method1(pyo3::intern!(py, "from_str"), (pystr,))
        } else if let Ok(pyulid) = value.cast_exact::<Self>() {
            pyulid.into_bound_py_any(py)
        } else {
            py_value_err!("Unrecognized format for ULID (strict)")
        }?;
        handler.call1((ulid,))
    }
}

impl From<Ulid> for RyUlid {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}

impl std::fmt::Display for RyUlid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ULID('{}')", self.0)
    }
}

#[derive(Clone, Copy)]
struct UuidLike(pub(crate) Uuid);

impl<'py> FromPyObject<'_, 'py> for UuidLike {
    type Error = PyErr;
    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> PyResult<Self> {
        use ryo3_core::PyCastExactOpt;
        if let Some(uuid_like) = obj.cast_exact_opt::<RyUuid>() {
            Ok(Self(*uuid_like.get().inner()))
        } else if let Ok(py_uuid) = obj.extract::<CPythonUuid>() {
            Ok(Self(py_uuid.into()))
        } else {
            py_type_err!("Expected a `uuid.UUID` instance.")
        }
    }
}

#[cfg(feature = "pydantic")]
mod pydantic {
    use pyo3::prelude::*;
    use pyo3::types::PyType;
    use ryo3_pydantic::GetPydanticCoreSchemaCls;

    use super::RyUlid;
    impl GetPydanticCoreSchemaCls for RyUlid {
        fn get_pydantic_core_schema<'py>(
            cls: &Bound<'py, PyType>,
            source: &Bound<'py, PyAny>,
            _handler: &Bound<'py, PyAny>,
        ) -> PyResult<Bound<'py, PyAny>> {
            use pyo3::types::PyDict;
            use ryo3_pydantic::interns;
            let py = source.py();
            let core_schema = ryo3_pydantic::core_schema(py)?;

            // oof this is hideous, but it works
            let str_schema_kwargs = PyDict::new(py);
            str_schema_kwargs.set_item(interns::pattern(py), pyo3::intern!(py, r"[A-Z0-9]{26}"))?;
            str_schema_kwargs.set_item(interns::min_length(py), 26)?;
            str_schema_kwargs.set_item(interns::max_length(py), 26)?;

            // more hideousness
            let bytes_schema_kwargs = PyDict::new(py);
            bytes_schema_kwargs.set_item(interns::min_length(py), 16)?;
            bytes_schema_kwargs.set_item(interns::max_length(py), 16)?;

            // actual validator functions
            let pydantic_validate = cls.getattr(interns::_pydantic_validate(py))?;
            let pydantic_validate_strict = cls.getattr(interns::_pydantic_validate_strict(py))?;

            let to_string_ser_schema_kwargs = PyDict::new(py);
            to_string_ser_schema_kwargs
                .set_item(interns::when_used(py), interns::json_unless_none(py))?;
            let to_string_ser_schema = core_schema.call_method(
                interns::to_string_ser_schema(py),
                (),
                Some(&to_string_ser_schema_kwargs),
            )?;

            let no_info_wrap_validator_function_kwargs = PyDict::new(py);
            no_info_wrap_validator_function_kwargs
                .set_item(interns::serialization(py), &to_string_ser_schema)?;

            // LAX union schema (allows ULID, string, bytes)
            let lax_union_schema = core_schema.call_method1(
                interns::union_schema(py),
                (vec![
                    core_schema
                        .call_method1(interns::is_instance_schema(py), (py.get_type::<Self>(),))?,
                    core_schema.call_method1(
                        interns::no_info_plain_validator_function(py),
                        (py.get_type::<Self>(),),
                    )?,
                    core_schema.call_method(
                        interns::str_schema(py),
                        (),
                        Some(&str_schema_kwargs),
                    )?,
                    core_schema.call_method(
                        interns::bytes_schema(py),
                        (),
                        Some(&bytes_schema_kwargs),
                    )?,
                ],),
            )?;

            let strict_union = core_schema.call_method1(
                interns::union_schema(py),
                (vec![
                    core_schema
                        .call_method1(interns::is_instance_schema(py), (py.get_type::<Self>(),))?,
                    core_schema.call_method(
                        interns::str_schema(py),
                        (),
                        Some(&str_schema_kwargs), // still allow canonical string
                    )?,
                ],),
            )?;

            let strict_schema = core_schema.call_method(
                interns::no_info_wrap_validator_function(py),
                (pydantic_validate_strict, strict_union),
                Some(&no_info_wrap_validator_function_kwargs),
            )?;

            let ulid_schema_kwargs = PyDict::new(py);
            ulid_schema_kwargs.set_item(interns::serialization(py), &to_string_ser_schema)?;

            let lax_schema = core_schema.call_method(
                interns::no_info_wrap_validator_function(py),
                (pydantic_validate, lax_union_schema),
                Some(&no_info_wrap_validator_function_kwargs),
            )?;
            let ulid_schema = core_schema.call_method(
                interns::lax_or_strict_schema(py),
                (lax_schema, strict_schema),
                Some(&ulid_schema_kwargs),
            )?;
            Ok(ulid_schema)
        }
    }
}
