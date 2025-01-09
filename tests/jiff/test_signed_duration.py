from __future__ import annotations

import datetime as pydt

import ry


def test_duration_from_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    assert pydelta.days == 1
    assert pydelta.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert pydelta.microseconds == 5
    assert ryduration.days == 1
    assert ryduration.seconds == (2 * 60 * 60) + (3 * 60) + 4
    assert ryduration.microseconds == 5


def test_duration_2_pydelta() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    roundtrip = ryduration.to_pytimedelta()
    assert isinstance(roundtrip, pydt.timedelta)
    assert roundtrip == pydelta


def test_equality() -> None:
    pydelta = pydt.timedelta(days=1, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration = ry.SignedDuration.from_pytimedelta(pydelta)

    pydelta2 = pydt.timedelta(days=0, hours=2, minutes=3, seconds=4, microseconds=5)
    ryduration2 = ry.SignedDuration.from_pytimedelta(pydelta2)
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


class TestSignedDurationAbs:
    def test_signed_duration_abs(self) -> None:
        sd = ry.SignedDuration(1, -1_999_999_999)

        assert abs(sd) == ry.SignedDuration(0, 999_999_999)
        assert sd.abs() == ry.SignedDuration(0, 999_999_999)

    def test_unsigned_abs(self) -> None:
        sd = ry.SignedDuration.MIN

        dur = sd.unsigned_abs()
        assert dur == ry.Duration(secs=9223372036854775808, nanos=999999999)


class TestSignedDurationStrings:
    """use jiff::SignedDuration;

    let duration: SignedDuration = "PT2h30m".parse()?;
    assert_eq!(duration.to_string(), "PT2H30M");

    // Or use the "friendly" format by invoking the alternate:
    assert_eq!(format!("{duration:#}"), "2h 30m");

    // Parsing automatically supports both the ISO 8601 and "friendly" formats:
    let duration: SignedDuration = "2h 30m".parse()?;
    assert_eq!(duration, SignedDuration::new(2 * 60 * 60 + 30 * 60, 0));
    let duration: SignedDuration = "2 hours, 30 minutes".parse()?;
    assert_eq!(duration, SignedDuration::new(2 * 60 * 60 + 30 * 60, 0));"""

    def test_signed_duration_string(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.string() == "PT2H30M"

    def test_signed_duration_parse(self) -> None:
        sd = ry.SignedDuration.parse("PT2H30M")
        assert sd.string(human=True) == "2h 30m"
        assert sd.string(True) == "2h 30m"
