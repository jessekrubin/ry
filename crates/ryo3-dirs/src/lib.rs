#![doc = include_str!("../README.md")]
use pyo3::prelude::*;

// | Function name      | Value on Linux/Redox                                                   | Value on Windows                  | Value on macOS                              |
// |--------------------| ---------------------------------------------------------------------- |-----------------------------------| ------------------------------------------- |
// | `home_dir`         | `Some($HOME)`                                                          | `Some({FOLDERID_Profile})`        | `Some($HOME)`                               |
// | `cache_dir`        | `Some($XDG_CACHE_HOME)`         or `Some($HOME`/.cache`)`              | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Caches`)`              |
// | `config_dir`       | `Some($XDG_CONFIG_HOME)`        or `Some($HOME`/.config`)`             | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Application Support`)` |
// | `config_local_dir` | `Some($XDG_CONFIG_HOME)`        or `Some($HOME`/.config`)`             | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Application Support`)` |
// | `data_dir`         | `Some($XDG_DATA_HOME)`          or `Some($HOME`/.local/share`)`        | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Application Support`)` |
// | `data_local_dir`   | `Some($XDG_DATA_HOME)`          or `Some($HOME`/.local/share`)`        | `Some({FOLDERID_LocalAppData})`   | `Some($HOME`/Library/Application Support`)` |
// | `executable_dir`   | `Some($XDG_BIN_HOME)`           or `Some($HOME`/.local/bin`)`          | `None`                            | `None`                                      |
// | `preference_dir`   | `Some($XDG_CONFIG_HOME)`        or `Some($HOME`/.config`)`             | `Some({FOLDERID_RoamingAppData})` | `Some($HOME`/Library/Preferences`)`         |
// | `runtime_dir`      | `Some($XDG_RUNTIME_DIR)`        or `None`                              | `None`                            | `None`                                      |
// | `state_dir`        | `Some($XDG_STATE_HOME)`         or `Some($HOME`/.local/state`)`        | `None`                            | `None`                                      |
// | `audio_dir`        | `Some(XDG_MUSIC_DIR)`           or `None`                              | `Some({FOLDERID_Music})`          | `Some($HOME`/Music/`)`                      |
// | `desktop_dir`      | `Some(XDG_DESKTOP_DIR)`         or `None`                              | `Some({FOLDERID_Desktop})`        | `Some($HOME`/Desktop/`)`                    |
// | `document_dir`     | `Some(XDG_DOCUMENTS_DIR)`       or `None`                              | `Some({FOLDERID_Documents})`      | `Some($HOME`/Documents/`)`                  |
// | `download_dir`     | `Some(XDG_DOWNLOAD_DIR)`        or `None`                              | `Some({FOLDERID_Downloads})`      | `Some($HOME`/Downloads/`)`                  |
// | `font_dir`         | `Some($XDG_DATA_HOME`/fonts/`)` or `Some($HOME`/.local/share/fonts/`)` | `None`                            | `Some($HOME`/Library/Fonts/`)`              |
// | `picture_dir`      | `Some(XDG_PICTURES_DIR)`        or `None`                              | `Some({FOLDERID_Pictures})`       | `Some($HOME`/Pictures/`)`                   |
// | `public_dir`       | `Some(XDG_PUBLICSHARE_DIR)`     or `None`                              | `Some({FOLDERID_Public})`         | `Some($HOME`/Public/`)`                     |
// | `template_dir`     | `Some(XDG_TEMPLATES_DIR)`       or `None`                              | `Some({FOLDERID_Templates})`      | `None`                                      |
// | `video_dir`        | `Some(XDG_VIDEOS_DIR)`          or `None`                              | `Some({FOLDERID_Videos})`         | `Some($HOME`/Movies/`)`                     |

macro_rules! dirs_pyfunction (
    ($name:ident, $dirs_fn:ident, $doc:expr) => {
        #[pyfunction]
        #[doc = $doc]
        #[must_use] pub fn $name() -> Option<std::ffi::OsString> {
            dirs::$dirs_fn().map(::std::path::PathBuf::into_os_string)
        }

        #[pyfunction]
        #[doc = $doc]
        #[must_use] pub fn $dirs_fn() -> Option<std::ffi::OsString> {
            dirs::$dirs_fn().map(::std::path::PathBuf::into_os_string)
        }
    };
);

dirs_pyfunction!(
    home,
    home_dir,
    r"Return home directory or None.

- lin: `Some($HOME)`
- win: `Some({FOLDERID_Profile})`
- mac: `Some($HOME)`
"
);

dirs_pyfunction!(
    cache,
    cache_dir,
    r"Return cache directory or None.

- lin: `Some($XDG_CACHE_HOME)` or `Some($HOME/.cache)`
- win: `Some({FOLDERID_LocalAppData})`
- mac: `Some($HOME/Library/Caches)`
"
);

dirs_pyfunction!(
    config,
    config_dir,
    r"Return config directory or None.

- lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
- win: `Some({FOLDERID_RoamingAppData})`
- mac: `Some($HOME/Library/Application Support)`
"
);

dirs_pyfunction!(
    config_local,
    config_local_dir,
    r"Return local config directory or None.

- lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
- win: `Some({FOLDERID_LocalAppData})`
- mac: `Some($HOME/Library/Application Support)`
"
);

dirs_pyfunction!(
    data,
    data_dir,
    r"Return data directory or None.

- lin: `Some($XDG_DATA_HOME)` or `Some($HOME/.local/share)`
- win: `Some({FOLDERID_RoamingAppData})`
- mac: `Some($HOME/Library/Application Support)`
"
);

dirs_pyfunction!(
    data_local,
    data_local_dir,
    r"Return local data directory or None.

- lin: `Some($XDG_DATA_HOME)` or `Some($HOME/.local/share)`
- win: `Some({FOLDERID_LocalAppData})`
- mac: `Some($HOME/Library/Application Support)`
"
);

dirs_pyfunction!(
    executable,
    executable_dir,
    r"Return executable directory or None.

- lin: `Some($XDG_BIN_HOME)` or `Some($HOME/.local/bin)`
- win: `None`
- mac: `None`
"
);

dirs_pyfunction!(
    preference,
    preference_dir,
    r"Return preference directory or None.

- lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
- win: `Some({FOLDERID_RoamingAppData})`
- mac: `Some($HOME/Library/Preferences)`
"
);

dirs_pyfunction!(
    runtime,
    runtime_dir,
    r"Return runtime directory or None.

- lin: `Some($XDG_RUNTIME_DIR)` or `None`
- win: `None`
- mac: `None`
"
);

dirs_pyfunction!(
    state,
    state_dir,
    r"Return state directory or None.

- lin: `Some($XDG_STATE_HOME)` or `Some($HOME/.local/state)`
- win: `None`
- mac: `None`
"
);

dirs_pyfunction!(
    audio,
    audio_dir,
    r"Return audio directory or None.

- lin: `Some(XDG_MUSIC_DIR)` or `None`
- win: `Some({FOLDERID_Music})`
- mac: `Some($HOME/Music/)`
"
);

dirs_pyfunction!(
    desktop,
    desktop_dir,
    r"Return desktop directory or None.

- lin: `Some(XDG_DESKTOP_DIR)` or `None`
- win: `Some({FOLDERID_Desktop})`
- mac: `Some($HOME/Desktop/)`
"
);

dirs_pyfunction!(
    document,
    document_dir,
    r"Return document directory or None.

- lin: `Some(XDG_DOCUMENTS_DIR)` or `None`
- win: `Some({FOLDERID_Documents})`
- mac: `Some($HOME/Documents/)`
"
);

dirs_pyfunction!(
    download,
    download_dir,
    r"Return download directory or None.

- lin: `Some(XDG_DOWNLOAD_DIR)` or `None`
- win: `Some({FOLDERID_Downloads})`
- mac: `Some($HOME/Downloads/)`
"
);

dirs_pyfunction!(
    font,
    font_dir,
    r"Return font directory or None.

- lin: `Some($XDG_DATA_HOME/fonts/)` or `Some($HOME/.local/share/fonts/)`
- win: `None`
- mac: `Some($HOME/Library/Fonts/)`
"
);

dirs_pyfunction!(
    picture,
    picture_dir,
    r"Return picture directory or None.

- lin: `Some(XDG_PICTURES_DIR)` or `None`
- win: `Some({FOLDERID_Pictures})`
- mac: `Some($HOME/Pictures/)`
"
);

dirs_pyfunction!(
    public,
    public_dir,
    r"Return public directory or None.

- lin: `Some(XDG_PUBLICSHARE_DIR)` or `None`
- win: `Some({FOLDERID_Public})`
- mac: `Some($HOME/Public/)`
"
);

dirs_pyfunction!(
    template,
    template_dir,
    r"Return template directory or None.

- lin: `Some(XDG_TEMPLATES_DIR)` or `None`
- win: `Some({FOLDERID_Templates})`
- mac: `None`
"
);

dirs_pyfunction!(
    video,
    video_dir,
    r"Return video directory or None.

- lin: `Some(XDG_VIDEOS_DIR)` or `None`
- win: `Some({FOLDERID_Videos})`
- mac: `Some($HOME/Movies/)`
"
);

pub fn pymod_register_suffix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(home_dir, m)?)?;
    m.add_function(wrap_pyfunction!(cache_dir, m)?)?;
    m.add_function(wrap_pyfunction!(config_dir, m)?)?;
    m.add_function(wrap_pyfunction!(config_local_dir, m)?)?;
    m.add_function(wrap_pyfunction!(data_dir, m)?)?;
    m.add_function(wrap_pyfunction!(data_local_dir, m)?)?;
    m.add_function(wrap_pyfunction!(executable_dir, m)?)?;
    m.add_function(wrap_pyfunction!(preference_dir, m)?)?;
    m.add_function(wrap_pyfunction!(runtime_dir, m)?)?;
    m.add_function(wrap_pyfunction!(state_dir, m)?)?;
    m.add_function(wrap_pyfunction!(audio_dir, m)?)?;
    m.add_function(wrap_pyfunction!(desktop_dir, m)?)?;
    m.add_function(wrap_pyfunction!(document_dir, m)?)?;
    m.add_function(wrap_pyfunction!(download_dir, m)?)?;
    m.add_function(wrap_pyfunction!(font_dir, m)?)?;
    m.add_function(wrap_pyfunction!(picture_dir, m)?)?;
    m.add_function(wrap_pyfunction!(public_dir, m)?)?;
    m.add_function(wrap_pyfunction!(template_dir, m)?)?;
    m.add_function(wrap_pyfunction!(video_dir, m)?)?;
    Ok(())
}

pub fn pymod_register_no_suffix(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(home, m)?)?;
    m.add_function(wrap_pyfunction!(cache, m)?)?;
    m.add_function(wrap_pyfunction!(config, m)?)?;
    m.add_function(wrap_pyfunction!(config_local, m)?)?;
    m.add_function(wrap_pyfunction!(data, m)?)?;
    m.add_function(wrap_pyfunction!(data_local, m)?)?;
    m.add_function(wrap_pyfunction!(executable, m)?)?;
    m.add_function(wrap_pyfunction!(preference, m)?)?;
    m.add_function(wrap_pyfunction!(runtime, m)?)?;
    m.add_function(wrap_pyfunction!(state, m)?)?;
    m.add_function(wrap_pyfunction!(audio, m)?)?;
    m.add_function(wrap_pyfunction!(desktop, m)?)?;
    m.add_function(wrap_pyfunction!(document, m)?)?;
    m.add_function(wrap_pyfunction!(download, m)?)?;
    m.add_function(wrap_pyfunction!(font, m)?)?;
    m.add_function(wrap_pyfunction!(picture, m)?)?;
    m.add_function(wrap_pyfunction!(public, m)?)?;
    m.add_function(wrap_pyfunction!(template, m)?)?;
    m.add_function(wrap_pyfunction!(video, m)?)?;
    Ok(())
}

pub fn pysubmod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pymod_register_suffix(m)?;
    pymod_register_no_suffix(m)?;
    Ok(())
}

pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pymod_register_suffix(m)?;
    Ok(())
}
