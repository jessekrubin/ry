from __future__ import annotations

import ry.dev as ry


def test_time_series_jiff_example() -> None:
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
    every_third_hour = []
    tspan = ry.Span().hours(3)
    for t in start.series(tspan):
        every_third_hour.append(t)
    assert every_third_hour == [
        ry.time(0, 0, 0, 0),
        ry.time(3, 0, 0, 0),
        ry.time(6, 0, 0, 0),
        ry.time(9, 0, 0, 0),
        ry.time(12, 0, 0, 0),
        ry.time(15, 0, 0, 0),
        ry.time(18, 0, 0, 0),
        ry.time(21, 0, 0, 0),
    ]


def test_time_series_jiff_example_go_back_every_6p5_hours() -> None:
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
    times = list(start.series(ry.Span().hours(-6).minutes(30)))
    assert times == [
        ry.time(23, 0, 0, 0),
        ry.time(16, 30, 0, 0),
        ry.time(10, 0, 0, 0),
        ry.time(3, 30, 0, 0),
    ]
