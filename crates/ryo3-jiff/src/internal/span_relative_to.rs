use crate::ry_date::RyDate;
use crate::ry_datetime::RyDateTime;
use crate::ry_zoned::RyZoned;
use pyo3::FromPyObject;

#[derive(Debug, Clone, FromPyObject)]
pub(crate) enum RySpanRelativeTo {
    Zoned(RyZoned),
    Date(RyDate),
    DateTime(RyDateTime),
}

impl From<RyZoned> for RySpanRelativeTo {
    fn from(z: RyZoned) -> Self {
        RySpanRelativeTo::Zoned(z)
    }
}

impl From<RyDate> for RySpanRelativeTo {
    fn from(d: RyDate) -> Self {
        RySpanRelativeTo::Date(d)
    }
}

impl From<RyDateTime> for RySpanRelativeTo {
    fn from(dt: RyDateTime) -> Self {
        RySpanRelativeTo::DateTime(dt)
    }
}
