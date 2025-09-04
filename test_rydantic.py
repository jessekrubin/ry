"""tests for pydantic + ry

Adapted from pydantic's own tests:
 - https://github.com/pydantic/pydantic/blob/main/tests/test_datetime.py

"""

import datetime as pydt
import typing as t

import pydantic
import pytest

import ry


class DateModel(pydantic.BaseModel):
    d: ry.Date


@pytest.mark.parametrize(
    "value,result",
    [
        # Valid inputs
        (1_493_942_400, ry.date(2017, 5, 5)),
        (1_493_942_400_000, ry.date(2017, 5, 5)),
        (0, ry.date(1970, 1, 1)),
        ("2012-04-23", ry.date(2012, 4, 23)),
        (b"2012-04-23", ry.date(2012, 4, 23)),
        (pydt.date(2012, 4, 9), ry.date(2012, 4, 9)),
        (pydt.datetime(2012, 4, 9, 0, 0), ry.date(2012, 4, 9)),
        (ry.date(2012, 4, 9), ry.date(2012, 4, 9)),
        (ry.datetime(2012, 4, 9, 0, 0, 0), ry.date(2012, 4, 9)),
        (
            ry.datetime(2012, 4, 9, 0, 0, 0).in_tz("America/New_York"),
            ry.date(2012, 4, 9),
        ),
        (ry.datetime(2012, 4, 9, 0, 0, 0).in_tz("UTC"), ry.date(2012, 4, 9)),
        # Invalid inputs
        # (pydt.datetime(2012, 4, 9, 12, 15), Err('Datetimes provided to dates should have zero time - e.g. be exact dates')),
        # ('x20120423', Err('Input should be a valid date or datetime, input is too short')),
        # ('2012-04-56', Err('Input should be a valid date or datetime, day value is outside expected range')),
        # (19_999_958_400, pydt.date(2603, 10, 11)),  # just before watershed
        # (20000044800, Err('type=date_from_datetime_inexact,')),  # just after watershed
        # (1_549_238_400, pydt.date(2019, 2, 4)),  # nowish in s
        # (1_549_238_400_000, pydt.date(2019, 2, 4)),  # nowish in ms
        # (1_549_238_400_000_000, Err('Input should be a valid date or datetime, dates after 9999')),  # nowish in μs
        # (1_549_238_400_000_000_000, Err('Input should be a valid date or datetime, dates after 9999')),  # nowish in ns
        # ('infinity', Err('Input should be a valid date or datetime, input is too short')),
        # (float('inf'), Err('Input should be a valid date or datetime, dates after 9999')),
        # (int('1' + '0' * 100), Err('Input should be a valid date or datetime, dates after 9999')),
        # (1e1000, Err('Input should be a valid date or datetime, dates after 9999')),
        # (float('-infinity'), Err('Input should be a valid date or datetime, dates before 0000')),
        # (float('nan'), Err('Input should be a valid date or datetime, NaN values not permitted')),
    ],
)
def test_date_parsing(value: t.Any, result: ry.Date) -> None:
    # if isinstance(result, Err):
    #     with pytest.raises(pydantic.ValidationError, match=result.message_escaped()):
    #         DateModel(d=value)
    # else:
    assert DateModel(d=value).d == result
