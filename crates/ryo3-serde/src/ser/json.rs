use pyo3::prelude::*;
use pyo3::types::PyString;
use ryo3_core::pystr_read_fast_opt;
use serde::ser::Error as SerError;

use crate::any_repr::any_repr;
use crate::errors::pyerr2sererr;
use crate::ob_type::PyObType;
use crate::ser::{PySerializeContext, SerializeTarget};

pub(crate) fn json_map_key_str<'a, E: SerError, T: SerializeTarget>(
    ctx: PySerializeContext<'_, T>,
    key: Borrowed<'a, '_, PyAny>,
) -> Result<&'a str, E> {
    match ctx.typeref.obtype_key(key) {
        PyObType::String => {
            #[expect(unsafe_code)]
            let key = unsafe { key.cast_unchecked::<PyString>() };
            #[expect(unsafe_code)]
            let key = unsafe { pystr_read_fast_opt(key) };
            key.ok_or_else(|| E::custom("invalid str object"))
        }
        PyObType::Bool => {
            if key.extract::<bool>().map_err(pyerr2sererr)? {
                Ok("true")
            } else {
                Ok("false")
            }
        }
        _ => {
            let key_repr = any_repr(key);
            Err(E::custom(format!(
                "{key_repr} is not serializable as map-key"
            )))
        }
    }
}
