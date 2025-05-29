#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyModule};

use pyo3::exceptions::{ PyTypeError, PyValueError};
use pyo3::prelude::*;
use ryo3_uuid::{CPythonUuid, PyUuid};
use ulid::Ulid;
use uuid::Uuid;
use std::alloc::System;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::SystemTime;


#[pyclass(name = "ULID", module = "ry.ulid", frozen, weakref)]
pub struct PyUlid(pub ulid::Ulid);

impl PyUlid {
    fn to_u128(&self) -> u128 {
        let b = self.0.to_bytes();
        u128::from_be_bytes(b)
    }

    fn hex2bytes(hex: &str) -> PyResult<[u8; 16]> {
        if hex.len() != 32 {
            return Err(PyValueError::new_err("Hex string must be exactly 32 characters long"));
        }
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

// def __init__(self, value: bytes | None = None) -> None:
//     if value is not None and len(value) != constants.BYTES_LEN:
//         raise ValueError("ULID has to be exactly 16 bytes long.")
//     self.bytes: bytes = (
//         value or ULID.from_timestamp(time.time_ns() // constants.NANOSECS_IN_MILLISECS).bytes
//     )

#[pymethods]
impl PyUlid {
    #[new]
    #[pyo3(signature = (value = None))]
    pub fn py_new(value: Option<&[u8]>) -> PyResult<Self> {
        if let Some(value) = value {
            let b: [u8; 16] = value
                .try_into()
                .map_err(|_| PyValueError::new_err("ULID must be exactly 16 bytes long"))?;

            let u = ulid::Ulid::from_bytes(b);
            Ok(PyUlid(u))
        } else {
            let ulid = ulid::Ulid::new();
            Ok(PyUlid(ulid))
        }
    }

    pub fn __str__(&self) -> String {
        self.0.to_string()
    }

    pub fn __repr__(&self) -> String {
        format!("ULID('{}')", self.0.to_string())
    }

    //     def __int__(self) -> int:
    //     """Encode this object as an integer."""
    //     return int.from_bytes(self.bytes, byteorder="big")

    // def __bytes__(self) -> bytes:
    //     """Encode this object as byte sequence."""
    //     return self.bytes

    fn __int__(&self) -> PyResult<u128> {
        let b = self.0.to_bytes();
        Ok(u128::from_be_bytes(b))
    }

    fn __hash__(&self) -> PyResult<u64> {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        Ok(hasher.finish())
    }
    // fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
    //     if let Ok(rs_uuid) = other.downcast::<PyUuid>() {
    //         let other = rs_uuid.get();

    //         match op {
    //             pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
    //             pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
    //             pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
    //             pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
    //             pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
    //             pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
    //         }
    //     } else {
    //         let other = other.extract::<CPythonUuid>()?;
    //         match op {
    //             pyo3::basic::CompareOp::Eq => Ok(self.0 == other.0),
    //             pyo3::basic::CompareOp::Ne => Ok(self.0 != other.0),
    //             pyo3::basic::CompareOp::Lt => Ok(self.0 < other.0),
    //             pyo3::basic::CompareOp::Le => Ok(self.0 <= other.0),
    //             pyo3::basic::CompareOp::Gt => Ok(self.0 > other.0),
    //             pyo3::basic::CompareOp::Ge => Ok(self.0 >= other.0),
    //         }
    //     }
    // }
    // fn __richcmp__(&self, other: &Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
    fn __richcmp__(&self, other: &Bound<'_, PyAny>, op: pyo3::basic::CompareOp) -> PyResult<bool> {
        if let Ok(pyint) = other.downcast::<pyo3::types::PyInt>() {
            let other: u128 = pyint.extract()?;
            return match op {
                pyo3::basic::CompareOp::Eq => Ok(self.to_u128() == other),
                pyo3::basic::CompareOp::Ne => Ok(self.to_u128() != other),
                pyo3::basic::CompareOp::Lt => Ok(self.to_u128() < other),
                pyo3::basic::CompareOp::Le => Ok(self.to_u128() <= other),
                pyo3::basic::CompareOp::Gt => Ok(self.to_u128() > other),
                pyo3::basic::CompareOp::Ge => Ok(self.to_u128() >= other),
            };
        } else if other.is_instance_of::<pyo3::types::PyString>() {
            let s = other.downcast::<pyo3::types::PyString>()?;
            // visitor.visit_str(&s.to_cow()?)
            let cs = s.to_str()?;
            let this_str = self.0.to_string();
            return match op {
                pyo3::basic::CompareOp::Eq => Ok(this_str.as_str() == cs),
                pyo3::basic::CompareOp::Ne => Ok(this_str.as_str() != cs),
                pyo3::basic::CompareOp::Lt => Ok(this_str.as_str() < cs),
                pyo3::basic::CompareOp::Le => Ok(this_str.as_str() <= cs),
                pyo3::basic::CompareOp::Gt => Ok(this_str.as_str() > cs),
                pyo3::basic::CompareOp::Ge => Ok(this_str.as_str() >= cs),
            };
        } else if let Ok(rs_ulid) = other.downcast::<PyUlid>() {
            let other = rs_ulid.borrow().0;
            match op {
                pyo3::basic::CompareOp::Eq => Ok(self.0 == other),
                pyo3::basic::CompareOp::Ne => Ok(self.0 != other),
                pyo3::basic::CompareOp::Lt => Ok(self.0 < other),
                pyo3::basic::CompareOp::Le => Ok(self.0 <= other),
                pyo3::basic::CompareOp::Gt => Ok(self.0 > other),
                pyo3::basic::CompareOp::Ge => Ok(self.0 >= other),
            }
        } else if let Ok(pybytes) = other.downcast::<PyBytes>() {
            let slice = pybytes.as_bytes();
            match slice.len() {
                16 => {
                    let ulid = ulid::Ulid::from_bytes(slice.try_into().unwrap());
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
        pyo3::types::PyBytes::new(py, &self.0.to_bytes())
    }

    #[staticmethod]
    fn from_bytes(bytes: [u8; 16]) -> PyResult<Self> {
        let ulid = ulid::Ulid::from_bytes(bytes);
        Ok(PyUlid(ulid))
    }

    fn to_bytes<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        self.__bytes__(py)
    }


    #[staticmethod]
    fn from_hex(hexstr: &str) -> PyResult<Self> {
        let b = Self::hex2bytes(hexstr)?;
        let ul = ulid::Ulid::from_bytes(b);
        Ok(PyUlid(ul))
    }

    #[staticmethod]
    fn from_int(bytes: u128) -> PyResult<Self> {
        let b = bytes.to_be_bytes();
        let ul = ulid::Ulid::from_bytes(b);
        Ok(PyUlid(ul))
    }

    #[staticmethod]
    fn from_string(bytes: &str) -> PyResult<Self> {
        ulid::Ulid::from_string(bytes)
            .map(Self::from)
            .map_err(|e| PyValueError::new_err(format!("Invalid ULID string: {}", e)))
    }

    #[staticmethod]
    fn from_str(bytes: &str) -> PyResult<Self> {
        PyUlid::from_string(bytes)
    }

    // @classmethod
    // @validate_type(int, float)
    // def from_timestamp(cls: type[U], value: float) -> U:
    //     """Create a new :class:`ULID`-object from a timestamp. The timestamp can be either a
    //     `float` representing the time in seconds (as it would be returned by :func:`time.time()`)
    //     or an `int` in milliseconds.

    //     Examples:

    //         >>> import time
    //         >>> ULID.from_timestamp(time.time())
    //         ULID(01E75QWN5HKQ0JAVX9FG1K4YP4)
    //     """
    //     if isinstance(value, float):
    //         value = int(value * constants.MILLISECS_IN_SECS)
    //     timestamp = int.to_bytes(value, constants.TIMESTAMP_LEN, "big")
    //     randomness = os.urandom(constants.RANDOMNESS_LEN)
    //     return cls.from_bytes(timestamp + randomness)

    #[staticmethod]
    fn from_timestamp(value: f64) -> PyResult<Self> {
        let millis = (value * 1000.0) as u64;
        let ulid = ulid::Ulid::from_datetime(SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(millis));
        Ok(PyUlid(ulid))
    }


    #[staticmethod]
    fn from_datetime(bytes: SystemTime) -> PyResult<Self> {
        Ok(Self::from(ulid::Ulid::from_datetime(bytes)))
    }

    #[staticmethod]
    fn from_uuid(uu: UuidLike) -> PyResult<Self> {
        let uu = uu.0;
        let ul = Ulid::from_bytes(*uu.as_bytes());
        Ok(PyUlid(ul))
    }

    fn to_uuid(&self) -> PyResult<ryo3_uuid::PyUuid> {
        let b = self.0.to_bytes();
        let a = ryo3_uuid::PyUuid::from(uuid::Uuid::from_bytes(b));
        Ok(a)
    }

    fn to_uuid4(&self) -> PyUuid {
        // ulid::Ulid does not have a direct conversion to UUID4, but we can create a UUID4 from the ULID bytes.
        // let bytes = self.0.to_bytes();
        // let uuid = uuid::Uuid::from_bytes(bytes);
        // Ok(uuid.to_string())
        let mut b = uuid::Builder::from_u128(self.to_u128());
        b.set_version(uuid::Version::Random);
        let u = b.into_uuid();
        PyUuid::from(u)
    }

    #[staticmethod]
    fn parse(other: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(pyint) = other.downcast::<pyo3::types::PyInt>() {
            let i = pyint.extract::<u128>()?;
            if let Ok(smaller_int) = u64::try_from(i) {
                // If the integer is small enough, we can use it directly
                // as a ULID timestamp.
                let ulid = ulid::Ulid::from_datetime(SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(smaller_int));
                return Ok(PyUlid(ulid));

            } else {
                // If the integer is too large, we treat it as a ULID.
                return Self::from_int(i);
            }
        } else if other.is_instance_of::<pyo3::types::PyString>() {
            let s = other.downcast::<pyo3::types::PyString>()?;
            let cs = s.to_str()?;
            // uuid string length
            match cs.len() {
                36 => {
                let uu = Uuid::parse_str(cs).map_err(|e| PyValueError::new_err(format!("Invalid UUID string: {}", e)))?;

                let ul = Ulid::from_bytes(*uu.as_bytes());
                return Ok(PyUlid(ul));
                }
                26 => {
                    // ulid string length
                    let ulid = ulid::Ulid::from_string(cs)
                        .map_err(|e| PyValueError::new_err(format!("Invalid ULID string: {}", e)))?;
                    return Ok(PyUlid(ulid));
                }
                32 => {
                    // hex string length
                    return Self::from_hex(cs);
                }
                //             raise ValueError(f"Cannot parse ULID from string of length {len_value}")
                _ => {
                    return Err(PyValueError::new_err(format!(
                        "Cannot parse ULID from string of length {}",
                        cs.len()
                    )));
                }

            }
        }
        // has to go through `isinstance` apparatus
        else if other.is_instance_of::<pyo3::types::PyFloat>() {
            let f = other.extract::<f64>()?;
            return Self::from_timestamp(f);
        }else if let Ok(rs_ulid) = other.downcast::<PyUlid>() {
            let inner = rs_ulid.borrow().0.clone();
            return Ok( PyUlid(inner));
        } else if other.is_instance_of::<PyBytes>() {
            let pybytes = other.downcast::<PyBytes>()?;
            let b = pybytes.extract::<[u8; 16]>()?;
            return Self::from_bytes(b);
        } else if let Ok(py_uuid) = other.downcast::<PyUuid>() {
            return Self::from_uuid(UuidLike(py_uuid.borrow().0));
        } else if let Ok(c_uuid) = other.extract::<CPythonUuid>() {
            return Self::from_uuid(UuidLike(c_uuid.into()));
        } else if let Ok(dt) = other.extract::<SystemTime>() {
            return Self::from_datetime(dt);
        }
        // raise TypeError(f"Cannot parse ULID from type {type(value)}")

        let other_type = other.get_type();
        let other_type_name = other_type.name().map(|e| e.as_borrowed().to_string()).unwrap_or(
            String::from("Unknown")
        );

        Err(PyTypeError::new_err(
            format!("Cannot parse ULID from type {}", other_type_name),
        ))

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
    fn datetime(&self) -> SystemTime{
        self.0.datetime()
    }

    #[getter]
    fn hex(&self) -> String {
        self.0.to_bytes().into_iter().map(|b| format!("{:02x}", b)).collect()
    }

    #[getter]
    fn milliseconds(&self) -> u64 {
        self.0.timestamp_ms()
    }

    #[getter]
    fn timestamp(&self) -> f64 {
        self.0.timestamp_ms() as f64 / 1000.0
    }
    //     if bytes.len() != 16 {
    //         return Err(PyValueError::new_err("Bytes must be exactly 16 bytes long"));
    //     }
    //     let ulid = ulid::Ulid::from_bytes(bytes.as_bytes());
    //     Ok(PyUlid(ulid))
    // }

    // pub fn to_bytes(&self) -> PyBytes {
    //     PyBytes::new(self.0.to_bytes())
    // }

    // pub fn __str__(&self) -> String {
    //     self.0.to_string()
    // }
}

impl From<ulid::Ulid> for PyUlid {
    fn from(ulid: ulid::Ulid) -> Self {
        PyUlid(ulid)
    }
}

pub struct UuidLike(pub(crate) uuid::Uuid);

impl FromPyObject<'_> for UuidLike {
    fn extract_bound(obj: &Bound<'_, PyAny>) -> PyResult<Self> {
        if let Ok(uuid_like) = obj.downcast::<PyUuid>() {
            return Ok(UuidLike(uuid_like.borrow().0));
        } else if let Ok(py_uuid) = obj.extract::<CPythonUuid>() {
            return Ok(UuidLike(py_uuid.into()));
        } else {
            return Err(PyTypeError::new_err("Expected a `uuid.UUID` instance."));
        }
        // let uuid_cls = get_uuid_cls(py)?;
        // if obj.is_instance(uuid_cls)? {
        //     let uuid_int: u128 = obj.getattr(intern!(py, "int"))?.extract()?;
        //     let bytes = uuid_int.to_be_bytes();
        //     Ok(UuidLike(uuid::Uuid::from_bytes(bytes)))
        // } else {
        // }
    }
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyUlid>()?;
    Ok(())
}
