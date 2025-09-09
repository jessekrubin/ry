import datetime as pydt
from typing import Any

import annotated_types
import pydantic
import pytest
from rich import print
from typing_extensions import Annotated

import ry


def _to_ry_date(v):
    if isinstance(v, ry.Date):
        return v
    if isinstance(v, pydt.date):
        # If ry.Date has a dedicated ctor, use it; otherwise parse ISO.
        # Replace with ry.Date.from_ymd(v.year, v.month, v.day) if you prefer.
        return ry.Date.parse(v.isoformat())
    if isinstance(v, str):
        return ry.Date.parse(v)
    raise TypeError(f"Cannot coerce {type(v).__name__} to ry.Date")


class PyDateModel(pydantic.BaseModel):
    date: pydt.date


class PyDateModelConstrained(pydantic.BaseModel):
    # date: condate(gt=pydt.date(2000, 1, 1))
    # Annotated[pydt.date, pydantic.constr(regex=r"^\d{4}-\d{2}-\d{2}$")]
    date: Annotated[  # pyright: ignore[reportReturnType]
        pydt.date,
        # Strict(strict) if strict is not None else None,
        annotated_types.Interval(gt=pydt.date(2000, 1, 1)),
    ]


class RyDateModel(pydantic.BaseModel):
    date: ry.Date


class RyDateModelConstrained(pydantic.BaseModel):
    # date: condate(gt=pydt.date(2000, 1, 1))
    # Annotated[pydt.date, pydantic.constr(regex=r"^\d{4}-\d{2}-\d{2}$")]
    date: Annotated[  # pyright: ignore[reportReturnType]
        ry.Date,
        # Strict(strict) if strict is not None else None,
        annotated_types.Interval(gt=ry.Date(2000, 1, 1).to_pydate()),
    ]


@pytest.mark.parametrize(
    "data",
    [
        pydt.date(2020, 1, 1),
        pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.timezone.utc),
        ry.Date(2020, 1, 1),
        "2020-01-01",
    ],
)
def test_date_inputs(data: Any):
    print(f"Input: {data!r}")
    # py_model = PyDateModel(date=data)
    ry_model = RyDateModel(date=data)
    # print(f"  PyDateModel: {py_model.date!r} ({type(py_model.date).__name__})")
    print(f"  RyDateModel: {ry_model.date!r} ({type(ry_model.date).__name__})")
    # assert py_model.date.isoformat() == ry_model.date.isoformat()

    assert isinstance(ry_model.date, ry.Date)

    # py_model_dump = PyDateModel(date=data).model_dump()


@pytest.mark.parametrize(
    "data",
    [
        pydt.date(2020, 1, 1),
        pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.timezone.utc),
        ry.Date(2020, 1, 1),
        ry.Date(2020, 1, 1).at(1, 2, 3, 4),
        ry.Date(2020, 1, 1).at(1, 2, 3, 4).in_tz("America/Los_Angeles"),
        "2020-01-01",
        # errors...
        ry.Date(2000, 1, 1),
    ],
)
def test_date_inputs2(data: Any):
    print(f"Input: {data!r}")
    # py_model = PyDateModel(date=data)
    ry_model = RyDateModel(date=data)
    # print(f"  PyDateModel: {py_model.date!r} ({type(py_model.date).__name__})")
    print(f"  RyDateModel: {ry_model.date!r} ({type(ry_model.date).__name__})")
    # assert py_model.date.isoformat() == ry_model.date.isoformat()

    assert isinstance(ry_model.date, ry.Date)

    model_dumped_json = ry_model.model_dump_json()

    print(f"  RyDateModel JSON: {model_dumped_json}")

    from_json = RyDateModel.model_validate_json(model_dumped_json)

    print(f"  RyDateModel from JSON: {from_json!r} ({type(from_json).__name__})")
    assert from_json == ry_model


def _diff_schemas(left, right):
    left_no_title = {k: v for k, v in left.items() if k != "title"}
    right_no_title = {k: v for k, v in right.items() if k != "title"}
    assert left_no_title == right_no_title


def test_date_json_schema():
    py_model = PyDateModel.model_json_schema()
    ry_model = RyDateModel.model_json_schema()
    _diff_schemas(py_model, ry_model)


from uuid import UUID


class UuidModel(pydantic.BaseModel):
    id: UUID


class RyTimeModel(pydantic.BaseModel):
    d: ry.Time


@pytest.mark.parametrize(
    "value,result",
    [
        # Valid inputs
        ("09:15:00", pydt.time(9, 15)),
        ("10:10", pydt.time(10, 10)),
        ("10:20:30.400", pydt.time(10, 20, 30, 400_000)),
        (b"10:20:30.400", pydt.time(10, 20, 30, 400_000)),
        (pydt.time(4, 8, 16), pydt.time(4, 8, 16)),
        (pydt.time(10, 20, 30, 400_000), pydt.time(10, 20, 30, 400_000)),
        # Ry types
        # ry.Time
        (ry.Time(9, 15), pydt.time(9, 15)),
        (ry.Time(10, 20, 30, 400_000_000), pydt.time(10, 20, 30, 400_000)),
        # ry.DateTime
        (ry.date(2024, 1, 1).at(4, 8, 16), pydt.time(4, 8, 16)),
        # ry.ZonedDateTime
        (ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC"), pydt.time(4, 8, 16)),
        # ry.Timestamp
        (
            ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC").timestamp(),
            pydt.time(4, 8, 16),
        ),
        # NOT IMPLEMENTED
        (3610, pydt.time(1, 0, 10, tzinfo=pydt.timezone.utc)),
        (3600.5, pydt.time(1, 0, 0, 500000, tzinfo=pydt.timezone.utc)),
        (86400 - 1, pydt.time(23, 59, 59, tzinfo=pydt.timezone.utc)),
        # Invalid inputs
        # ('4:8:16', Err('Input should be in a valid time format, invalid character in hour [type=time_parsing,')),
        # (86400, Err('Input should be in a valid time format, numeric times may not exceed 86,399 seconds')),
        # ('xxx', Err('Input should be in a valid time format, input is too short [type=time_parsing,')),
        # ('091500', Err('Input should be in a valid time format, invalid time separator, expected `:`')),
        # (b'091500', Err('Input should be in a valid time format, invalid time separator, expected `:`')),
        # ('09:15:90', Err('Input should be in a valid time format, second value is outside expected range of 0-59')),
        # ('11:05:00Y', Err('Input should be in a valid time format, invalid timezone sign')),
        # # https://github.com/pydantic/speedate/issues/10
        # ('11:05:00-05:30', time(11, 5, 0, tzinfo=create_tz(-330))),
        # ('11:05:00-0530', time(11, 5, 0, tzinfo=create_tz(-330))),
        # ('11:05:00Z', time(11, 5, 0, tzinfo=timezone.utc)),
        # ('11:05:00+00:00', time(11, 5, 0, tzinfo=timezone.utc)),
        # ('11:05-06:00', time(11, 5, 0, tzinfo=create_tz(-360))),
        # ('11:05+06:00', time(11, 5, 0, tzinfo=create_tz(360))),
        # ('11:05:00-25:00', Err('Input should be in a valid time format, timezone offset must be less than 24 hours')),
    ],
)
def test_time_parsing(value, result):
    # if isinstance(result, Err):
    #     with pytest.raises(ValidationError, match=result.message_escaped()):
    #         TimeModel(d=value)
    # else:
    if isinstance(value, float | int):
        with pytest.raises(NotImplementedError):
            _d = RyTimeModel(d=value)
    else:
        print(value)
        rs_time = ry.Time.from_pytime(result)
        assert RyTimeModel(d=value).d == rs_time
        assert RyTimeModel(d=value).d.to_pytime() == result


# print(UuidModel.model_json_schema())
# print(PyDateModelConstrained.model_json_schema())


# r = RyDateModelConstrained(date=ry.Date(2000, 1, 1))
# print(r)
def create_tz(minutes):
    return pydt.timezone(pydt.timedelta(minutes=minutes))


class PyDatetimeModel(pydantic.BaseModel):
    dt: pydt.datetime


class RyDatetimeModel(pydantic.BaseModel):
    dt: ry.DateTime


@pytest.mark.parametrize(
    "value,result",
    [
        # Valid inputs
        # values in seconds
        # (1_494_012_444.883_309, pydt.datetime(2017, 5, 5, 19, 27, 24, 883_309, tzinfo=pydt.timezone.utc)),
        # (1_494_012_444, pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        # values in ms
        # (1_494_012_444_000, pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        ("2012-04-23T09:15:00", pydt.datetime(2012, 4, 23, 9, 15)),
        (
            "2012-04-23T09:15:00Z",
            pydt.datetime(2012, 4, 23, 9, 15, 0, 0, tzinfo=pydt.timezone.utc),
        ),
        (
            "2012-04-23T10:20:30.400+02:30",
            pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(150)),
        ),
        (
            "2012-04-23T10:20:30.400+02:00",
            pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(120)),
        ),
        (
            "2012-04-23T10:20:30.400-02:00",
            pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(-120)),
        ),
        (
            b"2012-04-23T10:20:30.400-02:00",
            pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(-120)),
        ),
        (pydt.datetime(2017, 5, 5), pydt.datetime(2017, 5, 5)),
        # (0, pydt.datetime(1970, 1, 1, 0, 0, 0, tzinfo=pydt.timezone.utc)),
        # Numeric inputs as strings
        # ('1494012444.883309', pydt.datetime(2017, 5, 5, 19, 27, 24, 883309, tzinfo=pydt.timezone.utc)),
        # ('1494012444', pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        # (b'1494012444', pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        # ('1494012444000.883309', pydt.datetime(2017, 5, 5, 19, 27, 24, 883, tzinfo=pydt.timezone.utc)),
        # ('-1494012444000.883309', pydt.datetime(1922, 8, 29, 4, 32, 35, 999117, tzinfo=pydt.timezone.utc)),
        # (19_999_999_999, pydt.datetime(2603, 10, 11, 11, 33, 19, tzinfo=pydt.timezone.utc)),  # just before watershed
        # (20_000_000_001, pydt.datetime(1970, 8, 20, 11, 33, 20, 1000, tzinfo=pydt.timezone.utc)),
        # just after watershed
        # (1_549_316_052, pydt.datetime(2019, 2, 4, 21, 34, 12, 0, tzinfo=pydt.timezone.utc)),  # nowish in s
        # (1_549_316_052_104, pydt.datetime(2019, 2, 4, 21, 34, 12, 104_000, tzinfo=pydt.timezone.utc)),  # nowish in ms
        # Invalid inputs
        # (1_549_316_052_104_324, Err('Input should be a valid datetime, dates after 9999')),  # nowish in μs
        # (1_549_316_052_104_324_096, Err('Input should be a valid datetime, dates after 9999')),  # nowish in ns
        # (float('inf'), Err('Input should be a valid datetime, dates after 9999')),
        # (float('-inf'), Err('Input should be a valid datetime, dates before 0000')),
        # (1e50, Err('Input should be a valid datetime, dates after 9999')),
        # (float('nan'), Err('Input should be a valid datetime, NaN values not permitted')),
    ],
)
def test_datetime_parsing(value, result):
    # strip the tzinfo if it is not None...
    result = result.replace(tzinfo=None)
    if isinstance(value, float | int):
        with pytest.raises(NotImplementedError):
            _d = RyDatetimeModel(dt=value)
    # if isinstance(result, Err):
    #     with pytest.raises(ValidationError, match=result.message_escaped()):
    #         DatetimeModel(dt=value)
    rs_expected = ry.DateTime.from_pydatetime(result)
    assert RyDatetimeModel(dt=value).dt == rs_expected


class RyZonedDatetimeModel(pydantic.BaseModel):
    dt: ry.ZonedDateTime


#
@pytest.mark.parametrize(
    "value,result",
    [
        # Valid inputs
        # strings
        (
            "2012-04-23T09:15:00+00:00[UTC]",
            pydt.datetime(2012, 4, 23, 9, 15, tzinfo=pydt.timezone.utc),
        ),
        # ('2012-04-23T09:15:00Z', pydt.datetime(2012, 4, 23, 9, 15, 0, 0, tzinfo=pydt.timezone.utc)),
        # ('2012-04-23T10:20:30.400+02:30', pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(150))),
        # ('2012-04-23T10:20:30.400+02:00', pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(120))),
        # ('2012-04-23T10:20:30.400-02:00', pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(-120))),
        # (b'2012-04-23T10:20:30.400-02:00', pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, create_tz(-120))),
        # (pydt.datetime(2017, 5, 5), pydt.datetime(2017, 5, 5)),
        # (0, pydt.datetime(1970, 1, 1, 0, 0, 0, tzinfo=pydt.timezone.utc)),
        # Numeric inputs as strings
        # NOT IMPLEMENTED
        # values in seconds
        (
            1_494_012_444.883_309,
            pydt.datetime(2017, 5, 5, 19, 27, 24, 883_309, tzinfo=pydt.timezone.utc),
        ),
        (
            1_494_012_444,
            pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc),
        ),
        # values in ms
        (
            1_494_012_444_000,
            pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc),
        ),
        # ('1494012444.883309', pydt.datetime(2017, 5, 5, 19, 27, 24, 883309, tzinfo=pydt.timezone.utc)),
        # ('1494012444', pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        # (b'1494012444', pydt.datetime(2017, 5, 5, 19, 27, 24, tzinfo=pydt.timezone.utc)),
        # ('1494012444000.883309', pydt.datetime(2017, 5, 5, 19, 27, 24, 883, tzinfo=pydt.timezone.utc)),
        # ('-1494012444000.883309', pydt.datetime(1922, 8, 29, 4, 32, 35, 999117, tzinfo=pydt.timezone.utc)),
        (
            19_999_999_999,
            pydt.datetime(2603, 10, 11, 11, 33, 19, tzinfo=pydt.timezone.utc),
        ),  # just before watershed
        (
            20_000_000_001,
            pydt.datetime(1970, 8, 20, 11, 33, 20, 1000, tzinfo=pydt.timezone.utc),
        ),
        # just after watershed
        # (1_549_316_052, pydt.datetime(2019, 2, 4, 21, 34, 12, 0, tzinfo=pydt.timezone.utc)),  # nowish in s
        # (1_549_316_052_104, pydt.datetime(2019, 2, 4, 21, 34, 12, 104_000, tzinfo=pydt.timezone.utc)),  # nowish in ms
        # Invalid inputs
        # (1_549_316_052_104_324, Err('Input should be a valid datetime, dates after 9999')),  # nowish in μs
        # (1_549_316_052_104_324_096, Err('Input should be a valid datetime, dates after 9999')),  # nowish in ns
        # (float('inf'), Err('Input should be a valid datetime, dates after 9999')),
        # (float('-inf'), Err('Input should be a valid datetime, dates before 0000')),
        # (1e50, Err('Input should be a valid datetime, dates after 9999')),
        # (float('nan'), Err('Input should be a valid datetime, NaN values not permitted')),
    ],
)
def test_zoned_parsing(value, result):
    if isinstance(value, float | int):
        with pytest.raises(NotImplementedError):
            _d = RyZonedDatetimeModel(dt=value)
    # if isinstance(result, Err):
    #     with pytest.raises(ValidationError, match=result.message_escaped()):
    #         DatetimeModel(dt=value)
    else:
        rs_expected = ry.ZonedDateTime.from_pydatetime(result)
        assert isinstance(rs_expected, ry.ZonedDateTime)
        assert RyZonedDatetimeModel(dt=value).dt == rs_expected


# class TimedeltaModel(pydantic.BaseModel):
#     d: pydt.timedelta
#
#
# @pytest.mark.parametrize(
#     'delta',
#     [
#         timedelta(days=4, minutes=15, seconds=30, milliseconds=100),  # fractions of seconds
#         timedelta(hours=10, minutes=15, seconds=30),  # hours, minutes, seconds
#         timedelta(days=4, minutes=15, seconds=30),  # multiple days
#         timedelta(days=1, minutes=00, seconds=00),  # single day
#         timedelta(days=-4, minutes=15, seconds=30),  # negative durations
#         timedelta(minutes=15, seconds=30),  # minute & seconds
#         timedelta(seconds=30),  # seconds
#     ],
# )
# def test_parse_python_format(TimedeltaModel, delta):
#     assert TimedeltaModel(d=delta).d == delta
#     # assert TimedeltaModel(d=str(delta)).d == delta
#
#
class TimedeltaModel(pydantic.BaseModel):
    d: pydt.timedelta


class SignedDurationModel(pydantic.BaseModel):
    d: ry.SignedDuration


class TimeSpanModel(pydantic.BaseModel):
    d: ry.TimeSpan


@pytest.mark.parametrize(
    "value,result",
    [
        # seconds
        (pydt.timedelta(seconds=30), pydt.timedelta(seconds=30)),
        (30, pydt.timedelta(seconds=30)),
        (30.1, pydt.timedelta(seconds=30, milliseconds=100)),
        (9.9e-05, pydt.timedelta(microseconds=99)),
        # minutes seconds
        ("00:15:30", pydt.timedelta(minutes=15, seconds=30)),
        ("00:05:30", pydt.timedelta(minutes=5, seconds=30)),
        # hours minutes seconds
        ("10:15:30", pydt.timedelta(hours=10, minutes=15, seconds=30)),
        ("01:15:30", pydt.timedelta(hours=1, minutes=15, seconds=30)),
        # ('100:200:300', pydt.timedelta(hours=100, minutes=200, seconds=300)),
        # days
        # ('4d,00:15:30', pydt.timedelta(days=4, minutes=15, seconds=30)),
        # ('4d,10:15:30', pydt.timedelta(days=4, hours=10, minutes=15, seconds=30)),
        # fractions of seconds
        ("00:15:30.1", pydt.timedelta(minutes=15, seconds=30, milliseconds=100)),
        ("00:15:30.01", pydt.timedelta(minutes=15, seconds=30, milliseconds=10)),
        ("00:15:30.001", pydt.timedelta(minutes=15, seconds=30, milliseconds=1)),
        ("00:15:30.0001", pydt.timedelta(minutes=15, seconds=30, microseconds=100)),
        ("00:15:30.00001", pydt.timedelta(minutes=15, seconds=30, microseconds=10)),
        ("00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
        (b"00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
        # negative
        # ('-4d,00:15:30', pydt.timedelta(days=-4, minutes=-15, seconds=-30)),
        (-172800, pydt.timedelta(days=-2)),
        ("-00:15:30", pydt.timedelta(minutes=-15, seconds=-30)),
        ("-01:15:30", pydt.timedelta(hours=-1, minutes=-15, seconds=-30)),
        (-30.1, pydt.timedelta(seconds=-30, milliseconds=-100)),
        # iso_8601
        # ('30', Err('Input should be a valid timedelta, "day" identifier')),
        # ('P4Y', pydt.timedelta(days=1460)),
        # ('P4M', pydt.timedelta(days=120)),
        # ('P4W', pydt.timedelta(days=28)),
        # ('P4D', pydt.timedelta(days=4)),
        # ('P0.5D', pydt.timedelta(hours=12)),
        # ('PT5H', pydt.timedelta(hours=5)),
        # ('PT5M', pydt.timedelta(minutes=5)),
        # ('PT5S', pydt.timedelta(seconds=5)),
        # ('PT0.000005S', pydt.timedelta(microseconds=5)),
        # (b'PT0.000005S', pydt.timedelta(microseconds=5)),
    ],
)
def test_parse_signed_duration(value, result):
    if isinstance(value, float | int):
        with pytest.raises(NotImplementedError):
            _d = SignedDurationModel(d=value)
    else:
        assert SignedDurationModel(d=value).d == result


@pytest.mark.parametrize(
    "value,result",
    [
        # seconds
        (pydt.timedelta(seconds=30), pydt.timedelta(seconds=30)),
        # (30, pydt.timedelta(seconds=30)),
        # (30.1, pydt.timedelta(seconds=30, milliseconds=100)),
        # (9.9e-05, pydt.timedelta(microseconds=99)),
        # minutes seconds
        ("00:15:30", pydt.timedelta(minutes=15, seconds=30)),
        ("00:05:30", pydt.timedelta(minutes=5, seconds=30)),
        # hours minutes seconds
        ("10:15:30", pydt.timedelta(hours=10, minutes=15, seconds=30)),
        ("01:15:30", pydt.timedelta(hours=1, minutes=15, seconds=30)),
        ("100:200:300", pydt.timedelta(hours=100, minutes=200, seconds=300)),
        # days
        # ('4d,00:15:30', pydt.timedelta(days=4, minutes=15, seconds=30)),
        # ('4d,10:15:30', pydt.timedelta(days=4, hours=10, minutes=15, seconds=30)),
        # fractions of seconds
        ("00:15:30.1", pydt.timedelta(minutes=15, seconds=30, milliseconds=100)),
        ("00:15:30.01", pydt.timedelta(minutes=15, seconds=30, milliseconds=10)),
        ("00:15:30.001", pydt.timedelta(minutes=15, seconds=30, milliseconds=1)),
        ("00:15:30.0001", pydt.timedelta(minutes=15, seconds=30, microseconds=100)),
        ("00:15:30.00001", pydt.timedelta(minutes=15, seconds=30, microseconds=10)),
        ("00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
        (b"00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
        # negative
        # ('-4d,00:15:30', pydt.timedelta(days=-4, minutes=-15, seconds=-30)),
        (-172800, pydt.timedelta(days=-2)),
        ("-00:15:30", pydt.timedelta(minutes=-15, seconds=-30)),
        ("-01:15:30", pydt.timedelta(hours=-1, minutes=-15, seconds=-30)),
        (-30.1, pydt.timedelta(seconds=-30, milliseconds=-100)),
        # iso_8601
        # ('30', Err('Input should be a valid timedelta, "day" identifier')),
        # ('P4Y', pydt.timedelta(days=1460)),
        # ('P4M', pydt.timedelta(days=120)),
        ("P4W", pydt.timedelta(days=28)),
        ("P4D", pydt.timedelta(days=4)),
        # ('P0.5D', pydt.timedelta(hours=12)),
        ("PT5H", pydt.timedelta(hours=5)),
        ("PT5M", pydt.timedelta(minutes=5)),
        ("PT5S", pydt.timedelta(seconds=5)),
        ("PT0.000005S", pydt.timedelta(microseconds=5)),
        (b"PT0.000005S", pydt.timedelta(microseconds=5)),
    ],
)
def test_parse_timespan(value, result):
    if isinstance(value, float | int):
        with pytest.raises(NotImplementedError):
            _d = TimeSpanModel(d=value)
    else:
        expected_span = ry.TimeSpan.from_pytimedelta(result)

        # assert TimeSpanModel(d=value).d == expected_span
        assert TimeSpanModel(d=value).d.to_pytimedelta() == result
