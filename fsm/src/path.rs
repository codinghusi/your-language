

struct Path {
    pub path: Vec<EdgeType>
}

impl Path {
    pub fn add(&mut self, elements: Into<Vec<EdgeType>>) -> Self {
        self.path.append(elements.into());
        self
    }
}

impl From<Path> for Vec<EdgeType> {
    fn from(other: Path) -> Self {
        path.path
    }
}