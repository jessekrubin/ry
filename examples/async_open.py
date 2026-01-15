"""Async-file ops.

Demos:
- async file write/read
- async iteration o-lines
- async chunked reading (via read_stream_async)
- seeking/truncating

"""

import asyncio

import ry


async def main() -> None:
    # write
    async with ry.aopen("stuff-async.txt", mode="wb") as f:
        await f.write(b"hello\n")
        await f.write(b"world\n")

    # read
    async with ry.aopen("stuff-async.txt", mode="rb") as f:
        content = await f.read()
        print(f"data: {content!r}")

    # reading lines
    print("Iterating over lines:")
    async with ry.aopen("stuff-async.txt", mode="rb") as f:
        async for line in f:
            print(f">>> line: {line!r}")

    # reading chunks (w/ chunk_size of 4 (bytes))
    stream = await ry.read_stream_async("stuff-async.txt", chunk_size=4)
    async for chunk in stream:
        print(f">>> chunk: {chunk!r}")

    # seeking/truncating
    async with ry.aopen("stuff-async.txt", mode="rb+") as f:
        await f.seek(6)
        word = await f.read(5)
        print(f"read (seek-6): {word!r}")

        # truncate
        await f.truncate(6)

    # we good?
    async with ry.aopen("stuff-async.txt", mode="rb") as f:
        final_content = await f.read()
        assert final_content == b"hello\n"
        print(f"final content after truncate: {final_content!r}")

    # housekeeping
    await ry.remove_file_async("stuff-async.txt")


if __name__ == "__main__":
    asyncio.run(main())
