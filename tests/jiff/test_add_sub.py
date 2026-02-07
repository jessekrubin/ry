from __future__ import annotations

import pytest

import ry


@pytest.mark.parametrize(
    "value",
    [
        # time
        ry.time(14, 30, 0, 0),
        # date
        ry.date(2020, 8, 26),
        # time
        ry.time(6, 27, 0, 0),
        # datetime
        ry.datetime(2020, 8, 26, 6, 27, 0, 0),
        # timestamp
        ry.Timestamp.from_millisecond(1598438400000),
        # Zoned
        ry.datetime(2020, 8, 26, 6, 27, 0, 0).in_tz("America/New_York"),
        # signed-duration
        ry.Offset(seconds=90),
    ],
)
@pytest.mark.parametrize(
    "kw",
    [
        {"years": 1},
        {"months": 1},
        {"weeks": 1},
        {"days": 1},
        {"hours": 1},
        {"minutes": 1},
        {"seconds": 1},
        {"milliseconds": 1},
        {"microseconds": 1},
        {"nanoseconds": 1},
    ],
)
def test_add_sub(
    value: ry.Time | ry.DateTime | ry.Date | ry.Timestamp | ry.ZonedDateTime,
    kw: dict[str, int],
) -> None:
    if isinstance(value, (ry.Time, ry.Timestamp, ry.Offset)) and any(
        v in {"years", "months", "weeks", "days"} for v in kw
    ):
        with pytest.raises(TypeError, match="got an unexpected keyword argument"):
            _ = value.add(**kw)
        with pytest.raises(TypeError, match="got an unexpected keyword argument"):
            _ = value.sub(**kw)
        return

    assert isinstance(value.add(**kw), type(value))
    assert isinstance(value.sub(**kw), type(value))

    # test that cannot give both a duration and keywords
    timespan = ry.timespan(**kw)
    with pytest.raises(
        TypeError,
        match="add accepts either a span-like object or keyword units, not both",
    ):
        _ = value.add(timespan, **kw)
    with pytest.raises(
        TypeError,
        match="sub accepts either a span-like object or keyword units, not both",
    ):
        _ = value.sub(timespan, **kw)
