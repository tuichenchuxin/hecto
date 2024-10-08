use std::io::Error;
use std::fs::read_to_string;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let content = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in content.lines() {
            lines.push(value.to_string());
        }
        Ok(Self {lines})
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}