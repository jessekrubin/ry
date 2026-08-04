import ry


def test_signed_duration_absolute() -> None:
    neg = ry.SignedDuration(-10)
    assert neg.abs() == ry.SignedDuration(10)
    assert abs(neg) == ry.SignedDuration(10)


def test_signed_duration_absolute_same_instance() -> None:
    pos = ry.SignedDuration(10)
    assert pos.abs() is pos
    assert abs(pos) is pos


def test_timespan_absolute() -> None:
    neg = ry.timespan(seconds=-10)
    assert neg.abs() == ry.timespan(seconds=10)
    assert abs(neg) == ry.timespan(seconds=10)
