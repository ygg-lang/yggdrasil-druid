use std::fmt::{Arguments, Write};

pub(crate) struct Buffer<'i, C> {
    buffer: String,
    config: &'i C,
}

impl<'i, C> Write for Buffer<'i, C> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.buffer.write_char(c)
    }
    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> std::fmt::Result {
        self.buffer.write_fmt(args)
    }
}impl<'i, C> Buffer<'i, C> {
    pub fn new(config: &C) -> Self {
        Self {
            buffer: "".to_string(),
            config,
        }
    }
    pub fn finish(self) -> String {
        self.buffer
    }
}