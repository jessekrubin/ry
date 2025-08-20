# ry-dist-archives

**TLDR: If you are using an older version of `ry` and it is not longer on PyPI,
you can find it either on primary ry-repo's
[releases](https://github.com/jessekrubin/ry/releases) page, or in the
[ry-dist-archives](https://github.com/jessekrubin/ry-dist-archives)
repository.**

PyPI imposes a 10 GB limit on the total size of all files in a given project.
Once that limit is reached, you must either delete old files or request a size
increase. As of now (`2025-07-22T08:34:51.3412157-07:00[America/Los_Angeles]`),
the `ry` package is still very much in beta, and I (jesse) am not going to
request a size increase while the project remains pre-`0.1.0`.

`ry` builds are performed with github cicd, and published builds for versions
`0.44.0` are uploaded to github
[releases](https://github.com/jessekrubin/ry/releases), **but** publishing
releases was not previously a part of the cicd workflow.

The script for downloading wheels is located at
[ry-dist-archives/scripts/dl_versions.py](https://github.com/jessekrubin/ry-dist-archives/blob/main/scripts/dl_versions.py).
It serves as a good example of how to use `ry`'s HTTP client, JSON parsing and
dumping, and async file I/O tools.
