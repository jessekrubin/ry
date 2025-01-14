import ry

# get current directory
current_dir = ry.FsPath.cwd()

# write file
(current_dir / "test.txt").write_text("data!")

# read file
data = (current_dir / "test.txt").read_text()
print(data)
