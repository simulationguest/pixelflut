use std::fmt::Write;

const BUFFER_SIZE: usize = 32;

/// An allocation-free writable buffer
pub(crate) struct Buffer {
    inner: [u8; BUFFER_SIZE],
    pos: usize,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            inner: [0; BUFFER_SIZE],
            pos: 0,
        }
    }

    pub(crate) fn get_contents(&self) -> &[u8] {
        &self.inner[0..self.pos]
    }

    pub(crate) fn clear(&mut self) {
        self.pos = 0;
    }
}

impl Write for Buffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        let slice = &mut self.inner[self.pos..self.pos + s.len()];
        slice.copy_from_slice(s.as_bytes());
        self.pos += s.len();
        Ok(())
    }
}
