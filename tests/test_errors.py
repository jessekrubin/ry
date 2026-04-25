from __future__ import annotations

import pytest

import ry


class TestPanicError:
    def test_panic_raises(self) -> None:
        with pytest.raises(ry.PanicError):
            ry.panic()

    def test_panic_with_message(self) -> None:
        msg = "something went wrong"
        with pytest.raises(ry.PanicError, match=msg):
            ry.panic(msg)

    def test_panic_is_base_exception(self) -> None:
        assert issubclass(ry.PanicError, BaseException)

    def test_panic_not_caught_by_exception(self) -> None:
        with pytest.raises(ry.PanicError):
            ry.panic("should not be swallowed")


class TestUnreachableError:
    def test_unreachable_raises(self) -> None:
        with pytest.raises(ry.UnreachableError):
            ry.unreachable()

    def test_unreachable_with_message(self) -> None:
        msg = "this should never happen"
        with pytest.raises(ry.UnreachableError, match=msg):
            ry.unreachable(msg)

    def test_unreachable_is_assertion_error(self) -> None:
        assert issubclass(ry.UnreachableError, AssertionError)

    def test_unreachable_caught_by_assertion_error(self) -> None:
        with pytest.raises(AssertionError):
            ry.unreachable("caught as AssertionError")
