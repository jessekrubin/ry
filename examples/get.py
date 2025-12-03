"""Example of `ry.fetch` (async) and `ry.fetch_sync` (blocking)"""

import asyncio

import ry

try:
    from rich import print as echo
except ImportError:
    echo = print


async def main_async() -> None:
    response = await ry.fetch("https://httpbingo.org/anything")
    echo("Raw response:", response)
    echo("socket:", response.remote_addr)
    echo("url:", response.url)
    echo("status:", response.status)
    echo("headers:", response.headers)
    echo("http-version:", response.http_version)
    echo("content-length:", response.content_length)
    json_data = await response.json()
    echo("JSON data: ", json_data)
    echo("stringified: ", ry.stringify(json_data, fmt=True).decode())


def main_sync() -> None:
    response = ry.fetch_sync("https://httpbingo.org/anything")
    echo("Raw response:", response)
    echo("socket:", response.remote_addr)
    echo("url:", response.url)
    echo("status:", response.status)
    echo("headers:", response.headers)
    echo("http-version:", response.http_version)
    echo("content-length:", response.content_length)
    json_data = response.json()
    echo("JSON data: ", json_data)
    echo("stringified: ", ry.stringify(json_data, fmt=True).decode())


if __name__ == "__main__":
    echo("_________________")
    echo("~ ~ ~ ASYNC ~ ~ ~\n")
    asyncio.run(main_async())
    echo("\n_________________")
    echo("~ ~ ~ SYNC ~ ~ ~\n")
    main_sync()
