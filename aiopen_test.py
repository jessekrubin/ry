import asyncio

from ry.dev import aiopen


async def test_read_lines():
    # Test reading lines from a file
    with open("file.txt", "w") as f:
        f.write("line1\nline2\nline3\n")

    async with aiopen("file.txt") as f:
        async for line in f:
            print(line)  # noqa: T201


async def main():
    f = aiopen("file.txt", "w")
    print(f)  # noqa: T201
    await f.open()
    await f.write(b"hello\n")
    await f.close()

    # async with aiopen("file.txt") as f:
    #     await f.write(b"hello\n")

    await test_read_lines()

    # reading
    af = aiopen("file.txt")
    await af.open()
    data = await af.read()
    print(data)  # noqa: T201
    await af.close()

    async with aiopen("file.txt") as f:
        data = await f.read()
        datab = data.to_bytes()
        print(data)  # noqa: T201
        print(datab)  # noqa: T201
        print(datab.decode())  # noqa: T201
        print(data.decode())  # noqa: T201


asyncio.run(main())
