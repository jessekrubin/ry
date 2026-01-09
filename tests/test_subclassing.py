from __future__ import annotations

import typing as t

import pytest

import ry

_RY_TYPES: list[type] = [
    ry.AsyncFile,
    ry.AsyncFileReadStream,
    ry.BlockingClient,
    ry.Client,
    ry.Bytes,
    ry.Certificate,
    ry.CertificateRevocationList,
    ry.Cookie,
    ry.Date,
    ry.DateDifference,
    ry.DateTime,
    ry.DateTimeDifference,
    ry.DateTimeRound,
    ry.DirEntry,
    ry.Duration,
    ry.FileReadStream,
    ry.FileType,
    ry.FsPath,
    ry.Glob,
    ry.GlobSet,
    ry.Globster,
    ry.Headers,
    ry.HttpClient,
    ry.HttpStatus,
    ry.ISOWeekDate,
    ry.Identity,
    ry.Instant,
    ry.IpAddr,
    ry.Ipv4Addr,
    ry.Ipv6Addr,
    ry.Metadata,
    ry.Offset,
    ry.OffsetRound,
    ry.Pattern,
    ry.Proxy,
    ry.ReadDir,
    ry.Regex,
    ry.ReqwestError,
    ry.Response,
    ry.SignedDuration,
    ry.SignedDurationRound,
    ry.Size,
    ry.SizeFormatter,
    ry.SocketAddr,
    ry.SocketAddrV4,
    ry.SocketAddrV6,
    ry.SqlFormatter,
    ry.SqlfmtQueryParams,
    ry.Time,
    ry.TimeDifference,
    ry.TimeRound,
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
    ry.fnv1a,
    # submodules
    ry.ulid.ULID,
    ry.uuid.UUID,
    ry.xxhash.xxh3_64,
    ry.xxhash.xxh3_128,
    ry.xxhash.xxh32,
    ry.xxhash.xxh64,
]

_SUBCLASSING_OK: set[type] = {
    ry.Bytes,
}


def test_no_missing_types_from_ry_init(subtests: pytest.Subtests) -> None:
    """
    Test that all ry types are exported in the ry module.
    """
    ry_types = {
        getattr(ry, attr) for attr in dir(ry) if isinstance(getattr(ry, attr), type)
    }
    for ty in ry_types:
        with subtests.test(msg=f"Checking {ty.__name__}"):
            assert ty in _RY_TYPES, f"{ty.__name__} is missing from _RY_TYPES"


@pytest.mark.parametrize(
    "cls",
    [
        pytest.param(cls, id=cls.__name__)
        for cls in filter(lambda c: c not in _SUBCLASSING_OK, _RY_TYPES)
    ],
)
def test_subclassing_fails(cls: t.Any) -> None:
    """
    Test that all ry types are not subclassed.
    """
    with pytest.raises(TypeError):

        class _Subclass(cls):  # type: ignore[misc]
            ...
