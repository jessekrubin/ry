#![doc = include_str!("../README.md")]
use pyo3::exceptions::{PyOverflowError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::types::{PyBytes, PyDict, PyModule, PyType};
use pyo3::{IntoPyObjectExt, intern, prelude::*};
use ryo3_pydantic::GetPydanticCoreSchemaCls;
use ryo3_uuid::{CPythonUuid, PyUuid};
use std::fmt::Write;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;
use ulid::Ulid;
use uuid::Uuid;

static ULID_GENERATOR: OnceLock<Mutex<ulid::Generator>> = OnceLock::new();

fn generator() -> &'static Mutex<ulid::Generator> {
    ULID_GENERATOR.get_or_init(|| Mutex::new(ulid::Generator::new()))
}

fn gen_new() -> PyResult<Ulid> {
    generator()
        .lock()
        .map_err(|_| PyRuntimeError::new_err("ulid-generator-lock-error"))?
        .generate()
        .map_err(|_| PyOverflowError::new_err("ULID-generator overflow"))
}

#[pyclass(name = "ULID", frozen, weakref)]
#[cfg_attr(feature = "ry", pyo3(module = "ry.ulid"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PyUlid(Ulid);

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for PyUlid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ulid::deserialize(deserializer).map(PyUlid)
    }
}

impl PyUlid {
    fn to_u128(self) -> u128 {
        let b = self.0.to_bytes();
        u128::from_be_bytes(b)
    }

    fn hex2bytes(hex: &str) -> PyResult<[u8; 16]> {
        if hex.len() != 32 {
            return Err(PyValueError::new_err(
                "Hex string must be exactly 32 characters long",
            ));
        }
        #[expect(clippy::cast_possible_truncation)]
        let char2byte = |c: char| {
            c.to_digit(16)
                .map(|d| d as u8)
                .ok_or_else(|| PyValueError::new_err("Invalid hex character"))
        };

        let mut bytes = [0u8; 16];
        for (i, chunk) in hex.as_bytes().chunks(2).enumerate() {
            let high = char2byte(chunk[0] as char)?;
            let low = char2byte(chunk[1] as char)?;
            bytes[i] = (high << 4) | low;
        }
        Ok(bytes)
    }
}

#[pymethods]
impl PyUlid {
    #[new]
    #[pyo3(signature = (value = None))]
    pub fn py_new(value: Option<Bound<'_, PyAny>>) -> PyResult<Self> {
        if let Some(value) = value {
            // IF IS BYTES
            if let Ok(b) = value.cast::<PyBytes>() {
                let slice = b.as_bytes();
                let b: [u8; 16] = slice
                    .try_into()
                    .map_err(|_| PyValueError::new_err("ULID must be exactly 16 bytes long"))?;
                Ok(Self::from_bytes(b))
            } else if let Ok(str) = value.cast::<pyo3::types::PyString>() {
                let cs = str.to_str()?;
                Self::from_string(cs)
            } else {
                Err(PyTypeError::new_err(
                    "Expected a ULID string (26 or 32 characters) or bytes (16 bytes)",
                ))
            }
        } else {
            let ulid = gen_new()?;
            Ok(Self(ulid))
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("ULID('{}')", self.0.to_string())
    }

    fn __int__(&self) -> u128 {
        let b = self.0.to_bytes();
        u128::from_be_bytes(b)
    }

    fn __hash__(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }

    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        if let Ok(pyint) = other.cast::<pyo3::types::PyInt>() {
            let other: u128 = pyint.extract()?;
            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.to_u128() == other),
                pyo3::basic::CompareOp::Ne => Ok(self.to_u128() != other),
                pyo3::basic::CompareOp::Lt => Ok(self.to_u128() < other),
                pyo3::basic::CompareOp::Le => Ok(self.to_u128() <= other),
                pyo3::basic::CompareOp::Gt => Ok(self.to_u128() > other),
                pyo3::basic::CompareOp::Ge => Ok(self.to_u128() >= other),
            }
        } else if other.is_instance_of::<pyo3::types::PyString>() {
            let s = other.cast::<pyo3::types::PyString>()?;
            // visitor.visit_str(&s.to_cow()?)
            let cs = s.to_str()?;
            let this_str = self.0.to_string();
            match op {
                pyo3::basic::CompareOp::Eq => Ok(this_str.as_str() == cs),
                pyo3::basic::CompareOp::Ne => Ok(this_str.as_str() != cs),
                pyo3::basic::CompareOp::Lt => Ok(this_str.as_str() < cs),
                pyo3::basic::CompareOp::Le => Ok(this_str.as_str() <= cs),
                pyo3::basic::CompareOp::Gt => Ok(this_str.as_str() > cs),
                pyo3::basic::CompareOp::Ge => Ok(this_str.as_str() >= cs),
            }
        } else if let Ok(rs_ulid) = other.cast::<Self>() {
            let other = rs_ulid.borrow().0;
            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other),
            }
        } else if let Ok(pybytes) = other.cast::<PyBytes>() {
            let slice = pybytes.as_bytes();
            match slice.len() {
                16 => {
                    let ulid = Ulid::from_bytes(
                        slice
                            .try_into()
                            .expect("never to happen; checked length above"),
                    );
                    match op {
                        pyo3::basic::CompareOp::Eq => Ok(self.0 == ulid),
                        pyo3::basic::CompareOp::Ne => Ok(self.0 != ulid),
                        pyo3::basic::CompareOp::Lt => Ok(self.0 < ulid),
                        pyo3::basic::CompareOp::Le => Ok(self.0 <= ulid),
                        pyo3::basic::CompareOp::Gt => Ok(self.0 > ulid),
                        pyo3::basic::CompareOp::Ge => Ok(self.0 >= ulid),
                    }
                }
                _ => Err(PyValueError::new_err("Bytes must be exactly 16 bytes long")),
            }
        } else {
            match op {
                pyo3::basic::CompareOp::Eq => Ok(false),
                pyo3::basic::CompareOp::Ne => Ok(true),
                _ => Err(PyTypeError::new_err(
                    "Cannot compare ULID with the given type",
                )),
            }
        }
    }

    fn __bytes__<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, &self.0.to_bytes())
    }

    #[staticmethod]
    fn from_bytes(bytes: [u8; 16]) -> Self {
        let ulid = Ulid::from_bytes(bytes);
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
    fn from_int(bytes: u128) -> Self {
        let b = bytes.to_be_bytes();
        let ul = Ulid::from_bytes(b);
        Self(ul)
    }

    #[staticmethod]
    fn from_string(cs: &str) -> PyResult<Self> {
        if cs.len() == 26 {
            let ulid = Ulid::from_string(cs)
                .map_err(|e| PyValueError::new_err(format!("Invalid ULID string: {e}")))?;
            Ok(Self(ulid))
        } else if cs.len() == 32 {
            Self::from_hex(cs)
        } else {
            Err(PyValueError::new_err(
                "ULID string must be either 26 or 32 characters long",
            ))
        }
    }

    #[staticmethod]
    fn from_str(s: &str) -> PyResult<Self> {
        Self::from_string(s)
    }

    #[staticmethod]
    #[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn from_timestamp_seconds(value: f64) -> PyResult<Self> {
        if value < 0.0 {
            Err(PyValueError::new_err("Timestamp cannot be negative"))
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
            PyOverflowError::new_err("Timestamp exceeds the maximum value for SystemTime")
        })?;
        Ok(Self(Ulid::from_datetime(dt)))
    }

    #[staticmethod]
    fn from_timestamp(value: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pyint) = value.cast::<pyo3::types::PyInt>() {
            let i = pyint
                .extract::<u64>()
                .map_err(|_| PyOverflowError::new_err("value gt u64::MAX"))?;
            Self::from_timestamp_milliseconds(i)
        } else if let Ok(pyfloat) = value.cast::<pyo3::types::PyFloat>() {
            let f = pyfloat.extract::<f64>()?;
            Self::from_timestamp_seconds(f)
        } else {
            Err(PyTypeError::new_err(
                "Expected a float (seconds) or int (ms) for timestamp",
            ))
        }
    }

    #[staticmethod]
    fn from_datetime(bytes: SystemTime) -> Self {
        Self::from(Ulid::from_datetime(bytes))
    }

    #[staticmethod]
    fn from_uuid(uu: UuidLike) -> Self {
        let uu = uu.0;
        let ul = Ulid::from_bytes(*uu.as_bytes());
        Self(ul)
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_uuid(&self) -> PyUuid {
        let b = self.0.to_bytes();
        ryo3_uuid::PyUuid::from(Uuid::from_bytes(b))
    }

    #[expect(clippy::wrong_self_convention)]
    fn to_uuid4(&self) -> PyUuid {
        let mut b = uuid::Builder::from_u128(self.to_u128());
        b.set_version(uuid::Version::Random);
        PyUuid::from(b.into_uuid())
    }

    #[staticmethod]
    fn parse(other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pyint) = other.cast::<pyo3::types::PyInt>() {
            let i = pyint.extract::<u128>()?;
            if let Ok(smaller_int) = u64::try_from(i) {
                let ulid = Ulid::from_datetime(
                    SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(smaller_int),
                );
                return Ok(Self(ulid));
            }
            // If the integer is too large, we treat it as a ULID.
            Ok(Self::from_int(i))
        } else if other.is_instance_of::<pyo3::types::PyString>() {
            let s = other.cast::<pyo3::types::PyString>()?;
            let cs = s.to_str()?;
            // uuid string length
            match cs.len() {
                36 => {
                    let uu = Uuid::parse_str(cs)
                        .map_err(|e| PyValueError::new_err(format!("Invalid UUID string: {e}")))?;
                    let ul = Ulid::from_bytes(*uu.as_bytes());
                    Ok(Self(ul))
                }
                26 => {
                    let ulid = Ulid::from_string(cs)
                        .map_err(|e| PyValueError::new_err(format!("Invalid ULID string: {e}")))?;
                    Ok(Self(ulid))
                }
                32 => Self::from_hex(cs),
                _ => Err(PyValueError::new_err(format!(
                    "Cannot parse ULID from string of length {}",
                    cs.len()
                ))),
            }
        }
        // has to go through `isinstance` apparatus
        else if other.is_instance_of::<pyo3::types::PyFloat>() {
            let f = other.extract::<f64>()?;
            Self::from_timestamp_seconds(f)
        } else if let Ok(rs_ulid) = other.cast::<Self>() {
            let inner = rs_ulid.borrow().0;
            Ok(Self(inner))
        } else if other.is_instance_of::<PyBytes>() {
            let pybytes = other.cast::<PyBytes>()?;
            let b = pybytes.extract::<[u8; 16]>()?;
            Ok(Self::from_bytes(b))
        } else if let Ok(py_uuid) = other.cast::<PyUuid>() {
            return Ok(Self::from_uuid(UuidLike(*py_uuid.borrow().get())));
        } else if let Ok(c_uuid) = other.extract::<CPythonUuid>() {
            Ok(Self::from_uuid(UuidLike(c_uuid.into())))
        } else if let Ok(dt) = other.extract::<SystemTime>() {
            Ok(Self::from_datetime(dt))
        } else {
            let other_type = other.get_type();
            let other_type_name = other_type
                .name()
                .map_or_else(|_| String::from("Unknown"), |e| e.as_borrowed().to_string());
            Err(PyTypeError::new_err(format!(
                "Cannot parse ULID from type {other_type_name}"
            )))
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
    fn hex(&self) -> String {
        self.0
            .to_bytes()
            .into_iter()
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02X}");
                output
            })
    }

    #[getter]
    fn milliseconds(&self) -> u64 {
        self.0.timestamp_ms()
    }

    #[getter]
    #[expect(clippy::cast_precision_loss)]
    fn timestamp(&self) -> f64 {
        self.0.timestamp_ms() as f64 / 1000.0
    }

    /// This is a hideous function but I struggled through this to try to figure out how to
    /// do pydantic schema validators which I hope to do for jiff soon... (as-of: 2025-05-29)
    #[classmethod]
    fn __get_pydantic_core_schema__<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        Self::get_pydantic_core_schema(cls, source, handler)
    }

    #[classmethod]
    fn _pydantic_validate<'py>(
        cls: &Bound<'py, PyType>,
        value: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = value.py();
        let ulid = if let Ok(pyint) = value.cast::<pyo3::types::PyInt>() {
            cls.call_method1(intern!(py, "from_int"), (pyint,))
        } else if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            cls.call_method1(intern!(py, "from_str"), (pystr,))
        } else if let Ok(pyulid) = value.cast::<Self>() {
            pyulid.into_bound_py_any(py)
        } else if let Ok(pybytes) = value.cast::<PyBytes>() {
            cls.call_method1(intern!(py, "from_bytes"), (pybytes,))
        } else {
            Err(PyTypeError::new_err("Unrecognized format for ULID"))
        }?;
        handler.call1((ulid,))
    }

    #[classmethod]
    fn _pydantic_validate_strict<'py>(
        cls: &Bound<'py, PyType>,
        value: &Bound<'py, PyAny>,
        handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let py = value.py();
        let ulid = if let Ok(pystr) = value.cast::<pyo3::types::PyString>() {
            cls.call_method1(intern!(py, "from_str"), (pystr,))
        } else if let Ok(pyulid) = value.cast::<Self>() {
            pyulid.into_bound_py_any(py)
        } else {
            Err(PyValueError::new_err(
                "Unrecognized format for ULID (strict)",
            ))
        }?;
        handler.call1((ulid,))
    }
}

impl From<Ulid> for PyUlid {
    fn from(ulid: Ulid) -> Self {
        Self(ulid)
    }
}

#[derive(Clone, Copy)]
struct UuidLike(pub(crate) Uuid);

impl FromPyObject<'_> for UuidLike {
    fn extract_bound(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(uuid_like) = obj.cast::<PyUuid>() {
            return Ok(Self(*uuid_like.borrow().get()));
        } else if let Ok(py_uuid) = obj.extract::<CPythonUuid>() {
            return Ok(Self(py_uuid.into()));
        }
        Err(PyTypeError::new_err("Expected a `uuid.UUID` instance."))
    }
}

impl GetPydanticCoreSchemaCls for PyUlid {
    fn get_pydantic_core_schema<'py>(
        cls: &Bound<'py, PyType>,
        source: &Bound<'py, PyAny>,
        _handler: &Bound<'py, PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        use ryo3_pydantic::interns;
        let py = source.py();
        // let core_schema = py.import(intern!(py, "pydantic_core.core_schema"))?;
        let core_schema = ryo3_pydantic::core_schema(py)?;

        // let core_schema = core_schema.getattr(intern!(py, "core_schema"))?;

        // oy vey this is hideous, but it works
        let str_schema_kwargs = PyDict::new(py);
        str_schema_kwargs.set_item(interns::pattern(py), intern!(py, r"[A-Z0-9]{26}"))?;
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
                core_schema.call_method(interns::str_schema(py), (), Some(&str_schema_kwargs))?,
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

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUlid>()?;
    Ok(())
}
