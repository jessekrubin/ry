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


class TestTimeSeries:
    def test_time_series_jiff_example(self) -> None:
        """
        use jiff::{civil::{Time, time}, ToSpan};

        let start = Time::MIN;
        let mut every_third_hour = vec![];
        for t in start.series(3.hours()) {
            every_third_hour.push(t);
        }
        assert_eq!(every_third_hour, vec![
            time(0, 0, 0, 0),
            time(3, 0, 0, 0),
            time(6, 0, 0, 0),
            time(9, 0, 0, 0),
            time(12, 0, 0, 0),
            time(15, 0, 0, 0),
            time(18, 0, 0, 0),
            time(21, 0, 0, 0),
        ]);
        """
        start = ry.Time.MIN
        tspan = ry.TimeSpan().hours(3)
        every_third_hour = list(start.series(tspan))
        expected = [
            ry.time(0, 0, 0, 0),
            ry.time(3, 0, 0, 0),
            ry.time(6, 0, 0, 0),
            ry.time(9, 0, 0, 0),
            ry.time(12, 0, 0, 0),
            ry.time(15, 0, 0, 0),
            ry.time(18, 0, 0, 0),
            ry.time(21, 0, 0, 0),
        ]
        assert every_third_hour == expected

    def test_time_series_jiff_example_go_back_every_6p5_hours(self) -> None:
        """
        use jiff::{civil::{Time, time}, ToSpan};

        let start = time(23, 0, 0, 0);
        let times: Vec<Time> = start.series(-6.hours().minutes(30)).collect();
        assert_eq!(times, vec![
            time(23, 0, 0, 0),
            time(16, 30, 0, 0),
            time(10, 0, 0, 0),
            time(3, 30, 0, 0),
        ]);
        """
        start = ry.time(23, 0, 0, 0)
        times = list(start.series(ry.TimeSpan().hours(-6).minutes(30)))
        expected = [
            ry.time(23, 0, 0, 0),
            ry.time(16, 30, 0, 0),
            ry.time(10, 0, 0, 0),
            ry.time(3, 30, 0, 0),
        ]
        assert times == expected

    def test_time_series_jiff_hash_unique(self) -> None:
        """test time series + hash via set"""
        start = ry.Time.MIN
        tspan = ry.TimeSpan().hours(3)
        every_third_hour = list(start.series(tspan))
        expected = [
            ry.time(0, 0, 0, 0),
            ry.time(3, 0, 0, 0),
            ry.time(6, 0, 0, 0),
            ry.time(9, 0, 0, 0),
            ry.time(12, 0, 0, 0),
            ry.time(15, 0, 0, 0),
            ry.time(18, 0, 0, 0),
            ry.time(21, 0, 0, 0),
        ]
        assert every_third_hour == expected

        every_third_hour_set = set()
        for t in start.series(tspan):
            every_third_hour_set.add(t)
        for t in start.series(tspan):  # do it again!
            every_third_hour_set.add(t)
        assert len(every_third_hour_set) == 8
        assert set(every_third_hour) == every_third_hour_set
