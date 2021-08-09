use crate::{State, EdgeType};

#[derive(Debug)]
pub struct FSM {
    pub root: State
}

impl FSM {
    pub fn build(paths: Vec<Vec<EdgeType>>) -> Self {
        let mut root = State::new();
        for path in paths {
            root.add_path(&mut path.into_iter(), true);
        }
        Self { root }
    }

    pub fn parse(&self, input: &str) {
        self.root.parse((input))
    }
}