use std::io::Error;
use std::fs::read_to_string;
use crate::editor::view::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let content = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in content.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self {lines})
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}