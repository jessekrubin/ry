from __future__ import annotations

import ry


def test_strptime() -> None:
    """
    ```rust
    use jiff::civil::Time;

    // Parse with a 12-hour clock.
    let time = Time::strptime("%I:%M%P", "4:30pm")?;
    assert_eq!(time.to_string(), "16:30:00");
    ```
    """

    t = ry.Time.strptime("%I:%M%P", "4:30pm")
    assert str(t) == "16:30:00"


def test_strftime() -> None:
    t = ry.time(16, 30, 59, 0)
    string = t.strftime("%-I:%M%P")
    assert str(string) == "4:30pm"


def test_time_until() -> None:
    t1 = ry.time(22, 35, 1, 0)
    t2 = ry.time(22, 35, 3, 500_000_000)
    assert t1.until(t2) == ry.timespan(seconds=2, milliseconds=500)
    assert t2.until(t1) == ry.timespan(seconds=-2, milliseconds=500)

    t1 = ry.time(3, 24, 30, 3500)
    t2 = ry.time(15, 30, 0, 0)
    span = t1.until(t2)
    assert span.string() == "PT12h5m29.9999965s"

    # span = t1.until((ry.JiffUnit.Minute, t2))
    # assert span.string() == "PT725m29.9999965s"
