from __future__ import annotations

import json

from ry import Headers, __version__

HEADERS_DICT = {
    "User-Agent": "ry-reqwest/" + __version__,
    "Accept": "*/*",
    "Accept-Encoding": "gzip, deflate, br",
    "Connection": "keep-alive",
}

HEADERS_OBJ = Headers(HEADERS_DICT)


class TestHeadersJson:
    def test_headers_json(self) -> None:
        json_str = HEADERS_OBJ.stringify()
        assert isinstance(json_str, str)
        assert json_str == json.dumps(
            HEADERS_OBJ.to_dict(), sort_keys=True, separators=(",", ":")
        )

    def test_headers_json_fmt(self) -> None:
        json_str = HEADERS_OBJ.stringify(fmt=True)
        assert isinstance(json_str, str)
        assert json_str == json.dumps(HEADERS_OBJ.to_dict(), indent=2, sort_keys=True)

    def test_round_trip(self) -> None:
        # test round trip
        json_str = HEADERS_OBJ.stringify()
        assert isinstance(json_str, str)
        assert json_str == json.dumps(
            HEADERS_OBJ.to_dict(), sort_keys=True, separators=(",", ":")
        )
        h = Headers(json.loads(json_str))
        assert h == HEADERS_OBJ
        assert len(h) == len(HEADERS_OBJ)
        assert h.keys_len() == HEADERS_OBJ.keys_len()
        assert h.stringify() == HEADERS_OBJ.stringify()

        from_json = Headers(json.loads(json_str))
        assert from_json == HEADERS_OBJ
        assert len(from_json) == len(HEADERS_OBJ)


class TestHeadersObj:
    def test_headers_obj_repr(self) -> None:
        h = Headers({
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        evaled = eval(repr(h))
        assert isinstance(evaled, Headers)
        assert evaled == h

    def test_kwargs(self) -> None:
        h = Headers(**{
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        assert len(h) == 2
        assert h["Content-Type"] == "application/json"
        assert h["Accept"] == "application/json"

    def test_kwargs_and_dictionary(self) -> None:
        h = Headers(
            {
                "Content-Type": "application/json",
            },
            **{
                "Content-Type": "application/x-www-form-urlencoded",
                "Accept": "application/json",
            },
        )
        assert len(h) == 2
        # kwargs overwrite dictionary
        assert h["Content-Type"] == "application/x-www-form-urlencoded"
        assert h["Accept"] == "application/json"

    def test_len_and_keys_len(self) -> None:
        h = Headers({"Content-Type": "application/json", "Accept": "application/json"})
        h.append("content-Type", "application/xml")
        assert len(h) == 3
        assert h.keys_len() == 2

    def test_len_and_keys_len_clear(self) -> None:
        h = Headers({"Content-Type": "application/json", "Accept": "application/json"})
        h.append("content-Type", "application/xml")
        assert len(h) == 3
        assert h.keys_len() == 2
        h.clear()
        assert len(h) == 0
        assert h.keys_len() == 0

    def test_headers_multiple_same_key_get(self) -> None:
        h = Headers({"Content-Type": "application/json"})
        h.append("content-Type", "application/xml")
        content_type = h.get("Content-Type")
        assert content_type == "application/json"
        assert isinstance(content_type, str)

        # get via key
        content_type = h.get("content-type")
        assert content_type == "application/json"
        assert isinstance(content_type, str)

        # dump to dict and see if its the same...
        d = h.to_dict()
        expected_dict = {"content-type": ["application/json", "application/xml"]}
        assert d == expected_dict
        from_dict = Headers(d)
        assert from_dict == h

    def test_headers_multiple_same_key_get_all(self) -> None:
        h = Headers({"Content-Type": "application/json"})
        h.append("content-Type", "application/xml")
        all_content_type = h.get_all("Content-Type")
        assert all_content_type == ["application/json", "application/xml"]
        assert isinstance(all_content_type, list)

    def test_headers_pop(self) -> None:
        h = Headers({"Content-Type": "application/json"})
        h.append("Content-Type", "application/xml")
        content_type = h.pop("Content-Type")
        assert content_type == "application/json"
        assert isinstance(content_type, str)
        assert "Content-Type" not in h
        assert len(h) == 0
        assert h.keys_len() == 0

    def test_headers_keys_list(self) -> None:
        h = Headers({
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        keys = h.keys()
        assert isinstance(keys, list)
        keyset = set(keys)
        assert keyset == {"content-type", "accept"}

    def test_headers_dict_able(self) -> None:
        h = Headers({
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        d = dict(h)
        assert isinstance(d, dict)
        assert d == {
            "content-type": "application/json",
            "accept": "application/json",
        }

    def test_headers_update(self) -> None:
        h = Headers({
            "Content-Type": "application/json",
            "Accept": "application/json",
        })
        h.update({"Content-Type": "application/xml"})
        assert len(h) == 2
        assert h["Content-Type"] == "application/xml"
        assert h["Accept"] == "application/json"
