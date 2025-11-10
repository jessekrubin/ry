from __future__ import annotations

import typing as t

import pytest
from hypothesis import given

import ry

from ..strategies import st_durations


class TestDurationIsoformat:
    def test_duration_isoformat_min(self) -> None:
        min_dur = ry.Duration.MIN
        iso_str = min_dur.isoformat()
        assert iso_str == "PT0S"
        parsed_min_dur = ry.Duration.fromisoformat(iso_str)
        assert parsed_min_dur == min_dur
        parsed_min_dur_from_str = ry.Duration.from_str(iso_str)
        assert parsed_min_dur_from_str == min_dur

    def test_duration_isoformat_max(self) -> None:
        max_dur = ry.Duration.MAX
        iso_str = max_dur.isoformat()
        assert iso_str == "PT5124095576030431H15.999999999S"
        parsed_max_dur = ry.Duration.fromisoformat(iso_str)
        assert parsed_max_dur == max_dur

    @given(st_durations())
    def test_duration_isoformat(self, dur: ry.Duration) -> None:
        iso_str = dur.isoformat()
        parsed_dur = ry.Duration.fromisoformat(iso_str)
        assert parsed_dur == dur


class TestDurationFriendlyStr:
    def test_duration_friendly_min(self) -> None:
        min_dur = ry.Duration.MIN
        iso_str = min_dur.friendly()
        assert iso_str == "0s"
        parsed_min_dur = ry.Duration.from_str(iso_str)
        assert parsed_min_dur == min_dur

    def test_duration_friendly_max(self) -> None:
        max_dur = ry.Duration.MAX
        iso_str = max_dur.friendly()
        assert iso_str == "5124095576030431h 15s 999ms 999µs 999ns"  # noqa: RUF001
        parsed_max_dur = ry.Duration.from_str(iso_str)
        assert parsed_max_dur == max_dur

    @pytest.mark.parametrize(
        "designator, expected",
        [
            ("human", "5124095576030431h 15s 999ms 999us 999ns"),
            ("human-time", "5124095576030431h 15s 999ms 999us 999ns"),
            ("compact", "5124095576030431h 15s 999ms 999µs 999ns"),  # noqa: RUF001
            ("short", "5124095576030431hrs 15secs 999msecs 999µsecs 999nsecs"),  # noqa: RUF001
            (
                "verbose",
                "5124095576030431hours 15seconds 999milliseconds 999microseconds 999nanoseconds",
            ),
        ],
    )
    def test_duration_friendly_max_designator(
        self,
        designator: t.Literal["compact", "human", "human-time", "short", "verbose"],
        expected: str,
    ) -> None:
        max_dur = ry.Duration.MAX
        iso_str = max_dur.friendly(designator)
        assert iso_str == expected
        parsed_max_dur = ry.Duration.from_str(iso_str)
        assert parsed_max_dur == max_dur

    def test_duration_friendly_max_designator_wrong(
        self,
    ) -> None:
        max_dur = ry.Duration.MAX
        with pytest.raises(ValueError):
            _s = max_dur.friendly("dingo")  # type: ignore[arg-type]
        with pytest.raises(TypeError):
            _s = f"{max_dur:herm}"

    @given(st_durations())
    def test_duration_friendly(self, dur: ry.Duration) -> None:
        iso_str = dur.friendly()
        parsed_dur = ry.Duration.from_str(iso_str)
        assert parsed_dur == dur


class TestDurationFormat:
    def test_duration_friendly_min(self) -> None:
        min_dur = ry.Duration.MIN
        iso_str = f"dur: {min_dur}"
        assert iso_str == "dur: PT0S"

        frienldy_str = f"friendly dur: {min_dur:#}"
        assert frienldy_str == "friendly dur: 0s"

    def test_duration_friendly_max(self) -> None:
        min_dur = ry.Duration.MAX
        iso_str = f"dur: {min_dur}"
        assert iso_str == "dur: PT5124095576030431H15.999999999S"

        frienldy_str = f"friendly dur: {min_dur:#}"
        assert frienldy_str == "friendly dur: 5124095576030431h 15s 999ms 999µs 999ns"  # noqa: RUF001
