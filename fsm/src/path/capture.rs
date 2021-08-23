use crate::path::Path;
use std::ops::Range;

pub struct CaptureItem {
    pub ty: CaptureType,
    pub key: String,
    pub path: Path
}

pub enum CaptureType {
    Struct,
    Text
}

pub struct Capture {
    ty: CaptureType,
    key: String,
    range: Range<usize>
}

impl Capture {
    pub fn increment_offset(&mut self, offset: usize) -> &mut Self {
        self.range = self.range.start + offset .. self.range.end + offset;
        self
    }
}

pub struct CaptureTracker {
    pub current_index: usize,
    pub captures: Vec<Capture>
}

impl CaptureTracker {
    pub fn new() -> Self {
        Self {
            current_index: 0,
            captures: vec![]
        }
    }

    pub fn incr_index(&mut self) -> &mut Self {
        self.current_index += 1;
        self
    }

    pub fn add_capture(&mut self, key: String, ty: CaptureType, length: usize) -> &mut Self {
        self.captures.push(Capture {
            ty,
            key,
            range: self.current_index..self.current_index+length
        });
        self
    }

    pub fn merge_with_offset_increment(&mut self, other: CaptureTracker) -> &mut Self {
        for mut capture in other.captures {
            capture.increment_offset(self.current_index);
            self.captures.push(capture);
        }
        self
    }
}