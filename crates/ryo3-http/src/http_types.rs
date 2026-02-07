//! "new" types for the http crate
//!
//! Used for the py-conversions mod

use http::{HeaderMap, HeaderValue};
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyHttpHeaderMap(pub(crate) HeaderMap<HeaderValue>);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PyHttpMethod(pub(crate) http::Method);

impl PyHttpMethod {
    pub const GET: Self = Self(http::Method::GET);
    pub const POST: Self = Self(http::Method::POST);
    pub const PUT: Self = Self(http::Method::PUT);
    pub const DELETE: Self = Self(http::Method::DELETE);
    pub const HEAD: Self = Self(http::Method::HEAD);
    pub const OPTIONS: Self = Self(http::Method::OPTIONS);
    pub const PATCH: Self = Self(http::Method::PATCH);
    pub const TRACE: Self = Self(http::Method::TRACE);
    pub const CONNECT: Self = Self(http::Method::CONNECT);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyHttpHeaderName(pub(crate) http::HeaderName);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PyHttpHeaderNameRef<'a>(pub(crate) &'a http::HeaderName);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyHttpHeaderValue(pub(crate) HeaderValue);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyHttpHeaderValueRef<'a>(pub(crate) &'a HeaderValue);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PyHttpVersion(pub(crate) http::Version);

impl PyHttpVersion {
    /// `HTTP/0.9`
    pub const HTTP_09: Self = Self(http::Version::HTTP_09);

    /// `HTTP/1.0`
    pub const HTTP_10: Self = Self(http::Version::HTTP_10);

    /// `HTTP/1.1`
    pub const HTTP_11: Self = Self(http::Version::HTTP_11);

    /// `HTTP/2.0`
    pub const HTTP_2: Self = Self(http::Version::HTTP_2);

    /// `HTTP/3.0`
    pub const HTTP_3: Self = Self(http::Version::HTTP_3);
}
// ============================================================================
//  FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM ~ FROM
// ============================================================================

impl From<HeaderMap> for PyHttpHeaderMap {
    fn from(h: HeaderMap) -> Self {
        Self(h)
    }
}
impl From<PyHttpHeaderMap> for HeaderMap {
    fn from(h: PyHttpHeaderMap) -> Self {
        h.0
    }
}

impl From<PyHttpHeaderName> for http::HeaderName {
    fn from(h: PyHttpHeaderName) -> Self {
        h.0
    }
}

impl From<PyHttpHeaderValue> for HeaderValue {
    fn from(h: PyHttpHeaderValue) -> Self {
        h.0
    }
}

impl From<PyHttpMethod> for http::Method {
    fn from(h: PyHttpMethod) -> Self {
        h.0
    }
}

impl From<http::HeaderName> for PyHttpHeaderName {
    fn from(h: http::HeaderName) -> Self {
        Self(h)
    }
}

impl From<HeaderValue> for PyHttpHeaderValue {
    fn from(h: HeaderValue) -> Self {
        Self(h)
    }
}

impl From<http::Method> for PyHttpMethod {
    fn from(h: http::Method) -> Self {
        Self(h)
    }
}

impl From<http::Version> for PyHttpVersion {
    fn from(h: http::Version) -> Self {
        Self(h)
    }
}

impl From<PyHttpVersion> for http::Version {
    fn from(h: PyHttpVersion) -> Self {
        h.0
    }
}

// impl from ref
impl From<&HeaderValue> for PyHttpHeaderValue {
    // clone should be totally fine bc the http lib uses the `Bytes` crate
    fn from(h: &HeaderValue) -> Self {
        Self(h.clone())
    }
}

impl<'a> From<&'a HeaderValue> for PyHttpHeaderValueRef<'a> {
    fn from(h: &'a HeaderValue) -> Self {
        Self(h)
    }
}

// ============================================================================
// DEREF
// ============================================================================
impl Deref for PyHttpHeaderName {
    type Target = http::HeaderName;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for PyHttpHeaderValue {
    type Target = HeaderValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for PyHttpMethod {
    type Target = http::Method;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
