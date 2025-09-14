import ry


def test_strftime_timestamp() -> None:
    """Test strftime method of Timestamp.

    REF: https://docs.rs/jiff/latest/jiff/struct.Timestamp.html#method.strftime
    """
    ts = ry.Timestamp.from_second(86_400)
    string = ts.strftime("%a %b %e %I:%M:%S %p UTC %Y")
    assert string == "Fri Jan  2 12:00:00 AM UTC 1970"
    assert f"{ts:%a %b %e %I:%M:%S %p UTC %Y}" == "Fri Jan  2 12:00:00 AM UTC 1970"


def test_strftime_zoned_datetime() -> None:
    """Test strftime method of ZonedDateTime.

    REF: https://docs.rs/jiff/latest/jiff/struct.Zoned.html#method.strftime
    """
    zdt = ry.date(2024, 7, 15).at(16, 24, 59, 0).in_tz("America/New_York")
    string = zdt.strftime("%a %b %e %I:%M:%S %p %Z %Y")
    assert string == "Mon Jul 15 04:24:59 PM EDT 2024"
    assert f"{zdt:%a %b %e %I:%M:%S %p %Z %Y}" == "Mon Jul 15 04:24:59 PM EDT 2024"


def test_strftime_date() -> None:
    """Test strftime method of Date.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.Date.html#method.strftime
    """
    date = ry.date(2024, 7, 15)
    string = date.strftime("%Y-%m-%d is a %A")
    assert string == "2024-07-15 is a Monday"
    assert f"{date:%Y-%m-%d is a %A}" == "2024-07-15 is a Monday"


def test_strftime_time() -> None:
    """Test strftime method of Time.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.Time.html#method.strftime
    """
    t = ry.time(16, 30, 59, 0)
    string = t.strftime("%-I:%M%P")
    assert string == "4:30pm"
    assert f"{t:%-I:%M%P}" == "4:30pm"
    t_rounded = t.round("minute")
    string_rounded = t_rounded.strftime("%-I:%M%P")
    assert string_rounded == "4:31pm"
    assert f"{t_rounded:%-I:%M%P}" == "4:31pm"


def test_strftime_datetime() -> None:
    """Test strftime method of DateTime.

    REF: https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html#method.strftime
    """
    dt = ry.date(2024, 7, 15).at(16, 24, 59, 0)
    string = dt.strftime("%A, %B %e, %Y at %H:%M:%S")
    assert string == "Monday, July 15, 2024 at 16:24:59"
    assert f"{dt:%A, %B %e, %Y at %H:%M:%S}" == "Monday, July 15, 2024 at 16:24:59"
