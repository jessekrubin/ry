from __future__ import annotations

import pytest

import ry

_SUPPORTS_SERIES = [
    # date
    ry.date(2020, 8, 26),
    # time
    ry.time(6, 27, 0, 0),
    # datetime
    ry.datetime(2020, 8, 26, 6, 27, 0, 0),
    # timestamp
    ry.Timestamp.from_millisecond(1598438400000),
]


@pytest.mark.parametrize("obj", _SUPPORTS_SERIES)
def test_series(obj: ry.Date | ry.DateTime | ry.Timestamp) -> None:
    series = obj.series(ry.timespan(days=1, minutes=30, hours=6))
    value = series.take(10)
    assert all(isinstance(v, type(obj)) for v in value)


@pytest.mark.parametrize("obj", _SUPPORTS_SERIES)
def test_series_errors(obj: ry.Date | ry.DateTime | ry.Timestamp) -> None:
    with pytest.raises(ValueError):
        obj.series(ry.timespan())
