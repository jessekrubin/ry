"""test jiff examples section

tests based on the examples in the jiff documentation
"""

import ry.dev as ry
from ry.dev import Timestamp


def test_print_datetime_for_a_timestamp():
    ts = Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned(ry.TimeZone("America/New_York"))
    assert zdt.string() == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    assert ts.string() == "2024-07-10T21:19:25.567Z"
    assert ts.as_second() == 1720646365
    assert ts.as_nanosecond() == 1720646365567000000


def test_create_zoned_datetime_from_civil_time():
    zdt = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    assert zdt.string() == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_change_an_instant_from_one_time_zone_to_another():
    zdt1 = ry.date(1918, 11, 11).at(11, 0, 0, 0).intz("Europe/Paris")
    zdt2 = zdt1.intz("America/New_York")
    assert zdt2.string() == "1918-11-11T06:00:00-05:00[America/New_York]"


def test_find_duration_between_datetimes():
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    zdt2 = ry.date(2023, 12, 31).at(18, 30, 0, 0).intz("America/New_York")
    span = zdt2 - zdt1
    assert isinstance(span, ry.Span)
    assert str(span) == "PT29341h3m"


def test_add_duration_to_a_zoned_datetime():
    zdt1 = ry.date(2020, 8, 26).at(6, 27, 0, 0).intz("America/New_York")
    span = ry.Span().years(3).months(4).days(5).hours(12).minutes(3)
    zdt2 = zdt1.checked_add(span)
    assert zdt2.string() == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity():
    zdt = ry.date(2024, 3, 10).at(2, 30, 0, 0).intz("America/New_York")
    assert zdt.string() == "2024-03-10T03:30:00-04:00[America/New_York]"
    zdt = ry.date(2024, 11, 3).at(1, 30, 0, 0).intz("America/New_York")
    assert zdt.string() == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span():
    span: ry.Span = ry.Span.parse("P5y1w10dT5h59m")
    expected = ry.Span().years(5).weeks(1).days(10).hours(5).minutes(59)
    assert span == expected
    assert str(span) == "P5y1w10dT5h59m"


def test_using_strftime_and_strptime_for_formatting_and_parsing():
    zdt = ry.Zoned.strptime(
        "%A, %B %d, %Y at %I:%M%p %V",
        "Monday, July 15, 2024 at 5:30pm US/Eastern",
    )
    assert zdt.string() == "2024-07-15T17:30:00-04:00[US/Eastern]"
    zdt = ry.date(2024, 7, 15).at(17, 30, 59, 0).intz("Australia/Tasmania")
    string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %Z")
    assert string == "Monday, July 15, 2024 at 5:30pm AEST"

    zdt = ry.date(2024, 7, 15).at(17, 30, 59, 0).intz("Australia/Tasmania")
    string = zdt.strftime("%A, %B %d, %Y at %-I:%M%P %V")
    assert string == "Monday, July 15, 2024 at 5:30pm Australia/Tasmania"
