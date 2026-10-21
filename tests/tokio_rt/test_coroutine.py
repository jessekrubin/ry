from __future__ import annotations

import asyncio
import collections.abc
import contextvars
import gc
import os
import subprocess
import sys
import threading
import time
import typing as t

import pytest

import ry

if t.TYPE_CHECKING:
    from pathlib import Path


def _fd_count() -> int:
    return len(os.listdir("/proc/self/fd"))


class TestShape:
    async def test_is_coroutine(self) -> None:
        coro = ry.asleep(0)
        assert isinstance(coro, collections.abc.Coroutine)
        assert asyncio.iscoroutine(coro)
        assert isinstance(coro, collections.abc.Awaitable)
        assert type(coro).__name__ == "Coroutine"
        assert type(coro).__module__ == "ry.ryo3"
        await coro

    async def test_names(self) -> None:
        coro: t.Any = ry.asleep(0)
        assert coro.__qualname__ == "asleep"
        assert coro.__name__ == "asleep"
        assert repr(coro).startswith("<ry.Coroutine asleep at 0x")
        assert repr(coro).endswith(f"{id(coro):x}>")
        await coro

    def test_method_names(self, tmp_path: Path) -> None:
        coro: t.Any = ry.AsyncFile(tmp_path / "f.bin", "wb").close()
        assert coro.__qualname__ == "AsyncFile.close"
        assert coro.__name__ == "close"
        coro.close()

    async def test_task_repr(self) -> None:
        task = asyncio.ensure_future(ry.asleep(0))
        assert "coro=<asleep()>" in repr(task)
        await task


class TestLazy:
    @pytest.mark.parametrize("how", ["close", "drop"])
    async def test_never_awaited_does_nothing(self, tmp_path: Path, how: str) -> None:
        coros = [ry.write_async(tmp_path / f"{i}.bin", b"x") for i in range(8)]
        if how == "close":
            for coro in coros:
                coro.close()
        del coros
        gc.collect()
        await asyncio.sleep(0.1)
        assert list(tmp_path.iterdir()) == []


class TestResults:
    async def test_value(self) -> None:
        assert await ry.asleep(0) == 0

    async def test_await_twice_raises(self) -> None:
        coro = ry.asleep(0)
        await coro
        with pytest.raises(RuntimeError, match="already awaited"):
            await coro

    async def test_err_raises_as_is(self, tmp_path: Path) -> None:
        with pytest.raises(FileNotFoundError):
            await ry.read_async(tmp_path / "nope")


class TestThrowClose:
    async def test_throw_reraises(self) -> None:
        coro = ry.asleep(0)
        err = ValueError("nope")
        with pytest.raises(ValueError, match="nope") as exc_info:
            coro.throw(err)
        assert exc_info.value is err
        with pytest.raises(RuntimeError, match="cannot reuse already awaited"):
            coro.send(None)

    async def test_throw_legacy_signature(self) -> None:
        coro = ry.asleep(0)
        with pytest.raises(KeyError, match="dingo"):
            coro.throw(KeyError, "dingo", None)
        with pytest.raises(RuntimeError, match="already awaited"):
            coro.send(None)

    async def test_throw_mid_flight(self) -> None:
        coro = ry.asleep(10)
        assert coro.send(None) is not None  # suspended on the tokio timer
        with pytest.raises(ValueError, match="nope"):
            coro.throw(ValueError("nope"))
        with pytest.raises(RuntimeError, match="already awaited"):
            coro.send(None)

    async def test_close_then_send(self) -> None:
        coro = ry.asleep(0)
        coro.close()
        coro.close()
        with pytest.raises(RuntimeError, match="already awaited"):
            coro.send(None)


class TestAsyncioCompat:
    async def test_create_task_gather(self) -> None:
        tasks = [asyncio.create_task(ry.asleep(0)) for _ in range(16)]
        assert await asyncio.gather(*tasks) == [0] * 16
        assert await asyncio.gather(*(ry.asleep(0) for _ in range(16))) == [0] * 16

    async def test_wait_for(self) -> None:
        assert await asyncio.wait_for(ry.asleep(0), 5) == 0
        with pytest.raises(TimeoutError):
            await asyncio.wait_for(ry.asleep(10), 0.05)

    async def test_timeout(self) -> None:
        start = time.monotonic()
        with pytest.raises(TimeoutError):
            async with asyncio.timeout(0.05):
                await ry.asleep(10)
        assert time.monotonic() - start < 5

    async def test_task_group(self) -> None:
        async with asyncio.TaskGroup() as tg:
            tasks = [tg.create_task(ry.asleep(0.01)) for _ in range(8)]
        assert [task.result() for task in tasks] == [0.01] * 8

    async def test_task_group_sibling_failure(self) -> None:
        async def _boom() -> None:
            await asyncio.sleep(0.05)
            msg = "boom"
            raise ValueError(msg)

        tasks: list[asyncio.Task[t.Any]] = []

        async def _group() -> None:
            async with asyncio.TaskGroup() as tg:
                tasks.append(tg.create_task(ry.asleep(10)))
                tasks.append(tg.create_task(_boom()))

        with pytest.raises(ExceptionGroup) as exc_info:
            await _group()
        assert exc_info.group_contains(ValueError)
        assert tasks[0].cancelled()

    async def test_cancel(self) -> None:
        task = asyncio.ensure_future(ry.asleep(10))
        await asyncio.sleep(0)
        task.cancel("adios")
        with pytest.raises(asyncio.CancelledError):
            await task
        assert task.cancelled()

    async def test_cancel_before_first_step(self) -> None:
        task = asyncio.ensure_future(ry.asleep(0))
        task.cancel()
        with pytest.raises(asyncio.CancelledError):
            await task

    async def test_remove_done_callback_bound_method(self) -> None:
        """matches w/ `==` like asyncio.Future (bound methods are new objects)"""

        class _Cb:
            def cb(self, fut: t.Any) -> None: ...

        obj = _Cb()
        coro = ry.asleep(10)
        fut = coro.send(None)
        fut.add_done_callback(obj.cb)
        fut.add_done_callback(obj.cb)
        assert fut.remove_done_callback(obj.cb) == 2
        assert fut.remove_done_callback(obj.cb) == 0
        coro.close()

    async def test_contextvars(self) -> None:
        cv: contextvars.ContextVar[str] = contextvars.ContextVar("cv")
        cv.set("dingo")
        await ry.asleep(0)
        assert cv.get() == "dingo"

        async def _set_and_get() -> str:
            cv.set("bluey")
            await ry.asleep(0)
            return cv.get()

        assert await asyncio.create_task(_set_and_get()) == "bluey"
        assert cv.get() == "dingo"


class TestStress:
    async def test_gather_rounds(self, tmp_path: Path) -> None:
        p = tmp_path / "f.bin"
        p.write_bytes(b"x" * 1024)

        async def _round() -> None:
            sleeps = asyncio.gather(*(ry.asleep(0) for _ in range(500)))
            reads = asyncio.gather(*(ry.read_async(p) for _ in range(100)))
            assert await sleeps == [0] * 500
            assert all(len(b) == 1024 for b in await reads)

        for _ in range(20):
            await asyncio.wait_for(_round(), 30)

    async def test_interleaved_sleeps(self) -> None:
        async def _one(i: int) -> float:
            for _ in range(i % 4):
                await asyncio.sleep(0)
            coro = ry.asleep(0.001 * (i % 3))
            await asyncio.sleep(0)
            return await coro

        for _ in range(5):
            res = await asyncio.wait_for(
                asyncio.gather(*(_one(i) for i in range(400))), 30
            )
            assert res == [0.001 * (i % 3) for i in range(400)]


class TestLoops:
    def test_asyncio_run_many_no_fd_growth(self) -> None:
        for _ in range(10):
            asyncio.run(ry.asleep(0))
        gc.collect()
        before = _fd_count()
        for _ in range(300):
            asyncio.run(ry.asleep(0))
        gc.collect()
        assert _fd_count() - before <= 4

    def test_threads_own_loops(self) -> None:
        errors: list[BaseException] = []
        results: list[list[float]] = []

        async def _many() -> list[float]:
            return await asyncio.gather(*(ry.asleep(0) for _ in range(64)))

        def _run() -> None:
            try:
                for _ in range(4):
                    results.append(asyncio.run(asyncio.wait_for(_many(), 30)))
            except BaseException as e:  # noqa: BLE001
                errors.append(e)

        threads = [threading.Thread(target=_run) for _ in range(6)]
        for th in threads:
            th.start()
        for th in threads:
            th.join(60)
        assert not errors
        assert len(results) == 24
        assert all(r == [0] * 64 for r in results)

    def test_uvloop(self) -> None:
        uvloop = pytest.importorskip("uvloop")

        async def _many() -> list[float]:
            return await asyncio.gather(*(ry.asleep(0) for _ in range(64)))

        with asyncio.Runner(loop_factory=uvloop.new_event_loop) as runner:
            assert runner.run(_many()) == [0] * 64

    def test_closed_loop_in_flight(self) -> None:
        """close a loop w/ coroutines in flight ~ no hang/crash, next loop is fine"""
        loop = asyncio.new_event_loop()
        try:
            tasks = [loop.create_task(ry.asleep(10)) for _ in range(16)]
            loop.run_until_complete(asyncio.sleep(0.1))
        finally:
            loop.close()
        assert not any(task.done() for task in tasks)
        del loop, tasks
        gc.collect()
        assert asyncio.run(ry.asleep(0)) == 0

    def test_exit_with_coroutines_in_flight(self) -> None:
        """interpreter going down w/ stuff in flight doesnt crash"""
        script = """
import asyncio
import ry

async def main():
    tasks = [asyncio.ensure_future(ry.asleep(10)) for _ in range(32)]
    await asyncio.sleep(0.2)

loop = asyncio.new_event_loop()
loop.run_until_complete(main())
loop.close()
"""
        res = subprocess.run(  # noqa: S603
            [sys.executable, "-c", script],
            capture_output=True,
            timeout=30,
            check=False,
        )
        assert res.returncode == 0, res.stderr.decode()


class TestFatalErrors:
    def test_sys_exit_in_background_task_stops_loop(self) -> None:
        """`SystemExit` out of a task step woken by ry stops the loop (not swallowed)"""
        script = """
import asyncio, sys
import ry

async def bg():
    await ry.asleep(0.01)
    sys.exit(3)

async def main():
    task = asyncio.create_task(bg())
    await asyncio.sleep(30)

asyncio.run(main())
"""
        res = subprocess.run(  # noqa: S603
            [sys.executable, "-c", script],
            capture_output=True,
            timeout=20,
            check=False,
        )
        assert res.returncode == 3, res.stderr.decode()
        assert b"Exception ignored" not in res.stderr


class TestFallback:
    def test_no_add_reader(self, tmp_path: Path) -> None:
        class _NoReaderLoop(asyncio.SelectorEventLoop):
            def add_reader(self, *args: t.Any, **kwargs: t.Any) -> t.Any:  # type: ignore[override]
                raise NotImplementedError

        p = tmp_path / "f.bin"
        p.write_bytes(b"x" * 1024)

        async def _main() -> None:
            assert (
                await asyncio.gather(*(ry.asleep(0) for _ in range(200))) == [0] * 200
            )
            assert all(
                len(b) == 1024
                for b in await asyncio.gather(*(ry.read_async(p) for _ in range(50)))
            )

        with asyncio.Runner(loop_factory=_NoReaderLoop) as runner:
            runner.run(asyncio.wait_for(_main(), 30))
