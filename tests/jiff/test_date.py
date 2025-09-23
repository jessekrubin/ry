from __future__ import annotations

import ry


class TestDateUntil:
    """
    ```
    use jiff::{civil::date, ToSpan};

    let earlier = date(2006, 8, 24);
    let later = date(2019, 1, 31);
    assert_eq!(earlier.until(later)?, 4543.days());

    // Flipping the dates is fine, but you'll get a negative span.
    let earlier = date(2006, 8, 24);
    let later = date(2019, 1, 31);
    assert_eq!(later.until(earlier)?, -4543.days());
    ```
    """

    def test_date_until_overflow(self) -> None:
        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert earlier.until(later) == ry.timespan(days=4543)

        earlier = ry.date(2006, 8, 24)
        later = ry.date(2019, 1, 31)
        assert later.until(earlier) == ry.timespan(days=-4543)


class TestDateTomorrowYesterday:
    def test_date_tomorrow(self) -> None:
        d = ry.date(2023, 3, 14)
        assert d.tomorrow() == ry.date(2023, 3, 15)

    def test_date_yesterday(self) -> None:
        d = ry.date(2023, 3, 14)
        assert d.yesterday() == ry.date(2023, 3, 13)
