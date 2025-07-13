use std::io::stdin;

use crate::error::SQLError;

pub struct InputBuffer {
    buffer: String,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
        }
    }

    pub fn set_buffer(&mut self, new_buf: &str) {
        self.buffer = new_buf.into();
    }

    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }

    pub fn read_input(&mut self) -> Result<(), SQLError> {
        self.buffer.clear();
        match stdin().read_line(&mut self.buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    eprintln!("Error reading input");
                    std::process::exit(1);
                }
                if self.buffer.ends_with('\n') {
                    self.buffer.pop();
                    if self.buffer.ends_with('\r') {
                        self.buffer.pop();
                    }
                }
                return Ok(());
            }
            Err(err) => {
                return Err(SQLError::IOError(err));
            }
        }
    }
}
