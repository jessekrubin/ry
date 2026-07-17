"""ryo3-dirs ~ types"""

def audio_dir() -> str | None:
    """Return audio directory or None.

    - lin: `Some(XDG_MUSIC_DIR)` or `None`
    - win: `Some({FOLDERID_Music})`
    - mac: `Some($HOME/Music/)`
    """

def cache_dir() -> str | None:
    """Return cache directory or None.

    - lin: `Some($XDG_CACHE_HOME)` or `Some($HOME/.cache)`
    - win: `Some({FOLDERID_LocalAppData})`
    - mac: `Some($HOME/Library/Caches)`
    """

def config_dir() -> str | None:
    """Return config directory or None.

    - lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
    - win: `Some({FOLDERID_RoamingAppData})`
    - mac: `Some($HOME/Library/Application Support)`
    """

def config_local_dir() -> str | None:
    """Return local config directory or None.

    - lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
    - win: `Some({FOLDERID_LocalAppData})`
    - mac: `Some($HOME/Library/Application Support)`
    """

def data_dir() -> str | None:
    """Return data directory or None.

    - lin: `Some($XDG_DATA_HOME)` or `Some($HOME/.local/share)`
    - win: `Some({FOLDERID_RoamingAppData})`
    - mac: `Some($HOME/Library/Application Support)`
    """

def data_local_dir() -> str | None:
    """Return local data directory or None.

    - lin: `Some($XDG_DATA_HOME)` or `Some($HOME/.local/share)`
    - win: `Some({FOLDERID_LocalAppData})`
    - mac: `Some($HOME/Library/Application Support)`
    """

def desktop_dir() -> str | None:
    """Return desktop directory or None.

    - lin: `Some(XDG_DESKTOP_DIR)` or `None`
    - win: `Some({FOLDERID_Desktop})`
    - mac: `Some($HOME/Desktop/)`
    """

def document_dir() -> str | None:
    """Return document directory or None.

    - lin: `Some(XDG_DOCUMENTS_DIR)` or `None`
    - win: `Some({FOLDERID_Documents})`
    - mac: `Some($HOME/Documents/)`
    """

def download_dir() -> str | None:
    """Return download directory or None.

    - lin: `Some(XDG_DOWNLOAD_DIR)` or `None`
    - win: `Some({FOLDERID_Downloads})`
    - mac: `Some($HOME/Downloads/)`
    """

def executable_dir() -> str | None:
    """Return executable directory or None.

    - lin: `Some($XDG_BIN_HOME)` or `Some($HOME/.local/bin)`
    - win: `None`
    - mac: `None`
    """

def font_dir() -> str | None:
    """Return font directory or None.

    - lin: `Some($XDG_DATA_HOME/fonts/)` or `Some($HOME/.local/share/fonts/)`
    - win: `None`
    - mac: `Some($HOME/Library/Fonts/)`
    """

def home_dir() -> str | None:
    """Return home directory or None.

    - lin: `Some($HOME)`
    - win: `Some({FOLDERID_Profile})`
    - mac: `Some($HOME)`
    """

def picture_dir() -> str | None:
    """Return picture directory or None.

    - lin: `Some(XDG_PICTURES_DIR)` or `None`
    - win: `Some({FOLDERID_Pictures})`
    - mac: `Some($HOME/Pictures/)`
    """

def preference_dir() -> str | None:
    """Return preference directory or None.

    - lin: `Some($XDG_CONFIG_HOME)` or `Some($HOME/.config)`
    - win: `Some({FOLDERID_RoamingAppData})`
    - mac: `Some($HOME/Library/Preferences)`
    """

def public_dir() -> str | None:
    """Return public directory or None.

    - lin: `Some(XDG_PUBLICSHARE_DIR)` or `None`
    - win: `Some({FOLDERID_Public})`
    - mac: `Some($HOME/Public/)`
    """

def runtime_dir() -> str | None:
    """Return runtime directory or None.

    - lin: `Some($XDG_RUNTIME_DIR)` or `None`
    - win: `None`
    - mac: `None`
    """

def state_dir() -> str | None:
    """Return state directory or None.

    - lin: `Some($XDG_STATE_HOME)` or `Some($HOME/.local/state)`
    - win: `None`
    - mac: `None`
    """

def template_dir() -> str | None:
    """Return template directory or None.

    - lin: `Some(XDG_TEMPLATES_DIR)` or `None`
    - win: `Some({FOLDERID_Templates})`
    - mac: `None`
    """

def video_dir() -> str | None:
    """Return video directory or None.

    - lin: `Some(XDG_VIDEOS_DIR)` or `None`
    - win: `Some({FOLDERID_Videos})`
    - mac: `Some($HOME/Movies/)`
    """
