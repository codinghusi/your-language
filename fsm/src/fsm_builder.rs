use crate::FSM;
use crate::State;
use crate::Path;

pub struct FSM_Builder {
    paths: Vec<Path>
}

impl FSM_Builder {
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn from(&self, paths: Vec<Path>) -> Self {
        Self { paths }
    }

    pub fn add(&self, path: Path) -> u16 {
        let index = self.paths.len();
        self.paths.push(path);
        index
    }

    pub fn build(self) -> FSM {
        let mut root = State::new();
        for path in self.paths {
            root.add_path(&mut path.into_iter(), true);
        }
        FSM { root }
    }
}