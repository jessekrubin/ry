from __future__ import annotations

from ry import Headers


class TestHeadersObj:
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
        h = Headers(
            {
                "Content-Type": "application/json",
                "Accept": "application/json",
            }
        )
        keys = h.keys()
        assert isinstance(keys, list)
        keyset = set(keys)
        assert keyset == {"content-type", "accept"}

    def test_headers_dict_able(self) -> None:
        h = Headers(
            {
                "Content-Type": "application/json",
                "Accept": "application/json",
            }
        )
        d = dict(h)
        assert isinstance(d, dict)
        assert d == {
            "content-type": "application/json",
            "accept": "application/json",
        }

    def test_headers_update(self) -> None:
        h = Headers(
            {
                "Content-Type": "application/json",
                "Accept": "application/json",
            }
        )
        h.update({"Content-Type": "application/xml"})
        assert len(h) == 2
        assert h["Content-Type"] == "application/xml"
        assert h["Accept"] == "application/json"
