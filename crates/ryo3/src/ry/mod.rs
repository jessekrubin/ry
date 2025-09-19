//! ry module registration
//!
//! `ry` does all registration of pyo3 types/fns/classes/mods here

use crate::libs;
use pyo3::prelude::*;
use pyo3::{Bound, PyResult};

#[cfg(feature = "dev")]
pub mod dev;
pub mod sh;
pub mod submodules;

fn py_constants(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // u8
    m.add("U8_BITS", u8::BITS)?;
    m.add("U8_MAX", u8::MAX)?;
    m.add("U8_MIN", u8::MIN)?;
    // i8
    m.add("I8_BITS", i8::BITS)?;
    m.add("I8_MAX", i8::MAX)?;
    m.add("I8_MIN", i8::MIN)?;
    // u16
    m.add("U16_BITS", u16::BITS)?;
    m.add("U16_MAX", u16::MAX)?;
    m.add("U16_MIN", u16::MIN)?;
    // i16
    m.add("I16_BITS", i16::BITS)?;
    m.add("I16_MAX", i16::MAX)?;
    m.add("I16_MIN", i16::MIN)?;
    // u32
    m.add("U32_BITS", u32::BITS)?;
    m.add("U32_MAX", u32::MAX)?;
    m.add("U32_MIN", u32::MIN)?;
    // i32
    m.add("I32_BITS", i32::BITS)?;
    m.add("I32_MAX", i32::MAX)?;
    m.add("I32_MIN", i32::MIN)?;
    // u64
    m.add("U64_BITS", u64::BITS)?;
    m.add("U64_MAX", u64::MAX)?;
    m.add("U64_MIN", u64::MIN)?;
    // i64
    m.add("I64_BITS", i64::BITS)?;
    m.add("I64_MAX", i64::MAX)?;
    m.add("I64_MIN", i64::MIN)?;
    // u128
    m.add("U128_BITS", u128::BITS)?;
    m.add("U128_MAX", u128::MAX)?;
    m.add("U128_MIN", u128::MIN)?;
    // i128
    m.add("I128_MAX", i128::MAX)?;
    m.add("I128_MIN", i128::MIN)?;
    m.add("I128_BITS", i128::BITS)?;
    // usize
    m.add("USIZE_BITS", usize::BITS)?;
    m.add("USIZE_MAX", usize::MAX)?;
    m.add("USIZE_MIN", usize::MIN)?;
    // isize
    m.add("ISIZE_BITS", isize::BITS)?;
    m.add("ISIZE_MAX", isize::MAX)?;
    m.add("ISIZE_MIN", isize::MIN)?;

    // f32

    // macro_rules! f32_constant {
    //     ($name:ident) => {
    //         m.add(concat!("F32_", stringify!($name)), f32::$name)?;
    //     };
    // }
    // macro_rules! f32_const {
    //     ($name:ident) => {
    //         m.add(concat!("F32_", stringify!($name)), std::f32::consts::$name)?;
    //     };
    // }
    // f32_constant!(EPSILON);
    // f32_constant!(INFINITY);
    // f32_constant!(MANTISSA_DIGITS);
    // f32_constant!(MAX);
    // f32_constant!(MAX_10_EXP);
    // f32_constant!(MAX_EXP);
    // f32_constant!(MIN);
    // f32_constant!(MIN_10_EXP);
    // f32_constant!(MIN_EXP);
    // f32_constant!(MIN_POSITIVE);
    // f32_constant!(NAN);
    // f32_constant!(NEG_INFINITY);
    // f32_constant!(RADIX);

    // f32_const!(E);
    // f32_const!(FRAC_1_PI);
    // f32_const!(FRAC_1_SQRT_2);
    // f32_const!(FRAC_2_PI);
    // f32_const!(FRAC_2_SQRT_PI);
    // f32_const!(FRAC_PI_2);
    // f32_const!(FRAC_PI_3);
    // f32_const!(FRAC_PI_4);
    // f32_const!(FRAC_PI_6);
    // f32_const!(FRAC_PI_8);
    // f32_const!(LN_2);
    // f32_const!(LN_10);
    // f32_const!(LOG2_10);
    // f32_const!(LOG2_E);
    // f32_const!(LOG10_2);
    // f32_const!(LOG10_E);
    // f32_const!(PI);
    // f32_const!(SQRT_2);
    // f32_const!(TAU);
    // // f64
    // macro_rules! f64_constant {
    //     ($name:ident) => {
    //         m.add(concat!("F64_", stringify!($name)), f32::$name)?;
    //     };
    // }
    // macro_rules! f64_const {
    //     ($name:ident) => {
    //         m.add(concat!("F64_", stringify!($name)), std::f64::consts::$name)?;
    //     };
    // }

    // f64_constant!(EPSILON);
    // f64_constant!(INFINITY);
    // f64_constant!(MANTISSA_DIGITS);
    // f64_constant!(MAX);
    // f64_constant!(MAX_10_EXP);
    // f64_constant!(MAX_EXP);
    // f64_constant!(MIN);
    // f64_constant!(MIN_10_EXP);
    // f64_constant!(MIN_EXP);
    // f64_constant!(MIN_POSITIVE);
    // f64_constant!(NAN);
    // f64_constant!(NEG_INFINITY);
    // f64_constant!(RADIX);

    // f64_const!(E);
    // f64_const!(FRAC_1_PI);
    // f64_const!(FRAC_1_SQRT_2);
    // f64_const!(FRAC_2_PI);
    // f64_const!(FRAC_2_SQRT_PI);
    // f64_const!(FRAC_PI_2);
    // f64_const!(FRAC_PI_3);
    // f64_const!(FRAC_PI_4);
    // f64_const!(FRAC_PI_6);
    // f64_const!(FRAC_PI_8);
    // f64_const!(LN_2);
    // f64_const!(LN_10);
    // f64_const!(LOG2_10);
    // f64_const!(LOG2_E);
    // f64_const!(LOG10_2);
    // f64_const!(LOG10_E);
    // f64_const!(PI);
    // f64_const!(SQRT_2);
    // f64_const!(TAU);
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    py_constants(m)?;

    ryo3_std::pymod_add(m)?;
    ryo3_fspath::pymod_add(m)?;
    ryo3_quick_maths::pymod_add(m)?;
    ryo3_json::pymod_add(m)?;
    sh::pymod_add(m)?;
    libs::pymod_add(m)?;
    // register submodules
    submodules::pymod_add(m)?;
    // dev submodule
    #[cfg(feature = "dev")]
    dev::pymod_add(m)?;
    Ok(())
}
