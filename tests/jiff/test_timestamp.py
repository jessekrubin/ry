from __future__ import annotations

from hypothesis import given
from hypothesis.strategies import sampled_from

import ry

from .strategies import st_timestamps


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
    # alternatively testing take_until
    series = start.series(ry.TimeSpan()._hours(5))
    values = series.take_until(end)
    assert values == scan_times


@given(
    ts=st_timestamps(
        # wiggle room for offset testing bc could mayhaps overflow
        min_value=(ry.Timestamp.MIN + ry.TimeSpan(hours=48)),
        max_value=(ry.Timestamp.MAX - ry.TimeSpan(hours=48)),
    ),
    # sample from -18 hours to +18 hours
    off=sampled_from([
        ry.Offset.from_seconds(s) for s in range(-18 * 3600, 18 * 3600 + 1, 3600)
    ]),
)
def test_timestamp_display_with_offset(ts: ry.Timestamp, off: ry.Offset) -> None:
    s = ts.display_with_offset(off)
    assert isinstance(s, str)
    assert s.isascii()
    try:
        parsed = ry.Timestamp.parse(s)
        diff = ts - parsed
        assert (
            ts == parsed
            or diff.total_seconds() < ry.TimeSpan(seconds=1).total_seconds()
        ), (
            f"Expected parsed timestamp {parsed} to be equal to original {ts} "
            f"or differ by less than 1 seconds, got difference of {diff}"
        )
    except ValueError as ve:
        msg = (
            f"Failed to parse timestamp string '{s}' generated from "
            f"timestamp {ts} with offset {off}"
        )
        raise AssertionError(msg) from ve
