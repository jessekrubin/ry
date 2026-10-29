from __future__ import annotations

import copy
import typing as t
from pathlib import Path
from typing import TYPE_CHECKING

import pytest

import ry

if TYPE_CHECKING:
    from .conftest import ReqtestServer


class TestFormPart:
    def test_text(self) -> None:
        p = ry.FormPart("hello")
        assert p.value == "hello"
        assert p.filename is None
        assert p.mime is None
        assert p.headers is None

    def test_bytes(self) -> None:
        p = ry.FormPart(b"data", filename="x.bin", mime="application/octet-stream")
        assert isinstance(p.value, ry.Bytes)
        assert p.value == b"data"
        assert p.filename == "x.bin"
        assert p.mime == "application/octet-stream"

    def test_buffer_types(self) -> None:
        for buf in (bytearray(b"abc"), memoryview(b"abc"), ry.Bytes(b"abc")):
            assert ry.FormPart(buf).value == b"abc"

    def test_path(self, tmp_path: Path) -> None:
        f = tmp_path / "a.txt"
        p = ry.FormPart(f)
        assert isinstance(p.value, Path)
        assert p.value == f
        assert p.filename == "a.txt"
        assert p.mime == "text/plain"
        assert ry.FormPart(f, mime="text/csv").mime == "text/csv"

    @pytest.mark.parametrize(
        ("value", "expected"),
        [(1, "1"), (1.5, "1.5"), (True, "true"), (False, "false")],
    )
    def test_scalars_stringified(self, value: t.Any, expected: str) -> None:
        assert ry.FormPart(value).value == expected

    def test_headers(self) -> None:
        p = ry.FormPart("x", headers={"x-custom": "1"})
        assert p.headers == ry.Headers({"x-custom": "1"})

    def test_overrides_from_part(self) -> None:
        p = ry.FormPart(b"data", filename="a.csv", mime="text/csv")
        p2 = ry.FormPart(p, filename="b.csv")
        assert p2.filename == "b.csv"
        assert p2.mime == "text/csv"
        assert p.filename == "a.csv"

    def test_eq(self) -> None:
        assert ry.FormPart("x") == ry.FormPart("x")
        assert ry.FormPart("x") != ry.FormPart("x", filename="f")

    def test_repr(self) -> None:
        p = ry.FormPart(b"data", filename="x.csv", mime="text/csv")
        assert repr(p) == 'FormPart(Bytes(b"data"), filename="x.csv", mime="text/csv")'

    @pytest.mark.parametrize("value", [object(), None, ("x.csv", b"data"), {"a": 1}])
    def test_bad_value(self, value: t.Any) -> None:
        with pytest.raises(TypeError, match="invalid form-data value type"):
            ry.FormPart(value)

    def test_unknown_kwarg(self) -> None:
        with pytest.raises(TypeError, match="unexpected keyword argument: nope"):
            ry.FormPart("x", nope=1)  # type: ignore[call-arg]
        with pytest.raises(TypeError, match="unexpected keyword argument: nope"):
            ry.FormData().append("a", "x", nope=1)  # type: ignore[call-arg]

    def test_none_kwargs_are_noops(self) -> None:
        p = ry.FormPart(b"x", filename="f.bin")
        assert ry.FormPart(p, filename=None, mime=None, headers=None) == p

    def test_bad_mime(self) -> None:
        with pytest.raises(ValueError, match="invalid mime type"):
            ry.FormPart("x", mime="not-a-mime")


class TestFormData:
    def test_empty(self) -> None:
        fd = ry.FormData()
        assert len(fd) == 0
        assert not fd
        assert fd.keys() == []

    def test_from_dict(self) -> None:
        fd = ry.FormData({"a": "x", "n": 1, "tags": ["1", "2"]})
        assert fd.items() == [("a", "x"), ("n", "1"), ("tags", "1"), ("tags", "2")]

    def test_from_pairs(self) -> None:
        fd = ry.FormData([("a", "1"), ("a", "2"), ("b", b"x")])
        assert fd.keys() == ["a", "a", "b"]
        assert fd.getall("a") == ["1", "2"]

    def test_from_form_data(self) -> None:
        fd = ry.FormData({"a": "x"})
        fd2 = ry.FormData(fd)
        assert fd == fd2
        fd2.append("b", "y")
        assert fd != fd2

    def test_str_is_not_pairs(self) -> None:
        with pytest.raises(TypeError, match="got str"):
            ry.FormData("ab")  # type: ignore[arg-type]  # ty: ignore[invalid-argument-type]

    def test_bad_pairs(self) -> None:
        with pytest.raises(TypeError, match="pairs"):
            ry.FormData([("a", "b", "c")])  # type: ignore[list-item]  # ty: ignore[invalid-argument-type]

    def test_append_get(self) -> None:
        fd = ry.FormData()
        fd.append("a", "1")
        fd.append("a", "2")
        assert fd.get("a") == "1"
        assert fd["a"] == "1"
        assert fd.getall("a") == ["1", "2"]
        assert fd.get("missing") is None
        assert fd.get("missing", 42) == 42
        assert "a" in fd
        assert "missing" not in fd

    def test_get_returns_part_for_non_plain(self) -> None:
        fd = ry.FormData()
        fd.append("f", b"data", filename="x.bin")
        fd.append("t", "text", filename="t.txt")
        assert isinstance(fd["f"], ry.FormPart)
        assert isinstance(fd["t"], ry.FormPart)

    def test_set_js_semantics(self) -> None:
        fd = ry.FormData([("a", "1"), ("b", "x"), ("a", "2"), ("c", "y")])
        fd.set("a", "new")
        assert fd.items() == [("a", "new"), ("b", "x"), ("c", "y")]
        fd.set("z", "appended")
        assert fd.keys() == ["a", "b", "c", "z"]

    def test_setitem(self) -> None:
        fd = ry.FormData([("a", "1"), ("a", "2")])
        fd["a"] = "3"
        assert fd.getall("a") == ["3"]

    def test_delete(self) -> None:
        fd = ry.FormData([("a", "1"), ("b", "x"), ("a", "2")])
        fd.delete("a")
        assert fd.keys() == ["b"]
        fd.delete("missing")  # no error
        del fd["b"]
        assert len(fd) == 0
        with pytest.raises(KeyError):
            del fd["b"]

    def test_getitem_missing(self) -> None:
        with pytest.raises(KeyError):
            _ = ry.FormData()["nope"]

    def test_iter(self) -> None:
        fd = ry.FormData([("a", "1"), ("a", "2"), ("b", "3")])
        assert list(fd) == ["a", "a", "b"]
        assert fd.values() == ["1", "2", "3"]

    def test_copy(self) -> None:
        fd = ry.FormData({"a": "1"})
        for c in (fd.copy(), copy.copy(fd), copy.deepcopy(fd)):
            assert c == fd
            assert c is not fd
            c.append("b", "2")
            assert "b" not in fd

    def test_percent_encode(self) -> None:
        assert ry.FormData().percent_encode is None
        fd = ry.FormData(percent_encode="noop")
        assert fd.percent_encode == "noop"
        assert fd != ry.FormData()
        with pytest.raises(ValueError, match="invalid percent-encode"):
            ry.FormData(percent_encode="nope")  # type: ignore[arg-type]  # ty: ignore[invalid-argument-type]

    def test_repr(self) -> None:
        fd = ry.FormData({"a": "x"}, percent_encode="noop")
        fd.append("f", b"d", filename="f.bin")
        assert (
            repr(fd)
            == 'FormData([("a", "x"), ("f", FormPart(Bytes(b"d"), filename="f.bin"))], percent_encode="noop")'
        )


# =============================================================================
# over the wire
# =============================================================================
def _parts_by_name(payload: dict[str, t.Any]) -> dict[str, dict[str, t.Any]]:
    return {p["name"]: p for p in payload["parts"]}


def _form_with_file(tmp_path: Path) -> ry.FormData:
    f = tmp_path / "hello.txt"
    f.write_text("file-contents")
    fd = ry.FormData({"a": "x", "n": 1})
    fd.append("f", f)
    fd.append("blob", b"\x00\x01", filename="b.bin", mime="application/octet-stream")
    fd.append("h", "hv", headers={"x-part": "yes"})
    return fd


def _check_form_with_file(payload: dict[str, t.Any]) -> None:
    assert payload["content_type"].startswith("multipart/form-data; boundary=")
    # everything has a known length ~ so content-length is set (not chunked)
    assert payload["content_length"] == str(payload["received_bytes"])
    parts = _parts_by_name(payload)
    assert parts["a"]["value"] == "x"
    assert parts["a"]["filename"] is None
    assert parts["n"]["value"] == "1"
    assert parts["f"]["value"] == "file-contents"
    assert parts["f"]["filename"] == "hello.txt"
    assert parts["f"]["content_type"] == "text/plain"
    assert parts["blob"]["value"] == "\x00\x01"
    assert parts["blob"]["filename"] == "b.bin"
    assert parts["blob"]["content_type"] == "application/octet-stream"
    assert parts["h"]["headers"]["x-part"] == "yes"


class TestFormDataAsync:
    @pytest.mark.anyio
    async def test_multipart_kwarg(self, server: ReqtestServer, tmp_path: Path) -> None:
        res = await ry.Client().post(
            server.url / "upload", multipart=_form_with_file(tmp_path)
        )
        assert res.status_code == 200
        _check_form_with_file(await res.json())

    @pytest.mark.anyio
    async def test_body_kwarg(self, server: ReqtestServer, tmp_path: Path) -> None:
        res = await ry.fetch(
            server.url / "upload", method="POST", body=_form_with_file(tmp_path)
        )
        assert res.status_code == 200
        _check_form_with_file(await res.json())

    @pytest.mark.anyio
    async def test_multipart_dict(self, server: ReqtestServer, tmp_path: Path) -> None:
        f = tmp_path / "img.png"
        f.write_bytes(b"\x89PNG")
        res = await ry.Client().post(
            server.url / "upload",
            multipart={"a": "x", "tags": ["1", "2"], "f": f},
        )
        payload = await res.json()
        assert [p["name"] for p in payload["parts"]] == ["a", "tags", "tags", "f"]
        f_part = payload["parts"][-1]
        assert f_part["filename"] == "img.png"
        assert f_part["content_type"] == "image/png"

    @pytest.mark.anyio
    async def test_reusable(self, server: ReqtestServer, tmp_path: Path) -> None:
        fd = _form_with_file(tmp_path)
        client = ry.Client()
        for _ in range(3):
            res = await client.post(server.url / "upload", multipart=fd)
            _check_form_with_file(await res.json())

    @pytest.mark.anyio
    async def test_snapshot_at_request_time(self, server: ReqtestServer) -> None:
        fd = ry.FormData({"a": "1"})
        coro = ry.Client().post(server.url / "upload", multipart=fd)
        fd.append("b", "2")
        payload = await (await coro).json()
        assert [p["name"] for p in payload["parts"]] == ["a"]

    @pytest.mark.anyio
    async def test_missing_file_raises_on_await(
        self, server: ReqtestServer, tmp_path: Path
    ) -> None:
        fd = ry.FormData({"f": tmp_path / "nope.txt"})
        coro = ry.Client().post(server.url / "upload", multipart=fd)
        with pytest.raises(FileNotFoundError, match=r"nope\.txt"):
            await coro


class TestFormDataBlocking:
    def test_multipart_kwarg(self, server: ReqtestServer, tmp_path: Path) -> None:
        res = ry.BlockingClient().post(
            server.url / "upload", multipart=_form_with_file(tmp_path)
        )
        assert res.status_code == 200
        _check_form_with_file(res.json())

    def test_body_kwarg_fetch_sync(self, server: ReqtestServer, tmp_path: Path) -> None:
        res = ry.fetch_sync(
            server.url / "upload", method="POST", body=_form_with_file(tmp_path)
        )
        assert res.status_code == 200
        _check_form_with_file(res.json())

    def test_multipart_pairs(self, server: ReqtestServer) -> None:
        res = ry.BlockingClient().post(
            server.url / "upload", multipart=[("a", "1"), ("a", "2")]
        )
        assert [p["value"] for p in res.json()["parts"]] == ["1", "2"]

    def test_missing_file(self, server: ReqtestServer, tmp_path: Path) -> None:
        with pytest.raises(FileNotFoundError, match=r"nope\.txt"):
            ry.BlockingClient().post(
                server.url / "upload", multipart={"f": tmp_path / "nope.txt"}
            )

    def test_bad_value(self, server: ReqtestServer) -> None:
        with pytest.raises(TypeError, match="invalid form-data value type"):
            ry.BlockingClient().post(server.url / "upload", multipart={"a": object()})  # type: ignore[dict-item]  # ty: ignore[invalid-argument-type]
