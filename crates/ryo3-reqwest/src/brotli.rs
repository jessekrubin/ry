//! Incremental Brotli response decoding with Burli.

use std::io;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

use burli_decode::push::{DecodeStatus, PushDecoder};
use bytes::{Buf, Bytes, BytesMut};
use http_body::{Body, Frame};

const OUTPUT_CHUNK_SIZE: usize = 64 * 1024;

pub(crate) struct BrotliBody {
    body: reqwest::Body,
    decoder: PushDecoder,
    input: Bytes,
    output: BytesMut,
    state: State,
}

#[derive(Clone, Copy)]
enum State {
    Reading { started: bool },
    Decoding,
    Draining,
    Done,
}

impl BrotliBody {
    pub(crate) fn new(body: reqwest::Body) -> Self {
        Self {
            body,
            decoder: PushDecoder::new(),
            input: Bytes::new(),
            output: BytesMut::new(),
            state: State::Reading { started: false },
        }
    }

    fn fail(&mut self, error: io::Error) -> Poll<Option<io::Result<Frame<Bytes>>>> {
        self.state = State::Done;
        Poll::Ready(Some(Err(error)))
    }
}

impl Body for BrotliBody {
    type Data = Bytes;
    type Error = io::Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let this = self.get_mut();
        loop {
            match this.state {
                State::Reading { started } => {
                    match ready!(Pin::new(&mut this.body).poll_frame(cx)) {
                        Some(Ok(frame)) => match frame.into_data() {
                            Ok(data) if !data.is_empty() => {
                                this.input = data;
                                this.state = State::Decoding;
                            }
                            Ok(_) => {}
                            Err(frame) if !started => return Poll::Ready(Some(Ok(frame))),
                            Err(_) => return this.fail(incomplete_body()),
                        },
                        Some(Err(error)) => return this.fail(io::Error::other(error)),
                        None if started => return this.fail(incomplete_body()),
                        None => this.state = State::Done,
                    }
                }
                State::Decoding => {
                    this.output.resize(OUTPUT_CHUNK_SIZE, 0);
                    let progress = match this.decoder.decompress(&this.input, &mut this.output) {
                        Ok(progress) => progress,
                        Err(error) => {
                            return this.fail(io::Error::new(io::ErrorKind::InvalidData, error));
                        }
                    };
                    this.input.advance(progress.consumed);
                    this.state = match progress.status {
                        DecodeStatus::Finished => State::Draining,
                        DecodeStatus::NeedsInput if this.input.is_empty() => {
                            State::Reading { started: true }
                        }
                        _ => State::Decoding,
                    };
                    if progress.written != 0 {
                        let data = this.output.split_to(progress.written).freeze();
                        return Poll::Ready(Some(Ok(Frame::data(data))));
                    }
                }
                State::Draining => {
                    // Finish HTTP framing for errors and keep-alive reuse. Ignore
                    // trailing encoded data, but forward HTTP trailers.
                    match ready!(Pin::new(&mut this.body).poll_frame(cx)) {
                        Some(Ok(frame)) if frame.is_trailers() => {
                            return Poll::Ready(Some(Ok(frame)));
                        }
                        Some(Ok(_)) => {}
                        Some(Err(error)) => return this.fail(io::Error::other(error)),
                        None => this.state = State::Done,
                    }
                }
                State::Done => return Poll::Ready(None),
            }
        }
    }

    fn is_end_stream(&self) -> bool {
        matches!(self.state, State::Done)
    }
}

fn incomplete_body() -> io::Error {
    io::Error::new(
        io::ErrorKind::UnexpectedEof,
        "incomplete Brotli response body",
    )
}

#[cfg(test)]
mod tests {
    use futures_util::stream;
    use http::{HeaderMap, HeaderValue};
    use http_body_util::{BodyExt, StreamBody};

    use super::*;

    const ENCODED: &[u8] = b"\x1b\x13\x00\xf8\xa5\xb0\xb2\x82\x84\x00\x00\x1e";
    const DECODED: &[u8] = b"XXXXXXXXXXYYYYYYYYYY";

    fn from_frames(frames: Vec<io::Result<Frame<Bytes>>>) -> BrotliBody {
        BrotliBody::new(reqwest::Body::wrap(StreamBody::new(stream::iter(frames))))
    }

    #[tokio::test]
    async fn decoded_size_is_unknown() -> io::Result<()> {
        let body = BrotliBody::new(reqwest::Body::from(ENCODED));
        assert_eq!(body.size_hint().exact(), None);
        assert_eq!(body.collect().await?.to_bytes(), DECODED);
        Ok(())
    }

    #[tokio::test]
    async fn fragmented_input_and_trailers() -> io::Result<()> {
        let mut trailers = HeaderMap::new();
        trailers.insert("x-checksum", HeaderValue::from_static("preserved"));
        let mut frames = vec![Ok(Frame::data(Bytes::new()))];
        frames.extend(
            ENCODED
                .chunks(1)
                .map(|chunk| Ok(Frame::data(Bytes::copy_from_slice(chunk)))),
        );
        frames.push(Ok(Frame::data(Bytes::from_static(b"trailing bytes"))));
        frames.push(Ok(Frame::trailers(trailers.clone())));
        let mut body = from_frames(frames);
        let mut decoded = Vec::new();
        let mut received_trailers = None;
        while let Some(frame) = body.frame().await {
            match frame?.into_data() {
                Ok(data) => {
                    assert!(received_trailers.is_none());
                    decoded.extend_from_slice(&data);
                }
                Err(frame) => {
                    assert!(!body.is_end_stream());
                    received_trailers = Some(frame.into_trailers().expect("trailers"));
                }
            }
        }
        assert_eq!(decoded, DECODED);
        assert_eq!(received_trailers, Some(trailers));
        assert!(body.is_end_stream());
        assert!(body.frame().await.is_none());
        Ok(())
    }

    #[tokio::test]
    async fn truncated_input_fails_once() {
        let mut body = BrotliBody::new(reqwest::Body::from(&ENCODED[..1]));
        let error = body
            .frame()
            .await
            .expect("error frame")
            .expect_err("truncated input");
        assert_eq!(error.kind(), io::ErrorKind::UnexpectedEof);
        assert!(body.is_end_stream());
        assert!(body.frame().await.is_none());
    }

    #[tokio::test]
    async fn draining_preserves_transport_errors() -> io::Result<()> {
        let mut body = from_frames(vec![
            Ok(Frame::data(Bytes::from_static(ENCODED))),
            Err(io::Error::new(
                io::ErrorKind::ConnectionReset,
                "connection reset",
            )),
        ]);
        let frame = body.frame().await.expect("decoded frame")?;
        assert_eq!(frame.into_data().expect("data"), DECODED);
        assert!(body.frame().await.expect("transport error").is_err());
        assert!(body.is_end_stream());
        assert!(body.frame().await.is_none());
        Ok(())
    }
}
