#![doc = include_str!("../README.md")]
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::{PyResult, wrap_pyfunction};

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
        #[must_use] pub fn $name() -> Option<String> {
            dirs::$dirs_fn().map(|p| {
                p.to_string_lossy().to_string()
            })
        }

        #[pyfunction]
        #[doc = $doc]
        #[must_use] pub fn $dirs_fn() -> Option<String> {
            dirs::$dirs_fn().map(|p| {
                p.to_string_lossy().to_string()
            })
        }
    };
    ($name:ident, $dirs_fn:ident) => {
        #[pyfunction]
        #[must_use]
        pub fn $name() -> Option<String> {
            dirs::$dirs_fn().map(|p| {
                p.to_string_lossy().to_string()
            })
        }

        #[pyfunction]
        #[must_use]
        pub fn $dirs_fn() -> Option<String> {
            dirs::$dirs_fn().map(|p| {
                p.to_string_lossy().to_string()
            })
        }
    };
);

dirs_pyfunction!(
    home,
    home_dir,
    r"Return home directory string or None.

lin: `Some($HOME)`
win: `Some({FOLDERID_Profile})`
mac: `Some($HOME)`
"
);

dirs_pyfunction!(
    cache,
    cache_dir,
    r"Return cache directory string or None.
    lin: `Some($XDG_CACHE_HOME)` or `Some($HOME/.cache)`
    win: `Some({FOLDERID_LocalAppData})`
    mac: `Some($HOME/Library/Caches)`
    "
);

dirs_pyfunction!(config, config_dir);
dirs_pyfunction!(config_local, config_local_dir);
dirs_pyfunction!(data, data_dir);
dirs_pyfunction!(data_local, data_local_dir);
dirs_pyfunction!(executable, executable_dir);
dirs_pyfunction!(preference, preference_dir);
dirs_pyfunction!(runtime, runtime_dir);
dirs_pyfunction!(state, state_dir);
dirs_pyfunction!(audio, audio_dir);
dirs_pyfunction!(desktop, desktop_dir);
dirs_pyfunction!(document, document_dir);
dirs_pyfunction!(download, download_dir);
dirs_pyfunction!(font, font_dir);
dirs_pyfunction!(picture, picture_dir);
dirs_pyfunction!(public, public_dir);
dirs_pyfunction!(template, template_dir);
dirs_pyfunction!(video, video_dir);

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
pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pymod_register_suffix(m)?;
    pymod_register_no_suffix(m)?;
    Ok(())
}
