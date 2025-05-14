from __future__ import annotations

import pickle
import typing as t

import pytest

import ry

RY_JIFF_OBJECTS = [
    # date
    ry.date(2020, 8, 26),
    # time
    ry.time(6, 27, 0, 0),
    # datetime
    ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    # span
    ry.timespan(weeks=1),
    # timestamp
    ry.Timestamp.from_millisecond(1598438400000),
    # Zoned
    ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
    # signed-duration
    ry.SignedDuration(1, 1),
    # offset
    ry.Offset(1),
    # iso-week-date
    ry.date(2020, 8, 26).iso_week_date(),
]

RY_OBJECTS = [
    # duration
    ry.Duration(1, 1),
    # fspath
    ry.FsPath.cwd(),
    # url
    ry.URL("https://example.com"),
    # bytes
    ry.Bytes(bytes(list(range(256)))),
    # http headers
    ry.Headers({"Content-Type": "application/json", "X-Request-ID": "123"}),
    # size
    ry.Size(10_000),
    # jiff objects
    *RY_JIFF_OBJECTS,
]


@pytest.mark.parametrize("obj", RY_OBJECTS)
def test_pickling(obj: t.Any) -> None:
    pickled = pickle.dumps(obj)
    loaded = pickle.loads(pickled)
    assert loaded == obj
