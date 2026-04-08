use std::fmt;

use serde::ser::Error as SerError;

#[inline]
pub(crate) fn pyerr2sererr<I: fmt::Display, O: SerError>(err: I) -> O {
    O::custom(err.to_string())
}
