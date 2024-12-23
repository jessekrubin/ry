from __future__ import annotations

import ry


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


class TestInstantAddSub:
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
