#![cfg(test)]

use crate::{RyDate, RyDateTime, RySignedDuration, RySpan, RyTime, RyTimestamp, RyZoned};
use jiff::Span;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Stuff {
    date: RyDate,
    datetime: RyDateTime,
    signed_duration: RySignedDuration,
    span: RySpan,
    time: RyTime,
    timestamp: RyTimestamp,
    zoned: RyZoned,
}

// the test
#[test]
fn test_deserialize_and_serialize() {
    let ry_time = RyTime::py_new(Some(4), Some(3), Some(2), Option::from(1)).unwrap();
    let ry_date = RyDate::py_new(2025, 5, 21).unwrap();
    let ry_datetime = ry_date.at(4, 3, 2, 1);
    let ry_zoned = ry_datetime.in_tz("America/New_York").unwrap();
    let ry_signed_duration = RySignedDuration::py_new(123, 123).unwrap();
    let ry_timestamp = RyTimestamp::py_new(Some(1234), Some(5678)).unwrap();
    let span = Span::new().days(1).hours(2).minutes(4);
    let ry_span = RySpan::from(span);

    let s = Stuff {
        date: ry_date,
        datetime: ry_datetime,
        signed_duration: ry_signed_duration,
        span: ry_span,
        time: ry_time,
        timestamp: ry_timestamp,
        zoned: ry_zoned,
    };

    let serialized = serde_json::to_string_pretty(&s).unwrap();

    println!("Serialized: {serialized}");
    let deserialized: Stuff = serde_json::from_str(&serialized).unwrap();
    assert_eq!(s, deserialized);
}
