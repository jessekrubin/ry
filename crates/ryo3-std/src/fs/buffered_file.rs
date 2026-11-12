//! std file w/ read-ahead/write-behind buffers; used by `FileReadStream`,
//! `AsyncFile` and `AsyncFileReadStream`
//!
//! the `try_*` methods only touch the buffers (no io), so async callers can
//! answer buffer hits w/o a thread hop
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// Read/write buffer size (cpython's `io.DEFAULT_BUFFER_SIZE` as of 3.14).
pub const BUF_SIZE: usize = 128 * 1024;

/// Max up-front allocation for big reads; the buffer grows with the data.
const READ_PREALLOC_CAP: usize = 8 * 1024 * 1024;

/// A std file plus read-ahead/write-behind buffers, kinda like cpython's
/// `BufferedRandom`.
///
/// The two buffers are never both non-empty. `try_*` -> `None`/`false` means
/// it needs io.
pub struct BufferedFile {
    file: File,
    /// Probed on open (pipes, fifos and ttys can't seek).
    seekable: bool,
    /// Reads and writes at least this big skip the buffers; 0 means unbuffered.
    cap: usize,
    /// Read-ahead; the unread bytes are `rbuf[rpos..]`.
    rbuf: Vec<u8>,
    rpos: usize,
    /// Pending writes.
    wbuf: Vec<u8>,
}

impl BufferedFile {
    /// Opens `path` and probes whether it can seek.
    pub fn open(path: &Path, opts: &OpenOptions, cap: usize) -> io::Result<Self> {
        let mut file = opts.open(path)?;
        // lseek(0, SEEK_CUR) ~ what cpython's `FileIO.seekable` does
        let seekable = file.stream_position().is_ok();
        Ok(Self {
            file,
            seekable,
            cap,
            rbuf: Vec::new(),
            rpos: 0,
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

    /// Number of read-ahead bytes not consumed yet.
    #[must_use]
    pub fn buffered_len(&self) -> usize {
        self.unread().len()
    }

    fn unread(&self) -> &[u8] {
        &self.rbuf[self.rpos..]
    }

    fn consume_into(&mut self, n: usize, out: &mut Vec<u8>) {
        out.extend_from_slice(&self.rbuf[self.rpos..self.rpos + n]);
        self.rpos += n;
    }

    fn consume(&mut self, n: usize) -> Vec<u8> {
        let out = self.unread()[..n].to_vec();
        self.rpos += n;
        out
    }

    // ------------------------------------------------------------------------
    // buffer hits (no io)
    // ------------------------------------------------------------------------

    /// exactly `n` bytes if the read-ahead has them all
    pub fn try_read(&mut self, n: usize) -> Option<Vec<u8>> {
        (self.wbuf.is_empty() && self.unread().len() >= n).then(|| self.consume(n))
    }

    /// a line (or `limit` bytes) if the read-ahead has one
    pub fn try_readline(&mut self, limit: Option<usize>) -> Option<Vec<u8>> {
        if !self.wbuf.is_empty() {
            return None;
        }
        let unread = self.unread();
        let end = match memchr::memchr(b'\n', unread) {
            Some(i) => limit.map_or(i + 1, |limit| limit.min(i + 1)),
            None => limit.filter(|&limit| limit <= unread.len())?,
        };
        Some(self.consume(end))
    }

    /// up to `n` bytes without moving the position
    #[must_use]
    pub fn try_peek(&self, n: usize) -> Option<Vec<u8>> {
        let unread = self.unread();
        (self.wbuf.is_empty() && !unread.is_empty()).then(|| unread[..n.min(unread.len())].to_vec())
    }

    /// buffers `buf` if it fits; `false` means it needs `write`.
    pub fn try_write(&mut self, buf: &[u8]) -> bool {
        let fits = self.unread().is_empty() && self.wbuf.len() + buf.len() <= self.cap;
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

    /// Writes out pending writes. They are dropped on error, like cpython.
    pub fn flush(&mut self) -> io::Result<()> {
        if self.wbuf.is_empty() {
            return Ok(());
        }
        let res = self.file.write_all(&self.wbuf);
        self.wbuf.clear();
        res
    }

    // toss the read-ahead + move the os position back to the logical one
    fn discard_read_ahead(&mut self) -> io::Result<()> {
        let unread = self.unread().len();
        self.rbuf.clear();
        self.rpos = 0;
        if unread > 0 {
            let back = i64::try_from(unread).map_err(io::Error::other)?;
            self.file.seek(SeekFrom::Current(-back))?;
        }
        Ok(())
    }

    // refill the (empty) read-ahead; `false` at eof
    fn fill(&mut self) -> io::Result<bool> {
        self.flush()?;
        // unbuffered still needs a byte of lookahead for readline/peek
        self.rbuf.resize(self.cap.max(1), 0);
        self.rpos = 0;
        let n = loop {
            match self.file.read(&mut self.rbuf) {
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                res => break res,
            }
        }
        .inspect_err(|_| self.rbuf.clear())?;
        self.rbuf.truncate(n);
        Ok(n > 0)
    }

    /// up to `n` bytes without moving the position.
    pub fn peek(&mut self, n: usize) -> io::Result<Vec<u8>> {
        self.flush()?;
        if self.unread().is_empty() {
            self.fill()?;
        }
        let unread = self.unread();
        Ok(unread[..n.min(unread.len())].to_vec())
    }

    /// up to `n` bytes (fewer only at eof), like `BufferedReader.read(n)`.
    pub fn read(&mut self, n: usize) -> io::Result<Vec<u8>> {
        self.flush()?;
        let mut out = Vec::with_capacity(n.min(READ_PREALLOC_CAP));
        while out.len() < n {
            let take = (n - out.len()).min(self.unread().len());
            if take > 0 {
                self.consume_into(take, &mut out);
                continue;
            }
            let want = n - out.len();
            if want >= self.cap {
                // big read ~ straight into `out`, skip the read-ahead
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

    pub fn read_all(&mut self) -> io::Result<Vec<u8>> {
        self.flush()?;
        let mut out = self.consume(self.unread().len());
        self.file.read_to_end(&mut out)?;
        Ok(out)
    }

    /// The next line (or `limit` bytes); `None` at eof.
    pub fn readline(&mut self, limit: Option<usize>) -> io::Result<Option<Vec<u8>>> {
        let mut line = Vec::new();
        loop {
            let left = limit.map(|limit| limit - line.len());
            if let Some(rest) = self.try_readline(left) {
                // whole line was already buffered
                if line.is_empty() {
                    return Ok(Some(rest));
                }
                line.extend_from_slice(&rest);
                return Ok(Some(line));
            }
            // no newline (and under `limit`) in the read-ahead ~ take it all + refill
            self.consume_into(self.unread().len(), &mut line);
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
            if hint.is_some_and(|hint| total > hint) {
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

    /// Seeks relative to the logical position (not the read-ahead).
    pub fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.flush()?;
        let pos = match pos {
            SeekFrom::Current(off) => {
                let unread = i64::try_from(self.unread().len()).map_err(io::Error::other)?;
                SeekFrom::Current(off - unread)
            }
            other => other,
        };
        self.rbuf.clear();
        self.rpos = 0;
        self.file.seek(pos)
    }

    pub fn tell(&mut self) -> io::Result<u64> {
        self.flush()?;
        let unread = self.unread().len() as u64;
        Ok(self.file.stream_position()? - unread)
    }

    /// Truncates to `size` (default: the current position) without moving the
    /// position.
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
