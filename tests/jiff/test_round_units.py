from __future__ import annotations

from typing import TYPE_CHECKING

import ry

if TYPE_CHECKING:
    from ry.ryo3._jiff import JiffRoundMode, JiffUnit

_JIFF_UNITS: tuple[JiffUnit, ...] = (
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
)

_JIFF_ROUND_MODES: tuple[JiffRoundMode, ...] = (
    "ceil",
    "floor",
    "expand",
    "trunc",
    "half-ceil",
    "half-floor",
    "half-expand",
    "half-trunc",
    "half-even",
)


_TIMESTAMP_DIFFERENCE_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
]
_TIME_DIFFERENCE_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
]
_DATE_DIFFERENCE_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]
_DATETIME_DIFFERENCE_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]
_ZONED_DATETIME_DIFFERENCE_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
    "month",
    "year",
]


class TestDifferenceUnitsDefaults:
    def test_timestamp_difference(self) -> None:
        ts1 = ry.Timestamp.parse("2024-03-15 08:14:02.5001Z")
        ts2 = ry.Timestamp.parse("2024-03-15T08:16:03.0001Z")
        timestamp_difference_units = []
        for unit in _JIFF_UNITS:
            try:
                _diff = ts1.until(ts2, smallest=unit)  # type: ignore[arg-type]
                timestamp_difference_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(timestamp_difference_units) == set(_TIMESTAMP_DIFFERENCE_UNITS), (
            f"Expected all units to be valid, but got {timestamp_difference_units}"
        )

    def test_time_difference(self) -> None:
        t1 = ry.Time.now()
        t2 = t1 + ry.timespan(minutes=2, seconds=1)
        time_difference_units = []
        for unit in _JIFF_UNITS:
            try:
                _diff = t1.until(t2, smallest=unit)  # type: ignore[arg-type]
                time_difference_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(time_difference_units) == set(_TIME_DIFFERENCE_UNITS), (
            f"Expected all units to be valid, but got {time_difference_units}"
        )

    def test_date_difference(self) -> None:
        d1 = ry.Date.today()
        d2 = d1 + ry.timespan(days=5)
        date_difference_units = []
        for unit in _JIFF_UNITS:
            try:
                _diff = d1.until(d2, smallest=unit)  # type: ignore[arg-type]
                date_difference_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(date_difference_units) == set(_DATE_DIFFERENCE_UNITS), (
            f"Expected all units to be valid, but got {date_difference_units}"
        )

    def test_datetime_difference(self) -> None:
        dt1 = ry.ZonedDateTime.now().datetime()
        dt2 = dt1 - ry.timespan(days=5, hours=2, minutes=3)
        datetime_difference_units = []
        for unit in _JIFF_UNITS:
            try:
                _diff = dt1.until(dt2, smallest=unit)
                datetime_difference_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(datetime_difference_units) == set(_DATETIME_DIFFERENCE_UNITS), (
            f"Expected all units to be valid, but got {datetime_difference_units}"
        )

    def test_zoned_datetime_difference(self) -> None:
        zdt1 = ry.ZonedDateTime.now()
        zdt2 = zdt1 - ry.timespan(days=5, hours=2, minutes=3)
        zoned_datetime_difference_units = []
        for unit in _JIFF_UNITS:
            try:
                _diff = zdt1.until(zdt2, smallest=unit)
                zoned_datetime_difference_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(zoned_datetime_difference_units) == set(
            _ZONED_DATETIME_DIFFERENCE_UNITS
        ), f"Expected all units to be valid, but got {zoned_datetime_difference_units}"


_TIMESTAMP_ROUND_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
]
_TIME_ROUND_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
]
_DATETIME_ROUND_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
]
_ZONED_DATETIME_ROUND_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
    "day",
]
_OFFSET_ROUND_UNITS = [
    "second",
    "minute",
    "hour",
]
_SIGNED_DURATION_ROUND_UNITS = [
    "nanosecond",
    "microsecond",
    "millisecond",
    "second",
    "minute",
    "hour",
]


class TestRoundingAllUnits:
    def test_timestamp_rounding(self) -> None:
        ts = ry.Timestamp.now()
        timestamp_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = ts.round(unit)  # type: ignore[arg-type]
                timestamp_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(timestamp_smallest_units) == set(_TIMESTAMP_ROUND_UNITS), (
            f"Expected all units to be valid, but got {timestamp_smallest_units}"
        )

    def test_time_rounding(self) -> None:
        t = ry.Time.now()
        time_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = t.round(unit)  # type: ignore[arg-type]
                time_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(time_smallest_units) == set(_TIME_ROUND_UNITS), (
            f"Expected all units to be valid, but got {time_smallest_units}"
        )

    def test_datetime_rounding(self) -> None:
        dt = ry.ZonedDateTime.now().datetime()
        datetime_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = dt.round(unit)  # type: ignore[arg-type]
                datetime_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(datetime_smallest_units) == set(_DATETIME_ROUND_UNITS), (
            f"Expected all units to be valid, but got {datetime_smallest_units}"
        )

    def test_zoned_datetime_rounding(self) -> None:
        zdt = ry.ZonedDateTime.now()
        zoned_datetime_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = zdt.round(unit)  # type: ignore[arg-type]
                zoned_datetime_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(zoned_datetime_smallest_units) == set(_ZONED_DATETIME_ROUND_UNITS), (
            f"Expected all units to be valid, but got {zoned_datetime_smallest_units}"
        )

    def test_offset_rounding(self) -> None:
        off = ry.Offset.from_seconds(12345)
        offset_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = off.round(unit)  # type: ignore[arg-type]
                offset_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(offset_smallest_units) == set(_OFFSET_ROUND_UNITS), (
            f"Expected all units to be valid, but got {offset_smallest_units}"
        )

    def test_signed_duration_rounding(self) -> None:
        sd = ry.SignedDuration(123, 123)
        signed_duration_smallest_units = []
        for unit in _JIFF_UNITS:
            try:
                _rounded = sd.round(unit)  # type: ignore[arg-type]
                signed_duration_smallest_units.append(unit)
            except ValueError as _ve:
                ...
        assert set(signed_duration_smallest_units) == set(
            _SIGNED_DURATION_ROUND_UNITS
        ), f"Expected all units to be valid, but got {signed_duration_smallest_units}"
