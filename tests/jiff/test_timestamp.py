from __future__ import annotations

import datetime as pydt

from hypothesis import given
from hypothesis.strategies import sampled_from

import ry

from .strategies import st_timestamps


class TestTimestamp:
    ts = ry.Timestamp.parse("2026-01-30T18:59:33.123879Z")

    def test_to_pydatetime(self) -> None:
        py_dt = self.ts.to_pydatetime()
        to_py = self.ts.to_py()
        assert isinstance(py_dt, pydt.datetime)
        assert py_dt == to_py
        assert py_dt == pydt.datetime(2026, 1, 30, 18, 59, 33, 123879, tzinfo=pydt.UTC)

    def test_from_pydatetime(self) -> None:
        py_dt = pydt.datetime(2026, 1, 30, 18, 59, 33, 123879, tzinfo=pydt.UTC)
        ts = ry.Timestamp.from_pydatetime(py_dt)
        assert ts == self.ts

    def test_to_pydate(self) -> None:
        py_d = self.ts.to_pydate()
        assert isinstance(py_d, pydt.date)
        assert py_d == pydt.date(2026, 1, 30)

    def test_to_pytime(self) -> None:
        py_t = self.ts.to_pytime()
        assert isinstance(py_t, pydt.time)
        assert py_t == pydt.time(18, 59, 33, 123879)

    def test_to_date(self) -> None:
        d = self.ts.date()
        assert isinstance(d, ry.Date)
        assert d == ry.date(2026, 1, 30)

    def test_to_datetime(self) -> None:
        dt = self.ts.datetime()
        assert isinstance(dt, ry.DateTime)
        assert dt == ry.date(2026, 1, 30).at(18, 59, 33, 123879000)

    def test_to_time(self) -> None:
        t = self.ts.time()
        assert isinstance(t, ry.Time)
        assert t == ry.time(18, 59, 33, 123879000)

    def test_to_iso_week_date(self) -> None:
        iwd = self.ts.iso_week_date()
        assert isinstance(iwd, ry.ISOWeekDate)
        assert iwd == ry.ISOWeekDate(2026, 5, 5)

    # ---- isoformat ----
    def test_isoformat(self) -> None:
        iso_str = self.ts.isoformat()
        assert isinstance(iso_str, str)
        assert iso_str == "2026-01-30T18:59:33.123879Z"

    # ---- is zero ----
    def test_is_zero(self) -> None:
        ts_zero = ry.Timestamp(0, 0)
        assert ts_zero.is_zero is True
        assert self.ts.is_zero is False
        assert ry.Timestamp.MIN.is_zero is False
        assert ry.Timestamp.MAX.is_zero is False
        assert ry.Timestamp.UNIX_EPOCH.is_zero is True

    # --- as XYZ ---
    def test_as_microseconds(self) -> None:
        micros = self.ts.as_microsecond()
        assert isinstance(micros, int)
        assert micros == 1769799573123879

    def test_as_milliseconds(self) -> None:
        millis = self.ts.as_millisecond()
        assert isinstance(millis, int)
        assert millis == 1769799573123

    # --- signum ---
    def test_signum(self) -> None:
        ts_pos = ry.Timestamp(5, -999_999_999)
        assert ts_pos.signum() == 1
        assert ts_pos.as_second() == 4
        assert ts_pos.subsec_nanosecond == 1

        ts_neg = ry.Timestamp(-5, 999_999_999)
        assert ts_neg.signum() == -1
        assert ts_neg.as_second() == -4
        assert ts_neg.subsec_nanosecond == -1

    # -- strptime --
    def test_strptime(self) -> None:
        ts = ry.Timestamp.strptime("2024-07-14 21:14 -04:00", "%F %H:%M %:z")
        assert ts == ry.Timestamp.parse("2024-07-15T01:14:00Z")


class TestTimestampRichComparison:
    def test_eq(self) -> None:
        ts1 = ry.Timestamp.parse("2024-06-15T12:00:00Z")
        ts2 = ry.Timestamp.parse("2024-06-15T12:00:00Z")
        ts3 = ry.Timestamp.parse("2024-06-15T12:00:01Z")
        assert ts1 == ts2
        assert ts1 != ts3

    def test_lt_le_gt_ge(self) -> None:
        ts1 = ry.Timestamp.parse("2024-06-15T12:00:00Z")
        ts2 = ry.Timestamp.parse("2024-06-15T12:00:01Z")
        assert ts1 < ts2
        assert ts1 <= ts2
        assert ts2 > ts1
        assert ts2 >= ts1
        assert ts1 <= ts1
        assert ts1 >= ts1


class TestTimestampProperties:
    ts = ry.Timestamp.parse("2026-01-30T18:59:33.123879Z")

    def test_second(self) -> None:
        second = self.ts.second
        assert isinstance(second, int)
        assert second == 1769799573

    def test_nanosecond(self) -> None:
        assert self.ts.nanosecond == 123879000

    def test_subsec_nanosecond(self) -> None:
        assert self.ts.subsec_nanosecond == 123879000

    def test_subsec_microsecond(self) -> None:
        assert self.ts.subsec_microsecond == 123879

    def test_subsec_millisecond(self) -> None:
        assert self.ts.subsec_millisecond == 123


class TestTimestampFrom:
    def test_from_microsecond(self) -> None:
        ts_pos = ry.Timestamp.from_microsecond(1)
        assert ts_pos == ry.Timestamp.parse("1970-01-01T00:00:00.000001Z")
        ts_neg = ry.Timestamp.from_microsecond(-1)
        assert ts_neg == ry.Timestamp.parse("1969-12-31T23:59:59.999999Z")

    def test_from_nanosecond(self) -> None:
        ts_pos = ry.Timestamp.from_nanosecond(1)
        assert ts_pos == ry.Timestamp.parse("1970-01-01T00:00:00.000000001Z")
        ts_neg = ry.Timestamp.from_nanosecond(-1)
        assert ts_neg == ry.Timestamp.parse("1969-12-31T23:59:59.999999999Z")


class TestTimestampSinceUntil:
    def test_since(self) -> None:
        dearlier = ry.Timestamp.parse("2006-08-24T22:30:00Z")
        dlater = ry.Timestamp.parse("2019-01-31 21:00:00Z")
        dur = dlater - dearlier
        dur_since = dlater.since(dearlier)
        assert isinstance(dur, ry.TimeSpan)
        assert dur == ry.TimeSpan(seconds=392509800)
        assert dur == dur_since

    def test_timestamp_duration_until(self) -> None:
        earlier = ry.Timestamp.parse("2006-08-24T22:30:00Z")
        later = ry.Timestamp.parse("2019-01-31 21:00:00Z")
        dur = earlier.duration_until(later)
        assert isinstance(dur, ry.SignedDuration)
        assert dur == ry.SignedDuration(secs=392509800)

        dur_neg = later.duration_until(earlier)
        assert isinstance(dur_neg, ry.SignedDuration)
        assert dur_neg == ry.SignedDuration(secs=-392509800)

    def test_duration_since(self) -> None:
        earlier = ry.Timestamp.parse("2006-08-24T22:30:00Z")
        later = ry.Timestamp.parse("2019-01-31 21:00:00Z")
        dur = later.duration_since(earlier)
        assert isinstance(dur, ry.SignedDuration)
        assert dur == ry.SignedDuration(secs=392509800)


class TestTimestampSaturatingAddSub:
    def test_saturating_add(self) -> None:
        assert ry.Timestamp.MAX == ry.Timestamp.MAX.saturating_add(
            ry.TimeSpan()._nanoseconds(1)
        )
        assert ry.Timestamp.MIN == ry.Timestamp.MIN.saturating_add(
            ry.TimeSpan()._nanoseconds(-1)
        )
        assert ry.Timestamp.MAX == ry.Timestamp.UNIX_EPOCH.saturating_add(
            ry.SignedDuration.MAX
        )
        assert ry.Timestamp.MIN == ry.Timestamp.UNIX_EPOCH.saturating_add(
            ry.SignedDuration.MIN
        )
        assert ry.Timestamp.MAX == ry.Timestamp.UNIX_EPOCH.saturating_add(
            ry.Duration.MAX
        )

    def test_saturating_sub(self) -> None:
        assert ry.Timestamp.MIN == ry.Timestamp.MIN.saturating_sub(
            ry.TimeSpan()._nanoseconds(1)
        )
        assert ry.Timestamp.MAX == ry.Timestamp.MAX.saturating_sub(
            ry.TimeSpan()._nanoseconds(-1)
        )
        assert ry.Timestamp.MIN == ry.Timestamp.UNIX_EPOCH.saturating_sub(
            ry.SignedDuration.MAX
        )
        assert ry.Timestamp.MAX == ry.Timestamp.UNIX_EPOCH.saturating_sub(
            ry.SignedDuration.MIN
        )
        assert ry.Timestamp.MIN == ry.Timestamp.UNIX_EPOCH.saturating_sub(
            ry.Duration.MAX
        )


class TestTimestampRound:
    def test_timestamp_round_jiff_example(self) -> None:
        ts = ry.Timestamp.parse("2024-06-19 15:30:00Z")
        rounded = ts.round("hour")
        assert rounded == ry.Timestamp.parse("2024-06-19T16:00:00Z")

        ts = ry.Timestamp.parse("2024-06-19 15:29:59Z")
        rounded = ts.round("hour")
        assert rounded == ry.Timestamp.parse("2024-06-19T15:00:00Z")

    def test_timestamp_round_jiff_example_with_object(self) -> None:
        ts = ry.Timestamp.parse("2024-06-19 15:30:00Z")
        round_obj = ry.TimestampRound()._smallest("hour")._mode("half-expand")
        rounded = ts._round(round_obj)
        assert rounded == ry.Timestamp.parse("2024-06-19T16:00:00Z")

        ts = ry.Timestamp.parse("2024-06-19 15:29:59Z")
        round_obj = ry.TimestampRound()._smallest("hour")._mode("half-expand")
        rounded = ts._round(round_obj)
        assert rounded == ry.Timestamp.parse("2024-06-19T15:00:00Z")


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
