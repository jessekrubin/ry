from __future__ import annotations

import ry


def test_timestamp_series_jiff_example() -> None:
    """
    use jiff::{civil::{Time, time}, ToSpan};

    let start = Time::MIN;
    let mut every_third_hour = vec![];
    for t in start.series(3.hours()) {
        every_third_hour.push(t);
    }
    assert_eq!(every_third_hour, vec![
        time(0, 0, 0, 0),
        time(3, 0, 0, 0),
        time(6, 0, 0, 0),
        time(9, 0, 0, 0),
        time(12, 0, 0, 0),
        time(15, 0, 0, 0),
        time(18, 0, 0, 0),
        time(21, 0, 0, 0),
    ]);
    """
    start = ry.Timestamp.parse("2023-07-15 16:30:00-04")
    end = start + ry.TimeSpan()._hours(48)
    scan_times = []
    for ts in start.series(ry.TimeSpan()._hours(5)):
        if ts > end:
            break
        scan_times.append(ts)
    assert scan_times == [
        ry.Timestamp.parse("2023-07-15 16:30:00-04:00"),
        ry.Timestamp.parse("2023-07-15 21:30:00-04:00"),
        ry.Timestamp.parse("2023-07-16 02:30:00-04:00"),
        ry.Timestamp.parse("2023-07-16 07:30:00-04:00"),
        ry.Timestamp.parse("2023-07-16 12:30:00-04:00"),
        ry.Timestamp.parse("2023-07-16 17:30:00-04:00"),
        ry.Timestamp.parse("2023-07-16 22:30:00-04:00"),
        ry.Timestamp.parse("2023-07-17 03:30:00-04:00"),
        ry.Timestamp.parse("2023-07-17 08:30:00-04:00"),
        ry.Timestamp.parse("2023-07-17 13:30:00-04:00"),
    ]
