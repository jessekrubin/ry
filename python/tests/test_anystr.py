import ry

def test_anystr_string():
    s = 'abc'
    assert ry.anystr_noop(s) == s
    assert isinstance(ry.anystr_noop(s), str)

def test_anystr_bytes():
    b = b'abc'
    assert ry.anystr_noop(b) == b
    assert isinstance(ry.anystr_noop(b), bytes)