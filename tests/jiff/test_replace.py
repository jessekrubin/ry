import pytest

import ry


class TestTimeReplace:
    def test_time_replace_noop(self) -> None:
        midnight = ry.Time.midnight()
        no_replacement = midnight.replace()
        assert no_replacement == midnight

    @pytest.mark.parametrize(
        "kw",
        [
            {
                "millisecond": 12,
            },
            {
                "microsecond": 12,
            },
            {
                "nanosecond": 12,
            },
        ],
    )
    def test_replace_subsec_nanosecond_conflict(self, kw: dict[str, int]) -> None:
        midnight = ry.Time.midnight()
        with pytest.raises(
            TypeError,
            match="Cannot specify both subsec_nanosecond and millisecond/microsecond/nanosecond",
        ):
            midnight.replace(subsec_nanosecond=123456789, **kw)

    @pytest.mark.parametrize(
        "kw",
        [
            {
                "subsec_nanosecond": 123456789,
            },
            # equiv broken down
            {
                "millisecond": 123,
                "microsecond": 456,
                "nanosecond": 789,
            },
        ],
    )
    def test_replace_time_simple(self, kw: dict[str, int]) -> None:
        midnight = ry.Time.midnight()
        t = midnight.replace(hour=1, minute=2, second=3, **kw)
        assert t.hour == 1
        assert t.minute == 2
        assert t.second == 3
        assert t.subsec_nanosecond == 123456789
        assert t.millisecond == 123
        assert t.microsecond == 456
        assert t.nanosecond == 789
