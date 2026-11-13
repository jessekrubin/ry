//! std file + a read buffer and a write buffer; used by `FileReadStream`,
//! `AsyncFile` and `AsyncFileReadStream`
//!
//! the `try_*` methods never touch the file, only the buffers, so async code
//! can call them directly instead of sending them to a blocking thread
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::iter;
use std::path::Path;

/// Read/write buffer size (cpython's `io.DEFAULT_BUFFER_SIZE` as of 3.14).
pub const BUF_SIZE: usize = 128 * 1024;

/// Most a big `read(n)` allocates up front; past that it grows as data comes in.
const READ_PREALLOC_CAP: usize = 8 * 1024 * 1024;

/// A std file plus a read buffer and a write buffer, kinda like cpython's
/// `BufferedRandom`.
///
/// Only one of the two buffers has data at any time. A `try_*` returning
/// `None`/`false` means the buffers can't answer it and the caller has to do
/// REAL (american) io.
pub struct BufferedFile {
    file: File,
    /// Probed on open (pipes, fifos and ttys can't seek).
    seekable: bool,
    /// Reads and writes at least this big skip the buffers; 0 means unbuffered.
    cap: usize,
    /// Read buffer ~ read from the file but not handed out (yet)
    rbuf: ReadBuf,
    /// Written by the caller but not sent to the file yet
    wbuf: Vec<u8>,
}

impl BufferedFile {
    /// Opens `path` and probes whether it can seek.
    pub fn open(path: &Path, opts: &OpenOptions, cap: usize) -> io::Result<Self> {
        let mut file = opts.open(path)?;
        // same check as cpython's `FileIO.seekable` (lseek(0, SEEK_CUR))
        let seekable = file.stream_position().is_ok();
        Ok(Self {
            file,
            seekable,
            cap,
            rbuf: ReadBuf::default(),
            wbuf: Vec::new(),
        })
    }

    #[must_use]
    pub fn seekable(&self) -> bool {
        self.seekable
    }

    pub fn file_len(&self) -> io::Result<u64> {
        Ok(self.file.metadata()?.len())
    }

    /// Number o' bytes sitting in the read buf.
    #[must_use]
    pub fn buffered_len(&self) -> usize {
        self.rbuf.len()
    }

    // ------------------------------------------------------------------------
    // buffers only (no io)
    // ------------------------------------------------------------------------

    /// exactly `n` bytes, if the read buffer has that many
    pub fn try_read(&mut self, n: usize) -> Option<Vec<u8>> {
        // no pending-writes check needed: while writes are pending the read
        // buf is empty, so the only read that gets through is `read(0)`
        // (which shouldn't flush anyway iirc)
        (self.rbuf.len() >= n).then(|| self.rbuf.consume(n))
    }

    /// the next line (cut off at `limit`), if the read buffer has all of it
    pub fn try_readline(&mut self, limit: Option<usize>) -> Option<Vec<u8>> {
        // no pending-writes check needed, same reason as `try_read`: the only
        // call that gets through is `readline(0)`
        let end = self.rbuf.line_end(limit)?;
        Some(self.rbuf.consume(end))
    }

    /// up to `n` bytes without moving the position, if the read buffer has any
    #[must_use]
    pub fn try_peek(&self, n: usize) -> Option<Vec<u8>> {
        let unread = self.rbuf.unread();
        (self.wbuf.is_empty() && !unread.is_empty()).then(|| unread[..n.min(unread.len())].to_vec())
    }

    /// buffers `buf` if it fits; `false` means it needs `write`.
    pub fn try_write(&mut self, buf: &[u8]) -> bool {
        let fits = self.rbuf.is_empty() && self.wbuf.len() + buf.len() <= self.cap;
        if fits {
            self.wbuf.extend_from_slice(buf);
        }
        fits
    }

    #[must_use]
    pub fn has_pending_writes(&self) -> bool {
        !self.wbuf.is_empty()
    }

    // ------------------------------------------------------------------------
    // io
    // ------------------------------------------------------------------------

    /// Sends pending writes to the file. If that fails, whatever didn't get
    /// written stays pending (like cpython) and the next flush tries again.
    pub fn flush(&mut self) -> io::Result<()> {
        // not `write_all`: when it fails it doesn't say how much it wrote
        let mut written = 0;
        let res = loop {
            if written == self.wbuf.len() {
                break Ok(());
            }
            match self.file.write(&self.wbuf[written..]) {
                Ok(0) => break Err(io::ErrorKind::WriteZero.into()),
                Ok(n) => written += n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => break Err(e),
            }
        };
        self.wbuf.drain(..written);
        res
    }

    // drop the read buffer and seek the file back to where the caller is
    // (the file itself is ahead by however much is still buffered)
    fn discard_read_ahead(&mut self) -> io::Result<()> {
        // seek back first, so an unseekable file (fifo/pipe) errs w the read
        // buf not lost in the ether (this was gnarly)
        let unread = self.rbuf.len();
        if unread > 0 {
            let back = i64::try_from(unread).map_err(io::Error::other)?;
            self.file.seek(SeekFrom::Current(-back))?;
        }
        self.rbuf.clear();
        Ok(())
    }

    // refill the (empty) read buffer; `false` at eof
    fn fill(&mut self) -> io::Result<bool> {
        self.flush()?;
        // unbuffered (cap 0) still reads 1 byte at a time so readline/peek work
        Ok(self.rbuf.fill(&mut self.file, self.cap.max(1))? > 0)
    }

    /// up to `n` bytes without moving the position.
    pub fn peek(&mut self, n: usize) -> io::Result<Vec<u8>> {
        self.flush()?;
        if self.rbuf.is_empty() {
            self.fill()?;
        }
        let unread = self.rbuf.unread();
        Ok(unread[..n.min(unread.len())].to_vec())
    }

    /// up to `n` bytes (fewer only at eof), like `BufferedReader.read(n)`.
    pub fn read(&mut self, n: usize) -> io::Result<Vec<u8>> {
        self.flush()?;
        let mut out = Vec::with_capacity(n.min(READ_PREALLOC_CAP));
        while out.len() < n {
            let take = (n - out.len()).min(self.rbuf.len());
            if take > 0 {
                self.rbuf.consume_into(take, &mut out);
                continue;
            }
            let want = n - out.len();
            if want >= self.cap {
                // big read: straight into `out`, skip the read buf
                let limit = u64::try_from(want).unwrap_or(u64::MAX);
                (&mut self.file).take(limit).read_to_end(&mut out)?;
                break;
            }
            if !self.fill()? {
                break;
            }
        }
        Ok(out)
    }

    /// `size` byte chunks until eof (the last one can be shorter)
    pub fn chunks(&mut self, size: usize) -> impl Iterator<Item = io::Result<Vec<u8>>> + '_ {
        iter::from_fn(move || {
            self.read(size)
                .map(|chunk| (!chunk.is_empty()).then_some(chunk))
                .transpose()
        })
    }

    pub fn read_all(&mut self) -> io::Result<Vec<u8>> {
        self.flush()?;
        let mut out = self.rbuf.consume(self.rbuf.len());
        self.file.read_to_end(&mut out)?;
        Ok(out)
    }

    /// The next line (or `limit` bytes); `None` at eof.
    pub fn readline(&mut self, limit: Option<usize>) -> io::Result<Option<Vec<u8>>> {
        let mut line = Vec::new();
        loop {
            let left = limit.map(|limit| limit - line.len());
            if let Some(end) = self.rbuf.line_end(left) {
                self.rbuf.consume_into(end, &mut line);
                return Ok(Some(line));
            }
            // the buffer doesn't hold the whole line: take what's there and refill
            self.rbuf.consume_into(self.rbuf.len(), &mut line);
            if !self.fill()? {
                return Ok((!line.is_empty()).then_some(line));
            }
        }
    }

    /// Lines until eof or until their total size exceeds `hint`.
    pub fn readlines(&mut self, hint: Option<usize>) -> io::Result<Vec<Vec<u8>>> {
        let mut lines = Vec::new();
        let mut total = 0;
        while let Some(line) = self.readline(None)? {
            total += line.len();
            lines.push(line);
            // hint 0 == no limit (ala cpython)
            if hint.is_some_and(|hint| hint > 0 && total > hint) {
                break;
            }
        }
        Ok(lines)
    }

    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.discard_read_ahead()?;
        if self.wbuf.len() + buf.len() > self.cap {
            self.flush()?;
        }
        if buf.len() >= self.cap {
            self.file.write_all(buf)?;
        } else {
            self.wbuf.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    /// `SeekFrom::Current` is relative to where the caller is, not to how far
    /// the file has been read into the buffer.
    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.flush()?;
        let pos = match pos {
            SeekFrom::Current(off) => {
                let unread = i64::try_from(self.rbuf.len()).map_err(io::Error::other)?;
                // huge negative offsets overflowed
                let off = off.checked_sub(unread).ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "seek offset out of range")
                })?;
                SeekFrom::Current(off)
            }
            other => other,
        };
        // seek first, and if ok nuke the read buf. this fixes a seek failure
        // causing jumping to the end of the read-buf.
        let new_pos = self.file.seek(pos)?;
        self.rbuf.clear();
        Ok(new_pos)
    }

    pub fn tell(&mut self) -> io::Result<u64> {
        self.flush()?;
        let unread = self.rbuf.len() as u64;
        Ok(self.file.stream_position()? - unread)
    }

    /// Truncates to `size` (default: the current position) without moving the position.
    pub fn truncate(&mut self, size: Option<u64>) -> io::Result<u64> {
        self.flush()?;
        self.discard_read_ahead()?;
        let size = match size {
            Some(size) => size,
            None => self.file.stream_position()?,
        };
        self.file.set_len(size)?;
        Ok(size)
    }
}

impl Drop for BufferedFile {
    fn drop(&mut self) {
        // like std's `BufWriter`: best effort, errors ignored
        let _ = self.flush();
    }
}

/// Read buffer; `buf[pos..]` is what hasn't been handed out yet.
#[derive(Default)]
struct ReadBuf {
    buf: Vec<u8>,
    pos: usize,
}

impl ReadBuf {
    fn unread(&self) -> &[u8] {
        &self.buf[self.pos..]
    }

    fn len(&self) -> usize {
        self.buf.len() - self.pos
    }

    fn is_empty(&self) -> bool {
        self.pos == self.buf.len()
    }

    /// where the next line ends (cut off at `limit`), or `None` if the buffer
    /// doesn't hold all of it. only searches the first `limit` bytes
    fn line_end(&self, limit: Option<usize>) -> Option<usize> {
        let unread = self.unread();
        let window = limit.map_or(unread, |limit| &unread[..limit.min(unread.len())]);
        let newline = cfg_select! {
            feature = "memchr" => memchr::memchr(b'\n', window),
            _ => window.iter().position(|&b| b == b'\n'),
        };
        match newline {
            Some(i) => Some(i + 1),
            None => limit.filter(|&limit| limit <= unread.len()),
        }
    }

    fn consume(&mut self, n: usize) -> Vec<u8> {
        let out = self.unread()[..n].to_vec();
        self.pos += n;
        out
    }

    fn consume_into(&mut self, n: usize, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.unread()[..n]);
        self.pos += n;
    }

    fn clear(&mut self) {
        self.buf.clear();
        self.pos = 0;
    }

    /// refills w/ up to `n` bytes from `file`; returns how many (0 at eof)
    fn fill(&mut self, file: &mut File, n: usize) -> io::Result<usize> {
        self.buf.resize(n, 0);
        self.pos = 0;
        let read = loop {
            match file.read(&mut self.buf) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                res => break res,
            }
        }
        .inspect_err(|_| self.buf.clear())?;
        self.buf.truncate(read);
        Ok(read)
    }
}
