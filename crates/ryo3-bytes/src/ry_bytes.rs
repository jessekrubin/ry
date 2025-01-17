use crate::bytes::PyBytes;
use pyo3::prelude::*;

/// A wrapper around a [`bytes::Bytes`][].
///
/// This implements both import and export via the Python buffer protocol.
///
/// ### Buffer protocol import
///
/// This can be very useful as a general way to support ingest of a Python buffer protocol object.
///
/// The underlying [Bytes] manages the external memory, automatically calling the Python
/// buffer's release callback when the internal reference count reaches 0.
///
/// Note that converting this [`Bytes`] into a [BytesMut][::bytes::BytesMut] will always create a
/// deep copy of the buffer into newly allocated memory, since this `Bytes` is constructed from an
/// owner.
///
/// ### Buffer protocol export
///
/// PyBytes implements the Python buffer protocol to enable Python to access the underlying `Bytes`
/// data view without copies. In Python, this `PyBytes` object can be passed to Python `bytes` or
/// `memoryview` constructors, `numpy.frombuffer`, or any other function that supports buffer
/// protocol input.
#[pyclass(extends=PyBytes, name = "Bytes", subclass, frozen, sequence, weakref, module="ry.ryo3")]
pub struct RyBytes {}

#[pymethods]
impl RyBytes {
    /// Create a new `RyBytes` object from a Python buffer protocol object.
    #[new]
    fn py_new(buf: PyBytes) -> (Self, PyBytes) {
        (RyBytes {}, buf)
    }
}
