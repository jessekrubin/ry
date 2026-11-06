"""WebSocket payload throughput, excluding connection setup and teardown.

Build both revisions in release mode, then run this file against each package::

    PYTHONPATH=/path/to/before/python uv run pytest -vv \
        bench/bench_websocket_throughput.py --benchmark-only \
        --benchmark-json=scratch/websocket-before.json
    PYTHONPATH=/path/to/after/python uv run pytest -vv \
        bench/bench_websocket_throughput.py --benchmark-only \
        --benchmark-json=scratch/websocket-after.json

The Python ``websockets`` peer shares the client's asyncio loop, so these rates
include Python work at both ends. Each benchmark operation transfers 4,096
messages. JSON ``extra_info`` records
messages/second and payload MiB/second; pytest's OPS column counts batches/second.
"""

from __future__ import annotations

import asyncio
import typing as t
from contextlib import AsyncExitStack

import pytest
from websockets.asyncio.server import serve

import ry

if t.TYPE_CHECKING:
    from pytest_benchmark.fixture import BenchmarkFixture
    from websockets.asyncio.server import ServerConnection
    from websockets.http11 import Request, Response

_MESSAGES_PER_BATCH = 4_096
_ACK = b"done"


async def _serve_messages(
    connection: ServerConnection,
    payload: bytes,
    direction: t.Literal["send", "receive"],
) -> None:
    if direction == "send":
        received = 0
        async for message in connection:
            assert message == payload
            received += 1
            if received % _MESSAGES_PER_BATCH == 0:
                await connection.send(_ACK)
    else:
        async for request in connection:
            assert request == _ACK
            for _ in range(_MESSAGES_PER_BATCH):
                await connection.send(payload)


async def _transfer(
    client: ry.WebSocket, payload: bytes, direction: t.Literal["send", "receive"]
) -> None:
    if direction == "send":
        for _ in range(_MESSAGES_PER_BATCH):
            await client.send(payload)
        assert bytes(await client.recv()) == _ACK
    else:
        await client.send(_ACK)
        message = None
        for _ in range(_MESSAGES_PER_BATCH):
            message = await client.recv()
        if message is None:
            msg = "Did not receive any message"
            raise RuntimeError(msg)
        assert bytes(message) == payload


@pytest.mark.benchmark(group="websocket-throughput")
@pytest.mark.parametrize("direction", ["send", "receive"])
@pytest.mark.parametrize("payload_size", [32, 4_096])
@pytest.mark.parametrize("extra_headers", [0, 16])
def test_websocket_throughput(
    benchmark: BenchmarkFixture,
    direction: t.Literal["send", "receive"],
    payload_size: int,
    extra_headers: int,
) -> None:
    assert ry.__opt_level__ == 3, "Throughput benchmarks require a release build"
    payload = b"x" * payload_size
    headers = {f"X-Bench-{index}": f"value-{index}" for index in range(extra_headers)}

    def add_headers(
        connection: ServerConnection, request: Request, response: Response
    ) -> Response:
        del connection, request
        response.headers.update(headers)
        return response

    async def handle(connection: ServerConnection) -> None:
        await _serve_messages(connection, payload, direction)

    with asyncio.Runner() as runner:
        stack = AsyncExitStack()

        async def connect() -> ry.WebSocket:
            server = await stack.enter_async_context(
                serve(
                    handle,
                    "127.0.0.1",
                    0,
                    process_response=add_headers,
                    compression=None,
                    ping_interval=None,
                )
            )
            port = server.sockets[0].getsockname()[1]
            return await stack.enter_async_context(
                ry.WebSocket(f"ws://127.0.0.1:{port}")
            )

        try:
            client = runner.run(connect())
            handshake_headers = client.headers
            assert handshake_headers is not None

            def run_batch() -> None:
                runner.run(_transfer(client, payload, direction))

            benchmark.pedantic(  # type: ignore[no-untyped-call]
                run_batch, rounds=7, warmup_rounds=2, iterations=1
            )
            if benchmark.stats is not None:
                seconds = benchmark.stats.stats.median
                benchmark.extra_info.update(
                    messages_per_batch=_MESSAGES_PER_BATCH,
                    payload_bytes=payload_size,
                    extra_headers=extra_headers,
                    response_headers=len(handshake_headers),
                    direction=direction,
                    messages_per_second=_MESSAGES_PER_BATCH / seconds,
                    payload_mib_per_second=_MESSAGES_PER_BATCH
                    * payload_size
                    / seconds
                    / (1024 * 1024),
                    package=ry.__file__,
                    peer="same-loop-python-websockets",
                )
        finally:
            runner.run(stack.aclose())
