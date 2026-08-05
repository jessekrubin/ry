use std::ffi::{c_char, c_int, c_void};

/// Numpy Array Interface
///
/// [doc](https://numpy.org/doc/stable/reference/arrays.interface.html#object.__array_struct__)
/// [src](https://github.com/numpy/numpy/blob/37ee017a2227a3757ac27793e95c67553fabf4c8/numpy/_core/include/numpy/ndarraytypes.h#L1383)
///
/// ```c
/// typedef struct {
///     int two;              /*
///                            * contains the integer 2 as a sanity
///                            * check
///                            */
///
///     int nd;               /* number of dimensions */
///
///     char typekind;        /*
///                            * kind in array --- character code of
///                            * typestr
///                            */
///
///     int itemsize;         /* size of each element */
///
///     int flags;            /*
///                            * how should be data interpreted. Valid
///                            * flags are CONTIGUOUS (1), F_CONTIGUOUS (2),
///                            * ALIGNED (0x100), NOTSWAPPED (0x200), and
///                            * WRITEABLE (0x400).  ARR_HAS_DESCR (0x800)
///                            * states that arrdescr field is present in
///                            * structure
///                            */
///
///     npy_intp *shape;       /*
///                             * A length-nd array of shape
///                             * information
///                             */
///
///     npy_intp *strides;    /* A length-nd array of stride information */
///
///     void *data;           /* A pointer to the first element of the array */
///
///     PyObject *descr;      /*
///                            * A list of fields or NULL (ignored if flags
///                            * does not have ARR_HAS_DESCR flag set)
///                            */
/// } PyArrayInterface;
/// ```
#[repr(C)]
pub struct PyArrayInterface {
    /// contains the integer 2 as a sanity check
    pub two: c_int,
    /// n dims
    pub nd: c_int,
    /// kind in array; car code of typestr
    pub typekind: c_char,
    /// size of each element
    pub itemsize: c_int,
    /// how should be data interpreted
    ///
    /// flags:
    /// - `CONTIGUOUS` (1)
    /// - `F_CONTIGUOUS` (2)
    /// - `ALIGNED` (0x100)
    /// - `NOTSWAPPED` (0x200)
    /// - `WRITEABLE` (0x400)
    /// - `ARR_HAS_DESCR` (0x800) states that arrdescr field is present
    pub flags: c_int,
    /// A length-nd array of shape information
    pub shape: *const isize,
    /// A length-nd array of stride information
    pub strides: *const isize,
    /// A pointer to the first element of the array
    pub data: *const c_void,
    /// A list of fields or NULL (ignored if flags does not have `ARR_HAS_DESCR` flag set)
    pub descr: *mut pyo3::ffi::PyObject,
}
