use crate::{State, EdgeType};

pub struct FSM<'a> {
    root: State<'a>
}

impl<'a> FSM<'a> {
    pub fn build(paths: Vec<Vec<EdgeType<'a>>>) -> Self {
        let mut root = State::new();
        for path in paths {
            root.add_path(&mut path.into_iter(), true);
        }
        Self { root }
    }
}