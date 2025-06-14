use serde::ser::Error as SerError;
use std::fmt;

#[inline]
pub(crate) fn pyerr2sererr<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}
