use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    file_io: u64,
    net_io: u64
};

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats{wrapped: wrapped, file_io: 0, net_io: 0}
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn reads(&self) -> usize {
        unimplemented!()
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call reading {:?}", buf)
    }
}

pub struct WriteStats<W>{
    wrapped: W,
    file_io: u64,
    net_io: u64
};

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats{wrapped: _wrapped, file_io: 0, net_io: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        unimplemented!()
    }

    pub fn writes(&self) -> usize {
        unimplemented!()
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        unimplemented!("Collect statistics about this call writing {:?}", buf)
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}
