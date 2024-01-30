import ry


def test_ls(tmpdir):
    tmpdir.join("a.txt").write("hello")
    tmpdir.join("b.txt").write("world")
    assert set(ry.ls(tmpdir)) == {"a.txt", "b.txt"}


def test_ls_pathlib(tmpdir):
    ry.cd(tmpdir)
    tmpdir.join("a.txt").write("hello")
    tmpdir.join("b.txt").write("world")
    assert set(ry.ls()) == {"a.txt", "b.txt"}
