# `ryo3-dirs`

Wrapper around the `dirs` crate for Python.

- [crates.io](https://crates.io/crates/dirs)
- [docs.rs](https://docs.rs/dirs)

| Function name      | Value on Linux/Redox                                                   | Value on Windows                  | Value on macOS                              |
| ------------------ | ---------------------------------------------------------------------- | --------------------------------- | ------------------------------------------- |
| `home_dir`         | `Some($HOME)`                                                          | `Some({FOLDERID_Profile})`        | `Some($HOME)`                               |
| `cache_dir`        | `Some($XDG_CACHE_HOME)` or `Some($HOME`/.cache`)`                      | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Caches`)`              |
| `config_dir`       | `Some($XDG_CONFIG_HOME)` or `Some($HOME`/.config`)`                    | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Application Support`)` |
| `config_local_dir` | `Some($XDG_CONFIG_HOME)` or `Some($HOME`/.config`)`                    | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Application Support`)` |
| `data_dir`         | `Some($XDG_DATA_HOME)` or `Some($HOME`/.local/share`)`                 | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Application Support`)` |
| `data_local_dir`   | `Some($XDG_DATA_HOME)` or `Some($HOME`/.local/share`)`                 | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Application Support`)` |
| `executable_dir`   | `Some($XDG_BIN_HOME)` or `Some($HOME`/.local/bin`)`                    | `None`                            | `None`                                      |
| `preference_dir`   | `Some($XDG_CONFIG_HOME)` or `Some($HOME`/.config`)`                    | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Preferences`)`         |
| `runtime_dir`      | `Some($XDG_RUNTIME_DIR)` or `None`                                     | `None`                            | `None`                                      |
| `state_dir`        | `Some($XDG_STATE_HOME)` or `Some($HOME`/.local/state`)`                | `None`                            | `None`                                      |
| `audio_dir`        | `Some(XDG_MUSIC_DIR)` or `None`                                        | `Some({FOLDERID_Music})`          | `Some($HOME`/Music/`)`                      |
| `desktop_dir`      | `Some(XDG_DESKTOP_DIR)` or `None`                                      | `Some({FOLDERID_Desktop})`        | `Some($HOME`/Desktop/`)`                    |
| `document_dir`     | `Some(XDG_DOCUMENTS_DIR)` or `None`                                    | `Some({FOLDERID_Documents})`      | `Some($HOME`/Documents/`)`                  |
| `download_dir`     | `Some(XDG_DOWNLOAD_DIR)` or `None`                                     | `Some({FOLDERID_Downloads})`      | `Some($HOME`/Downloads/`)`                  |
| `font_dir`         | `Some($XDG_DATA_HOME`/fonts/`)` or `Some($HOME`/.local/share/fonts/`)` | `None`                            | `Some($HOME`/Library/Fonts/`)`              |
| `picture_dir`      | `Some(XDG_PICTURES_DIR)` or `None`                                     | `Some({FOLDERID_Pictures})`       | `Some($HOME`/Pictures/`)`                   |
| `public_dir`       | `Some(XDG_PUBLICSHARE_DIR)` or `None`                                  | `Some({FOLDERID_Public})`         | `Some($HOME`/Public/`)`                     |
| `template_dir`     | `Some(XDG_TEMPLATES_DIR)` or `None`                                    | `Some({FOLDERID_Templates})`      | `None`                                      |
| `video_dir`        | `Some(XDG_VIDEOS_DIR)` or `None`                                       | `Some({FOLDERID_Videos})`         | `Some($HOME`/Movies/`)`                     |
