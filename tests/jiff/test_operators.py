from __future__ import annotations

import ry


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

    time_add_span = t1 + span
    assert time_add_span == t2
    assert isinstance(time_add_span, ry.Timestamp)


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
