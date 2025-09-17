import subprocess
import sys
import tomllib

import ry

_GIT = ry.which("git")
GIT = "git"
if _GIT is None:
    print("git not found in PATH.")
    sys.exit(1)
else:
    GIT = str(_GIT)


def tag_exists(tag_name: str) -> bool:
    """Check if a tag exists in the Git repository."""

    try:
        subprocess.run(
            [GIT, "rev-parse", "--verify", f"refs/tags/{tag_name}"],
            capture_output=True,
            check=True,
        )
        return True
    except subprocess.CalledProcessError:
        return False


def create_tag(tag_name: str, commit: str = "HEAD") -> None:
    """Create a tag at the specified commit."""
    try:
        subprocess.run(
            [GIT, "tag", tag_name, commit],
            check=True,
        )
        print(f"Tag '{tag_name}' created.")
    except subprocess.CalledProcessError as e:
        print(f"Error creating tag '{tag_name}': {e}")
        sys.exit(1)


def push_tag(tag_name: str) -> None:
    """Push the tag to the remote repository."""
    try:
        subprocess.run(
            [GIT, "push", "origin", tag_name],
            check=True,
        )
        print(f"Tag '{tag_name}' pushed to the remote repository.")
    except subprocess.CalledProcessError as e:
        print(f"Error pushing tag '{tag_name}': {e}")
        sys.exit(1)


def _version() -> str:
    with open("Cargo.toml") as f:
        txt = f.read()

    data = tomllib.loads(txt)
    try:
        return str(data["workspace"]["package"]["version"])
    except KeyError as ke:
        msg = "No version found in Cargo.toml"
        raise ValueError(msg) from ke


def main() -> None:
    v = _version()

    if tag_exists(v):
        print(f"Tag '{v}' already exists.")
        return

    create_tag(f"v{v}")
    push_tag(f"v{v}")
    print(f"Tag '{v}' pushed to the remote repository.")


if __name__ == "__main__":
    main()
