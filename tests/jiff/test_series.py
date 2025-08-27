from __future__ import annotations

import pytest

import ry

_SUPPORTS_SERIES = [
    # date
    (ry.date(2020, 8, 26), ry.timespan(days=1)),
    # time
    (ry.time(6, 27, 0, 0), ry.timespan(hours=1)),
    # datetime
    (ry.datetime(2020, 8, 26, 6, 27, 0, 0), ry.timespan(days=1)),
    # timestamp
    (ry.Timestamp.from_millisecond(1598438400000), ry.timespan(seconds=1)),
]


@pytest.mark.parametrize("obj_timespan", _SUPPORTS_SERIES)
def test_series(
    obj_timespan: tuple[ry.Date | ry.DateTime | ry.Timestamp, ry.TimeSpan],
) -> None:
    obj, period = obj_timespan
    series = obj.series(period)
    value = series.take(10)
    assert all(isinstance(v, type(obj)) for v in value)
    next_value = next(series)
    assert isinstance(next_value, type(obj))
    for ix, el in enumerate(series):
        assert isinstance(el, type(obj))
        if ix == 9:
            break


@pytest.mark.parametrize("obj", [obj for obj, _ in _SUPPORTS_SERIES])
def test_series_errors(obj: ry.Date | ry.DateTime | ry.Timestamp) -> None:
    with pytest.raises(ValueError):
        obj.series(ry.timespan())
