use crate::SerializePyAny;
use crate::type_cache::PyTypeCache;

pub trait ObjTypeRef<'py> {
    fn type_ref(&self) -> &'py PyTypeCache;
}

pub trait SerializeWithObj<'a, 'py> {
    fn with_obj(&self, obj: &'a pyo3::Bound<'py, pyo3::PyAny>) -> SerializePyAny;
}
//
// impl<'a, 'py> SerializeWithObj<'a, 'py> for SerializePyAny<'py> {
//     fn with_obj(&self, obj: &'a pyo3::Bound<'py, pyo3::PyAny>) -> SerializePyAny {
//         SerializePyAny::new_with_depth(obj, self.default, self.depth + 1)
//     }
// }
