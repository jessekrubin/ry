"""ry's own `Coroutine` awaitable (what the async client hands back)"""

from __future__ import annotations

import asyncio
import collections.abc
import contextlib
import contextvars
import gc
import os
import subprocess
import sys
import threading
import typing as t
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


@contextlib.asynccontextmanager
async def _raw_server(
    handler: t.Callable[
        [asyncio.StreamReader, asyncio.StreamWriter], t.Awaitable[None]
    ],
) -> t.AsyncGenerator[str]:
    async with await asyncio.start_server(handler, "127.0.0.1", 0) as srv:
        port = srv.sockets[0].getsockname()[1]
        yield f"http://127.0.0.1:{port}/"


def _fd_count() -> int:
    return len(os.listdir("/proc/self/fd"))


async def _get_status(client: ry.Client, url: str) -> int:
    response = await client.get(url)
    await response.bytes()
    return response.status


class TestShape:
    """R1/R2 ~ looks like a coroutine to python + asyncio"""

    @pytest.mark.anyio
    async def test_is_coroutine(self, server: ReqtestServer) -> None:
        coro = ry.Client().get(str(server.url) + "howdy")
        assert isinstance(coro, collections.abc.Coroutine)
        assert asyncio.iscoroutine(coro)
        assert isinstance(coro, collections.abc.Awaitable)
        assert type(coro).__name__ == "Coroutine"
        assert type(coro).__module__ == "ry.ryo3"
        await coro

    @pytest.mark.anyio
    async def test_names(self, server: ReqtestServer) -> None:
        coro: t.Any = ry.Client().get(str(server.url) + "howdy")
        assert coro.__qualname__ == "Client.get"
        assert coro.__name__ == "get"
        assert repr(coro).startswith("<ry.Coroutine Client.get at 0x")
        assert repr(coro).endswith(f"{id(coro):x}>")
        await coro

    @pytest.mark.anyio
    @pytest.mark.parametrize(
        "method", ["get", "post", "put", "delete", "head", "options", "patch"]
    )
    async def test_method_qualnames(self, server: ReqtestServer, method: str) -> None:
        coro: t.Any = getattr(ry.Client(), method)(str(server.url) + "echo")
        assert coro.__qualname__ == f"Client.{method}"
        assert coro.__name__ == method
        response = await coro
        assert response.status == 200

    @pytest.mark.anyio
    async def test_fetch_qualname(self, server: ReqtestServer) -> None:
        coro: t.Any = ry.fetch(str(server.url) + "howdy")
        assert coro.__qualname__ == "fetch"
        assert coro.__name__ == "fetch"
        await coro
        coro = ry.Client().fetch(str(server.url) + "howdy")
        assert coro.__qualname__ == "Client.fetch"
        await coro

    @pytest.mark.anyio
    async def test_task_repr(self, server: ReqtestServer) -> None:
        task = asyncio.ensure_future(ry.Client().post(str(server.url) + "echo"))
        assert "coro=<Client.post()>" in repr(task)
        await task

    @pytest.mark.anyio
    async def test_response_method_qualnames(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        for name in ("bytes", "text", "json"):
            response = await client.get(url)
            coro = getattr(response, name)()
            assert coro.__qualname__ == f"Response.{name}"
            assert isinstance(coro, collections.abc.Coroutine)
            with contextlib.suppress(ValueError):  # json of howdy...
                await coro


class TestLazy:
    """R3 ~ nothing happens until it is awaited"""

    @pytest.mark.anyio
    @pytest.mark.parametrize("how", ["close", "drop"])
    async def test_never_awaited_sends_nothing(self, how: str) -> None:
        hits = 0

        async def _count(
            _reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            nonlocal hits
            hits += 1
            writer.close()

        async with _raw_server(_count) as url:
            client = ry.Client()
            coros = [client.get(url) for _ in range(8)]
            if how == "close":
                for coro in coros:
                    coro.close()
            del coros
            gc.collect()
            await asyncio.sleep(0.1)
        assert hits == 0


class TestResults:
    """R4 ~ values via StopIteration, errs as-is, single use"""

    @pytest.mark.anyio
    async def test_await_twice_raises(self, server: ReqtestServer) -> None:
        coro = ry.Client().get(str(server.url) + "howdy")
        response = await coro
        assert response.status == 200
        with pytest.raises(RuntimeError, match="already awaited"):
            await coro

    @pytest.mark.anyio
    async def test_err_raises_as_is(self) -> None:
        # nothing listening there
        with pytest.raises(ry.ReqwestError):
            await ry.Client().get("http://127.0.0.1:1/")

    @pytest.mark.anyio
    async def test_body_consumed_twice(self, server: ReqtestServer) -> None:
        response = await ry.Client().get(str(server.url) + "howdy")
        await response.bytes()
        with pytest.raises(Exception, match="consumed"):  # noqa: PT011
            await response.bytes()


class TestThrowClose:
    """R5 ~ throw/close drop the future"""

    @pytest.mark.anyio
    async def test_throw_reraises(self, server: ReqtestServer) -> None:
        coro = ry.Client().get(str(server.url) + "howdy")
        err = ValueError("nope")
        with pytest.raises(ValueError, match="nope") as exc_info:
            coro.throw(err)
        assert exc_info.value is err
        with pytest.raises(RuntimeError, match="cannot reuse already awaited"):
            coro.send(None)

    @pytest.mark.anyio
    async def test_throw_legacy_signature(self, server: ReqtestServer) -> None:
        coro = ry.Client().get(str(server.url) + "howdy")
        with pytest.raises(KeyError, match="dingo"):
            coro.throw(KeyError, "dingo", None)
        with pytest.raises(RuntimeError, match="already awaited"):
            coro.send(None)

    @pytest.mark.anyio
    async def test_throw_mid_flight(self) -> None:
        got_request = asyncio.Event()
        conn_closed = asyncio.Event()

        async def _never_answer(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            got_request.set()
            await reader.read()
            conn_closed.set()
            writer.close()

        async with _raw_server(_never_answer) as url:
            coro = ry.Client().get(url)
            pending = coro.send(None)
            assert pending is not None
            await asyncio.wait_for(got_request.wait(), 5)
            with pytest.raises(ValueError, match="nope"):
                coro.throw(ValueError("nope"))
            await asyncio.wait_for(conn_closed.wait(), 5)

    @pytest.mark.anyio
    async def test_close_then_send(self, server: ReqtestServer) -> None:
        coro = ry.Client().get(str(server.url) + "howdy")
        coro.close()
        coro.close()  # idempotent
        with pytest.raises(RuntimeError, match="already awaited"):
            coro.send(None)


class TestAsyncioCompat:
    @pytest.mark.anyio
    async def test_create_task_gather(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        tasks = [asyncio.create_task(_get_status(client, url)) for _ in range(16)]
        assert await asyncio.gather(*tasks) == [200] * 16
        responses = await asyncio.gather(*(client.get(url) for _ in range(16)))
        assert all(r.status == 200 for r in responses)

    @pytest.mark.anyio
    async def test_wait_for(self, server: ReqtestServer) -> None:
        response = await asyncio.wait_for(ry.Client().get(str(server.url) + "howdy"), 5)
        assert response.status == 200

    @pytest.mark.anyio
    async def test_timeout(self, server: ReqtestServer) -> None:
        async with asyncio.timeout(5):
            response = await ry.Client().get(str(server.url) + "howdy")
        assert response.status == 200

        async def _slow_read() -> None:
            async with asyncio.timeout(0.05):
                response = await ry.Client().get(str(server.url) + "slow")
                await response.bytes()

        with pytest.raises(TimeoutError):
            await _slow_read()

    @pytest.mark.anyio
    async def test_task_group(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        async with asyncio.TaskGroup() as tg:
            tasks = [tg.create_task(_get_status(client, url)) for _ in range(8)]
        assert [task.result() for task in tasks] == [200] * 8

    @pytest.mark.anyio
    async def test_task_group_sibling_failure(self, server: ReqtestServer) -> None:
        client = ry.Client()

        async def _boom() -> None:
            await asyncio.sleep(0.05)
            msg = "boom"
            raise ValueError(msg)

        async def _slow() -> None:
            response = await client.get(str(server.url) + "slow")
            await response.bytes()

        tasks: list[asyncio.Task[None]] = []

        async def _group() -> None:
            async with asyncio.TaskGroup() as tg:
                tasks.append(tg.create_task(_slow()))
                tasks.append(tg.create_task(_boom()))

        with pytest.raises(ExceptionGroup) as exc_info:
            await _group()
        assert exc_info.group_contains(ValueError)
        assert tasks[0].cancelled()

    @pytest.mark.anyio
    async def test_cancel(self, server: ReqtestServer) -> None:
        task = asyncio.ensure_future(ry.Client().get(str(server.url) + "slow"))
        await asyncio.sleep(0)
        task.cancel("adios")
        with pytest.raises(asyncio.CancelledError):
            await task
        assert task.cancelled()

    @pytest.mark.anyio
    async def test_cancel_before_first_step(self, server: ReqtestServer) -> None:
        task = asyncio.ensure_future(ry.Client().get(str(server.url) + "howdy"))
        task.cancel()
        with pytest.raises(asyncio.CancelledError):
            await task

    @pytest.mark.anyio
    async def test_remove_done_callback_bound_method(
        self, server: ReqtestServer
    ) -> None:
        """matches w/ `==` like asyncio.Future (bound methods are new objects)"""

        class _Cb:
            def cb(self, fut: t.Any) -> None: ...

        obj = _Cb()
        coro = ry.Client().get(str(server.url) + "slow")
        fut = coro.send(None)
        fut.add_done_callback(obj.cb)
        fut.add_done_callback(obj.cb)
        assert fut.remove_done_callback(obj.cb) == 2
        assert fut.remove_done_callback(obj.cb) == 0
        coro.close()

    @pytest.mark.anyio
    async def test_contextvars(self, server: ReqtestServer) -> None:
        """R12 ~ runs in the awaiting task's context (body pump too)"""
        cv: contextvars.ContextVar[str] = contextvars.ContextVar("cv")
        cv.set("dingo")

        async def _body() -> t.AsyncGenerator[bytes]:
            yield cv.get("nope").encode()

        response = await ry.Client().post(str(server.url) + "echo", body=_body())
        assert (await response.json())["body"] == "dingo"
        assert cv.get() == "dingo"

        async def _set_and_get() -> str:
            cv.set("bluey")
            await ry.Client().get(str(server.url) + "howdy")
            return cv.get()

        assert await asyncio.create_task(_set_and_get()) == "bluey"
        assert cv.get() == "dingo"

    @pytest.mark.anyio
    async def test_cancel_aborts_the_request(self) -> None:
        """cancel == drop ~ the tokio task gets aborted so the connection dies"""
        got_request = asyncio.Event()
        conn_closed = asyncio.Event()

        async def _never_answer(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            got_request.set()
            # blocks until the client hangs up
            await reader.read()
            conn_closed.set()
            writer.close()

        async with _raw_server(_never_answer) as url:
            client = ry.Client()
            task = asyncio.ensure_future(client.get(url))
            await asyncio.wait_for(got_request.wait(), 5)
            task.cancel()
            with pytest.raises(asyncio.CancelledError):
                await task
            await asyncio.wait_for(conn_closed.wait(), 5)

    @pytest.mark.anyio
    async def test_timeout_aborts_the_request(self) -> None:
        conn_closed = asyncio.Event()

        async def _never_answer(
            reader: asyncio.StreamReader, writer: asyncio.StreamWriter
        ) -> None:
            await reader.readuntil(b"\r\n\r\n")
            await reader.read()
            conn_closed.set()
            writer.close()

        async with _raw_server(_never_answer) as url:
            client = ry.Client()
            with pytest.raises(TimeoutError):
                await asyncio.wait_for(client.get(url), 0.1)
            await asyncio.wait_for(conn_closed.wait(), 5)


class TestStress:
    """R8 ~ no lost wakes"""

    @pytest.mark.anyio
    async def test_gather_rounds(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"

        async def _round() -> None:
            statuses = await asyncio.gather(
                *(_get_status(client, url) for _ in range(500))
            )
            assert statuses == [200] * 500

        for _ in range(20):
            await asyncio.wait_for(_round(), 30)

    @pytest.mark.anyio
    async def test_interleaved_sleeps(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"

        async def _one(i: int) -> int:
            for _ in range(i % 4):
                await asyncio.sleep(0)
            coro = client.get(url)
            await asyncio.sleep(0)
            response = await coro
            await asyncio.sleep(0)
            body = await response.bytes()
            return len(body)

        for _ in range(5):
            lens = await asyncio.wait_for(
                asyncio.gather(*(_one(i) for i in range(400))), 30
            )
            assert len(set(lens)) == 1

    @pytest.mark.anyio
    async def test_stream_stress(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "long"
        expected = "".join([f"howdy partner {i}\n" for i in range(100)]).encode()

        async def _one() -> bytes:
            response = await client.get(url)
            chunks = []
            async for chunk in response.stream():
                chunks.append(bytes(chunk))
                await asyncio.sleep(0)
            return b"".join(chunks)

        for _ in range(3):
            bodies = await asyncio.wait_for(
                asyncio.gather(*(_one() for _ in range(100))), 30
            )
            assert all(b == expected for b in bodies)


class TestLoops:
    """R10 ~ loops come and go"""

    def test_asyncio_run_many_no_fd_growth(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        for _ in range(10):
            assert asyncio.run(_get_status(client, url)) == 200
        gc.collect()
        before = _fd_count()
        for _ in range(300):
            assert asyncio.run(_get_status(client, url)) == 200
        gc.collect()
        # some slack for the connection pool
        assert _fd_count() - before <= 4

    def test_threads_own_loops_shared_client(self, server: ReqtestServer) -> None:
        client = ry.Client()
        url = str(server.url) + "howdy"
        errors: list[BaseException] = []
        results: list[list[int]] = []

        async def _many() -> list[int]:
            return await asyncio.gather(*(_get_status(client, url) for _ in range(64)))

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
        assert all(s == 200 for statuses in results for s in statuses)

    def test_uvloop(self, server: ReqtestServer) -> None:
        uvloop = pytest.importorskip("uvloop")
        client = ry.Client()
        url = str(server.url) + "howdy"

        async def _many() -> list[int]:
            return await asyncio.gather(*(_get_status(client, url) for _ in range(64)))

        with asyncio.Runner(loop_factory=uvloop.new_event_loop) as runner:
            assert runner.run(_many()) == [200] * 64

    def test_closed_loop_in_flight(self, server: ReqtestServer) -> None:
        """close a loop w/ requests in flight ~ no hang/crash, next loop is fine"""
        client = ry.Client()
        url = str(server.url)

        async def _slow() -> None:
            response = await client.get(url + "slow")
            async for _ in response.stream():
                pass

        loop = asyncio.new_event_loop()
        try:
            tasks = [loop.create_task(_slow()) for _ in range(16)]
            loop.run_until_complete(asyncio.sleep(0.3))
        finally:
            loop.close()
        assert not any(task.done() for task in tasks)
        del loop, tasks
        gc.collect()
        assert asyncio.run(_get_status(client, url + "howdy")) == 200

    def test_exit_with_requests_in_flight(self, server: ReqtestServer) -> None:
        """interpreter going down w/ stuff in flight doesnt crash"""
        script = """
import asyncio, sys
import ry

async def main(url):
    client = ry.Client()
    tasks = [asyncio.ensure_future(client.get(url)) for _ in range(32)]
    await asyncio.sleep(0.2)

loop = asyncio.new_event_loop()
loop.run_until_complete(main(sys.argv[1]))
loop.close()
"""
        res = subprocess.run(  # noqa: S603
            [sys.executable, "-c", script, str(server.url) + "slow"],
            capture_output=True,
            timeout=30,
            check=False,
        )
        assert res.returncode == 0, res.stderr.decode()


class TestFatalErrors:
    def test_sys_exit_in_background_task_stops_loop(
        self, server: ReqtestServer
    ) -> None:
        """`SystemExit` out of a task step woken by ry stops the loop (not swallowed)"""
        script = """
import asyncio, sys
import ry

async def bg(url):
    await ry.Client().get(url)
    sys.exit(3)

async def main(url):
    task = asyncio.create_task(bg(url))
    await asyncio.sleep(30)

asyncio.run(main(sys.argv[1]))
"""
        res = subprocess.run(  # noqa: S603
            [sys.executable, "-c", script, str(server.url) + "howdy"],
            capture_output=True,
            timeout=20,
            check=False,
        )
        assert res.returncode == 3, res.stderr.decode()
        assert b"Exception ignored" not in res.stderr


class TestFallback:
    """R9 ~ loops w/o `add_reader` still work (call_soon_threadsafe path)"""

    def test_no_add_reader(self, server: ReqtestServer) -> None:
        class _NoReaderLoop(asyncio.SelectorEventLoop):
            def add_reader(self, *args: t.Any, **kwargs: t.Any) -> t.Any:  # type: ignore[override]
                raise NotImplementedError

        client = ry.Client()
        url = str(server.url)

        async def _main() -> None:
            statuses = await asyncio.gather(
                *(_get_status(client, url + "howdy") for _ in range(200))
            )
            assert statuses == [200] * 200
            response = await client.get(url + "long")
            assert len([c async for c in response.stream()]) >= 1

            async def _body() -> t.AsyncGenerator[bytes]:
                for _ in range(64):
                    yield b"x" * 1024

            response = await client.post(url + "echo", body=_body())
            assert len((await response.json())["body"]) == 64 * 1024

        with asyncio.Runner(loop_factory=_NoReaderLoop) as runner:
            runner.run(asyncio.wait_for(_main(), 30))
