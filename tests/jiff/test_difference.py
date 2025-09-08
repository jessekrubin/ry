from __future__ import annotations

import ry


class TestDateDifference:
    def test_date_difference_docs_example(self) -> None:
        """
        use jiff::{civil::{Date, DateDifference}, RoundMode, ToSpan, Unit};

        let d1 = "2024-03-15".parse::<Date>()?;
        let d2 = "2030-09-13".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Year)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 6.years());

        // If the span were one day longer, it would round up to 7 years.
        let d2 = "2030-09-14".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Year)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 7.years());

        """

        d1 = ry.date(2024, 3, 15)
        d2 = ry.date(2030, 9, 13)
        diff = ry.DateDifference(d2)._smallest("year")._mode("half-expand")
        span = d1._until(diff)
        assert span == ry.timespan(years=6)

        d2 = ry.date(2030, 9, 14)
        span = d1._until(ry.DateDifference(d2)._smallest("year")._mode("half-expand"))
        assert span == ry.timespan(years=7)

    def test_date_until_smallest(self) -> None:
        """
        ```rust
        use jiff::{civil::{Date, DateDifference}, RoundMode, ToSpan, Unit};

        let d1 = "2024-03-15".parse::<Date>()?;
        let d2 = "2030-11-22".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Week)
                .largest(Unit::Week)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 349.weeks());
        ```
        """
        d1 = ry.date(2024, 3, 15)
        d2 = ry.date(2030, 11, 22)
        span = d1._until(
            ry.DateDifference(d2)
            ._smallest("week")
            ._largest("week")
            ._mode("half-expand")
        )
        assert span == ry.timespan(weeks=349)

    def test_date_until_mode(self) -> None:
        """
        ```rust
        use jiff::{civil::{Date, DateDifference}, RoundMode, ToSpan, Unit};

        let d1 = "2024-01-15".parse::<Date>()?;
        let d2 = "2024-08-16".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Month)
                .mode(RoundMode::Ceil),
        )?;
        // Only 7 months and 1 day elapsed, but we asked to always round up!
        assert_eq!(span, 8.months());

        // Since `Ceil` always rounds toward positive infinity, the behavior
        // flips for a negative span.
        let span = d1.since(
            DateDifference::new(d2)
                .smallest(Unit::Month)
                .mode(RoundMode::Ceil),
        )?;
        assert_eq!(span, -7.months());
        ```
        """
        d1 = ry.date(2024, 1, 15)
        d2 = ry.date(2024, 8, 16)
        span = d1._until(ry.DateDifference(d2)._smallest("month")._mode("ceil"))
        assert span == ry.timespan(months=8)

        span = d1._since(ry.DateDifference(d2)._smallest("month")._mode("ceil"))
        assert span == ry.timespan(months=-7)

    def test_date_difference_increment_docs_example(self) -> None:
        """
        ```rust
        use jiff::{civil::{Date, DateDifference}, RoundMode, ToSpan, Unit};

        let d1 = "2024-01-15".parse::<Date>()?;
        let d2 = "2024-08-15".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Month)
                .increment(2)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 8.months());

        // If our second date was just one day less, rounding would truncate
        // down to 6 months!
        let d2 = "2024-08-14".parse::<Date>()?;
        let span = d1.until(
            DateDifference::new(d2)
                .smallest(Unit::Month)
                .increment(2)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 6.months());
        ```
        """
        d1 = ry.date(2024, 1, 15)
        d2 = ry.date(2024, 8, 15)
        span = d1._until(
            ry.DateDifference(d2)._smallest("month")._increment(2)._mode("half-expand")
        )
        assert span == ry.timespan(months=8)

        d2 = ry.date(2024, 8, 14)
        span = d1._until(
            ry.DateDifference(d2)._smallest("month")._increment(2)._mode("half-expand")
        )
        assert span == ry.timespan(months=6)


class TestTimeDifference:
    def test_time_difference_docs_example(self) -> None:
        """
        ```rust
        use jiff::{civil::{Time, TimeDifference}, RoundMode, ToSpan, Unit};

        let t1 = "08:14:00.123456789".parse::<Time>()?;
        let t2 = "15:00".parse::<Time>()?;
        let span = t1.until(
            TimeDifference::new(t2)
                .smallest(Unit::Minute)
                .mode(RoundMode::HalfExpand)
                .increment(30),
        )?;
        assert_eq!(span, 7.hours());

        // One less minute, and because of the HalfExpand mode, the span would
        // get rounded down.
        let t2 = "14:59".parse::<Time>()?;
        let span = t1.until(
            TimeDifference::new(t2)
                .smallest(Unit::Minute)
                .mode(RoundMode::HalfExpand)
                .increment(30),
        )?;
        assert_eq!(span, 6.hours().minutes(30));
        ```
        """
        t1 = ry.time(8, 14, 0, 123456789)
        t2 = ry.time(15, 0, 0, 0)
        span = t1._until(
            ry.TimeDifference(t2)
            ._smallest("minute")
            ._mode("half-expand")
            ._increment(30)
        )
        assert span == ry.timespan(hours=7)

        t2 = ry.time(14, 59, 0, 0)
        span = t1._until(
            ry.TimeDifference(t2)
            ._smallest("minute")
            ._mode("half-expand")
            ._increment(30)
        )
        assert span == ry.timespan(hours=6, minutes=30)

    def test_time_difference_smallest(self) -> None:
        """
        ```rust
        use jiff::{civil::{Time, TimeDifference}, RoundMode, ToSpan, Unit};

        let t1 = "08:14:02.5001".parse::<Time>()?;
        let t2 = "08:30:03.0001".parse::<Time>()?;
        let span = t1.until(
            TimeDifference::new(t2)
                .smallest(Unit::Second)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 16.minutes().seconds(1));
        ```
        """
        t1 = ry.Time.parse("08:14:02.5001")
        t2 = ry.Time.parse("08:30:03.0001")
        span = t1._until(ry.TimeDifference(t2)._smallest("second")._mode("half-expand"))
        assert span == ry.timespan(minutes=16, seconds=1)

    def test_time_difference_mode(self) -> None:
        """
        ```rust
        use jiff::{civil::{Time, TimeDifference}, RoundMode, ToSpan, Unit};

        let t1 = "08:10".parse::<Time>()?;
        let t2 = "08:11".parse::<Time>()?;
        let span = t1.until(
            TimeDifference::new(t2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        // Only one minute elapsed, but we asked to always round up!
        assert_eq!(span, 1.hour());

        // Since `Ceil` always rounds toward positive infinity, the behavior
        // flips for a negative span.
        let span = t1.since(
            TimeDifference::new(t2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        assert_eq!(span, 0.hour());
        ```
        """
        t1 = ry.Time.parse("08:10")
        t2 = ry.Time.parse("08:11")
        span = t1._until(ry.TimeDifference(t2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=1)
        span = t1._since(ry.TimeDifference(t2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=0)


class TestDateTimeDifference:
    def test_datetime_difference_docs_example(self) -> None:
        """
        ```rust
        use jiff::{civil::{DateTime, DateTimeDifference}, RoundMode, ToSpan, Unit};

        let dt1 = "2024-03-15 08:14:00.123456789".parse::<DateTime>()?;
        let dt2 = "2030-03-22 15:00".parse::<DateTime>()?;
        let span = dt1.until(
            DateTimeDifference::new(dt2)
                .smallest(Unit::Minute)
                .largest(Unit::Year)
                .mode(RoundMode::HalfExpand)
                .increment(30),
        )?;
        assert_eq!(span, 6.years().days(7).hours(7));
        ```
        """
        dt1 = ry.DateTime.parse("2024-03-15 08:14:00.123456789")
        dt2 = ry.DateTime.parse("2030-03-22 15:00")
        span = dt1._until(
            ry.DateTimeDifference(dt2)
            ._smallest("minute")
            ._largest("year")
            ._mode("half-expand")
            ._increment(30)
        )
        assert span == ry.timespan(years=6, days=7, hours=7)

        span = dt1.until(
            dt2, smallest="minute", largest="year", mode="half-expand", increment=30
        )
        assert span == ry.timespan(years=6, days=7, hours=7)

    def test_datetime_difference_smallest(self) -> None:
        """
        ```rust
        use jiff::{
            civil::{DateTime, DateTimeDifference},
            RoundMode, ToSpan, Unit,
        };

        let dt1 = "2024-03-15 08:14".parse::<DateTime>()?;
        let dt2 = "2030-11-22 08:30".parse::<DateTime>()?;
        let span = dt1.until(
            DateTimeDifference::new(dt2)
                .smallest(Unit::Week)
                .largest(Unit::Week)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 349.weeks());
        ```
        """
        dt1 = ry.DateTime.parse("2024-03-15 08:14")
        dt2 = ry.DateTime.parse("2030-11-22 08:30")
        span = dt1._until(
            ry.DateTimeDifference(dt2)
            ._smallest("week")
            ._largest("week")
            ._mode("half-expand")
        )
        assert span == ry.timespan(weeks=349)

    def test_datetime_difference_largest(self) -> None:
        """
        ```rust
        use jiff::{civil::{DateTime, DateTimeDifference}, ToSpan, Unit};

        let dt1 = "2024-03-15 08:14".parse::<DateTime>()?;
        let dt2 = "2030-11-22 08:30".parse::<DateTime>()?;
        let span = dt1.until(
            DateTimeDifference::new(dt2).largest(Unit::Second),
        )?;
        assert_eq!(span, 211076160.seconds());
        ```
        """
        dt1 = ry.DateTime.parse("2024-03-15 08:14")
        dt2 = ry.DateTime.parse("2030-11-22 08:30")
        span = dt1._until(ry.DateTimeDifference(dt2)._largest("second"))
        assert span == ry.timespan(seconds=211076160)

    def test_datetime_difference_mode(self) -> None:
        """
        ```rust
        use jiff::{
            civil::{DateTime, DateTimeDifference},
            RoundMode, ToSpan, Unit,
        };

        let dt1 = "2024-03-15 08:10".parse::<DateTime>()?;
        let dt2 = "2024-03-15 08:11".parse::<DateTime>()?;
        let span = dt1.until(
            DateTimeDifference::new(dt2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        // Only one minute elapsed, but we asked to always round up!
        assert_eq!(span, 1.hour());

        // Since `Ceil` always rounds toward positive infinity, the behavior
        // flips for a negative span.
        let span = dt1.since(
            DateTimeDifference::new(dt2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        assert_eq!(span, 0.hour());
        ```
        """
        dt1 = ry.DateTime.parse("2024-03-15 08:10")
        dt2 = ry.DateTime.parse("2024-03-15 08:11")
        span = dt1._until(ry.DateTimeDifference(dt2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=1)

        span = dt1._since(ry.DateTimeDifference(dt2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=0)


class TestTimestampDifference:
    def test_timestamp_difference_docs_example(self) -> None:
        """
        ```rust
        use jiff::{RoundMode, Timestamp, TimestampDifference, ToSpan, Unit};

        let ts1 = "2024-03-15 08:14:00.123456789Z".parse::<Timestamp>()?;
        let ts2 = "2024-03-22 15:00Z".parse::<Timestamp>()?;
        let span = ts1.until(
            TimestampDifference::new(ts2)
                .smallest(Unit::Minute)
                .largest(Unit::Hour)
                .mode(RoundMode::HalfExpand)
                .increment(30),
        )?;
        assert_eq!(span, 175.hours());

        // One less minute, and because of the HalfExpand mode, the span would
        // get rounded down.
        let ts2 = "2024-03-22 14:59Z".parse::<Timestamp>()?;
        let span = ts1.until(
            TimestampDifference::new(ts2)
                .smallest(Unit::Minute)
                .largest(Unit::Hour)
                .mode(RoundMode::HalfExpand)
                .increment(30),
        )?;
        assert_eq!(span, 174.hours().minutes(30));

        ```
        """
        ts1 = ry.Timestamp.parse("2024-03-15 08:14:00.123456789Z")
        ts2 = ry.Timestamp.parse("2024-03-22 15:00Z")
        span = ts1._until(
            ry.TimestampDifference(ts2)
            ._smallest("minute")
            ._largest("hour")
            ._mode("half-expand")
            ._increment(30)
        )

        assert span == ry.timespan(hours=175)

        ts2 = ry.Timestamp.parse("2024-03-22 14:59Z")
        span = ts1._until(
            ry.TimestampDifference(ts2)
            ._smallest("minute")
            ._largest("hour")
            ._mode("half-expand")
            ._increment(30)
        )
        assert span == ry.timespan(hours=174, minutes=30)

    def test_timestamp_difference_smallest(self) -> None:
        """
        ```rust
        use jiff::{RoundMode, Timestamp, TimestampDifference, ToSpan, Unit};

        let ts1 = "2024-03-15 08:14:02.5001Z".parse::<Timestamp>()?;
        let ts2 = "2024-03-15T08:16:03.0001Z".parse::<Timestamp>()?;
        let span = ts1.until(
            TimestampDifference::new(ts2)
                .smallest(Unit::Second)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 121.seconds());
        ```
        """
        ts1 = ry.Timestamp.parse("2024-03-15 08:14:02.5001Z")
        ts2 = ry.Timestamp.parse("2024-03-15T08:16:03.0001Z")
        span = ts1._until(
            ry.TimestampDifference(ts2)._smallest("second")._mode("half-expand")
        )
        assert span == ry.timespan(seconds=121)

    def test_timestamp_difference_largest(self) -> None:
        """
        ```rust
        use jiff::{Timestamp, TimestampDifference, ToSpan, Unit};

        let ts1 = "2024-03-15 08:14Z".parse::<Timestamp>()?;
        let ts2 = "2030-11-22 08:30Z".parse::<Timestamp>()?;
        let span = ts1.until(
            TimestampDifference::new(ts2).largest(Unit::Second),
        )?;
        assert_eq!(span, 211076160.seconds());
        ```
        """
        ts1 = ry.Timestamp.parse("2024-03-15 08:14Z")
        ts2 = ry.Timestamp.parse("2030-11-22 08:30Z")
        span = ts1._until(ry.TimestampDifference(ts2)._largest("second"))
        assert span == ry.timespan(seconds=211076160)

    def test_timestamp_difference_mode(self) -> None:
        """
        ```rust
        use jiff::{RoundMode, Timestamp, TimestampDifference, ToSpan, Unit};

        let ts1 = "2024-03-15 08:10Z".parse::<Timestamp>()?;
        let ts2 = "2024-03-15 08:11Z".parse::<Timestamp>()?;
        let span = ts1.until(
            TimestampDifference::new(ts2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        // Only one minute elapsed, but we asked to always round up!
        assert_eq!(span, 1.hour());

        // Since `Ceil` always rounds toward positive infinity, the behavior
        // flips for a negative span.
        let span = ts1.since(
            TimestampDifference::new(ts2)
                .smallest(Unit::Hour)
                .mode(RoundMode::Ceil),
        )?;
        assert_eq!(span, 0.hour());
        ```
        """
        ts1 = ry.Timestamp.parse("2024-03-15 08:10Z")
        ts2 = ry.Timestamp.parse("2024-03-15 08:11Z")
        span = ts1._until(ry.TimestampDifference(ts2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=1)

        span = ts1._since(ry.TimestampDifference(ts2)._smallest("hour")._mode("ceil"))
        assert span == ry.timespan(hours=0)

    def test_timestamp_difference_increment(self) -> None:
        """
        ```rust
        use jiff::{RoundMode, Timestamp, TimestampDifference, ToSpan, Unit};

        let ts1 = "2024-03-15 08:19Z".parse::<Timestamp>()?;
        let ts2 = "2024-03-15 12:52Z".parse::<Timestamp>()?;

        let span = ts1.until(
            TimestampDifference::new(ts2)
                .smallest(Unit::Minute)
                .increment(5)
                .mode(RoundMode::HalfExpand),
        )?;
        assert_eq!(span, 275.minutes());
        ```
        """

        ts1 = ry.Timestamp.parse("2024-03-15 08:19Z")
        ts2 = ry.Timestamp.parse("2024-03-15 12:52Z")
        span = ts1._until(
            ry.TimestampDifference(ts2)
            ._smallest("minute")
            ._increment(5)
            ._mode("half-expand")
        )
        assert span == ry.timespan(minutes=275)
