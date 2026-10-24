//! sse decoder adapted from `sse-core` (v0.2.3)
//!
//! BIG PROPS TO PIZZASBEARS
//!
//! changes from `sse-core`:
//! - std instead of alloc/core (no `no_std`)
//! - no `thiserror`/`serde`
//! - dropped the bits ry doesn't use (`new`/`Default`/`reconnect*`/`clear`)
//! - crate-private + clippy-ified (no unwraps)
//! - zero-copy `data` (`Utf8Bytes`; single lines are sliced out of the chunk)
//!
//! REF: <https://github.com/PizzasBear/sse-rs/blob/aad868a4d388af83d0b93b6ba6a83fc54ec74f12/sse-core/src/decode.rs>
//! REF: <https://crates.io/crates/sse-core>
//! REF: <https://github.com/PizzasBear/sse-rs/blob/aad868a4d388af83d0b93b6ba6a83fc54ec74f12/LICENSE-MIT>
//!
//! ---
//!
//! `sse-core` is licensed `MIT OR Apache-2.0`; used here under MIT:
//!
//! Copyright (c) 2026 Max Shteimberg
//!
//! Permission is hereby granted, free of charge, to any
//! person obtaining a copy of this software and associated
//! documentation files (the "Software"), to deal in the
//! Software without restriction, including without
//! limitation the rights to use, copy, modify, merge,
//! publish, distribute, sublicense, and/or sell copies of
//! the Software, and to permit persons to whom the Software
//! is furnished to do so, subject to the following
//! conditions:
//!
//! The above copyright notice and this permission notice
//! shall be included in all copies or substantial portions
//! of the Software.
//!
//! THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
//! ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
//! TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
//! PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
//! SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
//! CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
//! OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
//! IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.
use std::borrow::Cow;
use std::num::{NonZeroU8, NonZeroUsize};
use std::ops::Deref;
use std::sync::Arc;
use std::{fmt, iter, mem, str};

use bytes::{Buf, Bytes};
use memchr::{memchr, memchr2};

/// Represents a single Server-Sent Event message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MessageEvent {
    /// The event name (defaults to `"message"`).
    pub(crate) event: Cow<'static, str>,
    /// The payload data.
    pub(crate) data: Utf8Bytes,
    /// The `Last-Event-ID` sent by the server, if any.
    pub(crate) last_event_id: Option<Arc<str>>,
}

/// `Bytes` known to be utf-8 (AKA a string)
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Utf8Bytes(Bytes);

impl Utf8Bytes {
    /// zero-copy for valid utf-8; invalid bytes get replaced (and copied)
    fn from_bytes_lossy(bytes: Bytes) -> Self {
        if str::from_utf8(&bytes).is_ok() {
            Self(bytes)
        } else {
            Self(Bytes::from(String::from_utf8_lossy(&bytes).into_owned()))
        }
    }

    #[inline]
    pub(crate) fn as_str(&self) -> &str {
        // SAFETY: only ever built from valid utf-8 (see `from_bytes_lossy`)
        #[expect(unsafe_code)]
        unsafe {
            str::from_utf8_unchecked(&self.0)
        }
    }

    #[inline]
    pub(crate) fn as_bytes(&self) -> &Bytes {
        &self.0
    }
}

impl Deref for Utf8Bytes {
    type Target = str;

    #[inline]
    fn deref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
impl From<&str> for Utf8Bytes {
    fn from(s: &str) -> Self {
        Self(Bytes::copy_from_slice(s.as_bytes()))
    }
}

#[cfg(test)]
impl From<String> for Utf8Bytes {
    fn from(s: String) -> Self {
        Self(Bytes::from(s))
    }
}

/// Commands and payloads yielded by the SSE stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SseEvent {
    /// A standard data message.
    Message(MessageEvent),
    /// A server request to change the client's reconnect time (in milliseconds).
    Retry(u32),
}

/// Error indicating that a parsed field exceeded the maximum allowed buffer size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub(crate) struct PayloadTooLargeError;

impl fmt::Display for PayloadTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("payload exceeded the allotted buffer size limit")
    }
}

impl std::error::Error for PayloadTooLargeError {}

const MAX_DEBUG_SIZE: usize = 200;

struct ShowBigStr<'a>(&'a str);

impl fmt::Debug for ShowBigStr<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut end = self.0.len().min(MAX_DEBUG_SIZE);
        while !self.0.is_char_boundary(end) {
            end -= 1;
        }
        let s = &self.0[..end];

        fmt::Debug::fmt(s, f)?;
        if end < self.0.len() {
            write!(f, "... ({} bytes total)", self.0.len())?;
        }

        Ok(())
    }
}

struct ShowBigBuf<'a>(&'a [u8]);

impl fmt::Debug for ShowBigBuf<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (buf, truncated) = match self.0.len() {
            ..=MAX_DEBUG_SIZE => (self.0, false),
            _ => (&self.0[..MAX_DEBUG_SIZE], true),
        };

        let mut chunks = buf.utf8_chunks().peekable();

        f.write_str("\"")?;
        while let Some(chunk) = chunks.next() {
            fmt::Display::fmt(&chunk.valid().escape_debug(), f)?;

            let invalid = chunk.invalid();
            if invalid.is_empty() {
                continue;
            }

            // If we truncated and this is the very last chunk, the invalid bytes
            // are almost certainly just a sliced multi-byte UTF-8 character.
            if truncated && chunks.peek().is_none() {
                break;
            }

            for &byte in invalid {
                write!(f, "\\x{byte:02X}")?;
            }
        }

        if truncated {
            write!(f, "\"... ({} bytes total)", self.0.len())?;
        } else {
            f.write_str("\"")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
enum ValueMode {
    Data,
    Event,
    Retry,
    Id,
}

impl ValueMode {
    const fn field_name(self) -> &'static str {
        match self {
            Self::Data => "data",
            Self::Event => "event",
            Self::Retry => "retry",
            Self::Id => "id",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Mode {
    Bom { bytes_read: u8 },
    Field(Option<(ValueMode, NonZeroU8)>),
    Value(ValueMode),
    Ignore,
    PostCr,
    PostColon(ValueMode),
}

/// The core state-machine parser for SSE.
///
/// This decoder does not perform any I/O. It consumes bytes from a given buffer
/// and yields parsed [`SseEvent`]s. It is suitable for `no_std` environments.
#[derive(Clone)]
pub(crate) struct SseDecoder {
    mode: Mode,
    last_event_id: Option<Arc<str>>,
    staged_last_event_id: Option<Arc<str>>,
    last_event_id_buf: Vec<u8>,
    event_buf: Vec<u8>,
    /// single `data:` line sliced from the chunk (trailing `\n` implied)
    data_line: Option<Bytes>,
    data_buf: Vec<u8>,
    retry_buf: Option<u32>,
    max_payload_size: NonZeroUsize,
    corrupted: bool,
}

impl SseDecoder {
    /// Creates a new decoder with a custom maximum payload size limit.
    ///
    /// This is useful in memory-constrained environments or when connecting to
    /// untrusted servers to prevent memory exhaustion from unbounded input.
    ///
    /// The limit applies independently to each of the three values an event
    /// accumulates — its data, its name and its ID — so peak memory is a small
    /// multiple of `max_payload_size` rather than exactly that. See
    /// [`next()`](Self::next) for what each of them accumulates.
    #[inline]
    #[must_use]
    pub(crate) fn with_limit(max_payload_size: NonZeroUsize) -> Self {
        Self {
            mode: Mode::Bom { bytes_read: 0 },
            last_event_id: None,
            staged_last_event_id: None,
            last_event_id_buf: vec![],
            event_buf: vec![],
            data_line: None,
            data_buf: vec![],
            retry_buf: None,
            max_payload_size,
            corrupted: false,
        }
    }

    /// Returns the current `Last-Event-ID` known to the decoder, if any.
    #[inline]
    #[must_use]
    pub(crate) fn last_event_id(&self) -> Option<&Arc<str>> {
        self.last_event_id.as_ref()
    }

    fn mark_corrupted(&mut self) {
        self.clear_bufs();
        self.corrupted = true;
    }

    /// Appends the newline separating two `data` lines, subject to the same limit
    /// as the line contents. `consume_until_newline` never sees these bytes, so
    /// without this a stream of empty `data` lines would grow `data_buf` unbounded.
    ///
    /// A *trailing* separator is allowed to sit one byte past the limit, since it is
    /// popped at dispatch rather than delivered; anything appended after it is
    /// checked against the full length and so still errors.
    fn push_data_newline(&mut self) -> Result<(), PayloadTooLargeError> {
        self.unslice_data_line();
        if self.max_payload_size.get() < self.data_buf.len() {
            self.mark_corrupted();
            return Err(PayloadTooLargeError);
        }
        self.data_buf.push(b'\n');
        Ok(())
    }

    /// move the sliced line (+ `\n`) into `data_buf`
    fn unslice_data_line(&mut self) {
        if let Some(line) = self.data_line.take() {
            self.data_buf.reserve(line.len() + 1);
            self.data_buf.extend_from_slice(&line);
            self.data_buf.push(b'\n');
        }
    }

    /// slice the line if it's the first + all in this chunk, else `data_buf`
    fn consume_data(&mut self, buf: &mut impl Buf) -> Result<(), PayloadTooLargeError> {
        if self.data_line.is_none()
            && self.data_buf.is_empty()
            && let Some(i) = memchr2(b'\r', b'\n', buf.chunk())
        {
            if self.max_payload_size.get() < i {
                self.mode = Mode::Ignore;
                return Err(PayloadTooLargeError);
            }
            self.mode = match buf.chunk()[i] {
                b'\r' => Mode::PostCr,
                _ => Mode::Field(None),
            };
            self.data_line = Some(buf.copy_to_bytes(i));
            buf.advance(1);
            return Ok(());
        }
        self.unslice_data_line();
        if consume_until_newline(
            &mut self.mode,
            Some(&mut self.data_buf),
            self.max_payload_size,
            buf,
        )? {
            self.push_data_newline()?;
        }
        Ok(())
    }

    #[inline]
    fn clear_bufs(&mut self) {
        self.data_line = None;
        self.data_buf.clear();
        self.event_buf.clear();
        self.last_event_id_buf.clear();
        self.staged_last_event_id = self.last_event_id.clone();
    }

    /// An event boundary that yields nothing — the blank line after a comment, a
    /// keepalive, a discarded event — is by far the most common one on an idle
    /// stream, so that case is kept small enough to inline into [`next()`](Self::next)
    /// and the message-building tail is pushed out of line into
    /// [`dispatch_message()`](Self::dispatch_message).
    #[inline]
    fn dispatch(&mut self, cr: bool) -> Option<SseEvent> {
        self.mode = if cr { Mode::PostCr } else { Mode::Field(None) };

        if self.corrupted {
            self.corrupted = false;
            // bufs should be clear already
            return None;
        }

        self.last_event_id = self.staged_last_event_id.clone();

        // No data means no event, per the spec's "if the data buffer is empty" step.
        if self.data_line.is_none() && self.data_buf.is_empty() {
            self.event_buf.clear();
            return None;
        }

        Some(self.dispatch_message())
    }

    /// The allocating half of [`dispatch()`](Self::dispatch), split out so that
    /// inlining the no-event fast path above does not drag this into every one of
    /// its call sites.
    fn dispatch_message(&mut self) -> SseEvent {
        let data = match self.data_line.take() {
            Some(line) => line,
            None => self.take_data_buf(),
        };

        let event = match &*self.event_buf {
            b"" => Cow::Borrowed("message"),
            event_buf => Cow::Owned(from_utf8_lossy(event_buf).into_owned()),
        };
        self.event_buf.clear();

        SseEvent::Message(MessageEvent {
            data: Utf8Bytes::from_bytes_lossy(data),
            event,
            last_event_id: self.last_event_id.clone(),
        })
    }

    fn take_data_buf(&mut self) -> Bytes {
        // The trailing separator appended by the last `data` line is not part of
        // the payload.
        if let Some(b'\n') = self.data_buf.last() {
            self.data_buf.pop();
        }

        // Hand the accumulated bytes over to `Bytes` rather than copying them
        // into a fresh allocation: on a large event that copy costs more than the
        // rest of the decoder put together. `data_buf` is replaced with an empty
        // buffer of the same capacity so the next event still accumulates without
        // reallocating, which leaves peak memory where it was — the copying version
        // also held the payload twice for the duration of the copy.
        let data = mem::take(&mut self.data_buf);
        self.data_buf = Vec::with_capacity(data.capacity());
        Bytes::from(data)
    }

    /// Consumes bytes from the provided buffer and attempts to yield an event.
    ///
    /// The decoder does not store unparsed bytes internally. It reads directly
    /// from the provided buffer, advancing the buffer's cursor only for the bytes
    /// it successfully parses.
    ///
    /// If `Ok(None)` is returned, the provided buffer has been exhausted and
    /// more bytes are needed to complete the current event. You should fetch more
    /// data, append it to your buffer, and call `next()` again.
    ///
    /// # Errors
    ///
    /// Returns a [`PayloadTooLargeError`] when an event's accumulated data, name
    /// or ID would exceed the maximum payload size configured for this decoder.
    /// The limit is *not* per line:
    ///
    /// * `data` accumulates every `data:` line of the current event, joined by
    ///   newlines, so the limit bounds the whole event payload and is only reset
    ///   once the event is dispatched.
    /// * `event` and `id` are reset by each `event:` / `id:` field, so for those
    ///   the limit does bound a single line.
    ///
    /// The offending event is discarded rather than silently truncated: the
    /// decoder skips the rest of the current event and resynchronizes at the next
    /// event boundary, so it stays usable after the error. `retry:` fields are
    /// still honoured while the discarded event is being skipped.
    #[expect(clippy::too_many_lines)]
    pub(crate) fn next(
        &mut self,
        buf: &mut impl Buf,
    ) -> Option<Result<SseEvent, PayloadTooLargeError>> {
        // # 9.2.5 Parsing an event stream
        //
        // stream        = [ bom ] *event
        // event         = *( comment / field ) end-of-line
        // comment       = colon *any-char end-of-line
        // field         = 1*name-char [ colon [ space ] *any-char ] end-of-line
        // end-of-line   = ( cr lf / cr / lf )
        //
        // ; characters
        // lf            = %x000A ; U+000A LINE FEED (LF)
        // cr            = %x000D ; U+000D CARRIAGE RETURN (CR)
        // space         = %x0020 ; U+0020 SPACE
        // colon         = %x003A ; U+003A COLON (:)
        // bom           = %xFEFF ; U+FEFF BYTE ORDER MARK
        // name-char     = %x0000-0009 / %x000B-000C / %x000E-0039 / %x003B-10FFFF
        //                 ; a scalar value other than U+000A LINE FEED (LF), U+000D CARRIAGE RETURN (CR), or U+003A COLON (:)
        // any-char      = %x0000-0009 / %x000B-000C / %x000E-10FFFF
        //                 ; a scalar value other than U+000A LINE FEED (LF) or U+000D CARRIAGE RETURN (CR)

        loop {
            let chunk = buf.chunk();
            if chunk.is_empty() {
                return None;
            }

            match &mut self.mode {
                Mode::Bom { bytes_read } => {
                    const BOM: &[u8; 3] = b"\xef\xbb\xbf";

                    let b0 = chunk[0];

                    if b0 != BOM[*bytes_read as usize] {
                        self.mode = match *bytes_read {
                            0 => Mode::Field(None),
                            _ => Mode::Ignore,
                        };
                        continue;
                    }

                    buf.advance(1);
                    *bytes_read += 1;

                    if BOM.len() <= *bytes_read as usize {
                        self.mode = Mode::Field(None);
                    }
                }
                Mode::Field(None) => {
                    let b0 = chunk[0];
                    buf.advance(1);
                    let mode = match b0 {
                        b'd' if !self.corrupted => ValueMode::Data,
                        b'e' if !self.corrupted => ValueMode::Event,
                        b'i' if !self.corrupted => ValueMode::Id,
                        b'r' => ValueMode::Retry,

                        b'\n' | b'\r' => match self.dispatch(b0 == b'\r') {
                            Some(ev) => return Some(Ok(ev)),
                            None => continue,
                        },

                        _ => {
                            self.mode = Mode::Ignore;

                            // Skip the rest of the line here rather than looping back
                            // through the `self.mode` dispatch: comments are the most
                            // common line shape in a keepalive-heavy stream, and the
                            // extra trip through the state machine costs more than the
                            // scan it guards.
                            consume_until_newline(&mut self.mode, None, self.max_payload_size, buf)
                                .expect("there should be no payload to grow too large");

                            // A comment is usually the last line of its event — a
                            // keepalive is nothing but a comment and the blank line
                            // after it — so consume that blank line here too when it
                            // is the empty dispatch, which is the only outcome a
                            // keepalive can have. Anything that would actually yield
                            // an event, or resume in another mode, is left to the
                            // regular path below rather than duplicated here.
                            let blank_line_follows = matches!(self.mode, Mode::Field(None))
                                && buf.chunk().first() == Some(&b'\n');

                            if blank_line_follows && !self.corrupted && self.data_buf.is_empty() {
                                buf.advance(1);
                                self.last_event_id = self.staged_last_event_id.clone();
                                self.event_buf.clear();
                            }
                            continue;
                        }
                    };
                    self.mode = Mode::Field(Some((mode, NonZeroU8::MIN)));
                }
                &mut Mode::Field(Some((mode, ref mut len))) => {
                    let cmp = &mode.field_name().as_bytes()[len.get() as usize..];
                    if iter::zip(chunk, cmp).any(|(ch0, ch1)| ch0 != ch1) {
                        self.mode = Mode::Ignore;
                        continue;
                    }
                    let Some(&b_post) = chunk.get(cmp.len()) else {
                        *len = len.saturating_add(u8::try_from(chunk.len()).unwrap_or(u8::MAX));
                        buf.advance(chunk.len());
                        continue;
                    };
                    buf.advance(cmp.len() + 1);

                    match b_post {
                        b'\n' => self.mode = Mode::Field(None),
                        b'\r' => self.mode = Mode::PostCr,
                        b':' => {
                            match mode {
                                ValueMode::Data => {}
                                ValueMode::Event => self.event_buf.clear(),
                                ValueMode::Id => self.last_event_id_buf.clear(),
                                ValueMode::Retry => self.retry_buf = None,
                            }

                            // Skip the optional space here rather than spending a
                            // whole trip through the dispatch on a single byte. If
                            // the chunk ends on the colon there is nothing to look
                            // at yet, so fall back to resuming in `PostColon`.
                            self.mode = match buf.chunk().first() {
                                Some(&b' ') => {
                                    buf.advance(1);
                                    Mode::Value(mode)
                                }
                                Some(_) => Mode::Value(mode),
                                None => Mode::PostColon(mode),
                            };
                            continue;
                        }
                        _ => {
                            self.mode = Mode::Ignore;
                            continue;
                        }
                    }

                    // A field name with no colon carries the empty string as its
                    // value, so the target buffer must be reset, not left alone.
                    // `retry` is the exception: it accumulates nothing, and the
                    // `retry:` form already clears `retry_buf` before parsing into it.
                    match mode {
                        ValueMode::Data => {
                            if let Err(err) = self.push_data_newline() {
                                return Some(Err(err));
                            }
                        }
                        ValueMode::Event => self.event_buf.clear(),
                        ValueMode::Id => {
                            self.last_event_id_buf.clear();
                            self.staged_last_event_id = None;
                        }
                        ValueMode::Retry => {}
                    }
                }
                Mode::Value(ValueMode::Retry) => {
                    let mut advanced = 0;
                    let mut return_event = false;

                    for &b in chunk {
                        advanced += 1;
                        match b {
                            b'0'..=b'9' => {
                                let digit = u32::from(b & 0xf);

                                let retry_buf = self.retry_buf.unwrap_or(0);
                                let Some(retry_buf) = retry_buf.checked_mul(10) else {
                                    self.mode = Mode::Ignore;
                                    break;
                                };
                                let Some(retry_buf) = retry_buf.checked_add(digit) else {
                                    self.mode = Mode::Ignore;
                                    break;
                                };
                                self.retry_buf = Some(retry_buf);
                            }
                            b'\r' => {
                                self.mode = Mode::PostCr;
                                return_event = true;
                                break;
                            }
                            b'\n' => {
                                self.mode = Mode::Field(None);
                                return_event = true;
                                break;
                            }
                            _ => {
                                self.mode = Mode::Ignore;
                                break;
                            }
                        }
                    }

                    buf.advance(advanced);

                    if let (true, Some(retry_buf)) = (return_event, self.retry_buf) {
                        return Some(Ok(SseEvent::Retry(retry_buf)));
                    }
                }
                Mode::Value(ValueMode::Data) => {
                    if let Err(err) = self.consume_data(buf) {
                        self.mark_corrupted();
                        return Some(Err(err));
                    }
                }
                Mode::Value(ValueMode::Event) => {
                    if let Err(err) = consume_until_newline(
                        &mut self.mode,
                        Some(&mut self.event_buf),
                        self.max_payload_size,
                        buf,
                    ) {
                        self.mark_corrupted();
                        return Some(Err(err));
                    }
                }
                Mode::Value(ValueMode::Id) => {
                    match consume_until_newline(
                        &mut self.mode,
                        Some(&mut self.last_event_id_buf),
                        self.max_payload_size,
                        buf,
                    ) {
                        Ok(true) => {
                            if memchr(0, &self.last_event_id_buf).is_none() {
                                self.staged_last_event_id = match &*self.last_event_id_buf {
                                    [] => None,
                                    buf => Some(from_utf8_lossy(buf).into()),
                                };
                            }
                            self.last_event_id_buf.clear();
                        }
                        Ok(false) => {}
                        Err(err) => {
                            self.mark_corrupted();
                            return Some(Err(err));
                        }
                    }
                }
                Mode::Ignore => {
                    consume_until_newline(&mut self.mode, None, self.max_payload_size, buf)
                        .expect("there should be no payload to grow too large");
                }
                Mode::PostCr => {
                    if chunk[0] == b'\n' {
                        buf.advance(1);
                    }
                    self.mode = Mode::Field(None);
                }
                Mode::PostColon(value) => {
                    if chunk[0] == b' ' {
                        buf.advance(1);
                    }
                    self.mode = Mode::Value(*value);
                }
            }
        }
    }
}

impl fmt::Debug for SseDecoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SseDecoder")
            .field("mode", &self.mode)
            .field(
                "last_event_id",
                &self.last_event_id.as_deref().map(ShowBigStr),
            )
            .field(
                "staged_last_event_id",
                &self.staged_last_event_id.as_deref().map(ShowBigStr),
            )
            .field("last_event_id_buf", &ShowBigBuf(&self.last_event_id_buf))
            .field("event_buf", &ShowBigBuf(&self.event_buf))
            .field("data_line", &self.data_line.as_deref().map(ShowBigBuf))
            .field("data_buf", &ShowBigBuf(&self.data_buf))
            .field("retry_buf", &self.retry_buf)
            .field("max_payload_size", &self.max_payload_size)
            .field("corrupted", &self.corrupted)
            .finish()
    }
}

/// Drop-in replacement for [`String::from_utf8_lossy`] that validates with
/// [`str::from_utf8`] first.
///
/// The lossy conversion walks its input with the scalar `Utf8Chunks` iterator
/// even when the input needs no repair at all, which measures ~7x slower than
/// `str::from_utf8`'s word-at-a-time ASCII scan. Every event carrying a payload
/// goes through here, so the valid case is worth splitting out; malformed input
/// still takes the same lossy path it always did.
#[inline]
fn from_utf8_lossy(buf: &[u8]) -> Cow<'_, str> {
    match str::from_utf8(buf) {
        Ok(s) => Cow::Borrowed(s),
        Err(_) => String::from_utf8_lossy(buf),
    }
}

fn consume_until_newline(
    mode: &mut Mode,
    mut out: Option<&mut Vec<u8>>,
    max_size: NonZeroUsize,
    buf: &mut impl Buf,
) -> Result<bool, PayloadTooLargeError> {
    loop {
        let chunk = buf.chunk();
        if chunk.is_empty() {
            return Ok(false);
        }

        let Some(i) = memchr2(b'\r', b'\n', chunk) else {
            if let Some(out) = out.as_deref_mut() {
                if max_size.get() < out.len() + chunk.len() {
                    out.clear();
                    *mode = Mode::Ignore;
                    return Err(PayloadTooLargeError);
                }
                out.extend_from_slice(chunk);
            }
            buf.advance(chunk.len());
            continue;
        };

        if let Some(out) = out.as_deref_mut() {
            if max_size.get() < out.len() + i {
                out.clear();
                *mode = Mode::Ignore;
                return Err(PayloadTooLargeError);
            }
            out.extend_from_slice(&chunk[..i]);
        }

        *mode = match chunk[i] {
            b'\r' => Mode::PostCr,
            b'\n' => Mode::Field(None),
            _ => unreachable!(),
        };

        buf.advance(i + 1);

        return Ok(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sse::SSE_MAX_EVENT_SIZE;

    #[test]
    fn hard_parse() -> Result<(), PayloadTooLargeError> {
        use core::slice;

        // Based on https://github.com/jpopesculian/eventsource-stream/blob/v0.2.3/tests/eventsource-stream.rs
        let bytes = "\u{FEFF}data: x

:

event: my-event\r
data:line1
data: line2
data
:
id: my-id
:should be ignored too\rretry:42
retry:

data:second

data:ignored
";

        let mut decoder = SseDecoder::with_limit(SSE_MAX_EVENT_SIZE);

        let events = bytes
            .bytes()
            .filter_map(|b| decoder.next(&mut slice::from_ref(&b)))
            .collect::<Result<Vec<_>, PayloadTooLargeError>>()?;

        let id = Some("my-id".into());

        assert_eq!(
            events,
            &[
                SseEvent::Message(MessageEvent {
                    event: "message".into(),
                    data: "x".into(),
                    last_event_id: None
                }),
                SseEvent::Retry(42),
                SseEvent::Message(MessageEvent {
                    event: "my-event".into(),
                    data: "line1\nline2\n".into(),
                    last_event_id: id.clone()
                }),
                SseEvent::Message(MessageEvent {
                    event: "message".into(),
                    data: "second".into(),
                    last_event_id: id.clone()
                })
            ]
        );

        // Feeding the whole buffer at once has to agree byte for byte with feeding it
        // one byte at a time. The two exercise different code: several arms take a
        // short cut when the bytes they need are already in the chunk, and those short
        // cuts are unreachable when every chunk is a single byte.
        let mut decoder = SseDecoder::with_limit(SSE_MAX_EVENT_SIZE);
        let mut buf = bytes.as_bytes();
        let whole = iter::from_fn(|| decoder.next(&mut buf)).collect::<Result<Vec<_>, _>>()?;
        assert_eq!(whole, events);

        Ok(())
    }

    /// Per the spec, a line with no colon is a field whose value is the empty string,
    /// so a bare `event` / `id` must *reset* its buffer rather than leave the previous
    /// value in place. Regression test: `id` leaking across events also leaks into the
    /// `Last-Event-ID` sent on reconnect.
    #[test]
    fn test_valueless_fields() {
        fn messages(bytes: &str) -> Vec<(String, String, Option<String>)> {
            let mut decoder = SseDecoder::with_limit(SSE_MAX_EVENT_SIZE);
            let mut buf = bytes.as_bytes();
            let mut out = vec![];
            while let Some(SseEvent::Message(msg)) =
                decoder.next(&mut buf).transpose().expect("not too large")
            {
                out.push((
                    msg.event.into_owned(),
                    msg.data.to_string(),
                    msg.last_event_id.map(|id| id.to_string()),
                ));
            }
            out
        }

        // A bare `event` resets the event type back to the "message" default.
        assert_eq!(
            messages("event: custom\nevent\ndata: x\n\n"),
            [("message".into(), "x".into(), None)],
        );

        // A bare `id` clears the ID, exactly like the `id:` form does.
        for bytes in [
            "id: abc\ndata: 1\n\nid\ndata: 2\n\n",
            "id: abc\ndata: 1\n\nid:\ndata: 2\n\n",
        ] {
            assert_eq!(
                messages(bytes),
                [
                    ("message".into(), "1".into(), Some("abc".into())),
                    ("message".into(), "2".into(), None),
                ],
                "for {bytes:?}",
            );
        }

        // A bare `data` still appends an empty line rather than resetting.
        assert_eq!(
            messages("data: a\ndata\ndata: b\n\n"),
            [("message".into(), "a\n\nb".into(), None)],
        );
    }

    /// Valueless `data` lines never reach `consume_until_newline`, so the separator each
    /// one appends has to be counted on its own. Regression test: without that check a
    /// server could grow `data_buf` a byte at a time, with no bound and no error, and
    /// have the oversized event delivered anyway.
    #[test]
    fn test_valueless_data_respects_the_limit() {
        let limit = NonZeroUsize::new(10).expect("non-zero");

        // A payload made only of separators is still bounded by the limit.
        let input = "data\n".repeat(100) + "\n";
        let mut buf = input.as_bytes();
        let mut decoder = SseDecoder::with_limit(limit);
        assert_eq!(decoder.next(&mut buf), Some(Err(PayloadTooLargeError)));

        // The rest of the event is skipped as a unit, leaving the decoder usable.
        assert_eq!(decoder.next(&mut buf), None);
        assert!(buf.is_empty());

        let mut buf: &[u8] = b"data: after\n\n";
        assert_eq!(
            decoder.next(&mut buf),
            Some(Ok(SseEvent::Message(MessageEvent {
                event: "message".into(),
                data: "after".into(),
                last_event_id: None,
            }))),
        );

        // A payload that exactly fills the limit is still legal, whatever it is made of.
        let input = "data\n".repeat(limit.get() + 1) + "\n";
        let mut buf = input.as_bytes();
        let mut decoder = SseDecoder::with_limit(limit);
        assert_eq!(
            decoder.next(&mut buf),
            Some(Ok(SseEvent::Message(MessageEvent {
                event: "message".into(),
                data: "\n".repeat(limit.get()).into(),
                last_event_id: None,
            }))),
        );
    }

    /// Invalid UTF-8 is lossy-converted rather than rejected, in every field that
    /// accumulates text. Regression test for [`from_utf8_lossy`]: it validates with
    /// `str::from_utf8` first and only falls back to the lossy walk, so the repaired
    /// output has to stay byte-for-byte what `String::from_utf8_lossy` produced.
    #[test]
    fn test_invalid_utf8_is_lossy() {
        let mut buf: &[u8] =
            b"event: ev\xffent\nid: my\xff-id\ndata: he\xed\xa0\x80llo\ndata: \xf0\x9f\x92\xa9 ok\n\n";

        let mut decoder = SseDecoder::with_limit(SSE_MAX_EVENT_SIZE);
        let Some(Ok(SseEvent::Message(msg))) = decoder.next(&mut buf) else {
            panic!("expected a message");
        };

        assert_eq!(msg.event, String::from_utf8_lossy(b"ev\xffent"));
        assert_eq!(
            &*msg.data,
            String::from_utf8_lossy(b"he\xed\xa0\x80llo\n\xf0\x9f\x92\xa9 ok")
        );
        assert_eq!(
            msg.last_event_id.as_deref(),
            Some(&*String::from_utf8_lossy(b"my\xff-id")),
        );

        // A valid multi-byte sequence must survive untouched.
        assert!(msg.data.contains('\u{1F4A9}'));
        assert!(buf.is_empty());
    }

    #[test]
    fn test_limits() {
        let mut stream: &[u8] = b"
data: 0123456789
id: my-id

id: 01234567890
data: thing
event: ev

data: mid

event: jojo
id: ignored
data: 01234
data: 56789
retry: 10

event: final
data

";

        let my_id = Some("my-id".into());

        let mut decoder = SseDecoder::with_limit(NonZeroUsize::new(10).expect("non-zero"));

        assert_eq!(
            decoder.next(&mut stream),
            Some(Ok(SseEvent::Message(MessageEvent {
                event: "message".into(),
                data: "0123456789".into(),
                last_event_id: my_id.clone(),
            })))
        );
        assert_eq!(decoder.next(&mut stream), Some(Err(PayloadTooLargeError)));
        assert_eq!(
            decoder.next(&mut stream),
            Some(Ok(SseEvent::Message(MessageEvent {
                event: "message".into(),
                data: "mid".into(),
                last_event_id: my_id.clone()
            })))
        );
        assert_eq!(decoder.next(&mut stream), Some(Err(PayloadTooLargeError)));
        assert_eq!(decoder.next(&mut stream), Some(Ok(SseEvent::Retry(10))));
        assert_eq!(
            decoder.next(&mut stream),
            Some(Ok(SseEvent::Message(MessageEvent {
                event: "final".into(),
                data: "".into(),
                last_event_id: my_id.clone()
            })))
        );
        assert!(stream.is_empty());
    }

    fn messages_from_bytes(chunks: &[&'static [u8]]) -> Vec<(Bytes, Utf8Bytes)> {
        let mut decoder = SseDecoder::with_limit(SSE_MAX_EVENT_SIZE);
        let mut out = vec![];
        for &c in chunks {
            let chunk = Bytes::from_static(c);
            let mut buf = chunk.clone();
            while let Some(ev) = decoder.next(&mut buf) {
                if let SseEvent::Message(msg) = ev.expect("not too large") {
                    out.push((chunk.clone(), msg.data));
                }
            }
        }
        out
    }

    fn is_within(chunk: &Bytes, data: &Utf8Bytes) -> bool {
        chunk.as_ptr_range().contains(&data.as_bytes().as_ptr())
    }

    #[test]
    fn test_single_line_is_zero_copy() {
        let got = messages_from_bytes(&[b": hi\n\nevent: e\ndata: one\n\ndata: two\r\n\r\n"]);
        assert_eq!(got.len(), 2);
        for ((chunk, data), expected) in got.iter().zip(["one", "two"]) {
            assert_eq!(&**data, expected);
            assert!(is_within(chunk, data), "{expected:?} was copied");
        }
    }

    #[test]
    fn test_multi_line_and_split_lines_still_join() {
        let got = messages_from_bytes(&[
            b"data: a\ndata: b\n\n",
            b"data: a\ndata\ndata: b\n\n",
            b"data: sp",
            b"lit\n\n",
            b"data: x\n",
            b"data: y\n\n",
        ]);
        let datas = got.iter().map(|(_, d)| &**d).collect::<Vec<_>>();
        assert_eq!(datas, ["a\nb", "a\n\nb", "split", "x\ny"]);
    }

    #[test]
    fn test_single_line_respects_the_limit() {
        let mut decoder = SseDecoder::with_limit(NonZeroUsize::new(4).expect("non-zero"));
        let mut buf = Bytes::from_static(b"data: toolong\n\ndata: ok\n\n");
        assert_eq!(decoder.next(&mut buf), Some(Err(PayloadTooLargeError)));
        let Some(Ok(SseEvent::Message(msg))) = decoder.next(&mut buf) else {
            panic!("expected a message");
        };
        assert_eq!(&*msg.data, "ok");
    }

    #[test]
    fn test_invalid_utf8_single_line_is_lossy() {
        let got = messages_from_bytes(&[b"data: a\xffb\n\n"]);
        assert_eq!(&*got[0].1, "a\u{FFFD}b");
    }
}
