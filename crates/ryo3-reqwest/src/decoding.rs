//! Content decoding policy at the boundary between reqwest and Ry responses.

use http::header::{ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_LENGTH};
use http::{HeaderMap, HeaderValue};

use crate::brotli::BrotliBody;
use crate::response::ResponseParts;
use crate::response_head::RyResponseHead;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[expect(clippy::struct_excessive_bools)]
pub(crate) struct Decoding {
    pub(crate) gzip: bool,
    pub(crate) brotli: bool,
    pub(crate) deflate: bool,
    pub(crate) zstd: bool,
}

impl Default for Decoding {
    fn default() -> Self {
        Self {
            gzip: true,
            brotli: true,
            deflate: true,
            zstd: true,
        }
    }
}

impl Decoding {
    pub(crate) fn configure(
        self,
        builder: reqwest::ClientBuilder,
        mut headers: HeaderMap,
    ) -> reqwest::ClientBuilder {
        if !headers.contains_key(ACCEPT_ENCODING)
            && let Some(accept) = self.accept_encoding()
        {
            headers.insert(ACCEPT_ENCODING, accept);
        }
        builder
            .gzip(self.gzip)
            // Guard against another dependency enabling reqwest's Brotli feature.
            .no_brotli()
            .deflate(self.deflate)
            .zstd(self.zstd)
            .default_headers(headers)
    }

    /// Match tower-http's preference order for the supported encodings.
    fn accept_encoding(self) -> Option<HeaderValue> {
        let encodings = [
            (self.zstd, "zstd"),
            (self.gzip, "gzip"),
            (self.deflate, "deflate"),
            (self.brotli, "br"),
        ]
        .into_iter()
        .filter_map(|(enabled, name)| enabled.then_some(name))
        .collect::<Vec<_>>();
        if encodings.is_empty() {
            None
        } else {
            Some(HeaderValue::from_str(&encodings.join(",")).expect("static encoding names"))
        }
    }

    /// Normalize headers and body before either kind of Ry response is built.
    pub(crate) fn decode(self, response: reqwest::Response) -> ResponseParts {
        // reqwest keeps the URL outside its http::Response parts.
        let url = response.url().clone();
        let remote_addr = response.remote_addr();
        let mut content_length = response.content_length();
        let response: http::Response<reqwest::Body> = response.into();
        let (mut parts, mut body) = response.into_parts();
        if self.brotli
            && parts.headers.get(CONTENT_ENCODING) == Some(&HeaderValue::from_static("br"))
        {
            parts.headers.remove(CONTENT_ENCODING);
            parts.headers.remove(CONTENT_LENGTH);
            content_length = None;
            body = reqwest::Body::wrap(BrotliBody::new(body));
        }
        ResponseParts {
            head: RyResponseHead::from_parts(
                parts.status,
                parts.headers,
                url,
                content_length,
                parts.version,
                remote_addr,
            ),
            body,
        }
    }
}
