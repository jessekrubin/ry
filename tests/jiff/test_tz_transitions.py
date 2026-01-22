import ry


class TestTimeZoneTransitions:
    """
    PRECEDING-DOCS: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.preceding
    FOLLOWING-DOCS: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#method.following
    """

    def test_preceding_example_time_since_previous_transition(self) -> None:
        """hand translated from rust eg
        REF: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#example-time-since-the-previous-transition
        """
        now: ry.ZonedDateTime = ry.ZonedDateTime.parse(
            "2024-12-31 18:25-05[US/Eastern]"
        )
        transitions = now.timezone.preceding(now.timestamp())
        transition = transitions[0]
        prev_at = transition["timestamp"].to_zoned(now.timezone)
        span = now.since(prev_at, largest="year")
        assert f"{span:#}" == "1mo 27d 17h 25m"

    def test_show_5_previous_tz_transitions(self) -> None:
        """hand translated from rust eg
        REF: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#example-show-the-5-previous-time-zone-transitions
        """
        now: ry.ZonedDateTime = ry.ZonedDateTime.parse(
            "2024-12-31 18:25-05[US/Eastern]"
        )
        transitions = now.timezone.preceding(now.timestamp(), 5)
        result = [
            (
                t["timestamp"].to_zoned(now.timezone),
                t["offset"],
                t["abbreviation"],
            )
            for t in transitions
        ]
        # fmt: off
        expected = [
            (ry.ZonedDateTime.parse("2024-11-03 01:00-05[US/Eastern]"), ry.Offset(-5), "EST"),
            (ry.ZonedDateTime.parse("2024-03-10 03:00-04[US/Eastern]"), ry.Offset(-4), "EDT"),
            (ry.ZonedDateTime.parse("2023-11-05 01:00-05[US/Eastern]"), ry.Offset(-5), "EST"),
            (ry.ZonedDateTime.parse("2023-03-12 03:00-04[US/Eastern]"), ry.Offset(-4), "EDT"),
            (ry.ZonedDateTime.parse("2022-11-06 01:00-05[US/Eastern]"), ry.Offset(-5), "EST"),
        ]
        # fmt: on
        assert result == expected

    def test_time_until_next_transition(self) -> None:
        """hand translated from rust eg
        REF: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#example-time-until-the-next-transition
        """
        now: ry.ZonedDateTime = ry.ZonedDateTime.parse(
            "2024-12-31 18:25-05[US/Eastern]"
        )
        transitions = now.timezone.following(now.timestamp())
        transition = transitions[0]
        next_at = transition["timestamp"].to_zoned(now.timezone)
        span = now.until(next_at, largest="year")
        assert f"{span:#}" == "2mo 8d 7h 35m"

    def test_show_5_next_tz_transitions(self) -> None:
        """REF: https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html#example-show-the-5-next-time-zone-transitions"""
        now: ry.ZonedDateTime = ry.ZonedDateTime.parse(
            "2024-12-31 18:25-05[US/Eastern]"
        )
        result = [
            (
                t["timestamp"].to_zoned(now.timezone),
                t["offset"],
                t["abbreviation"],
            )
            for t in now.timezone.following(now.timestamp(), 5)
        ]
        # fmt: off
        expected = [
            (ry.ZonedDateTime.parse("2025-03-09 03:00-04[US/Eastern]"), ry.Offset(-4), "EDT"),
            (ry.ZonedDateTime.parse("2025-11-02 01:00-05[US/Eastern]"), ry.Offset(-5), "EST"),
            (ry.ZonedDateTime.parse("2026-03-08 03:00-04[US/Eastern]"), ry.Offset(-4), "EDT"),
            (ry.ZonedDateTime.parse("2026-11-01 01:00-05[US/Eastern]"), ry.Offset(-5), "EST"),
            (ry.ZonedDateTime.parse("2027-03-14 03:00-04[US/Eastern]"), ry.Offset(-4), "EDT"),
        ]
        # fmt: on
        assert result == expected
