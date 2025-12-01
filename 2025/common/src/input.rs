use std::io::Result;
use std::{fs, path::Path};

pub struct Input {
    pub raw: String,
}

impl Input {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Input> {
        Ok(Input {
            raw: fs::read_to_string(path)?,
        })
    }

    pub fn from_default() -> Result<Input> {
        Input::from_file("input.txt")
    }

    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.raw.lines()
    }
}
