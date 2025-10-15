pub(crate) mod option;
pub(crate) mod primitive;

/// Helper struct for writing formatted output to a byte buffer
pub(crate) struct BufferWriter<'a> {
    buffer: &'a mut [u8],
    pos: usize,
}

impl<'a> BufferWriter<'a> {
    pub(crate) fn new(buffer: &'a mut [u8]) -> Self {
        BufferWriter { buffer, pos: 0 }
    }

    pub(crate) fn written(&self) -> usize {
        self.pos
    }
}

impl core::fmt::Write for BufferWriter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        if self.pos + bytes.len() > self.buffer.len() {
            return Err(core::fmt::Error);
        }
        self.buffer[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();
        Ok(())
    }
}
