import asyncio

from ry.dev import aiopen


async def main():
    async with await aiopen("file.txt", "w") as f:
        await f.write(b"hello\n")

    async with await aiopen("file.txt", "r") as f:
        data = await f.read()
        datab = data.to_bytes()
        print(data)  # noqa: T201
        print(datab)  # noqa: T201
        print(datab.decode())  # noqa: T201
        print(data.decode())  # noqa: T201


asyncio.run(main())
