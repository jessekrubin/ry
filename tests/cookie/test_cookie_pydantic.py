import pydantic
import pytest

import ry


class RyCookieModel(pydantic.BaseModel):
    cookie: ry.Cookie


def test_cookie_pydantic() -> None:
    parsed = RyCookieModel(cookie="name=value; HttpOnly").cookie  # type: ignore[arg-type]
    assert parsed == ry.Cookie.parse("name=value; HttpOnly")
    assert parsed.http_only is True

    parsed_bytes = RyCookieModel(cookie=b"name=value; Secure").cookie  # type: ignore[arg-type]
    assert parsed_bytes == ry.Cookie.parse("name=value; Secure")
    assert parsed_bytes.secure is True

    cookie = ry.Cookie("name", "value", path="/")
    assert RyCookieModel(cookie=cookie).cookie == cookie
    assert RyCookieModel(cookie=cookie).model_dump_json() == (
        '{"cookie":"name=value; Path=/"}'
    )


def test_cookie_pydantic_fails() -> None:
    with pytest.raises(pydantic.ValidationError, match="Cookie validation error"):
        RyCookieModel(cookie="=value")  # type: ignore[arg-type]


def test_cookie_model_schema() -> None:
    schema = RyCookieModel.model_json_schema()
    assert schema == {
        "title": "RyCookieModel",
        "type": "object",
        "properties": {"cookie": {"title": "Cookie", "type": "string"}},
        "required": ["cookie"],
    }
