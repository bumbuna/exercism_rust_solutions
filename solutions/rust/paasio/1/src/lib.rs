use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrapper: R,
    bytes_read: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        Self {
            wrapper: _wrapped,
            bytes_read: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let s = self.wrapper.read(buf)?;
        self.bytes_read += s;
        self.reads += 1;
        Ok(s)
    }
}

pub struct WriteStats<W> {
    wrapper: W,
    bytes_wrote: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        Self {
            wrapper: _wrapped,
            bytes_wrote: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapper
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_wrote
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let b = self.wrapper.write(buf)?;
        self.bytes_wrote += b;
        self.writes += 1;
        Ok(b)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapper.flush()
    }
}
