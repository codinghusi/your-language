use std::ops::Range;

use crate::path::Path;

#[derive(Clone)]
pub struct CaptureItem {
    pub ty: CaptureType,
    pub key: String,
    pub path: Path,
}

#[derive(Clone)]
pub enum CaptureType {
    Struct,
    Text,
}

pub struct Capture {
    range: Range<usize>,
}

impl Capture {
    pub fn increment_offset(&mut self, offset: usize) -> &mut Self {
        self.range = self.range.start + offset..self.range.end + offset;
        self
    }
}
