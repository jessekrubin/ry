from __future__ import annotations

import datetime as pydt

from hypothesis import assume, given
from hypothesis import strategies as st

import ry

from ..strategies import st_timezones

# settings.register_profile("slow", max_examples=2_000, deadline=2_000)  # noqa: ERA001
# settings.load_profile("slow")  # noqa: ERA001


@given(st.datetimes(timezones=st.none()))
def test_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that datetime.isoformat() produces the expected string."""

    py_isoformat = dt.isoformat()
    ry_dt = ry.DateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


@given(st.dates())
def test_date_isoformat(d: pydt.datetime) -> None:
    py_isoformat = d.isoformat()
    ry_dt = ry.Date.from_pydate(d)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


@given(st.times())
def test_time_isoformat(d: pydt.time) -> None:
    py_isoformat = d.isoformat()
    ry_dt = ry.Time.from_pytime(d)
    ry_isoformat = ry_dt.isoformat()
    assert ry_isoformat == py_isoformat, f"py: {py_isoformat}\nry: {ry_isoformat}"


def _test_zoned_datetime_isoformat(dt: pydt.datetime) -> None:
    py_isoformat = dt.isoformat()
    ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    is_eq = ry_isoformat == py_isoformat
    ry_prefix_ok = py_isoformat.startswith(ry_isoformat)
    if not is_eq and not ry_prefix_ok:
        py_zdt_utc = dt.astimezone(pydt.UTC)
        ry_zdt_utc = ry_dt.inutc()

        is_imaginary = datetime_does_not_exist(dt)
        msg_lines = (
            "____________\nISO format mismatch:",
            f"input datetime: {dt} (repr: {dt!r})",
            f"is_imaginary: {is_imaginary}",
            f"py: {py_isoformat}",
            f"ry: {ry_isoformat}",
            f"is_eq: {is_eq}",
            f"ry_prefix_ok: {ry_prefix_ok}",
            f"py_zdt_utc: {py_zdt_utc.isoformat()}",
            f"ry_zdt_utc: {ry_zdt_utc.isoformat()}",
        )
        assert ry_isoformat == py_isoformat or py_isoformat.startswith(ry_isoformat), (
            "\n".join(msg_lines)
        )


def datetime_does_not_exist(value: pydt.datetime) -> bool:
    """Return True if the given datetime is "imaginary" (i.e. does not exist).

    This function tests whether the given datetime can be round-tripped to and
    from UTC.  It is an exact inverse of (and very similar to) the dateutil method
    https://dateutil.readthedocs.io/en/stable/tz.html#dateutil.tz.datetime_exists

    NOTE: Taken from `hypothesis.strategies._internal.datetime`
    """
    # Naive datetimes cannot be imaginary, but we need this special case because
    # chaining .astimezone() ends with *the system local timezone*, not None.
    # See bug report in https://github.com/HypothesisWorks/hypothesis/issues/2662
    if value.tzinfo is None:
        return False
    try:
        # Does the naive portion of the datetime change when round-tripped to
        # UTC?  If so, or if this overflows, we say that it does not exist.
        roundtrip = value.astimezone(pydt.UTC).astimezone(value.tzinfo)
    except OverflowError:
        # Overflows at datetime.min or datetime.max boundary condition.
        # Rejecting these is acceptable, because timezones are close to
        # meaningless before ~1900 and subject to a lot of change by
        # 9999, so it should be a very small fraction of possible values.
        return True

    if (
        value.tzinfo is not roundtrip.tzinfo
        and value.utcoffset() != roundtrip.utcoffset()
    ):
        # This only ever occurs during imaginary (i.e. nonexistent) datetimes,
        # and only for pytz timezones which do not follow PEP-495 semantics.
        # (may exclude a few other edge cases, but you should use zoneinfo anyway)
        return True

    assert value.tzinfo is roundtrip.tzinfo, "so only the naive portions are compared"
    return value != roundtrip


@given(st.datetimes(timezones=st_timezones(), allow_imaginary=False))
def test_zoned_datetime_isoformat(dt: pydt.datetime) -> None:
    """Test that ZondedDateTime.isoformat() produces the expected string."""

    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware

    py_isoformat = dt.isoformat()
    ry_zdt = ry.ZonedDateTime.from_pydatetime(dt)
    ry_isoformat = ry_zdt.isoformat()
    assert ry_isoformat == py_isoformat

    _test_zoned_datetime_isoformat(dt)


@given(st.datetimes(timezones=st_timezones(), allow_imaginary=False))
def test_zoned_datetime_iso_format_works_with_py_datetime_from(
    dt: pydt.datetime,
) -> None:
    """Test that ZondedDateTime.isoformat() produces the expected string."""
    assume(dt.tzinfo is not None)  # Ensure the datetime is timezone-aware
    ry_dt = ry.ZonedDateTime.from_pydatetime(dt)
    ry_isoformat = ry_dt.isoformat()
    py_from_zdt_isoformat = pydt.datetime.fromisoformat(ry_isoformat)
    assert isinstance(py_from_zdt_isoformat, pydt.datetime), (
        f"Expected a datetime instance, got {type(py_from_zdt_isoformat)}"
    )
