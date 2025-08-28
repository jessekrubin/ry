import ry


def test_socketaddr() -> None:
    # Test parsing a valid socket address
    a = ry.SocketAddr.parse("0.0.0.0:8080")
    assert str(a) == "0.0.0.0:8080"
    assert isinstance(hash(a), int)
