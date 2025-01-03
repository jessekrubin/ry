use crate::{JiffUnit, RyDateTimeRound};
use jiff::civil::DateTimeRound;
use jiff::{SpanRound, ZonedRound};
use pyo3::FromPyObject;

#[derive(Debug, Clone, FromPyObject)]
pub enum IntoDateTimeRound {
    RyDateTimeRound(RyDateTimeRound),
    JiffUnit(JiffUnit),
}

impl From<IntoDateTimeRound> for DateTimeRound {
    fn from(value: IntoDateTimeRound) -> Self {
        match value {
            IntoDateTimeRound::RyDateTimeRound(round) => DateTimeRound::new()
                .smallest(round.smallest.0)
                .mode(round.mode.0)
                .increment(round.increment),
            IntoDateTimeRound::JiffUnit(unit) => unit.0.into(),
        }
    }
}
impl From<IntoDateTimeRound> for ZonedRound {
    // fn from(val : ) -> Self {
    //     match val {
    //         TODO: this is ugly
    // crate::ry_zoned::IntoZonedRound::RyDateTimeRound(round) => ZonedRound::new()
    //     .smallest(round.smallest.0)
    //     .mode(round.mode.0)
    //     .increment(round.increment),
    // crate::ry_zoned::IntoZonedRound::JiffUnit(unit) => unit.0.into(),
    // }
    // }
    fn from(value: IntoDateTimeRound) -> Self {
        match value {
            IntoDateTimeRound::RyDateTimeRound(round) => ZonedRound::new()
                .smallest(round.smallest.0)
                .mode(round.mode.0)
                .increment(round.increment),
            IntoDateTimeRound::JiffUnit(unit) => unit.0.into(),
        }
    }
}

impl From<IntoDateTimeRound> for SpanRound<'_> {
    fn from(value: IntoDateTimeRound) -> Self {
        match value {
            IntoDateTimeRound::RyDateTimeRound(round) => SpanRound::new()
                .smallest(round.smallest.0)
                .mode(round.mode.0)
                .increment(round.increment),
            IntoDateTimeRound::JiffUnit(unit) => unit.0.into(),
        }
    }
}
