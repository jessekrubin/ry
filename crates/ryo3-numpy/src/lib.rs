#![doc = include_str!("../README.md")]
mod dtype;
mod obtype;
mod py_array_interface;
pub use dtype::NumpyDType;
pub use obtype::{NumpyObType, NumpyTypeCache};
pub use py_array_interface::PyArrayInterface;
