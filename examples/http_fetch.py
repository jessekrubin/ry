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
