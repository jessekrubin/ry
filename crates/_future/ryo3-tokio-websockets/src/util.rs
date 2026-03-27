use http::Uri;
use pyo3::prelude::*;
use ryo3_core::py_value_error;
use ryo3_url::UrlLike;

pub(crate) fn parse_uri(url: &UrlLike) -> PyResult<Uri> {
    url.0
        .as_str()
        .parse::<Uri>()
        .map_err(|err| py_value_error!("invalid websocket uri: {err}"))
}
