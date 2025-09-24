from __future__ import annotations

import pytest
from hypothesis import given

import ry

from ..strategies import st_durations


class TestInstant:
    def test_instant_now(self) -> None:
        inst = ry.Instant().now()
        assert isinstance(inst, ry.Instant)

    def test_instant_elapsed(self) -> None:
        inst = ry.Instant().now()
        ry.sleep(0.1)
        dur = inst.elapsed()
        assert isinstance(dur, ry.Duration)

    def test_instant_function(self) -> None:
        inst = ry.instant()
        assert isinstance(inst, ry.Instant)
        assert inst.elapsed().as_secs_f64() >= 0.0

    def test_instance_repr(self) -> None:
        inst = ry.Instant().now()
        repr_str = repr(inst)
        assert isinstance(repr_str, str)
        assert repr_str.startswith("Instant<{") and repr_str.endswith("}>")

    def test_hash(self) -> None:
        inst1 = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        assert hash(inst1) != hash(inst2)
        assert hash(inst1) == hash(inst1)
        assert hash(inst2) == hash(inst2)
        d = {inst1: "first", inst2: "second"}
        assert d[inst1] == "first"
        assert d[inst2] == "second"


class TestInstantComparison:
    def test_instant_eq(self) -> None:
        inst1 = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        assert inst1 != inst2
        assert inst1 == inst1
        assert inst2 >= inst2
        assert inst1 <= inst1
        assert not (inst1 == inst2)

    def test_instant_ord(self) -> None:
        inst1 = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        assert inst1 < inst2
        assert inst2 > inst1
        assert inst1 <= inst2
        assert inst2 >= inst1
        assert inst1 <= inst1
        assert inst2 >= inst2


class TestInstantArithmetic:
    def test_instant_add(self) -> None:
        inst = ry.Instant().now()
        dur = ry.Duration.from_secs(1)
        inst2 = inst + dur
        assert isinstance(inst2, ry.Instant)

    def test_instant_sub_duration(self) -> None:
        inst = ry.Instant().now()
        dur = ry.Duration.from_secs(1)
        inst2 = inst - dur
        assert isinstance(inst2, ry.Instant)

    def test_instant_sub_instant(self) -> None:
        inst = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        dur = inst2 - inst
        assert isinstance(dur, ry.Duration)

    def test_instant_sub_type_error(self) -> None:
        inst = ry.Instant().now()
        with pytest.raises(TypeError):
            _ = inst - 123  # type: ignore

    @given(dur=st_durations())
    def test_instant_checked_add(self, dur: ry.Duration) -> None:
        inst = ry.Instant().now()
        dur = ry.Duration.from_secs(1)
        res = inst.checked_add(dur)
        assert res is not None or isinstance(res, ry.Instant)

    @given(dur=st_durations())
    def test_instant_checked_sub_duration(self, dur: ry.Duration) -> None:
        inst = ry.Instant().now()
        dur = ry.Duration.from_secs(1)
        res = inst.checked_sub(dur)
        assert res is not None or isinstance(res, ry.Instant)

    def test_checked_duration_since(self) -> None:
        inst = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        res = inst2.checked_duration_since(inst)
        assert res is not None or isinstance(res, ry.Duration)

    def test_saturating_duration_since(self) -> None:
        inst = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        res = inst2.saturating_duration_since(inst)
        assert isinstance(res, ry.Duration)

    def test_duration_since(self) -> None:
        inst = ry.Instant().now()
        ry.Duration.from_secs_f64(0.1).sleep()
        inst2 = ry.Instant().now()
        res = inst2.duration_since(inst)
        assert isinstance(res, ry.Duration)
