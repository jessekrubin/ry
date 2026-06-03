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


def git_output(*args: str) -> str:
    """Run git and return stripped stdout."""
    try:
        result = subprocess.run(
            [GIT, *args],
            capture_output=True,
            check=True,
            text=True,
        )
    except subprocess.CalledProcessError as e:
        print(f"Error running git {' '.join(args)}: {e.stderr.strip()}")
        sys.exit(1)
    return result.stdout.strip()


def git_run(*args: str) -> None:
    """Run git and exit on failure."""
    try:
        subprocess.run([GIT, *args], check=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running git {' '.join(args)}: {e}")
        sys.exit(1)


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


def assert_releaseable_main() -> None:
    """Ensure the release tag is created from current origin/main."""

    branch = git_output("branch", "--show-current")
    if branch != "main":
        print(f"Refusing to release from branch {branch!r}; switch to main first.")
        sys.exit(1)

    if git_output("status", "--porcelain"):
        print(
            "Refusing to release with a dirty worktree; commit or stash changes first."
        )
        sys.exit(1)

    git_run("fetch", "--quiet", "origin", "main", "--tags")

    head = git_output("rev-parse", "HEAD")
    origin_main = git_output("rev-parse", "origin/main")
    if head != origin_main:
        print("Refusing to release because HEAD is not origin/main.")
        print(f"HEAD:        {head}")
        print(f"origin/main: {origin_main}")
        sys.exit(1)


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
    assert_releaseable_main()

    v = _version()
    tag_name = f"v{v}"

    if tag_exists(tag_name):
        print(f"Tag '{tag_name}' already exists.")
        return

    create_tag(tag_name)
    push_tag(tag_name)
    print(f"Tag '{tag_name}' pushed to the remote repository.")


if __name__ == "__main__":
    main()
