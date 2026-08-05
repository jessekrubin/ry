use std::ffi::{c_char, c_int};

use crate::PyArrayInterface;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumpyDType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F16,
    F32,
    F64,
    // more
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumpyTypeKind {
    Bool = b'b' as isize,
    Int = b'i' as isize,
    UInt = b'u' as isize,
    Float = b'f' as isize,
    // NOT SERDE-ABLE
    Bitfield = b't' as isize,
    Complex = b'c' as isize,
    Object = b'O' as isize,
    ByteString = b'S' as isize,
    Unicode = b'U' as isize,
    Void = b'V' as isize,
}

impl NumpyDType {
    #[must_use]
    pub fn from_typekind_and_itemsize(typekind: c_char, itemsize: c_int) -> Option<Self> {
        let kind = u8::try_from(typekind).ok()?;
        match (kind, itemsize) {
            (b'b', 1) => Some(Self::Bool),
            (b'i', 1) => Some(Self::I8),
            (b'i', 2) => Some(Self::I16),
            (b'i', 4) => Some(Self::I32),
            (b'i', 8) => Some(Self::I64),
            (b'u', 1) => Some(Self::U8),
            (b'u', 2) => Some(Self::U16),
            (b'u', 4) => Some(Self::U32),
            (b'u', 8) => Some(Self::U64),
            (b'f', 2) => Some(Self::F16),
            (b'f', 4) => Some(Self::F32),
            (b'f', 8) => Some(Self::F64),
            _ => None,
        }
    }

    #[must_use]
    pub fn from_array_interface(pai: &PyArrayInterface) -> Option<Self> {
        Self::from_typekind_and_itemsize(pai.typekind, pai.itemsize)
    }

    #[must_use]
    pub fn typekind(&self) -> NumpyTypeKind {
        match self {
            Self::Bool => NumpyTypeKind::Bool,
            Self::I8 | Self::I16 | Self::I32 | Self::I64 => NumpyTypeKind::Int,
            Self::U8 | Self::U16 | Self::U32 | Self::U64 => NumpyTypeKind::UInt,
            Self::F16 | Self::F32 | Self::F64 => NumpyTypeKind::Float,
        }
    }

    #[must_use]
    pub const fn itemsize(self) -> usize {
        match self {
            Self::Bool | Self::I8 | Self::U8 => 1,
            Self::I16 | Self::U16 | Self::F16 => 2,
            Self::I32 | Self::U32 | Self::F32 => 4,
            Self::I64 | Self::U64 | Self::F64 => 8,
        }
    }
}
