//! Incremental Brotli response decoding with Burli.

use std::io;

use burli_decode::push::{DecodeStatus, PushDecoder};
use bytes::{Buf, Bytes};
use futures_util::stream;
use http_body_util::BodyExt;

const OUTPUT_CHUNK_SIZE: usize = 64 * 1024;

struct BrotliBody {
    body: reqwest::Body,
    decoder: PushDecoder,
    input: Bytes,
    status: DecodeStatus,
    started: bool,
}

impl BrotliBody {
    async fn next(&mut self) -> io::Result<Option<Bytes>> {
        let mut output = vec![0; OUTPUT_CHUNK_SIZE];
        loop {
            if self.status == DecodeStatus::Finished {
                // Finish the HTTP body as well, retaining transport errors and
                // allowing keep-alive reuse. Like reqwest, ignore trailing data
                // after the first Brotli stream.
                while let Some(frame) = self.body.frame().await {
                    frame.map_err(io::Error::other)?;
                }
                return Ok(None);
            }

            if self.status == DecodeStatus::NeedsInput && self.input.is_empty() {
                match self.body.frame().await {
                    Some(Ok(frame)) => {
                        let Ok(data) = frame.into_data() else {
                            continue;
                        };
                        if data.is_empty() {
                            continue;
                        }
                        self.input = data;
                        self.started = true;
                    }
                    Some(Err(error)) => return Err(io::Error::other(error)),
                    None if !self.started => return Ok(None),
                    None => {
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            "incomplete Brotli response body",
                        ));
                    }
                }
            }

            let progress = self
                .decoder
                .decompress(&self.input, &mut output)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
            self.input.advance(progress.consumed);
            self.status = progress.status;
            if progress.written != 0 {
                output.truncate(progress.written);
                return Ok(Some(Bytes::from(output)));
            }
        }
    }
}

pub(crate) fn decode_body(body: reqwest::Body) -> reqwest::Body {
    let state = BrotliBody {
        body,
        decoder: PushDecoder::new(),
        input: Bytes::new(),
        status: DecodeStatus::NeedsInput,
        started: false,
    };
    reqwest::Body::wrap_stream(stream::try_unfold(state, |mut state| async move {
        state
            .next()
            .await
            .map(|chunk| chunk.map(|data| (data, state)))
    }))
}
