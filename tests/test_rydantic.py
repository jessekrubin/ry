"""Test ry-pydantic integration

Many tests for datetime-et-al parsing are adapted from pydantic's datetime tests.

REF(s):
    datetime: https://github.com/pydantic/pydantic/blob/main/tests/test_datetime.py
    url: https://github.com/pydantic/pydantic/blob/main/tests/test_networks.py

"""

import datetime as pydt
import typing as t
import zoneinfo
from ipaddress import IPv4Address, IPv6Address

import pydantic
import pytest
from typing_extensions import TypedDict

import ry


# DATE MODELS
class PyDateModel(pydantic.BaseModel):
    date: pydt.date


class RyDateModel(pydantic.BaseModel):
    date: ry.Date


class RyIsoWeekDateModel(pydantic.BaseModel):
    iwd: ry.ISOWeekDate


# TIME MODELS
class PyTimeModel(pydantic.BaseModel):
    d: pydt.time


class RyTimeModel(pydantic.BaseModel):
    d: ry.Time


# DATETIME MODELS
class PyDatetimeModel(pydantic.BaseModel):
    dt: pydt.datetime


class RyDatetimeModel(pydantic.BaseModel):
    dt: ry.DateTime


class RyZonedDatetimeModel(pydantic.BaseModel):
    dt: ry.ZonedDateTime


# TIMESTAMP MODELS
class RyTimestampModel(pydantic.BaseModel):
    ts: ry.Timestamp


# TIMESTAMP MODELS
class RyOffsetModel(pydantic.BaseModel):
    off: ry.Offset


# DURATION MODELS
class PyTimedeltaModel(pydantic.BaseModel):
    d: pydt.timedelta


class RyDurationModel(pydantic.BaseModel):
    d: ry.Duration


class RySignedDurationModel(pydantic.BaseModel):
    d: ry.SignedDuration


class RyTimeSpanModel(pydantic.BaseModel):
    d: ry.TimeSpan


# URL MODELS
class PyUrlModel(pydantic.BaseModel):
    url: pydantic.AnyUrl


class RyUrlModel(pydantic.BaseModel):
    url: ry.URL


class _TestJsonSchemas(TypedDict):
    name: str
    ry_model: type[pydantic.BaseModel]
    py_model: type[pydantic.BaseModel]


_MODELS_SCHEMAS = [
    # STD
    _TestJsonSchemas(
        name="duration",
        ry_model=RySignedDurationModel,
        py_model=PyTimedeltaModel,
    ),
    # JIFF
    _TestJsonSchemas(
        name="date",
        ry_model=RyDateModel,
        py_model=PyDateModel,
    ),
    _TestJsonSchemas(
        name="time",
        ry_model=RyTimeModel,
        py_model=PyTimeModel,
    ),
    _TestJsonSchemas(
        name="datetime",
        ry_model=RyDatetimeModel,
        py_model=PyDatetimeModel,
    ),
    _TestJsonSchemas(
        name="signed_duration",
        ry_model=RySignedDurationModel,
        py_model=PyTimedeltaModel,
    ),
    _TestJsonSchemas(
        name="timespan",
        ry_model=RyTimeSpanModel,
        py_model=PyTimedeltaModel,
    ),
    # URL
    _TestJsonSchemas(
        name="url",
        ry_model=RyUrlModel,
        py_model=PyUrlModel,
    ),
]


class TestJsonSchemas:
    def _diff_schemas(self, left: dict[str, t.Any], right: dict[str, t.Any]) -> None:
        left_no_title = {k: v for k, v in left.items() if k != "title"}
        right_no_title = {k: v for k, v in right.items() if k != "title"}
        assert left_no_title == right_no_title

    @pytest.mark.parametrize("schema", _MODELS_SCHEMAS)
    def test_json_schemas(self, schema: _TestJsonSchemas) -> None:
        py_model = schema["py_model"].model_json_schema()
        ry_model = schema["ry_model"].model_json_schema()
        self._diff_schemas(py_model, ry_model)


def _create_tz(minutes: int) -> pydt.tzinfo:
    return pydt.timezone(pydt.timedelta(minutes=minutes))


class TestDuration:
    @pytest.mark.parametrize(
        "value,result",
        [
            # seconds
            (ry.Duration.MAX, None),
            (ry.Duration.MIN, pydt.timedelta(seconds=0)),
            (ry.Duration(30), pydt.timedelta(seconds=30)),
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
            ("100:200:300", pydt.timedelta(hours=100, minutes=200, seconds=300)),
            # days
            # fractions of seconds
            ("00:15:30.1", pydt.timedelta(minutes=15, seconds=30, milliseconds=100)),
            ("00:15:30.01", pydt.timedelta(minutes=15, seconds=30, milliseconds=10)),
            ("00:15:30.001", pydt.timedelta(minutes=15, seconds=30, milliseconds=1)),
            ("00:15:30.0001", pydt.timedelta(minutes=15, seconds=30, microseconds=100)),
            ("00:15:30.00001", pydt.timedelta(minutes=15, seconds=30, microseconds=10)),
            ("00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
            (
                b"00:15:30.000001",
                pydt.timedelta(minutes=15, seconds=30, microseconds=1),
            ),
            # iso_8601
            ("PT5H", pydt.timedelta(hours=5)),
            ("PT5M", pydt.timedelta(minutes=5)),
            ("PT5S", pydt.timedelta(seconds=5)),
            ("PT0.000005S", pydt.timedelta(microseconds=5)),
            (b"PT0.000005S", pydt.timedelta(microseconds=5)),
        ],
    )
    def test_parse_duration_ok(
        self,
        value: float | str | bytes | pydt.datetime,
        result: pydt.timedelta | None,
    ) -> None:
        m = RyDurationModel(d=value)  # type: ignore[arg-type]
        assert isinstance(m.d, ry.Duration)
        if result is not None:
            assert m.d.to_pytimedelta() == result
        as_json = m.model_dump_json()
        from_json = m.model_validate_json(as_json)
        if result is not None:
            assert from_json.d.to_pytimedelta() == result

        # check json same
        if value != "100:200:300" and result is not None:  # fails pydantic?
            ry_json = m.model_dump_json()
            tm = PyTimedeltaModel(d=result)
            tm_json = tm.model_dump_json()
            assert tm_json == ry_json

    @pytest.mark.parametrize(
        "value,result",
        [
            (-172800, pydt.timedelta(days=-2)),
            ("-00:15:30", pydt.timedelta(minutes=-15, seconds=-30)),
            ("-01:15:30", pydt.timedelta(hours=-1, minutes=-15, seconds=-30)),
            (-30.1, pydt.timedelta(seconds=-30, milliseconds=-100)),
        ],
    )
    def test_parse_duration_err_negative(
        self, value: float | str, result: pydt.timedelta
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _m = RyDurationModel(d=value)  # type: ignore[arg-type]

        # Convert negative values to positive for parsing to sanity check
        positive_value = (
            abs(value) if isinstance(value, (int, float)) else value.lstrip("-")
        )
        m = RyDurationModel(d=positive_value)  # type: ignore[arg-type]
        assert m.d.to_pytimedelta() == -result
        as_json = m.model_dump_json()
        from_json = m.model_validate_json(as_json)
        assert from_json.d.to_pytimedelta() == -result

    @pytest.mark.parametrize(
        "value",
        [
            "30",
            "4d,00:15:30",
            "4d,10:15:30",
            "-4d,00:15:30",
            "-4d,10:15:30",
            "P4Y",
            "P4M",
            "P4W",
            "P4D",
            "P0.5D",
            # numbers nan/inf
            float("inf"),
            float("-inf"),
            float("nan"),
            # totally insane value
            complex(1, 2),
        ],
    )
    def test_parse_duration_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _m = RyDurationModel(d=value)


class TestDate:
    @pytest.mark.parametrize(
        "data",
        [
            pydt.date(2020, 1, 1),
            pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.UTC),
            ry.Date(2020, 1, 1),
            ry.Date(2020, 1, 1).at(1, 2, 3, 4),
            ry.Date(2020, 1, 1).at(1, 2, 3, 4).in_tz("America/Los_Angeles"),
            "2020-01-01",
        ],
    )
    def test_date_inputs(self, data: pydt.date | pydt.datetime | ry.Date | str) -> None:
        ry_model = RyDateModel(date=data)  # type: ignore[arg-type]
        assert isinstance(ry_model.date, ry.Date)

        model_dumped_json = ry_model.model_dump_json()

        from_json = RyDateModel.model_validate_json(model_dumped_json)
        assert from_json == ry_model
        assert from_json.date == ry_model.date

    @pytest.mark.parametrize(
        "value,result",
        [
            ("2012-04-23", pydt.date(2012, 4, 23)),
            (b"2012-04-23", pydt.date(2012, 4, 23)),
            (pydt.date(2012, 4, 9), pydt.date(2012, 4, 9)),
            (pydt.datetime(2012, 4, 9, 0, 0), pydt.date(2012, 4, 9)),
        ],
    )
    def test_date_parsing_ok(
        self, value: pydt.date | pydt.datetime | ry.Date | str, result: pydt.date
    ) -> None:
        ry_date = ry.Date.from_pydate(result)
        m = RyDateModel(date=value)  # type: ignore[arg-type]
        assert m.date == ry_date
        assert m.date.to_pydate() == result

        as_json = m.model_dump_json()
        from_json = RyDateModel.model_validate_json(as_json)
        assert from_json.date == ry_date
        assert from_json.date.to_pydate() == result

    @pytest.mark.parametrize(
        "raw",
        [
            # TBD if we want to support int/float inputs
            0,
            1_493_942_400,
            1_493_942_400_000,
            "x20120423",
            "2012-04-56",
            19_999_958_400,
            20000044800,
            1_549_238_400,
            1_549_238_400_000,
            1_549_238_400_000_000,
            1_549_238_400_000_000_000,
            "infinity",
            float("inf"),
            int("1" + "0" * 100),
            1e1000,
            float("-infinity"),
            float("nan"),
        ],
    )
    def test_date_parsing_err(
        self,
        raw: pydt.date | pydt.datetime | ry.Date | str,
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyDateModel(date=raw)  # type: ignore[arg-type]


class TestISOWeekdateDate:
    @pytest.mark.parametrize(
        "data",
        [
            pydt.date(2020, 1, 1),
            pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.UTC),
            ry.Date(2020, 1, 1),
            ry.Date(2020, 1, 1).at(1, 2, 3, 4),
            ry.Date(2020, 1, 1).at(1, 2, 3, 4).in_tz("America/Los_Angeles"),
            "2020-01-01",
        ],
    )
    def test_date_inputs(self, data: pydt.date | pydt.datetime | ry.Date | str) -> None:
        ry_model = RyIsoWeekDateModel(iwd=data)  # type: ignore[arg-type]
        assert isinstance(ry_model.iwd, ry.ISOWeekDate)

        model_dumped_json = ry_model.model_dump_json()

        from_json = RyIsoWeekDateModel.model_validate_json(model_dumped_json)
        assert from_json == ry_model
        assert from_json.iwd == ry_model.iwd

    @pytest.mark.parametrize(
        "value,result",
        [
            ("2012-04-23", pydt.date(2012, 4, 23)),
            (b"2012-04-23", pydt.date(2012, 4, 23)),
            (pydt.date(2012, 4, 9), pydt.date(2012, 4, 9)),
            (pydt.datetime(2012, 4, 9, 0, 0), pydt.date(2012, 4, 9)),
        ],
    )
    def test_date_parsing_ok(
        self, value: pydt.date | pydt.datetime | ry.Date | str, result: pydt.date
    ) -> None:
        ry_date = ry.ISOWeekDate.from_pydate(result)
        m = RyIsoWeekDateModel(iwd=value)  # type: ignore[arg-type]
        assert m.iwd == ry_date
        assert m.iwd.to_pydate() == result

        as_json = m.model_dump_json()
        from_json = RyIsoWeekDateModel.model_validate_json(as_json)
        assert from_json.iwd == ry_date
        assert from_json.iwd.to_pydate() == result

    @pytest.mark.parametrize(
        "raw",
        [
            # TBD if we want to support int/float inputs
            0,
            1_493_942_400,
            1_493_942_400_000,
            "x20120423",
            "2012-04-56",
            19_999_958_400,
            20000044800,
            1_549_238_400,
            1_549_238_400_000,
            1_549_238_400_000_000,
            1_549_238_400_000_000_000,
            "infinity",
            float("inf"),
            int("1" + "0" * 100),
            1e1000,
            float("-infinity"),
            float("nan"),
        ],
    )
    def test_iwd_parsing_err(
        self,
        raw: pydt.date | pydt.datetime | ry.ISOWeekDate | str,
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyIsoWeekDateModel(iwd=raw)  # type: ignore[arg-type]


class TestTime:
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
            ("11:05:00-05:30", pydt.time(11, 5, 0, tzinfo=_create_tz(-330))),
            ("11:05:00-0530", pydt.time(11, 5, 0, tzinfo=_create_tz(-330))),
            ("11:05:00+00:00", pydt.time(11, 5, 0, tzinfo=pydt.UTC)),
            ("11:05-06:00", pydt.time(11, 5, 0, tzinfo=_create_tz(-360))),
            ("11:05+06:00", pydt.time(11, 5, 0, tzinfo=_create_tz(360))),
            ("11:05:00-25:00", pydt.time(11, 5, 0)),
        ],
    )
    def test_time_parsing_ok(
        self, value: float | ry.Time | str, result: pydt.time
    ) -> None:
        rs_time = ry.Time.from_pytime(result)
        m = RyTimeModel(d=value)  # type: ignore[arg-type]
        assert isinstance(m.d, ry.Time)
        assert m.d == rs_time
        assert m.d == rs_time
        if result.tzinfo is None:
            assert m.d.to_pytime() == result

        as_json = m.model_dump_json()
        from_json = RyTimeModel.model_validate_json(as_json)
        assert from_json.d == rs_time

    @pytest.mark.parametrize(
        "value",
        [
            # Invalid inputs
            "4:8:16",
            86400,
            "xxx",
            "091500",
            b"091500",
            "09:15:90",
            "11:05:00Y",
            "11:05:00Z11:05:00-05:30",
        ],
    )
    def test_time_parsing_err(
        self,
        value: float | ry.Time | str,
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyTimeModel(d=value)  # type: ignore[arg-type]

    @pytest.mark.parametrize(
        "value,result",
        [
            # NOT IMPLEMENTED
            (3610, pydt.time(1, 0, 10, tzinfo=pydt.UTC)),
            (3600.5, pydt.time(1, 0, 0, 500000, tzinfo=pydt.UTC)),
            (86400 - 1, pydt.time(23, 59, 59, tzinfo=pydt.UTC)),
        ],
    )
    def test_time_parsing_tbd(
        self, value: float | ry.Time | str, result: pydt.time
    ) -> None:
        assert isinstance(value, float | int)
        assert isinstance(result, pydt.time)
        pytest.skip("tbd")
        with pytest.raises(pydantic.ValidationError):
            _d = RyTimeModel(d=value)


class TestDatetime:
    @pytest.mark.parametrize(
        "value,result",
        [
            ("2012-04-23T09:15:00", pydt.datetime(2012, 4, 23, 9, 15)),
            (
                "2012-04-23T09:15:00Z",
                pydt.datetime(2012, 4, 23, 9, 15, 0, 0, tzinfo=pydt.UTC),
            ),
            (
                "2012-04-23T10:20:30.400+02:30",
                pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, _create_tz(150)),
            ),
            (
                "2012-04-23T10:20:30.400+02:00",
                pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, _create_tz(120)),
            ),
            (
                "2012-04-23T10:20:30.400-02:00",
                pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, _create_tz(-120)),
            ),
            (
                b"2012-04-23T10:20:30.400-02:00",
                pydt.datetime(2012, 4, 23, 10, 20, 30, 400_000, _create_tz(-120)),
            ),
            (pydt.datetime(2017, 5, 5), pydt.datetime(2017, 5, 5)),
            # --- ry-jiff-types ---
            # ry.DateTime
            (ry.date(2024, 1, 1).at(4, 8, 16), pydt.datetime(2024, 1, 1, 4, 8, 16)),
            # ry.ZonedDateTime
            (
                ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC"),
                pydt.datetime(2024, 1, 1, 4, 8, 16, tzinfo=pydt.UTC),
            ),
            # ry.Timestamp
            (
                ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC").timestamp(),
                pydt.datetime(2024, 1, 1, 4, 8, 16),
            ),
        ],
    )
    def test_datetime_parsing_ok(
        self, value: float | str | bytes | pydt.datetime, result: pydt.datetime
    ) -> None:
        result = result.replace(tzinfo=None)
        rs_expected = ry.DateTime.from_pydatetime(result)
        m = RyDatetimeModel(dt=value)  # type: ignore[arg-type]
        assert isinstance(m.dt, ry.DateTime)
        assert m.dt == rs_expected

        as_json = m.model_dump_json()
        from_json = RyDatetimeModel.model_validate_json(as_json)
        assert from_json.dt == rs_expected

    @pytest.mark.parametrize(
        "value",
        [
            0,
            # Numeric inputs as strings
            "1494012444.883309",
            "1494012444",
            b"1494012444",
            "1494012444000.883309",
            "-1494012444000.883309",
            19_999_999_999,
            20_000_000_001,
            # just after watershed
            1_549_316_052,
            1_549_316_052_104,
            # Invalid inputs
            1_549_316_052_104_324,
            1_549_316_052_104_324_096,
            float("inf"),
            float("-inf"),
            1e50,
            float("nan"),
        ],
    )
    def test_datetime_parsing_err(
        self,
        value: float | str | bytes | pydt.datetime,
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyDatetimeModel(dt=value)  # type: ignore[arg-type]


class TestZonedDatetime:
    @pytest.mark.parametrize(
        "value,result",
        [
            (
                "2012-04-23T09:15:00+00:00[UTC]",
                pydt.datetime(2012, 4, 23, 9, 15, tzinfo=pydt.UTC),
            ),
            (
                b"2012-04-23T09:15:00+00:00[UTC]",
                pydt.datetime(2012, 4, 23, 9, 15, tzinfo=pydt.UTC),
            ),
            (
                pydt.datetime(2012, 4, 23, 9, 15, tzinfo=pydt.UTC),
                pydt.datetime(2012, 4, 23, 9, 15, tzinfo=pydt.UTC),
            ),
            # --- ry-jiff-types ---
            # ry.ZonedDateTime
            (
                ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC"),
                pydt.datetime(2024, 1, 1, 4, 8, 16, tzinfo=pydt.UTC),
            ),
            # ry.Timestamp
            (
                ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC").timestamp(),
                pydt.datetime(2024, 1, 1, 4, 8, 16, tzinfo=pydt.UTC),
            ),
        ],
    )
    def test_zoned_parsing_ok(
        self, value: float | str | bytes | pydt.datetime, result: pydt.datetime
    ) -> None:
        rs_expected = ry.ZonedDateTime.from_pydatetime(result)
        assert isinstance(rs_expected, ry.ZonedDateTime)
        m = RyZonedDatetimeModel(dt=value)  # type: ignore[arg-type]
        assert isinstance(m.dt, ry.ZonedDateTime)
        assert m.dt == rs_expected

        as_json = m.model_dump_json()
        from_json = RyZonedDatetimeModel.model_validate_json(as_json)
        assert from_json.dt == rs_expected

    @pytest.mark.parametrize(
        "value",
        [
            1_494_012_444.883_309,
            1_494_012_444,
            1_494_012_444_000,
            19_999_999_999,
            20_000_000_001,
            "2012-04-23T09:15:00Z",
            "2012-04-23T10:20:30.400+02:30",
            "2012-04-23T10:20:30.400+02:00",
            "2012-04-23T10:20:30.400-02:00",
            float("inf"),
            float("-inf"),
            1e50,
            float("nan"),
        ],
    )
    def test_zoned_parsing_err(
        self, value: float | str | bytes | pydt.datetime
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyZonedDatetimeModel(dt=value)  # type: ignore[arg-type]


class TestSignedDuration:
    @pytest.mark.parametrize(
        "value,result",
        [
            # self
            (ry.SignedDuration(secs=30), pydt.timedelta(seconds=30)),
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
            ("100:200:300", pydt.timedelta(hours=100, minutes=200, seconds=300)),
            # days
            # fractions of seconds
            ("00:15:30.1", pydt.timedelta(minutes=15, seconds=30, milliseconds=100)),
            ("00:15:30.01", pydt.timedelta(minutes=15, seconds=30, milliseconds=10)),
            ("00:15:30.001", pydt.timedelta(minutes=15, seconds=30, milliseconds=1)),
            ("00:15:30.0001", pydt.timedelta(minutes=15, seconds=30, microseconds=100)),
            ("00:15:30.00001", pydt.timedelta(minutes=15, seconds=30, microseconds=10)),
            ("00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
            (
                b"00:15:30.000001",
                pydt.timedelta(minutes=15, seconds=30, microseconds=1),
            ),
            # negative
            (-172800, pydt.timedelta(days=-2)),
            ("-00:15:30", pydt.timedelta(minutes=-15, seconds=-30)),
            ("-01:15:30", pydt.timedelta(hours=-1, minutes=-15, seconds=-30)),
            (-30.1, pydt.timedelta(seconds=-30, milliseconds=-100)),
            # iso_8601
            ("PT5H", pydt.timedelta(hours=5)),
            ("PT5M", pydt.timedelta(minutes=5)),
            ("PT5S", pydt.timedelta(seconds=5)),
            ("PT0.000005S", pydt.timedelta(microseconds=5)),
            (b"PT0.000005S", pydt.timedelta(microseconds=5)),
        ],
    )
    def test_parse_signed_duration_ok(
        self, value: float | str | bytes | pydt.datetime, result: pydt.timedelta
    ) -> None:
        m = RySignedDurationModel(d=value)  # type: ignore[arg-type]
        assert m.d.to_py() == result
        as_json = m.model_dump_json()
        from_json = m.model_validate_json(as_json)
        assert from_json.d == m.d

    @pytest.mark.parametrize(
        "value",
        [
            "30",
            "4d,00:15:30",
            "4d,10:15:30",
            "-4d,00:15:30",
            "-4d,10:15:30",
            "P4Y",
            "P4M",
            "P4W",
            "P4D",
            "P0.5D",
            # bad val
            float("inf"),
            float("nan"),
            # wrong type
            complex(1, 2),  # hot damn too complex for signed duration
        ],
    )
    def test_parse_signed_duration_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _m = RySignedDurationModel(d=value)


class TestTimeSpan:
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
            ("100:200:300", pydt.timedelta(hours=100, minutes=200, seconds=300)),
            # days
            # fractions of seconds
            ("00:15:30.1", pydt.timedelta(minutes=15, seconds=30, milliseconds=100)),
            ("00:15:30.01", pydt.timedelta(minutes=15, seconds=30, milliseconds=10)),
            ("00:15:30.001", pydt.timedelta(minutes=15, seconds=30, milliseconds=1)),
            ("00:15:30.0001", pydt.timedelta(minutes=15, seconds=30, microseconds=100)),
            ("00:15:30.00001", pydt.timedelta(minutes=15, seconds=30, microseconds=10)),
            ("00:15:30.000001", pydt.timedelta(minutes=15, seconds=30, microseconds=1)),
            (
                b"00:15:30.000001",
                pydt.timedelta(minutes=15, seconds=30, microseconds=1),
            ),
            # negative
            (-172800, pydt.timedelta(days=-2)),
            ("-00:15:30", pydt.timedelta(minutes=-15, seconds=-30)),
            ("-01:15:30", pydt.timedelta(hours=-1, minutes=-15, seconds=-30)),
            (-30.1, pydt.timedelta(seconds=-30, milliseconds=-100)),
            # iso_8601
            ("P4W", pydt.timedelta(days=28)),
            ("P4D", pydt.timedelta(days=4)),
            ("PT5H", pydt.timedelta(hours=5)),
            ("PT5M", pydt.timedelta(minutes=5)),
            ("PT5S", pydt.timedelta(seconds=5)),
            ("PT0.000005S", pydt.timedelta(microseconds=5)),
            (b"PT0.000005S", pydt.timedelta(microseconds=5)),
        ],
    )
    def test_parse_timespan_ok(
        self, value: str | float, result: pydt.timedelta
    ) -> None:
        m = RyTimeSpanModel(d=value)  # type: ignore[arg-type]
        assert isinstance(m.d, ry.TimeSpan)
        assert (
            m.d.total_seconds() == ry.TimeSpan.from_pytimedelta(result).total_seconds()
        )
        as_json = m.model_dump_json()
        from_json = RyTimeSpanModel.model_validate_json(as_json)
        assert from_json.d.to_pytimedelta() == result

    @pytest.mark.parametrize(
        "value",
        [
            "4d,00:15:30",
            "4d,10:15:30",
            "-4d,00:15:30",
            "30",
            "P0.5D",
        ],
    )
    def test_parse_timespan_err(self, value: str) -> None:
        with pytest.raises(pydantic.ValidationError):
            _m = RyTimeSpanModel(d=value)  # type: ignore[arg-type]

    @pytest.mark.parametrize(
        "value",
        [
            "P4Y",
            "P4M",
            # "P0.5D", possible burnt sushi bug?
        ],
    )
    def test_parse_timespan_special(self, value: str) -> None:
        m = RyTimeSpanModel(d=value)  # type: ignore[arg-type]
        assert isinstance(m.d, ry.TimeSpan)
        as_json = m.model_dump_json()
        from_json = RyTimeSpanModel.model_validate_json(as_json)
        assert isinstance(from_json.d, ry.TimeSpan)


_TIMESTAMP_OK = [
    # ry.DateTime
    ry.date(2024, 1, 1).at(4, 8, 16),
    # ry.ZonedDateTime
    ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC"),
    ry.date(2024, 1, 1).at(4, 8, 16).in_tz("America/Los_Angeles"),
    # pydt.datetime with tzinfo
    ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC").to_pydatetime(),
    ry.date(2024, 1, 1).at(4, 8, 16).in_tz("America/Los_Angeles").to_pydatetime(),
    # ry.Timestamp
    ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC").timestamp(),
    # str version of the above...
    "2024-01-01T04:08:16+00:00[UTC]",
    "2024-01-01T04:08:16-08:00[America/Los_Angeles]",
    "2024-01-01 04:08:16+00:00",
    "2024-01-01 04:08:16-08:00",
    "2024-01-01T04:08:16Z",
    # bytes version of the above...
    b"2024-01-01T04:08:16+00:00[UTC]",
    b"2024-01-01T04:08:16-08:00[America/Los_Angeles]",
    b"2024-01-01 04:08:16+00:00",
    b"2024-01-01 04:08:16-08:00",
    b"2024-01-01T04:08:16Z",
]


class TestTimestamp:
    @pytest.mark.parametrize(
        "value",
        _TIMESTAMP_OK,
    )
    def test_timestamp_parsing_ok(
        self, value: ry.DateTime | ry.ZonedDateTime | ry.Timestamp
    ) -> None:
        m = RyTimestampModel(ts=value)  # type: ignore[arg-type]
        assert isinstance(m.ts, ry.Timestamp)

        as_json = m.model_dump_json(
            indent=2,
        )
        from_json = RyTimestampModel.model_validate_json(as_json)
        assert from_json.ts == m.ts

    @pytest.mark.parametrize(
        "value",
        [
            # Invalid inputs
            "2024-01-01T04:08:16",  # no tzinfo
            "dickbutt",
            # totally wrong type
            complex(1, 2),
        ],
    )
    def test_timestamp_parsing_err(
        self,
        value: str,
    ) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RyTimestampModel(ts=value)  # type: ignore[arg-type]


# ---------------------------------------------------------------------------
# URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL ~ URL
# ---------------------------------------------------------------------------

_OK_URLS = [
    "http://example.org",
    "http://test",
    "http://localhost",
    "https://example.org/whatever/next/",
    "postgres://user:pass@localhost:5432/app",
    "postgres://just-user@localhost:5432/app",
    "postgresql+asyncpg://user:pass@localhost:5432/app",
    "postgresql+pg8000://user:pass@localhost:5432/app",
    "postgresql+psycopg://postgres:postgres@localhost:5432/hatch",
    "postgresql+psycopg2://postgres:postgres@localhost:5432/hatch",
    "postgresql+psycopg2cffi://user:pass@localhost:5432/app",
    "postgresql+py-postgresql://user:pass@localhost:5432/app",
    "postgresql+pygresql://user:pass@localhost:5432/app",
    "mysql://user:pass@localhost:3306/app",
    "mysql+mysqlconnector://user:pass@localhost:3306/app",
    "mysql+aiomysql://user:pass@localhost:3306/app",
    "mysql+asyncmy://user:pass@localhost:3306/app",
    "mysql+mysqldb://user:pass@localhost:3306/app",
    "mysql+pymysql://user:pass@localhost:3306/app?charset=utf8mb4",
    "mysql+cymysql://user:pass@localhost:3306/app",
    "mysql+pyodbc://user:pass@localhost:3306/app",
    "mariadb://user:pass@localhost:3306/app",
    "mariadb+mariadbconnector://user:pass@localhost:3306/app",
    "mariadb+pymysql://user:pass@localhost:3306/app",
    "snowflake://user:pass@myorganization-myaccount",
    "snowflake://user:pass@myorganization-myaccount/testdb/public?warehouse=testwh&role=myrole",
    "foo-bar://example.org",
    "foo.bar://example.org",
    "foo0bar://example.org",
    "https://example.org",
    "http://localhost",
    "http://localhost/",
    "http://localhost:8000",
    "http://localhost:8000/",
    "https://foo_bar.example.com/",
    "ftp://example.org",
    "ftps://example.org",
    "http://example.co.jp",
    "http://www.example.com/a%C2%B1b",
    "http://www.example.com/~username/",
    "http://info.example.com?fred",
    "http://info.example.com/?fred",
    "http://xn--mgbh0fb.xn--kgbechtv/",
    "http://example.com/blue/red%3Fand+green",
    "http://www.example.com/?array%5Bkey%5D=value",
    "http://xn--rsum-bpad.example.org/",
    "http://123.45.67.8/",
    "http://123.45.67.8:8329/",
    "http://[2001:db8::ff00:42]:8329",
    "http://[2001::1]:8329",
    "http://[2001:db8::1]/",
    "http://www.example.com:8000/foo",
    "http://www.cwi.nl:80/%7Eguido/Python.html",
    "https://www.python.org/путь",
    "http://андрей@example.com",
    "https://exam_ple.com/",
    "http://twitter.com/@handle/",
    "http://11.11.11.11.example.com/action",
    "http://abc.11.11.11.11.example.com/action",
    "http://example#",
    "http://example/#",
    "http://example/#fragment",
    "http://example/?#",
    "http://example.org/path#",
    "http://example.org/path#fragment",
    "http://example.org/path?query#",
    "http://example.org/path?query#fragment",
]


@pytest.mark.parametrize(
    "val",
    [
        *_OK_URLS,
        *(b.encode("utf-8") for b in _OK_URLS),
    ],
)
def test_url_parsing_ok(val: str | bytes) -> None:
    model_obj = RyUrlModel(url=val)  # type: ignore[arg-type]
    url_obj = ry.URL.parse(val)
    assert isinstance(model_obj.url, ry.URL)
    assert model_obj.url == url_obj
    model_dumped_json = model_obj.model_dump_json()
    from_json = RyUrlModel.model_validate_json(model_dumped_json)
    assert from_json == model_obj
    assert from_json.url == model_obj.url

    model_obj_from_url = RyUrlModel(url=url_obj)
    assert model_obj_from_url == model_obj


@pytest.mark.parametrize(
    "value,err_msg",
    [
        ("http:///", "empty host"),
        ("http://??", "empty host"),
        ("$https://example.org", "relative URL without a base"),
        ("../icons/logo.gif", "relative URL without a base"),
        ("abc", "relative URL without a base"),
        ("..", "relative URL without a base"),
        ("/", "relative URL without a base"),
        ("+http://example.com/", "relative URL without a base"),
        ("ht*tp://example.com/", "relative URL without a base"),
        (" ", "relative URL without a base"),
        ("", "input is empty"),
        ("http://2001:db8::ff00:42:8329", "invalid port number"),
        ("http://[192.168.1.1]:8329", "invalid IPv6 address"),
        ("http://example.com:99999", "invalid port number"),
        # TODO: figure out how to make validation errors more specific (eg `type_error`)
        # TODO: Mimic Pydantic error message
        #       The Pydantic error looks like: 'Input should be a valid URL, invalid port number',
        (None, "Expected str or bytes or URL object"),
    ],
)
def test_url_parsing_err(value: str | None, err_msg: str) -> None:
    with pytest.raises(pydantic.ValidationError) as exc_info:
        RyUrlModel(url=value)  # type: ignore[arg-type]
    assert len(exc_info.value.errors(include_url=False)) == 1, exc_info.value.errors(
        include_url=False
    )
    error = exc_info.value.errors(include_url=False)[0]
    assert error["type"] == "value_error"
    assert err_msg in error["msg"]


# =============================================================================
class FutureThingsMaybe:
    @staticmethod
    def future_config_support_maybe() -> list[dict[str, t.Any]]:
        _KWARG_OPTIONS = {  # noqa: N806
            "ser_json_temporal": ["iso8601", "seconds", "milliseconds"],
            "ser_json_timedelta": ["iso8601", "float"],
        }

        _CONFIG_COMBOS = [  # noqa: N806
            {},
            *({"ser_json_temporal": v} for v in _KWARG_OPTIONS["ser_json_temporal"]),
            *({"ser_json_timedelta": v} for v in _KWARG_OPTIONS["ser_json_timedelta"]),
            *(
                {"ser_json_temporal": vt, "ser_json_timedelta": vd}
                for vt in _KWARG_OPTIONS["ser_json_temporal"]
                for vd in _KWARG_OPTIONS["ser_json_timedelta"]
            ),
        ]
        return _CONFIG_COMBOS


# ============================================================================
# IP Address ~ IP Address ~ IP Address ~ IP Address ~ IP Address ~ IP Address
# ---------------------------------------------------------------------------
# adapted from pydantic tests.
# REF: https://github.com/pydantic/pydantic/blob/main/tests/test_networks_ipaddress.py
# ============================================================================


class PyIpv4Addr(pydantic.BaseModel):
    ip: IPv4Address


class PyIpv6Addr(pydantic.BaseModel):
    ip: IPv6Address


class PyIpAddr(pydantic.BaseModel):
    ip: pydantic.IPvAnyAddress


class RyIpAddr(pydantic.BaseModel):
    ip: ry.IpAddr


class RyIpv4Addr(pydantic.BaseModel):
    ip: ry.Ipv4Addr


class RyIpv6Addr(pydantic.BaseModel):
    ip: ry.Ipv6Addr


def test_ipv4addr_schemas() -> None:
    py_json_schema = PyIpv4Addr.model_json_schema()
    ry_json_schema = RyIpv4Addr.model_json_schema()
    assert py_json_schema == {
        "properties": {"ip": {"format": "ipv4", "title": "Ip", "type": "string"}},
        "required": ["ip"],
        "title": "PyIpv4Addr",
        "type": "object",
    }
    assert ry_json_schema == {
        "properties": {"ip": {"format": "ipv4", "title": "Ip", "type": "string"}},
        "required": ["ip"],
        "title": "RyIpv4Addr",
        "type": "object",
    }


def test_ipv6addr_schemas() -> None:
    py_json_schema = PyIpv6Addr.model_json_schema()
    ry_json_schema = RyIpv6Addr.model_json_schema()
    assert py_json_schema == {
        "properties": {"ip": {"format": "ipv6", "title": "Ip", "type": "string"}},
        "required": ["ip"],
        "title": "PyIpv6Addr",
        "type": "object",
    }
    assert ry_json_schema == {
        "properties": {"ip": {"format": "ipv6", "title": "Ip", "type": "string"}},
        "required": ["ip"],
        "title": "RyIpv6Addr",
        "type": "object",
    }


def test_ipaddr_schemas() -> None:
    py_json_schema = PyIpAddr.model_json_schema()
    ry_json_schema = RyIpAddr.model_json_schema()

    assert py_json_schema == {
        "properties": {
            "ip": {"format": "ipvanyaddress", "title": "Ip", "type": "string"}
        },
        "required": ["ip"],
        "title": "PyIpAddr",
        "type": "object",
    }
    assert ry_json_schema == {
        "properties": {
            "ip": {
                "anyOf": [
                    {"format": "ipv4", "type": "string"},
                    {"format": "ipv6", "type": "string"},
                ],
                "title": "Ip",
            }
        },
        "required": ["ip"],
        "title": "RyIpAddr",
        "type": "object",
    }


@pytest.mark.parametrize(
    "value,cls",
    [
        ("0.0.0.0", IPv4Address),  # noqa: S104
        ("1.1.1.1", IPv4Address),
        ("10.10.10.10", IPv4Address),
        ("192.168.0.1", IPv4Address),
        ("255.255.255.255", IPv4Address),
        ("::1:0:1", IPv6Address),
        ("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff", IPv6Address),
        (b"\x00\x00\x00\x00", IPv4Address),
        (b"\x01\x01\x01\x01", IPv4Address),
        (b"\n\n\n\n", IPv4Address),
        (b"\xc0\xa8\x00\x01", IPv4Address),
        (b"\xff\xff\xff\xff", IPv4Address),
        (
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01",
            IPv6Address,
        ),
        (
            b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff",
            IPv6Address,
        ),
        (0, IPv4Address),
        (16_843_009, IPv4Address),
        (168_430_090, IPv4Address),
        (3_232_235_521, IPv4Address),
        (4_294_967_295, IPv4Address),
        (4_294_967_297, IPv6Address),
        (340_282_366_920_938_463_463_374_607_431_768_211_455, IPv6Address),
        (IPv4Address("192.168.0.1"), IPv4Address),
        (IPv6Address("::1:0:1"), IPv6Address),
    ],
)
def test_ipaddress_success(
    value: str | bytes | int | IPv4Address | IPv6Address,
    cls: type[IPv4Address] | type[IPv6Address],
) -> None:
    assert PyIpAddr(ip=value).ip == cls(value)  # type: ignore[arg-type]
    assert RyIpAddr(ip=value).ip.to_pyipaddress() == cls(value)  # type: ignore[arg-type]


@pytest.mark.parametrize(
    "value",
    [
        "0.0.0.0",  # noqa: S104
        "1.1.1.1",
        "10.10.10.10",
        "192.168.0.1",
        "255.255.255.255",
        b"\x00\x00\x00\x00",
        b"\x01\x01\x01\x01",
        b"\n\n\n\n",
        b"\xc0\xa8\x00\x01",
        b"\xff\xff\xff\xff",
        0,
        16_843_009,
        168_430_090,
        3_232_235_521,
        4_294_967_295,
        IPv4Address("0.0.0.0"),  # noqa: S104
        IPv4Address("1.1.1.1"),
        IPv4Address("10.10.10.10"),
        IPv4Address("192.168.0.1"),
        IPv4Address("255.255.255.255"),
        ry.Ipv4Addr.from_str("0.0.0.0"),  # noqa: S104
        ry.Ipv4Addr.from_str("1.1.1.1"),
        ry.Ipv4Addr.from_str("10.10.10.10"),
        ry.Ipv4Addr.from_str("192.168.0.1"),
        ry.Ipv4Addr.from_str("255.255.255.255"),
    ],
)
def test_ipv4addr_ok(value: str | bytes | int | IPv4Address | ry.Ipv4Addr) -> None:
    assert PyIpv4Addr(ip=value).ip == IPv4Address(value)  # type: ignore[arg-type]
    assert RyIpv4Addr(ip=value).ip.to_py() == IPv4Address(value)  # type: ignore[arg-type]


@pytest.mark.parametrize(
    "value",
    [
        "::1:0:1",
        "ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff",
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01",
        b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff",
        4_294_967_297,
        340_282_366_920_938_463_463_374_607_431_768_211_455,
        IPv6Address("::1:0:1"),
        IPv6Address("ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff"),
    ],
)
def test_ipv6addr_ok(value: str | bytes | int | IPv6Address | ry.Ipv6Addr) -> None:
    assert PyIpv6Addr(ip=value).ip == IPv6Address(value)  # type: ignore[arg-type]
    assert RyIpv6Addr(ip=value).ip.to_py() == IPv6Address(value)  # type: ignore[arg-type]


@pytest.mark.parametrize("value", ["hello,world", "192.168.0.1.1.1", -1, 2**128 + 1])
def test_ipaddr_err(value: str | int) -> None:

    with pytest.raises(pydantic.ValidationError) as exc_info:
        RyIpAddr(ip=value)  # type: ignore[arg-type]
    assert exc_info.value.error_count() == 1


@pytest.mark.parametrize(
    "value", ["hello,world", "192.168.0.1.1.1", -1, 2**32 + 1, IPv6Address("::0:1:0")]
)
def test_ipv4addr_err(value: str | int | IPv6Address) -> None:
    with pytest.raises(pydantic.ValidationError) as exc_info:
        RyIpv4Addr(ip=value)  # type: ignore[arg-type]
    assert exc_info.value.error_count() == 1


@pytest.mark.parametrize(
    "value",
    ["hello,world", "192.168.0.1.1.1", -1, 2**128 + 1, IPv4Address("192.168.0.1")],
)
def test_ipv6addr_err(value: str | int | IPv4Address) -> None:
    with pytest.raises(pydantic.ValidationError) as exc_info:
        RyIpv6Addr(ip=value)  # type: ignore[arg-type]
    assert exc_info.value.error_count() == 1


class RySocketAddr(pydantic.BaseModel):
    sock: ry.SocketAddr


class RySocketAddrV4(pydantic.BaseModel):
    sock: ry.SocketAddrV4


class RySocketAddrV6(pydantic.BaseModel):
    sock: ry.SocketAddrV6


def test_socketaddr_schemas() -> None:
    ry_json_schema = RySocketAddr.model_json_schema()
    assert ry_json_schema == {
        "properties": {"sock": {"title": "Sock", "type": "string"}},
        "required": ["sock"],
        "title": "RySocketAddr",
        "type": "object",
    }

    ryv4_json_schema = RySocketAddrV4.model_json_schema()
    assert ryv4_json_schema == {
        "properties": {"sock": {"title": "Sock", "type": "string"}},
        "required": ["sock"],
        "title": "RySocketAddrV4",
        "type": "object",
    }

    ryv6_json_schema = RySocketAddrV6.model_json_schema()
    assert ryv6_json_schema == {
        "properties": {"sock": {"title": "Sock", "type": "string"}},
        "required": ["sock"],
        "title": "RySocketAddrV6",
        "type": "object",
    }


class TestSocketAddr:
    @pytest.mark.parametrize(
        "value,result",
        [
            ("192.168.0.1:8080", "192.168.0.1:8080"),
            (b"192.168.0.1:8080", "192.168.0.1:8080"),
            (ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080), "192.168.0.1:8080"),
            (ry.SocketAddr(ry.Ipv4Addr(192, 168, 0, 1), 8080), "192.168.0.1:8080"),
            (ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080), "[::1]:8080"),
        ],
    )
    def test_socketaddrv4_parsing_ok(
        self,
        value: str | bytes | ry.SocketAddr | ry.SocketAddrV4 | ry.SocketAddrV6,
        result: str,
    ) -> None:
        m = RySocketAddr(sock=value)  # type: ignore[arg-type]
        assert isinstance(m.sock, ry.SocketAddr)
        assert str(m.sock) == result

        as_json = m.model_dump_json()
        from_json = RySocketAddr.model_validate_json(as_json)
        assert str(from_json.sock) == result

    @pytest.mark.parametrize(
        "value",
        [
            complex(1, 2),
        ],
    )
    def test_socketaddrv4_parsing_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RySocketAddr(sock=value)


class TestSocketAddrV4:
    @pytest.mark.parametrize(
        "value,result",
        [
            ("192.168.0.1:8080", "192.168.0.1:8080"),
            (b"192.168.0.1:8080", "192.168.0.1:8080"),
            (ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080), "192.168.0.1:8080"),
            (
                ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080).to_socketaddr(),
                "192.168.0.1:8080",
            ),
        ],
    )
    def test_socketaddrv4_parsing_ok(
        self,
        value: str | bytes | ry.SocketAddr | ry.SocketAddrV4 | ry.SocketAddrV6,
        result: str,
    ) -> None:
        m = RySocketAddrV4(sock=value)  # type: ignore[arg-type]
        assert isinstance(m.sock, ry.SocketAddrV4)
        assert str(m.sock) == result

        as_json = m.model_dump_json()
        from_json = RySocketAddrV4.model_validate_json(as_json)
        assert str(from_json.sock) == result

    @pytest.mark.parametrize(
        "value",
        [
            complex(1, 2),
            ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080),
            ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080).to_socketaddr(),
        ],
    )
    def test_socketaddrv4_parsing_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RySocketAddrV4(sock=value)


class TestSocketAddrV6:
    @pytest.mark.parametrize(
        "value,result",
        [
            (ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080), "[::1]:8080"),
            (ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080).to_socketaddr(), "[::1]:8080"),
            ("[::1]:8080", "[::1]:8080"),
            (b"[::1]:8080", "[::1]:8080"),
        ],
    )
    def test_socketaddrv6_parsing_ok(
        self,
        value: str | bytes | ry.SocketAddr | ry.SocketAddrV4 | ry.SocketAddrV6,
        result: str,
    ) -> None:
        m = RySocketAddrV6(sock=value)  # type: ignore[arg-type]
        assert isinstance(m.sock, ry.SocketAddrV6)
        assert str(m.sock) == result

        as_json = m.model_dump_json()
        from_json = RySocketAddrV6.model_validate_json(as_json)
        assert str(from_json.sock) == result

    @pytest.mark.parametrize(
        "value",
        [
            complex(1, 2),
            ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080),
            ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080).to_socketaddr(),
            ry.SocketAddr(ry.Ipv4Addr(192, 168, 0, 1), 8080),
        ],
    )
    def test_socketaddrv6_parsing_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _d = RySocketAddrV6(sock=value)


class TestOffset:
    @pytest.mark.parametrize(
        "value,result",
        [
            # self
            (ry.Offset.UTC, pydt.timedelta(0)),
            (ry.Offset.MIN, pydt.timedelta(days=-2, seconds=79201)),
            (ry.Offset.MAX, pydt.timedelta(days=1, seconds=7199)),
            # strings/bytes
            ("+02:30", pydt.timedelta(hours=2, minutes=30)),
            ("-05:00", pydt.timedelta(hours=-5)),
            (b"+02:30", pydt.timedelta(hours=2, minutes=30)),
            (b"-05:00", pydt.timedelta(hours=-5)),
            # seconds
            (pydt.timedelta(seconds=30), pydt.timedelta(seconds=30)),
            (ry.SignedDuration(secs=30), pydt.timedelta(seconds=30)),
            (-pydt.timedelta(seconds=30), pydt.timedelta(seconds=-30)),
            (-ry.SignedDuration(secs=30), pydt.timedelta(seconds=-30)),
            # tzinfo
            (zoneinfo.ZoneInfo("UTC"), pydt.timedelta(0)),
        ],
    )
    def test_offset_parsing_ok(self, value: t.Any, result: pydt.timedelta) -> None:
        m = RyOffsetModel(off=value)
        assert m.off.to_pytimedelta() == result

        as_json = m.model_dump_json()
        from_json = RyOffsetModel.model_validate_json(as_json)
        assert from_json.off.to_pytimedelta() == result

    @pytest.mark.parametrize(
        "value",
        [
            # bad type
            complex(1, 2),
            # too big
            pydt.timedelta(seconds=93599 + 1),
            ry.SignedDuration(secs=93599 + 1),
            pydt.timedelta(seconds=-93599 - 1),
            ry.SignedDuration(secs=-93599 - 1),
        ],
    )
    def test_offset_parsing_err(self, value: t.Any) -> None:
        with pytest.raises(pydantic.ValidationError):
            _m = RyOffsetModel(off=value)
