import ry


def test_parse_with_params_dict() -> None:
    url = ry.URL.parse_with_params(
        "https://example.net?dont=clobberme",
        dict([("lang", "rust"), ("browser", "servo")]),
    )
    assert str(url) == "https://example.net/?dont=clobberme&lang=rust&browser=servo"


def test_parse_new_params_kwarg() -> None:
    url = ry.URL(
        "https://example.net?dont=clobberme",
        params=dict([("lang", "rust"), ("browser", "servo")]),
    )
    assert str(url) == "https://example.net/?dont=clobberme&lang=rust&browser=servo"


# def test_parse_with_params_list() -> None:
#     url = ry.URL.parse_with_params("https://example.net?dont=clobberme",
#                                    [("lang", "rust"), ("browser", "servo")])
#     assert url.as_str() == "https://example.net/?dont=clobberme&lang=rust&browser=servo"
