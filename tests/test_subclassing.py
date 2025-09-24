from __future__ import annotations

import typing as t

import pytest

import ry

_RY_TYPES = [
    ry.AsyncFile,
    ry.Date,
    ry.DateDifference,
    ry.DateTime,
    ry.DateTimeDifference,
    ry.DateTimeRound,
    ry.Duration,
    ry.FileReadStream,
    ry.FileType,
    ry.FnvHasher,
    ry.FsPath,
    ry.Glob,
    ry.GlobSet,
    ry.Globster,
    ry.Headers,
    ry.HttpClient,
    ry.HttpStatus,
    ry.ISOWeekDate,
    ry.Instant,
    ry.IpAddr,
    ry.Ipv4Addr,
    ry.Ipv6Addr,
    ry.Metadata,
    ry.Offset,
    ry.Pattern,
    ry.Regex,
    ry.ReqwestError,
    ry.SignedDuration,
    ry.Size,
    ry.SizeFormatter,
    ry.SqlfmtQueryParams,
    ry.Time,
    ry.TimeDifference,
    ry.TimeSpan,
    ry.TimeZone,
    ry.TimeZoneDatabase,
    ry.Timestamp,
    ry.TimestampDifference,
    ry.TimestampRound,
    ry.URL,
    ry.WalkdirGen,
    ry.ZonedDateTime,
    ry.ZonedDateTimeDifference,
    ry.ZonedDateTimeRound,
    # submodules
    ry.ulid.ULID,
    ry.uuid.UUID,
    ry.xxhash.xxh3_64,
    ry.xxhash.xxh3_128,
    ry.xxhash.xxh32,
    ry.xxhash.xxh64,
]


@pytest.mark.parametrize(
    "cls", [pytest.param(cls, id=cls.__name__) for cls in _RY_TYPES]
)
def test_subclassing_fails(cls: t.Any) -> None:
    """
    Test that all ry types are not subclassed.
    """
    with pytest.raises(TypeError):

        class _Subclass(cls):  # type: ignore[misc]
            ...
