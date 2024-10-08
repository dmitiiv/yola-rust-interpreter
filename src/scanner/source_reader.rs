use std::{
    fs::File,
    io::{Error, Read},
    string::FromUtf8Error,
};

pub struct SourceReader {
    buffer: Vec<u8>,
}

impl SourceReader {
    pub fn new() -> SourceReader {
        SourceReader { buffer: vec![] }
    }

    pub fn read_sources(&mut self, path: &str) -> Result<(), Error> {
        let mut file = File::open(path)?;
        self.buffer.clear();

        file.read_to_end(&mut self.buffer)?;

        Ok(())
    }

    pub fn get_sources(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.buffer.clone())
    }
}
