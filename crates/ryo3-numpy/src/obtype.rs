use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;

use crate::NumpyDType;

#[doc(hidden)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumpyObType {
    Array,
    Scalar(NumpyDType),
}
static NUMPY_TYPES: PyOnceLock<NumpyTypeCache> = PyOnceLock::new();

#[derive(Copy, Clone, Debug)]
pub struct NumpyTypeCache {
    // ndarray
    array: usize,
    // scalars
    bool_: usize,
    int8: usize,
    int16: usize,
    int32: usize,
    int64: usize,
    uint8: usize,
    uint16: usize,
    uint32: usize,
    uint64: usize,
    float16: usize,
    float32: usize,
    float64: usize,
}

impl NumpyTypeCache {
    pub fn new(py: Python<'_>) -> PyResult<Self> {
        let numpy = py.import("numpy")?;
        // BOOM tiny func to get the pointer of a numpy type object
        let npt_ptr =
            |name: &str| -> PyResult<usize> { Ok(numpy.getattr(name)?.as_ptr() as usize) };
        Ok(Self {
            array: npt_ptr("ndarray")?,
            bool_: npt_ptr("bool_")?,
            int8: npt_ptr("int8")?,
            int16: npt_ptr("int16")?,
            int32: npt_ptr("int32")?,
            int64: npt_ptr("int64")?,
            uint8: npt_ptr("uint8")?,
            uint16: npt_ptr("uint16")?,
            uint32: npt_ptr("uint32")?,
            uint64: npt_ptr("uint64")?,
            float16: npt_ptr("float16")?,
            float32: npt_ptr("float32")?,
            float64: npt_ptr("float64")?,
        })
    }

    pub fn cached(py: Python<'_>) -> PyResult<&'static Self> {
        NUMPY_TYPES.get_or_try_init(py, || Self::new(py))
    }

    #[inline]
    #[must_use]
    pub fn obtype(&self, ptr: usize) -> Option<NumpyObType> {
        if ptr == self.array {
            return Some(NumpyObType::Array);
        }

        macro_rules! ptr2dtype {
            ($field:ident, $dtype:ident) => {
                if ptr == self.$field {
                    return Some(NumpyObType::Scalar(NumpyDType::$dtype));
                }
            };
        }
        ptr2dtype!(bool_, Bool);
        ptr2dtype!(int8, I8);
        ptr2dtype!(int16, I16);
        ptr2dtype!(int32, I32);
        ptr2dtype!(int64, I64);
        ptr2dtype!(uint8, U8);
        ptr2dtype!(uint16, U16);
        ptr2dtype!(uint32, U32);
        ptr2dtype!(uint64, U64);
        ptr2dtype!(float16, F16);
        ptr2dtype!(float32, F32);
        ptr2dtype!(float64, F64);
        None
    }
}
