"""Test ry-pydantic integration

Many tests for datetime-et-al parsing are adapted from pydantic's datetime tests.

REF: https://github.com/pydantic/pydantic/blob/main/tests/test_datetime.py

"""

import datetime as pydt
from typing import Any

import pydantic
import pytest

import ry


# DATE MODELS
class PyDateModel(pydantic.BaseModel):
    date: pydt.date


class RyDateModel(pydantic.BaseModel):
    date: ry.Date


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


# DURATION MODELS
class PyTimedeltaModel(pydantic.BaseModel):
    d: pydt.timedelta


class RySignedDurationModel(pydantic.BaseModel):
    d: ry.SignedDuration


class RyTimeSpanModel(pydantic.BaseModel):
    d: ry.TimeSpan


class TestJsonSchemas:
    def _diff_schemas(self, left: dict[str, Any], right: dict[str, Any]) -> None:
        left_no_title = {k: v for k, v in left.items() if k != "title"}
        right_no_title = {k: v for k, v in right.items() if k != "title"}
        assert left_no_title == right_no_title

    def test_date_json_schema(self) -> None:
        py_model = PyDateModel.model_json_schema()
        ry_model = RyDateModel.model_json_schema()
        self._diff_schemas(py_model, ry_model)

    def test_time_model_schema(self) -> None:
        py_schema = PyTimeModel.model_json_schema()
        ry_schema = RyTimeModel.model_json_schema()
        self._diff_schemas(py_schema, ry_schema)

    def test_datetime_model_schema(self) -> None:
        py_schema = PyDatetimeModel.model_json_schema()
        ry_schema = RyDatetimeModel.model_json_schema()
        self._diff_schemas(py_schema, ry_schema)

    def test_signed_duration_model_schema(self) -> None:
        self._diff_schemas(
            RySignedDurationModel.model_json_schema(),
            PyTimedeltaModel.model_json_schema(),
        )

    def test_span_model_schema(self) -> None:
        self._diff_schemas(
            RyTimeSpanModel.model_json_schema(),
            PyTimedeltaModel.model_json_schema(),
        )


def create_tz(minutes: int) -> pydt.tzinfo:
    return pydt.timezone(pydt.timedelta(minutes=minutes))


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
            ("11:05:00-05:30", pydt.time(11, 5, 0, tzinfo=create_tz(-330))),
            ("11:05:00-0530", pydt.time(11, 5, 0, tzinfo=create_tz(-330))),
            ("11:05:00+00:00", pydt.time(11, 5, 0, tzinfo=pydt.UTC)),
            ("11:05-06:00", pydt.time(11, 5, 0, tzinfo=create_tz(-360))),
            ("11:05+06:00", pydt.time(11, 5, 0, tzinfo=create_tz(360))),
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
        assert m.d == result
        as_json = m.model_dump_json()
        from_json = m.model_validate_json(as_json)
        assert from_json.d == result

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
        ],
    )
    def test_parse_signed_duration_err(self, value: Any) -> None:
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
