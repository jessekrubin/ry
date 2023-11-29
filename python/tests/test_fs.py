import ry

def test_read_string(tmp_path):
    p = tmp_path / 'test.txt'
    p.write_text('hello')
    ry.cd(tmp_path)
    assert ry.read_text(
        'test.txt'
    ) == 'hello'

def test_read_bytes(tmp_path):
    p = tmp_path / 'test.txt'
    p.write_bytes(b'hello')
    ry.cd(tmp_path)
    assert ry.read_bytes(
        'test.txt'
    ) == b'hello'
