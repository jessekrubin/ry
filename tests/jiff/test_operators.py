from __future__ import annotations

import pytest

import ry


@pytest.mark.parametrize(
    "val_spanish",
    [
        # time
        (ry.time(14, 30, 0, 0), ry.timespan(hours=2, minutes=15)),
        (ry.time(23, 45, 30, 0), ry.SignedDuration(secs=-3600)),
        # datetime
        (ry.datetime(2021, 5, 15, 9, 0, 0, 0), ry.timespan(days=1, hours=3)),
        (ry.datetime(2020, 1, 1, 0, 0, 0, 0), ry.SignedDuration(secs=-7200)),
        # date
        (ry.date(2021, 12, 31), ry.timespan(days=5)),
        (
            ry.date(2021, 12, 31),
            ry.timespan(days=5).to_signed_duration(ry.ZonedDateTime.now()),
        ),
        # timestamp
        (ry.Timestamp(second=1622520000, nanosecond=0), ry.timespan(hours=4)),
        (ry.Timestamp(second=1622520000, nanosecond=0), ry.SignedDuration(secs=-1800)),
        # zoned datetime
        (
            ry.date(2022, 3, 10).at(12, 0, 0, 0).in_tz("Europe/Madrid"),
            ry.timespan(hours=6, minutes=30),
        ),
        (
            ry.date(2022, 3, 10).at(12, 0, 0, 0).in_tz("Europe/Madrid"),
            ry.SignedDuration(secs=-5400),
        ),
    ],
)
def test_span_addition_commutative(
    val_spanish: tuple[
        ry.Time | ry.DateTime | ry.Date | ry.Timestamp | ry.ZonedDateTime,
        ry.TimeSpan | ry.SignedDuration,
    ],
) -> None:
    val, span = val_spanish

    assert val + span == span + val


def test_time_sub() -> None:
    t1 = ry.time(16, 30, 59, 0)
    t2 = ry.time(16, 30, 0, 0)
    span = t1 - t2
    assert str(span) == "PT59S"
    assert isinstance(span, ry.TimeSpan)

    inplace_time = ry.time(16, 30, 59, 0)
    inplace_time -= span

    assert inplace_time == t2
    assert isinstance(inplace_time, ry.Time)

    time_sub_span = t1 - span
    assert time_sub_span == t2
    assert isinstance(time_sub_span, ry.Time)


def test_time_add() -> None:
    t1 = ry.time(16, 30, 59, 0)
    t2 = ry.time(16, 30, 0, 0)
    span = t1 - t2
    assert str(span) == "PT59S"
    assert isinstance(span, ry.TimeSpan)

    inplace_span = ry.time(16, 30, 0, 0)
    inplace_span += span
    assert inplace_span == t1
    assert isinstance(inplace_span, ry.Time)

    time_add_span = t2 + span
    assert time_add_span == t1
    assert isinstance(time_add_span, ry.Time)


def test_time_add_commutative() -> None:
    t1 = ry.time(16, 30, 0, 0)
    span = ry.timespan(seconds=59)
    assert t1 + span == span + t1


def test_datetime_sub() -> None:
    t1 = ry.datetime(2021, 1, 1, 16, 30, 59, 0)
    t2 = ry.datetime(2021, 1, 1, 16, 30, 0, 0)
    span = t1 - t2
    assert str(span) == "PT59S"
    assert isinstance(span, ry.TimeSpan)

    inplace_dt = ry.datetime(2021, 1, 1, 16, 30, 59, 0)
    inplace_dt -= span
    time_sub_span = t1 - span
    assert time_sub_span == t2
    assert isinstance(time_sub_span, ry.DateTime)


def test_datetime_add() -> None:
    t1 = ry.datetime(2021, 1, 1, 16, 30, 59, 0)
    t2 = ry.datetime(2021, 1, 1, 16, 30, 0, 0)
    span = t1 - t2
    assert str(span) == "PT59S"
    assert isinstance(span, ry.TimeSpan)

    inplace_dt = ry.datetime(2021, 1, 1, 16, 30, 0, 0)
    inplace_dt += span
    assert inplace_dt == t1
    assert isinstance(inplace_dt, ry.DateTime)

    time_add_span = t2 + span
    assert time_add_span == t1
    assert isinstance(time_add_span, ry.DateTime)


def test_datetime_add_commutative() -> None:
    t1 = ry.datetime(2021, 1, 1, 16, 30, 0, 0)
    span = ry.timespan(seconds=59)
    assert t1 + span == span + t1


def test_date_sub() -> None:
    t1 = ry.date(2021, 1, 1)
    t2 = ry.date(2021, 1, 2)
    span = t1 - t2
    assert str(span) == "-P1D"
    assert isinstance(span, ry.TimeSpan)

    inplace_span = ry.date(2021, 1, 1)
    inplace_span -= span
    assert inplace_span == t2
    assert isinstance(inplace_span, ry.Date)

    time_sub_span = t1 - span
    assert time_sub_span == t2
    assert isinstance(time_sub_span, ry.Date)


def test_date_add() -> None:
    t1 = ry.date(2021, 1, 1)
    t2 = ry.date(2021, 1, 2)
    span = t2 - t1
    assert str(span) == "P1D"
    assert isinstance(span, ry.TimeSpan)

    inplace_span = ry.date(2021, 1, 1)
    inplace_span += span
    assert inplace_span == t2
    assert isinstance(inplace_span, ry.Date)

    time_add_span = t1 + span
    assert time_add_span == t2
    assert isinstance(time_add_span, ry.Date)


def test_date_add_commutative() -> None:
    t1 = ry.date(2021, 1, 1)
    span = ry.timespan(days=1)
    assert t1 + span == span + t1


def test_timestamp_sub() -> None:
    t1 = ry.Timestamp(
        second=1609459200,
        nanosecond=0,
    )
    t2 = ry.Timestamp(
        second=1609459201,
    )
    span = t1 - t2
    assert str(span) == "-PT1S"
    assert isinstance(span, ry.TimeSpan)

    inplace_ts = ry.Timestamp(
        second=1609459200,
        nanosecond=0,
    )
    inplace_ts -= span
    assert inplace_ts == t2
    assert isinstance(inplace_ts, ry.Timestamp)

    time_sub_span = t1 - span
    assert time_sub_span == t2
    assert isinstance(time_sub_span, ry.Timestamp)


def test_timestamp_add() -> None:
    t1 = ry.Timestamp(
        second=1609459200,
        nanosecond=0,
    )
    t2 = ry.Timestamp(
        second=1609459201,
    )
    span = t2 - t1

    assert str(span) == "PT1S"
    assert isinstance(span, ry.TimeSpan)

    inplace_span = ry.Timestamp(
        second=1609459200,
        nanosecond=0,
    )
    inplace_span += span
    assert inplace_span == t2
    assert isinstance(inplace_span, ry.Timestamp)

    timestamp_add_span = t1 + span
    assert timestamp_add_span == t2
    assert isinstance(timestamp_add_span, ry.Timestamp)

    timestamp_add_span_fn = t1 + span
    assert timestamp_add_span_fn == t2
    assert isinstance(timestamp_add_span_fn, ry.Timestamp)


def test_timestamp_add_commutative() -> None:
    t1 = ry.Timestamp(
        second=1609459200,
        nanosecond=0,
    )
    span = ry.timespan(seconds=1)
    assert t1 + span == span + t1


def test_zoned_sub() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    zdt2 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/Los_Angeles")
    span = zdt1 - zdt2
    assert str(span) == "-PT3H"
    assert isinstance(span, ry.TimeSpan)

    inplace_zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    inplace_zdt -= span
    assert inplace_zdt == zdt2
    assert isinstance(inplace_zdt, ry.ZonedDateTime)

    time_sub_span = zdt1 - span
    assert time_sub_span == zdt2
    assert isinstance(time_sub_span, ry.ZonedDateTime)


def test_zoned_add() -> None:
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    zdt2 = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/Los_Angeles")
    span = zdt1 - zdt2
    assert str(span) == "-PT3H"
    assert isinstance(span, ry.TimeSpan)

    inplace_zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/Los_Angeles")
    inplace_zdt += span
    assert inplace_zdt == zdt1
    assert isinstance(inplace_zdt, ry.ZonedDateTime)

    time_add_span = zdt2 + span
    assert time_add_span == zdt1
    assert isinstance(time_add_span, ry.ZonedDateTime)


def test_zoned_add_commutative() -> None:
    zdt = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("UTC")
    span = ry.timespan(hours=3)
    assert zdt + span == span + zdt
