import ry


def mk_dir_tree(tmp_path):
    abcd = tmp_path / "a" / "b" / "c" / "d"
    abcd.mkdir(parents=True)
    efgh = tmp_path / "e" / "f" / "g" / "h"
    efgh.mkdir(parents=True)

    files_to_create = [
        abcd / "test.txt",
        abcd / "test2.txt",
        efgh / "test.txt",
        efgh / "test2.txt",
    ]
    for f in files_to_create:
        rel_filepath = f.relative_to(tmp_path)
        f.write_text(str(rel_filepath))

    return abcd


def test_walk_dir_dirpath_string(tmp_path):
    mk_dir_tree(tmp_path)

    paths = []
    for f in ry.walkdir(str(tmp_path)):
        print(f)
        paths.append(f)
    print(paths)
    assert False


def test_walk_dir_dirpath_pathlib_path(tmp_path):
    mk_dir_tree(tmp_path)

    paths = []
    for f in ry.walkdir(tmp_path):
        print(f)
        paths.append(f)
    print(paths)
    assert False


def test_walk_dir_dirpath_none_use_pwd(tmp_path):
    mk_dir_tree(tmp_path)
    ry.cd(tmp_path)

    paths = []
    for f in ry.walkdir():
        print(f)
        paths.append(f)
    print(paths)
    assert False
