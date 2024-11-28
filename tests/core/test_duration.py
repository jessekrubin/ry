from __future__ import annotations

import datetime as pydt

import ry


def test_duration_from_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.Duration.from_pytimedelta(pydelta)

    assert pydelta.days == 1
    assert pydelta.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert pydelta.microseconds == 5
    assert ryduration.days == 1
    assert ryduration.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert ryduration.microseconds == 5


def test_duration_2_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.Duration.from_pytimedelta(pydelta)

    roundtrip = ryduration.to_pytimedelta()
    assert isinstance(roundtrip, pydt.timedelta)
    assert roundtrip == pydelta


def test_equality() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.Duration.from_pytimedelta(pydelta)

    pydelta2 = pydt.timedelta(days=0, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration2 = ry.Duration.from_pytimedelta(pydelta2)
    assert ryduration != ryduration2
    assert ryduration == ryduration
    assert ryduration2 == ryduration2
    assert ryduration2 != ryduration

    assert ryduration == pydelta
    assert pydelta == ryduration
    assert ryduration2 == pydelta2
    assert pydelta2 == ryduration2

    assert ryduration != pydelta2
    assert pydelta2 != ryduration
    assert ryduration2 != pydelta
    assert pydelta != ryduration2
