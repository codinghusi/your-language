use crate::path::capture::{CaptureItem, CaptureType};

use super::Edge;

#[derive(Clone)]
pub struct Path {
    pub items: Vec<Edge>,
}

impl Path {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn from(items: Vec<Edge>) -> Self {
        Self { items }
    }

    pub fn add(mut self, element: Edge) -> Self {
        self.items.push(element);
        self
    }

    pub fn char(self, char: char) -> Self {
        self.add(Edge::Char(char))
    }

    pub fn then(mut self, mut path: Path) -> Self {
        self.items.append(&mut path.items);
        self
    }

    pub fn one_of(self, paths: Vec<Path>) -> Self {
        self.add(Edge::OneOf(paths))
    }

    pub fn one_of_chars(self, chars: &str) -> Self {
        self.one_of(chars.chars().map(|c| Path::new().char(c)).collect())
    }

    pub fn string(mut self, str: &str) -> Self {
        for c in str.chars() {
            self = self.char(c);
        }
        self
    }

    pub fn capture(self, key: String, path: Path) -> Self {
        self.add(Edge::Capture(CaptureItem {
            ty: CaptureType::Struct,
            key,
            path,
        }))
    }

    pub fn capture_text(self, key: String, path: Path) -> Self {
        self.add(Edge::Capture(CaptureItem {
            ty: CaptureType::Text,
            key,
            path,
        }))
    }

    pub fn optional(self, path: Path) -> Self {
        self.add(Edge::Optional(path))
    }

    pub fn optional_string(self, str: &str) -> Self {
        self.optional(Path::new().string(str))
    }

    pub fn cycle(self, path: Path) -> Self {
        self.add(Edge::Cycle(path))
    }

    pub fn end(self) -> Self {
        self.add(Edge::Final)
    }
}
