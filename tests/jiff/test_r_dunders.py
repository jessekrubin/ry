# ruff: noqa: PLC2801

from __future__ import annotations

import datetime as pydt
import typing as t

import pytest

import ry

_Temporal: t.TypeAlias = (
    ry.Date | ry.DateTime | ry.Time | ry.Timestamp | ry.ZonedDateTime
)
_TTemporal = t.TypeVar(
    "_TTemporal", ry.Date, ry.DateTime, ry.Time, ry.Timestamp, ry.ZonedDateTime
)


_TEMPORALS: list[_Temporal] = [
    ry.date(2021, 12, 31),
    ry.datetime(2021, 5, 15, 9, 0, 0, 0),
    ry.time(14, 30, 0, 0),
    ry.Timestamp(second=1622520000, nanosecond=0),
    ry.date(2022, 3, 10).at(12, 0, 0, 0).in_tz("Europe/Madrid"),
]
_TEMPORAL_IDS = [type(v).__name__ for v in _TEMPORALS]

_SPANISH: list[ry.TimeSpan | ry.SignedDuration | ry.Duration | pydt.timedelta] = [
    ry.timespan(hours=2, minutes=15),
    ry.SignedDuration(secs=5400),
    ry.Duration(secs=3600),
    pydt.timedelta(hours=3),
]
_SPANISH_IDS = [type(v).__name__ for v in _SPANISH]


class TestReflectedOperators:
    # =========================================================================
    # OK
    # =========================================================================
    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    @pytest.mark.parametrize("spanish", _SPANISH, ids=_SPANISH_IDS)
    def test_temporal_radd_commutative(
        self,
        val: _TTemporal,
        spanish: ry.TimeSpan | ry.SignedDuration | ry.Duration | pydt.timedelta,
    ) -> None:
        expected = val + spanish
        assert spanish + val == expected

    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    @pytest.mark.parametrize(
        "duration_like",
        [ry.Duration(secs=3600), pydt.timedelta(hours=3)],
        ids=["Duration", "timedelta"],
    )
    def test_temporal_radd_duration_like(
        self, val: _TTemporal, duration_like: ry.Duration | pydt.timedelta
    ) -> None:
        assert val.__radd__(duration_like) == val + duration_like

    # =========================================================================
    # ERR/NOT-IMPL
    # =========================================================================
    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    @pytest.mark.parametrize(
        "spanish",
        [ry.timespan(hours=2, minutes=15), ry.SignedDuration(secs=5400)],
        ids=["TimeSpan", "SignedDuration"],
    )
    def test_temporal_radd_only_duration_like(
        self, val: _Temporal, spanish: ry.TimeSpan | ry.SignedDuration
    ) -> None:
        assert val.__radd__(spanish) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]

    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    @pytest.mark.parametrize("other", [1, 1.5, "1h", None, b"x"])
    def test_temporal_radd_unsupported(
        self, val: _Temporal, other: float | str | bytes | None
    ) -> None:
        assert val.__radd__(other) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]
        with pytest.raises(TypeError):
            _ = other + val  # type: ignore[operator]

    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    def test_temporal_radd_self_unsupported(self, val: _Temporal) -> None:
        assert val.__radd__(val) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]
        with pytest.raises(TypeError):
            _ = val + val  # type: ignore[operator]

    @pytest.mark.parametrize("val", _TEMPORALS, ids=_TEMPORAL_IDS)
    @pytest.mark.parametrize("spanish", _SPANISH, ids=_SPANISH_IDS)
    def test_temporal_rsub_spanish_unsupported(
        self,
        val: _Temporal,
        spanish: ry.TimeSpan | ry.SignedDuration | ry.Duration | pydt.timedelta,
    ) -> None:
        """Test that reflected subtraction with a temporal and a Spanish duration-like object is unsupported

        should be NotImplemented
        """
        assert val.__rsub__(spanish) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]
        with pytest.raises(TypeError):
            _ = spanish - val  # type: ignore[operator]

    def test_timespan_radd_rsub(self) -> None:
        a = ry.timespan(hours=1, minutes=30)
        b = ry.timespan(hours=3)
        assert a.__radd__(b) == b + a == ry.timespan(hours=4, minutes=30)
        assert a.__rsub__(b) == b - a == ry.timespan(hours=1, minutes=30)
        assert b.__rsub__(a) == a - b == ry.timespan(hours=-1, minutes=-30)

    def test_signed_duration_rtruediv(self) -> None:
        a = ry.SignedDuration(secs=4)
        b = ry.SignedDuration(secs=10)
        res = a.__rtruediv__(b)
        assert isinstance(res, float)
        assert res == b / a == 2.5

    @pytest.mark.parametrize(
        "other",
        [2, 2.0, ry.Duration(secs=10), pydt.timedelta(seconds=10)],
        ids=lambda v: type(v).__name__,
    )
    def test_signed_duration_rtruediv_unsupported(self, other: object) -> None:
        sd = ry.SignedDuration(secs=4)
        # no reflected version
        assert sd.__rtruediv__(other) is NotImplemented  # type: ignore[operator]  # ty: ignore[invalid-argument-type]
        with pytest.raises(TypeError):
            _ = other / sd  # type: ignore[operator]
