import typing as t

import hypothesis.strategies as st
from hypothesis import given, note

import ry


def test_cookie_repr_simple() -> None:
    cookie = ry.Cookie(name="sessionid", value="abc123")
    repr_str = repr(cookie)
    assert repr_str == 'Cookie("sessionid", "abc123")'
    assert eval("ry." + repr_str) == cookie


_DOMAINS: list[str | None] = [
    None,
    "example.com",
    "sub.example.com",
    "anotherdomain.org",
]
_PATHS: list[str | None] = [None, "/", "/path", "/another/path"]
_SAME_SITES: list[str | None] = [None, "Lax", "Strict", "None"]
_SECURES: list[bool | None] = [None, True, False]
_HTTP_ONLY: list[bool | None] = [None, True, False]
_DURATIONS: list[ry.Duration | None] = [None, ry.Duration(secs=30)]


class _CookieKwargs(t.TypedDict):
    domain: str | None
    path: str | None
    max_age: ry.Duration | None
    secure: bool | None
    http_only: bool | None
    same_site: t.Literal["Lax", "Strict", "None"] | None


def st_cookie_kwargs() -> st.SearchStrategy:
    return st.fixed_dictionaries({
        "domain": st.sampled_from(_DOMAINS),
        "path": st.sampled_from(_PATHS),
        "max_age": st.sampled_from(_DURATIONS),
        "secure": st.sampled_from(_SECURES),
        "http_only": st.sampled_from(_HTTP_ONLY),
        "same_site": st.sampled_from(_SAME_SITES),
    })


@given(st_cookie_kwargs())
def test_cookie_repr_hypothesis(
    kwargs: _CookieKwargs,
) -> None:
    cookie = ry.Cookie(name="cname", value="cvalue", **kwargs)
    note(f"Cookie: {cookie}")
    repr_str = repr(cookie)
    note(f"repr: {repr_str}")
    evaled = eval(
        repr_str,
        {
            "Cookie": ry.Cookie,
            "Duration": ry.Duration,
        },
    )
    assert evaled == cookie
