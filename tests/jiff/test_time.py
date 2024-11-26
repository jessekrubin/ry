from typing_extensions import reveal_type

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
    # /// Returns a span representing the elapsed time from this time until
    # /// the given `other` time.
    # ///
    # /// When `other` is earlier than this time, the span returned will be
    # /// negative.
    # ///
    # /// Depending on the input provided, the span returned is rounded. It may
    # /// also be balanced down to smaller units than the default. By default,
    # /// the span returned is balanced such that the biggest possible unit is
    # /// hours.
    # ///
    # /// This operation is configured by providing a [`TimeDifference`]
    # /// value. Since this routine accepts anything that implements
    # /// `Into<TimeDifference>`, once can pass a `Time` directly. One
    # /// can also pass a `(Unit, Time)`, where `Unit` is treated as
    # /// [`TimeDifference::largest`].
    # ///
    # /// # Properties
    # ///
    # /// As long as no rounding is requested, it is guaranteed that adding the
    # /// span returned to the `other` time will always equal this time.
    # ///
    # /// # Errors
    # ///
    # /// An error can occur if `TimeDifference` is misconfigured. For example,
    # /// if the smallest unit provided is bigger than the largest unit, or if
    # /// the largest unit is bigger than [`Unit::Hour`].
    # ///
    # /// It is guaranteed that if one provides a time with the default
    # /// [`TimeDifference`] configuration, then this routine will never fail.
    # ///
    # /// # Examples
    # ///
    # /// ```
    # /// use jiff::{civil::time, ToSpan};
    # ///
    # /// let t1 = time(22, 35, 1, 0);
    # /// let t2 = time(22, 35, 3, 500_000_000);
    # /// assert_eq!(t1.until(t2)?, 2.seconds().milliseconds(500));
    # /// // Flipping the dates is fine, but you'll get a negative span.
    # /// assert_eq!(t2.until(t1)?, -2.seconds().milliseconds(500));
    # ///
    # /// # Ok::<(), Box<dyn std::error::Error>>(())
    # /// ```
    # ///
    # /// # Example: using smaller units
    # ///
    # /// This example shows how to contract the span returned to smaller units.
    # /// This makes use of a `From<(Unit, Time)> for TimeDifference`
    #     /// trait implementation.
    # ///
    # /// ```
    # /// use jiff::{civil::time, Unit, ToSpan};
    # ///
    # /// let t1 = time(3, 24, 30, 3500);
    # /// let t2 = time(15, 30, 0, 0);
    # ///
    # /// // The default limits spans to using "hours" as the biggest unit.
    # /// let span = t1.until(t2)?;
    # /// assert_eq!(span.to_string(), "PT12h5m29.9999965s");
    # ///
    # /// // But we can ask for smaller units, like capping the biggest unit
    # /// // to minutes instead of hours.
    # /// let span = t1.until((Unit::Minute, t2))?;
    # /// assert_eq!(span.to_string(), "PT725m29.9999965s");
    # ///
    # /// # Ok::<(), Box<dyn std::error::Error>>(())
    # /// ```
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
