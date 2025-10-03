import pytest

import ry


def test_to_string() -> None:
    c = ry.Cookie(
        "name", "value", http_only=True, secure=True, path="/", domain="example.com"
    )
    expected = "name=value; HttpOnly; Secure; Path=/; Domain=example.com"
    assert str(c) == expected
    assert c == ry.Cookie.parse(expected)


def test_permanent() -> None:
    c = ry.Cookie("name", "value", permanent=False)
    assert c.max_age is None
    # TODO expires tests

    c = ry.Cookie("name", "value", permanent=True)
    assert c.max_age is not None


def test_removal() -> None:
    c = ry.Cookie("name", "value", removal=True)
    assert c.max_age is not None
    assert c.max_age is not None
    assert c.max_age == ry.Duration.ZERO
    # TODO expires tests


def test_not_eq() -> None:
    c1 = ry.Cookie("name", "value1")
    c2 = ry.Cookie("name", "value2")
    assert c1 != c2
    assert not (c1 == c2)


def test_hash() -> None:
    c1 = ry.Cookie("name", "value1")
    c2 = ry.Cookie("name", "value1")
    assert hash(c1) == hash(c2)

    c1 = ry.Cookie("name", "value1", path="/")
    c2 = ry.Cookie("name", "value2", path="/")
    assert hash(c1) != hash(c2)


def test_encoded() -> None:
    c = ry.Cookie("my name", "this; value?", secure=True)
    assert c.encoded() == "my%20name=this%3B%20value%3F; Secure"
    assert c.encoded_stripped() == "my%20name=this%3B%20value%3F"


def test_stripped() -> None:
    c = ry.Cookie("key?", "value", secure=True, path="/")
    assert c.stripped() == "key?=value"
    assert c.stripped_encoded() == "key%3F=value"


@pytest.mark.parametrize(
    "string",
    [
        "asd;lfkjas;dlfkjas;dlfkjas;ldfkjasd;lfkj",
        "=value",
    ],
)
def test_parse_fails(string: str) -> None:
    with pytest.raises(ValueError):
        ry.Cookie.parse(string)


@pytest.mark.parametrize(
    "string,name,value",
    [
        ("name=value", "name", "value"),
        ("name=value; Path=/", "name", "value"),
        ("name=value; Domain=example.com", "name", "value"),
    ],
)
def test_parse_encoded(string: str, name: str, value: str) -> None:
    c = ry.Cookie.parse(string)
    assert c.name == name
    assert c.value == value


@pytest.mark.parametrize(
    "string",
    [
        "asd;lfkjas;dlfkjas;dlfkjas;ldfkjasd;lfkj",
        "=value",
    ],
)
def test_parse_encoded_fails(string: str) -> None:
    with pytest.raises(ValueError):
        ry.Cookie.parse_encoded(string)


class TestCookieProperties:
    def test_domain(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.domain is None
        c = ry.Cookie("name", "value", domain="example.com")
        assert c.domain == "example.com"

    @pytest.mark.parametrize(
        "string,domain",
        [
            ("name=value", None),
            ("name=value; Domain=crates.io", "crates.io"),
            ("name=value; Domain=.crates.io", "crates.io"),
            ("name=value; Domain=..crates.io", ".crates.io"),
        ],
    )
    def test_domain_parse(self, string: str, domain: str | None) -> None:
        c = ry.Cookie.parse(string)
        assert c.domain == domain

    def test_name(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.name == "name"
        c = ry.Cookie.parse("name=value")
        assert c.name == "name"

    def test_value(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.value == "value"
        c = ry.Cookie.parse("name=value")
        assert c.value == "value"

    def test_value_trimmed(self) -> None:
        c = ry.Cookie("name", '"value"')
        assert c.value == '"value"'
        assert c.value_trimmed == "value"
        c = ry.Cookie.parse('name="value"')
        assert c.value == '"value"'
        assert c.value_trimmed == "value"

    def test_name_value_tuple(self) -> None:
        c = ry.Cookie("name", "value")
        assert isinstance(c.name_value, tuple)
        assert c.name_value == ("name", "value")
        c = ry.Cookie.parse("name=value")
        assert c.name_value == ("name", "value")

    def test_name_value_trimmed_tuple(self) -> None:
        c = ry.Cookie("name", '"value"')
        assert isinstance(c.name_value_trimmed, tuple)
        assert c.name_value_trimmed == ("name", "value")
        c = ry.Cookie.parse('name="value"')
        assert c.name_value_trimmed == ("name", "value")

    @pytest.mark.parametrize(
        "string,same_site",
        [
            ("name=value; SameSite=Lax", "Lax"),
            ("name=value; SameSite=Strict", "Strict"),
            ("name=value; SameSite=None", "None"),
            ("name=value", None),
        ],
    )
    def test_same_site(self, string: str, same_site: str | None) -> None:
        c = ry.Cookie.parse(string)
        assert c.same_site == same_site
        c = ry.Cookie("name", "value")
        assert c.same_site is None
        c = ry.Cookie("name", "value", same_site="None")
        assert c.same_site == "None"

    def test_secure(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.secure is None
        c = ry.Cookie("name", "value", secure=True)
        assert c.secure is True

    @pytest.mark.parametrize(
        "string,secure",
        [
            ("name=value; Secure", True),
            ("name=value; Secure", True),
            ("name=value", None),
        ],
    )
    def test_secure_parse(self, string: str, secure: bool | None) -> None:  # noqa: FBT001
        c = ry.Cookie.parse(string)
        assert c.secure == secure

    def test_partitioned(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.partitioned is None
        c = ry.Cookie("name", "value", partitioned=True)
        assert c.partitioned is True

    @pytest.mark.parametrize(
        "string,partitioned",
        [
            ("name=value; Partitioned", True),
            ("name=value", None),
        ],
    )
    def test_partitioned_parse(self, string: str, partitioned: bool | None) -> None:  # noqa: FBT001
        c = ry.Cookie.parse(string)
        assert c.partitioned == partitioned

    def test_http_only(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.http_only is None
        c = ry.Cookie("name", "value", http_only=True)
        assert c.http_only is True

    @pytest.mark.parametrize(
        "string,http_only",
        [
            ("name=value; HttpOnly", True),
            ("name=value; HttpOnly", True),
            ("name=value", None),
        ],
    )
    def test_http_only_parse(self, string: str, http_only: bool | None) -> None:  # noqa: FBT001
        c = ry.Cookie.parse(string)
        assert c.http_only == http_only

    def test_max_age(self) -> None:
        c = ry.Cookie("name", "value")
        assert c.max_age is None
        # TODO: allow seconds as int/float
        c = ry.Cookie("name", "value", max_age=ry.Duration(secs=3600))
        max_age = c.max_age
        assert max_age is not None
        assert max_age == ry.Duration(secs=3600)
