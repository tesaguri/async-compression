use crate::codec::Encode;
use std::io::Result;

use flate2::Compression;

#[derive(Debug)]
pub struct DeflateEncoder {
    inner: crate::codec::FlateEncoder,
}

impl DeflateEncoder {
    pub(crate) fn new(level: Compression) -> Self {
        Self {
            inner: crate::codec::FlateEncoder::new(level, false),
        }
    }
}

impl Encode for DeflateEncoder {
    fn encode(&mut self, input: &[u8], output: &mut [u8]) -> Result<(usize, usize)> {
        self.inner.encode(input, output)
    }

    fn finish(&mut self, output: &mut [u8]) -> Result<(bool, usize)> {
        self.inner.finish(output)
    }
}
