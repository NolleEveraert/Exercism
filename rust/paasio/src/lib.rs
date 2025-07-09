use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrapped: R,
    bytes: usize,
    operations: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped: wrapped,
            bytes: 0,
            operations: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        return &self.wrapped;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes;
    }

    pub fn reads(&self) -> usize {
        return self.operations;
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes = self.wrapped.read(buf)?;
        self.bytes += bytes;
        self.operations += 1;
        Ok(bytes)
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    bytes: usize,
    operations: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped: wrapped,
            bytes: 0,
            operations: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        return &self.wrapped;
    }

    pub fn bytes_through(&self) -> usize {
        return self.bytes;
    }

    pub fn writes(&self) -> usize {
        return self.operations;
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes = self.wrapped.write(buf)?;
        self.bytes += bytes;
        self.operations += 1;
        Ok(bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
