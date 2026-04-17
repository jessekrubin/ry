from __future__ import annotations

import typing as t

import pytest

import ry

_RY_TYPES: list[type] = [
    ry.AsyncFile,
    ry.AsyncFileReadStream,
    ry.BlockingClient,
    ry.Bytes,
    ry.Certificate,
    ry.CertificateRevocationList,
    ry.Client,
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
    ry.GlobPattern,
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
    ry.WebSocket,
    ry.WsMessage,
    ry.ZonedDateTime,
    ry.ZonedDateTimeDifference,
    ry.ZonedDateTimeRound,
    ry.fnv1a,
    ry.sha1,
    ry.sha224,
    ry.sha256,
    ry.sha384,
    ry.sha3_256,
    ry.sha3_384,
    ry.sha3_512,
    ry.sha512,
    ry.sha512_256,
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


def is_exception_subclass(cls: type) -> bool:
    """Check if a class is a subclass of Exception."""
    return issubclass(cls, BaseException)


def test_no_missing_types_from_ry_init(subtests: pytest.Subtests) -> None:
    """
    Test that all ry types are exported in the ry module.
    """
    ry_types = {
        getattr(ry, attr)
        for attr in dir(ry)
        if isinstance(getattr(ry, attr), type)
        and not is_exception_subclass(getattr(ry, attr))
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


@pytest.mark.parametrize(
    "cls",
    [
        pytest.param(cls, id=cls.__name__)
        for cls in filter(lambda c: hasattr(c, "__match_args__"), _RY_TYPES)
    ],
)
def test_match_args_defined(cls: type) -> None:
    """
    Test that all ry types with __match_args__ defined have the correct value.
    """
    if hasattr(cls, "__match_args__"):
        type_match_args = cls.__match_args__
        assert isinstance(type_match_args, tuple)
        assert all(isinstance(arg, str) for arg in type_match_args), (
            f"{cls.__name__}.__match_args__ must be a tuple of strings, "
            f"got {type_match_args}"
        )
