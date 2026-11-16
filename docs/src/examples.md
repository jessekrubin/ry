# Examples

- [async_open](#async_open)
- [fspath](#fspath)
- [get](#get)
- [http_fetch](#http_fetch)
- [jiff](#jiff)
- [sqlformat](#sqlformat)
- [walking](#walking)
- [xxhashing](#xxhashing)

## async_open

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

## fspath

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

## get

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

## http_fetch

```python
"""Example of using the `ry.fetch` function to make http requests

The stuff at the top of this file is a simple http server for example purposes
"""

from __future__ import annotations

import asyncio
import json
from http.server import BaseHTTPRequestHandler, HTTPServer
from threading import Thread
from typing import Any

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
    host: str = "127.0.0.1", port: int = 8888, *, logging: bool = False
) -> HTTPServer:
    class HttpRequestHandlerNoLog(HTTPRequestHandler):
        def log_message(self, format: str, *args: Any) -> None: ...

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

## jiff

```python
"""Jiff examples (v3)

Translated jiff-examples from jiff-v1.0's docs

REF: https://docs.rs/jiff/1/jiff/#examples
DATE: 2026-11-16
"""

from __future__ import annotations

import json
from dataclasses import dataclass

import ry


def test_quick_example() -> None:
    time = ry.Timestamp.parse("2024-07-11T01:14:00Z")
    zoned = time.in_tz("America/New_York").add(ry.TimeSpan(months=1, hours=2))
    assert str(zoned) == "2024-08-10T23:14:00-04:00[America/New_York]"
    # Or, if you want an RFC3339 formatted string:
    assert str(zoned.timestamp()) == "2024-08-11T03:14:00Z"


def test_get_current_time_in_system_tz() -> None:
    now = ry.ZonedDateTime.now()
    assert isinstance(now, ry.ZonedDateTime)
    assert now.tz.name


def test_print_current_time_rounded_to_second() -> None:
    rounded = ry.ZonedDateTime.now().round("second")
    # nanoseconds should be zero after rounding to second
    assert rounded.nanosecond == 0


def test_print_todays_date_at_specific_time() -> None:
    zdt = ry.ZonedDateTime.now().replace(hour=14, minute=0, second=0, nanosecond=0)
    assert (zdt.hour, zdt.minute, zdt.second, zdt.nanosecond) == (14, 0, 0, 0)

    # or w/ a `Time`
    zdt = ry.ZonedDateTime.now().replace(time=ry.time(14, 0, 0, 0))
    assert zdt.time() == ry.time(14, 0, 0, 0)


def test_print_current_unix_timestamp() -> None:
    ts = ry.Timestamp.now()
    sec = ts.as_second()
    assert isinstance(sec, int) and sec > 1_600_000_000
    # nanosecond count divided by 1e9 should equal seconds
    assert ts.as_nanosecond() // 1_000_000_000 == sec


def test_print_datetime_for_a_timestamp() -> None:
    ts = ry.Timestamp.from_millisecond(1_720_646_365_567)
    zdt = ts.to_zoned_datetime(ry.TimeZone("America/New_York"))
    assert str(zdt) == "2024-07-10T17:19:25.567-04:00[America/New_York]"
    # Or if you just want the RFC 3339 time without bothering with time zones:
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
    assert f"{span:#}" == "29341h 3m"
    # bigger units (not reversible) via `until` w/ `largest`
    span2 = a.until(b, largest="year")
    assert f"{span2:#}" == "3y 4mo 5d 12h 3m"


def test_add_duration_to_a_zoned_datetime() -> None:
    start = ry.date(2020, 8, 26).at(6, 27, 0, 0).in_tz("America/New_York")
    span = ry.TimeSpan(years=3, months=4, days=5, hours=12, minutes=3)
    finish = start.add(span)  # python ~ everything is `checked`
    assert str(finish) == "2023-12-31T18:30:00-05:00[America/New_York]"


def test_dealing_with_ambiguity() -> None:
    # 2:30 on 2024-03-10 in New York didn't exist. It's a "gap."
    # The compatible strategy selects the datetime after the gap.
    gap = ry.date(2024, 3, 10).at(2, 30, 0, 0).in_tz("America/New_York")
    assert str(gap) == "2024-03-10T03:30:00-04:00[America/New_York]"

    # 1:30 on 2024-11-03 in New York appeared twice. It's a "fold."
    # The compatible strategy selects the datetime before the fold.
    fold = ry.date(2024, 11, 3).at(1, 30, 0, 0).in_tz("America/New_York")
    assert str(fold) == "2024-11-03T01:30:00-04:00[America/New_York]"


def test_parsing_a_span() -> None:
    expected = ry.TimeSpan(years=5, weeks=1, days=10, hours=5, minutes=59)

    # ISO 8601
    iso = ry.TimeSpan.parse("P5y1w10dT5h59m")
    assert iso == expected
    assert str(iso) == "P5Y1W10DT5H59M"

    # "friendly" format(s)
    for friendly in (
        "5 years, 1 week, 10 days, 5 hours, 59 minutes",
        "5yrs 1wk 10d 5hrs 59mins",
        "5y 1w 10d 5h 59m",
    ):
        assert ry.TimeSpan.parse(friendly) == expected
    assert f"{expected:#}" == "5y 1w 10d 5h 59m"
    assert expected.friendly() == "5y 1w 10d 5h 59m"


def test_parsing_an_rfc2822_datetime_string() -> None:
    base = ry.ZonedDateTime.parse_rfc2822("Thu, 29 Feb 2024 05:34 -0500")
    hobart = base.in_tz("Australia/Hobart")
    kolkata = base.in_tz("Asia/Kolkata")
    assert hobart.format_rfc2822() == "Thu, 29 Feb 2024 21:34:00 +1100"
    assert kolkata.format_rfc2822() == "Thu, 29 Feb 2024 16:04:00 +0530"


def test_using_strftime_and_strptime() -> None:
    # NOTE: ry's arg order is (string, format) like python's `strptime`
    zdt = ry.ZonedDateTime.strptime(
        "Monday, July 15, 2024 at 5:30pm America/New_York",
        "%A, %B %d, %Y at %I:%M%p %Q",
    )
    assert str(zdt) == "2024-07-15T17:30:00-04:00[America/New_York]"

    hobart = ry.date(2024, 7, 15).at(17, 30, 59, 0).in_tz("Australia/Hobart")
    # %-I instead of %I means no padding; %Z is the tz abbreviation (not
    # parsable since abbreviations are ambiguous)
    formatted = hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Z")
    assert formatted == "Monday, July 15, 2024 at 5:30pm AEST"
    # %Q is the IANA tz name (parsable)
    formatted = hobart.strftime("%A, %B %d, %Y at %-I:%M%P %Q")
    assert formatted == "Monday, July 15, 2024 at 5:30pm Australia/Hobart"


@dataclass
class Record:
    timestamp: ry.Timestamp

    def to_json(self) -> str:
        return json.dumps(
            {"timestamp": self.timestamp.as_second()}, separators=(",", ":")
        )

    @classmethod
    def from_json(cls, raw: str) -> Record:
        data = json.loads(raw)
        return cls(timestamp=ry.Timestamp.from_second(data["timestamp"]))


def test_serializing_and_deserializing_integer_timestamps() -> None:
    raw = '{"timestamp":1517644800}'
    got = Record.from_json(raw)
    assert got.timestamp == ry.Timestamp.from_second(1_517_644_800)
    assert got.to_json() == raw


def main() -> None:
    test_quick_example()
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

## sqlformat

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

## walking

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
    for direntry in ry.walkdir(dir2walk):
        print(direntry, type(direntry))

    _print_br("Only files")
    # walking only files
    for filepath in ry.walkdir(dir2walk, dirs=False):
        print(filepath)
        assert ry.FsPath(filepath).is_file()

    _print_br("Only directories")
    # walking only directories
    for filepath in ry.walkdir(dir2walk, files=False):
        print(filepath)
        assert ry.FsPath(filepath).is_dir()

    _print_br("Glob pattern")
    # globset/globster
    for filepath in ry.walkdir(
        dir2walk,
        glob=ry.globster([
            "*.py",
        ]),
    ):
        assert str(filepath).endswith(".py")


if __name__ == "__main__":
    main()
```

## xxhashing

```python
"""Example o' xxhash-ing files in this directory both async/sync

Demonstrates:
  - files are read via `ry.read_stream`/`ry.read_stream_async` in chunks
  - xxhashing is done via `ry.xxh64` hasher
  - files are found via `ry.glob`
  - timing is done via `ry.instant()` to compute `ry.Duration`
  - timing is friendly printed w/ `{duration:#}` (see `ry.Duration.__format__`)

"""

import asyncio

import ry
from ry import xxh64

_PWD = ry.FsPath(__file__).resolve().parent
_FILES = ry.glob(str(_PWD / "**" / "*.py"), dtype=ry.FsPath).collect()


def hash_file_sync(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    for chunk in ry.read_stream(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


def hash_examples_sync() -> list[tuple[str, str]]:
    return [hash_file_sync(f) for f in _FILES]


async def hash_file_async(path: ry.FsPath) -> tuple[str, str]:
    hasher = xxh64()

    async for chunk in ry.read_stream_async(path):
        hasher.update(chunk)
    return str(path), hasher.hexdigest()


async def hash_examples_async() -> list[tuple[str, str]]:
    return await asyncio.gather(*(hash_file_async(f) for f in _FILES))


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
    main()
```
