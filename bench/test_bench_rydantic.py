from __future__ import annotations

import dataclasses
import datetime as pydt
import typing as t
import zoneinfo
from ipaddress import IPv4Address, IPv6Address
from typing import TYPE_CHECKING

import pydantic
import pytest

import ry

if TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture


@dataclasses.dataclass
class _RydanticBench:
    rytype: type
    py_inputs: list[t.Any]
    json_inputs: list[str] = dataclasses.field(default_factory=list)

    def type_adapter(self) -> pydantic.TypeAdapter:
        return pydantic.TypeAdapter(self.rytype)


_UTC_DATETIME = pydt.datetime(2024, 1, 1, 4, 8, 16, tzinfo=pydt.UTC)
_RY_DATE = ry.Date(2020, 1, 1)
_RY_DATETIME = ry.DateTime(2020, 1, 1, 0, 0, 0)
_RY_ZONED = ry.date(2024, 1, 1).at(4, 8, 16).in_tz("UTC")
_RY_TIMESTAMP = _RY_ZONED.timestamp()
_RY_IPV4 = ry.Ipv4Addr.from_str("192.168.0.1")
_RY_IPV6 = ry.Ipv6Addr("::1")
_RY_SOCKET_V4 = ry.SocketAddrV4(ry.Ipv4Addr(192, 168, 0, 1), 8080)
_RY_SOCKET_V6 = ry.SocketAddrV6(ry.Ipv6Addr("::1"), 8080)

_BENCHMARKS = [
    _RydanticBench(
        ry.Date,
        [
            "2020-01-01",
            b"2020-01-01",
            pydt.date(2020, 1, 1),
            pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.UTC),
            _RY_DATE,
            _RY_ZONED,
            _RY_DATETIME,
            _RY_TIMESTAMP,
        ],
        ['"2020-01-01"'],
    ),
    _RydanticBench(
        ry.ISOWeekDate,
        [
            "2020-01-01",
            b"2020-01-01",
            pydt.date(2020, 1, 1),
            pydt.datetime(2020, 1, 1, 12, 0, 0, tzinfo=pydt.UTC),
            _RY_DATE,
            _RY_ZONED,
            ry.ISOWeekDate.from_pydate(pydt.date(2020, 1, 1)),
        ],
        ['"2020-01-01"'],
    ),
    _RydanticBench(
        ry.Time,
        [
            "09:15:00",
            b"10:20:30.400",
            pydt.time(4, 8, 16),
            ry.Time(9, 15),
            ry.Time(10, 20, 30, 400_000_000),
            ry.date(2024, 1, 1).at(4, 8, 16),
            _RY_ZONED,
            _RY_TIMESTAMP,
        ],
        ['"09:15:00"', '"10:20:30.400"'],
    ),
    _RydanticBench(
        ry.DateTime,
        [
            "2012-04-23T09:15:00",
            b"2012-04-23T10:20:30.400-02:00",
            pydt.datetime(2017, 5, 5),
            ry.date(2024, 1, 1).at(4, 8, 16),
            _RY_ZONED,
            _RY_TIMESTAMP,
        ],
        ['"2012-04-23T09:15:00"', '"2012-04-23T10:20:30.400+02:00"'],
    ),
    _RydanticBench(
        ry.ZonedDateTime,
        [
            "2012-04-23T09:15:00+00:00[UTC]",
            b"2012-04-23T09:15:00+00:00[UTC]",
            _UTC_DATETIME,
            _RY_ZONED,
            _RY_TIMESTAMP,
        ],
        ['"2012-04-23T09:15:00+00:00[UTC]"'],
    ),
    _RydanticBench(
        ry.Timestamp,
        [
            _RY_TIMESTAMP,
            _RY_DATETIME,
            _RY_ZONED,
            "2024-01-01T04:08:16+00:00[UTC]",
            b"2024-01-01T04:08:16+00:00[UTC]",
        ],
        ['"2024-01-01T04:08:16+00:00[UTC]"'],
    ),
    _RydanticBench(
        ry.Offset,
        [
            ry.Offset.UTC,
            "+02:30",
            b"-05:00",
            pydt.timedelta(seconds=30),
            ry.SignedDuration(secs=30),
            -pydt.timedelta(seconds=30),
            -ry.SignedDuration(secs=30),
            zoneinfo.ZoneInfo("UTC"),
        ],
        ['"+02:30"', '"-05:00"'],
    ),
    _RydanticBench(
        ry.TimeZone,
        [
            "America/New_York",
            "UTC",
            "utc",
            b"America/Chicago",
            ry.TimeZone.get("America/Denver"),
            ry.TimeZone.UTC(),
            zoneinfo.ZoneInfo("America/Los_Angeles"),
        ],
        ['"UTC"', '"America/New_York"'],
    ),
    _RydanticBench(
        ry.Duration,
        [
            ry.Duration(30),
            pydt.timedelta(seconds=30),
            30,
            30.1,
            "00:15:30",
            "10:15:30",
            "00:15:30.000001",
            b"00:15:30.000001",
            "PT5H",
            b"PT0.000005S",
        ],
        ['"00:15:30"', '"PT5H"'],
    ),
    _RydanticBench(
        ry.SignedDuration,
        [
            ry.SignedDuration(secs=30),
            pydt.timedelta(seconds=30),
            30,
            30.1,
            "00:15:30",
            "-01:15:30",
            -172800,
            "PT5H",
            b"PT0.000005S",
        ],
        ['"00:15:30"', '"-01:15:30"', '"PT5H"'],
    ),
    _RydanticBench(
        ry.TimeSpan,
        [
            ry.TimeSpan(seconds=30),
            pydt.timedelta(seconds=30),
            30,
            30.1,
            "00:15:30",
            "-01:15:30",
            "P4W",
            "PT5H",
            b"PT0.000005S",
        ],
        ['"00:15:30"', '"P4W"', '"PT5H"'],
    ),
    _RydanticBench(
        ry.URL,
        [
            "http://example.org",
            b"https://example.org/whatever/next/",
            "postgres://user:pass@localhost:5432/app",
            ry.URL.parse("http://example.org"),
        ],
        ['"http://example.org"', '"postgres://user:pass@localhost:5432/app"'],
    ),
    _RydanticBench(
        ry.Ipv4Addr,
        [
            "192.168.0.1",
            b"\xc0\xa8\x00\x01",
            3_232_235_521,
            IPv4Address("192.168.0.1"),
            _RY_IPV4,
        ],
        ['"192.168.0.1"'],
    ),
    _RydanticBench(
        ry.Ipv6Addr,
        [
            "::1:0:1",
            b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x01",
            4_294_967_297,
            IPv6Address("::1:0:1"),
            _RY_IPV6,
        ],
        ['"::1:0:1"'],
    ),
    _RydanticBench(
        ry.IpAddr,
        [
            "192.168.0.1",
            "::1:0:1",
            IPv4Address("192.168.0.1"),
            IPv6Address("::1:0:1"),
            _RY_IPV4,
            _RY_IPV6,
        ],
        ['"192.168.0.1"', '"::1:0:1"'],
    ),
    _RydanticBench(
        ry.SocketAddr,
        [
            "192.168.0.1:8080",
            b"192.168.0.1:8080",
            _RY_SOCKET_V4,
            _RY_SOCKET_V4.to_socketaddr(),
            _RY_SOCKET_V6,
        ],
        ['"192.168.0.1:8080"', '"[::1]:8080"'],
    ),
    _RydanticBench(
        ry.SocketAddrV4,
        [
            "192.168.0.1:8080",
            b"192.168.0.1:8080",
            _RY_SOCKET_V4,
            _RY_SOCKET_V4.to_socketaddr(),
        ],
        ['"192.168.0.1:8080"'],
    ),
    _RydanticBench(
        ry.SocketAddrV6,
        [
            "[::1]:8080",
            b"[::1]:8080",
            _RY_SOCKET_V6,
            _RY_SOCKET_V6.to_socketaddr(),
        ],
        ['"[::1]:8080"'],
    ),
]


@pytest.mark.parametrize(
    ("rybench"),
    [pytest.param(b, id=f"ry.{b.rytype.__name__}") for b in _BENCHMARKS if b.py_inputs],
)
def test_rydantic_bench_python(
    benchmark: BenchmarkFixture,
    rybench: _RydanticBench,
) -> None:
    adapter = rybench.type_adapter()
    benchmark.group = f"pydantic-{rybench.rytype.__name__}-validate-python"

    def _fn():
        for value in rybench.py_inputs:
            adapter.validate_python(value)

    benchmark(_fn)


@pytest.mark.parametrize(
    ("rybench"),
    [
        pytest.param(b, id=f"ry.{b.rytype.__name__}")
        for b in _BENCHMARKS
        if b.json_inputs
    ],
)
def test_rydantic_bench_json(
    benchmark: BenchmarkFixture,
    rybench: _RydanticBench,
) -> None:
    adapter = rybench.type_adapter()
    benchmark.group = f"pydantic-{rybench.rytype.__name__}-validate-json"

    def _fn():
        for value in rybench.json_inputs:
            adapter.validate_json(value)

    benchmark(_fn)
