# async_open

```python
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

    # reading chunks (w/ read_size of 4 (bytes))
    stream = await ry.read_stream_async("stuff-async.txt", read_size=4)
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
```

# fspath

```python
import ry

# get current directory
current_dir = ry.FsPath.cwd()

# write file
(current_dir / "test.txt").write_text("data!")

# read file
data = (current_dir / "test.txt").read_text()
print(data)
```

# get

```python
"""Example of `ry.fetch` (async) and `ry.fetch_sync` (blocking)"""

import asyncio

import ry

try:
    from rich import print as echo
except ImportError:
    echo = print  # type: ignore[assignment]


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
```

# http_fetch

```python
"""Example of using the `ry.fetch` function to make http requests

The stuff at the top of this file is a simple http server for example purposes
"""

from __future__ import annotations

import asyncio
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from threading import Thread

# =============================================================================
import ry


def _print_break() -> None:
    print("\n" + "=" * 79 + "\n")


async def main(server_url: str = "http://127.0.0.1:8000") -> None:
    # -------------------------------------------------------------------------
    # GET
    # -------------------------------------------------------------------------
    _print_break()
    response = await ry.fetch(server_url)
    print("Raw response:", response)
    json_data = await response.json()
    print("JSON data:\n", json.dumps(json_data, indent=2))

    # **THE RESPONSE HAS BEEN CONSUMED**
    # No you cannot get the json again.... You are responsible for storing
    # The response has been consumed. This is how http requests really work.
    # Libraries like requests, httpx, aiohttp, etc. store the response data
    # in memory so you can access it multiple times, ry mirrors how fetch
    # works in reqwest which is also kinda how fetch works in
    # jawascript/interface-script.
    try:
        _json_data = await response.json()
    except ValueError as e:
        print("Error:", e)

    # -------------------------------------------------------------------------
    # POST
    # -------------------------------------------------------------------------
    _print_break()
    post_response = await ry.fetch(
        server_url, method="POST", body=b"post post post... dumb as a post"
    )
    print("Raw post response:", post_response)
    post_response_data = await post_response.json()
    print("JSON post response:\n", json.dumps(post_response_data, indent=2))

    # -------------------------------------------------------------------------
    # STREAMING
    # -------------------------------------------------------------------------
    _print_break()
    long_body = "\n".join([f"dingo{i}" for i in range(1000)]).encode()
    response = await ry.fetch(server_url, method="POST", body=long_body)

    async for chunk in response.bytes_stream():
        assert isinstance(chunk, ry.Bytes)  # tis a bytes
        py_bytes = bytes(chunk)
        assert isinstance(py_bytes, bytes)
        assert py_bytes == chunk
        print("chunk this! len =:", len(chunk))


# -----------------------------------------------------------------------------
# HTTP SERVER THAT DOES SUPER SIMPLE JSON RESPONSES
# -----------------------------------------------------------------------------
class HTTPRequestHandler(BaseHTTPRequestHandler):
    def do_GET(self) -> None:
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        res_data = {
            "path": self.path,
            "method": "GET",
            "data": {
                "dog": "dingo",
                "oreo": "mcflurry",
            },
        }
        res_bytes = json.dumps(res_data).encode()
        self.wfile.write(res_bytes)

    def do_POST(self) -> None:
        self.send_response(200)
        self.send_header("Content-type", "application/json")
        self.end_headers()
        body = self.rfile.read(int(self.headers["Content-Length"]))

        res_data = {
            "path": self.path,
            "method": "POST",
            "body": body.decode(),
            "data": {
                "dog": "dingo",
                "oreo": "mcflurry",
            },
        }
        res_bytes = json.dumps(res_data).encode()
        self.wfile.write(res_bytes)


def start_server(
    host: str = "127.0.0.1", port: int = 8888, logging: bool = False
) -> HTTPServer:
    class HttpRequestHandlerNoLog(HTTPRequestHandler):
        def log_message(self, format, *args):  # type: ignore[no-untyped-def]
            ...

    server_address = (host, port)
    handler = HttpRequestHandlerNoLog if not logging else HTTPRequestHandler
    httpd = HTTPServer(server_address, handler)
    Thread(target=httpd.serve_forever, daemon=True).start()
    return httpd


if __name__ == "__main__":
    server = start_server(logging=True)
    try:
        asyncio.run(
            main(server_url=f"http://{server.server_name}:{server.server_port}")
        )
    except KeyboardInterrupt:
        print("KeyboardInterrupt")
    finally:
        server.shutdown()
```

# jiff_examples

```python
"""Jiff examples (v2)

Translated jiff-examples from jiff-v2's docs

REF: https://docs.rs/jiff/latest/jiff/#examples
DATE: 2025-05-23
"""

from __future__ import annotations

import json
from dataclasses import dataclass

import ry


def test_get_current_time_in_system_tz() -> None:
    now = ry.ZonedDateTime.now()
    assert isinstance(now, ry.ZonedDateTime)
    assert now.tz.name


def test_print_current_time_rounded_to_second() -> None:
    rounded = ry.ZonedDateTime.now().round("second")
    # nanoseconds should be zero after rounding to second
    assert rounded.nanosecond == 0


def test_print_todays_date_at_specific_time() -> None:
    zdt = ry.ZonedDateTime.now().replace(
        hour=14, minute=0, second=0, nanosecond=0
    )
    assert zdt.hour == 14 and zdt.minute == 0 and zdt.second == 0
    assert zdt.nanosecond == 0


def test_print_current_unix_timestamp() -> None:
    ts = ry.Timestamp.now()
    sec = ts.as_second()
    ns = ts.as_nanosecond()
    assert isinstance(sec, int) and sec > 1_600_000_000
    # nanosecond count divided by 1e9 should equal seconds
    assert ns // 1_000_000_000 == sec


def test_print_datetime_for_a_timestamp() -> None:
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned(ry.TimeZone("America/New_York"))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    assert str(ts) == "2024-07-10T21:19:25.567Z"


def test_create_zoned_datetime_from_civil_time() -> None:
    zdt = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    assert str(zdt) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_change_an_instant_from_one_timezone_to_another() -> None:
    paris = ry.date(1918, 11, 11).at(11, 0, 0, 0).in_tz("Europe/Paris")
    nyc = paris.in_tz("America/New_York")
    assert str(nyc) == "1918-11-11T06:00:00-05:00[America/New_York]"


def test_find_duration_between_two_zoned_datetimes() -> None:
    a = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    b = ry.date(2023, 12, 31).at(18, 30, 0, 0).in_tz("America/New_York")
    span = b - a
    assert str(span) == "PT29341H3M"
    # until: specify largest unit via helper
    span2 = a.until(b, largest="year")
    assert str(span2) == "P3Y4M5DT12H3M"


def test_add_duration_to_a_zoned_datetime() -> None:
    start = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    span = ry.TimeSpan()._years(3)._months(4)._days(5)._hours(12)._minutes(3)
    finish = start.add(span)  # previously `checked_add`
    assert str(finish) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    gap = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")
    assert str(gap) == "2024-03-10T03:30:00-04:00[America/New_York]"

    fold = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")
    assert str(fold) == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span() -> None:
    iso = ry.TimeSpan.parse("P5y1w10dT5h59m")
    expected = (
        ry.TimeSpan()._years(5)._weeks(1)._days(10)._hours(5)._minutes(59)
    )
    assert iso == expected
    assert str(iso) == "P5Y1W10DT5H59M"

    from_friendly = ry.TimeSpan.parse(
        "5 years, 1 week, 10 days, 5 hours, 59 minutes"
    )
    assert iso == from_friendly
    assert from_friendly.to_string(friendly=True) == "5y 1w 10d 5h 59m"
    assert from_friendly.friendly() == "5y 1w 10d 5h 59m"
    assert str(from_friendly) == "P5Y1W10DT5H59M"


def test_parsing_an_rfc2822_datetime_string() -> None:
    base = ry.ZonedDateTime.parse_rfc2822("Thu, 29 Feb 2024 05:34 -0500")
    tas = base.in_tz("Australia/Tasmania")
    kol = base.in_tz("Asia/Kolkata")
    assert tas.format_rfc2822() == "Thu, 29 Feb 2024 21:34:00 +1100"
    assert kol.format_rfc2822() == "Thu, 29 Feb 2024 16:04:00 +0530"


def test_using_strftime_and_strptime() -> None:
    zdt = ry.ZonedDateTime.strptime(
        "Monday, July 15, 2024 at 5:30pm US/Eastern",
        "%A, %B %d, %Y at %I:%M%p %Q",
    )
    assert str(zdt) == "2024-07-15T17:30:00-04:00[US/Eastern]"

    tas = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Tasmania")
    formatted = tas.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
    assert formatted == "Monday, July 15, 2024 at 5:30pm Australia/Tasmania"


@dataclass
class Record:
    timestamp: ry.Timestamp

    def to_json(self) -> str:
        return json.dumps({"timestamp": self.timestamp.as_second()})

    @classmethod
    def from_json(cls, raw: str) -> Record:
        data = json.loads(raw)
        return cls(timestamp=ry.Timestamp.from_second(data["timestamp"]))


def test_serializing_and_deserializing_integer_timestamps() -> None:
    src = Record(timestamp=ry.Timestamp.from_second(1_517_644_800))
    wire = src.to_json()
    got = Record.from_json(wire)
    assert got.timestamp == src.timestamp
    assert wire == '{"timestamp": 1517644800}'


def main() -> None:
    test_get_current_time_in_system_tz()
    test_print_current_time_rounded_to_second()
    test_print_todays_date_at_specific_time()
    test_print_current_unix_timestamp()
    test_print_datetime_for_a_timestamp()
    test_create_zoned_datetime_from_civil_time()
    test_change_an_instant_from_one_timezone_to_another()
    test_find_duration_between_two_zoned_datetimes()
    test_add_duration_to_a_zoned_datetime()
    test_dealing_with_ambiguity()
    test_parsing_a_span()
    test_parsing_an_rfc2822_datetime_string()
    test_using_strftime_and_strptime()
    test_serializing_and_deserializing_integer_timestamps()


if __name__ == "__main__":
    main()
```

# sqlformat

```python
import ry

QUERY_MBTILES_DUPLICATE_METADATA_ROWS = """
-- returns the rows that are duplicate (name, value) pairs
select name, value, count(*) as count from metadata group by name having count (*) > 1 and count (distinct value) = 1;
"""

assert (
    ry.sqlfmt(QUERY_MBTILES_DUPLICATE_METADATA_ROWS)
    == """
-- returns the rows that are duplicate (name, value) pairs
select
  name,
  value,
  count(*) as count
from
  metadata
group by
  name
having
  count (*) > 1
  and count (distinct value) = 1;
""".strip()
)


assert (
    ry.sqlfmt(QUERY_MBTILES_DUPLICATE_METADATA_ROWS, uppercase=True)
    == """
-- returns the rows that are duplicate (name, value) pairs
SELECT
  name,
  value,
  count(*) AS count
FROM
  metadata
GROUP BY
  name
HAVING
  count (*) > 1
  AND count (DISTINCT value) = 1;
""".strip()
)
```

# walking

```python
"""
ry.walkdir example
"""

from __future__ import annotations

import os

import ry

PWD = os.path.dirname(os.path.abspath(__file__))


def _print_br(s: str | None = None) -> None:
    print("_" * 79)
    if s:
        print(s)


def main() -> None:
    dir2walk = PWD

    _print_br("Walking the directory tree")
    # Walking the directory tree
    for filepath in ry.walkdir(dir2walk):
        print(filepath)

    _print_br("Walking the directory tree with entries")
    # Walking the directory tree
    for direntry in ry.walkdir(dir2walk, objects=True):
        print(direntry, type(direntry))

    _print_br("Walking the directory tree with depth 1")
    # walking only files
    for filepath in ry.walkdir(dir2walk, dirs=False):
        print(filepath)
        assert ry.FsPath(filepath).is_file()

    # walking only directories
    for filepath in ry.walkdir(dir2walk, files=False):
        print(filepath)
        assert ry.FsPath(filepath).is_dir()

    # globset/globster
    for filepath in ry.walkdir(
        dir2walk,
        glob=ry.globster([
            "*.py",
        ]),
    ):
        assert filepath.endswith(".py")


if __name__ == "__main__":
    main()
```

# xxhashing

```python
"""Example o' xxhash-ing files in this directory both async/sync

Demonstrates:
  - files are read via `ry.read_stream`/`ry.read_stream_async` in chunks
  - xxhashing is done via `ry.xxhash.xxh64` hasher
  - files are found via `ry.glob`
  - timing is done via `ry.instant()` to compute `ry.Duration`
  - timing is friendly printed w/ `{duration:#}` (see `ry.Duration.__format__`)

"""

import asyncio

import ry
from ry.xxhash import xxh64

_PWD = ry.FsPath(__file__).resolve().parent


def hash_file_sync(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    for chunk in ry.read_stream(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


def hash_examples_sync() -> list[tuple[str, str]]:
    files = ry.glob(str(_PWD / "**" / "*.py"), dtype=ry.FsPath).collect()

    return [hash_file_sync(f) for f in files]


async def hash_file_async(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    async for chunk in ry.read_stream_async(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


async def hash_examples_async() -> list[tuple[str, str]]:
    files = ry.glob(str(_PWD / "**" / "*.py"), dtype=ry.FsPath).collect()

    return await asyncio.gather(*(hash_file_async(f) for f in files))


def main_sync() -> tuple[list[tuple[str, str]], ry.Duration]:
    start = ry.instant()
    hahses_sync = hash_examples_sync()
    dt = start.elapsed()
    return hahses_sync, dt


async def main_async() -> tuple[list[tuple[str, str]], ry.Duration]:
    start = ry.instant()
    hahses_async = await hash_examples_async()
    dt = start.elapsed()
    return hahses_async, dt


def main() -> None:
    res_sync, dt_sync = main_sync()
    res_async, dt_async = asyncio.run(main_async())
    print(f"HASHING  (SYNC) TOOK: {dt_sync:#}")
    print(f"HASHING (ASYNC) TOOK: {dt_async:#}")
    assert sorted(res_sync) == sorted(res_async)
    print("SYNC/ASYNC RESULTS MATCH!")


if __name__ == "__main__":
    main_sync()

    asyncio.run(main_async())
```
