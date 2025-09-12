import asyncio

import ry

try:
    from rich import print  # noqa: A004
except ImportError:
    ...


async def main() -> None:
    response = await ry.fetch("https://httpbin.org/anything")
    print("Raw response:", response)
    print("socket:", response.remote_addr)
    print("url:", response.url)
    print("status:", response.status)
    print("headers:", response.headers)
    print("http-version:", response.http_version)
    print("content-length:", response.content_length)
    json_data = await response.json()
    print("JSON data: ", json_data)

    print("stringified: ", ry.stringify(json_data, fmt=True).decode())


if __name__ == "__main__":
    asyncio.run(main())
