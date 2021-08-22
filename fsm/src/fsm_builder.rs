use crate::FSM;
use crate::path::Path;
use crate::state_edge::{StateEdge, MergeStatus};
use std::rc::Rc;
use std::convert::TryInto;

pub struct FSM_Builder {
    paths: Vec<Path>
}

impl FSM_Builder {
    pub fn new() -> Self {
        Self { paths: vec![] }
    }

    pub fn from(paths: Vec<Path>) -> Self {
        Self { paths }
    }

    pub fn add(&mut self, path: Path) -> &mut Self {
        self.paths.push(path);
        self
    }

    pub fn build(&self) -> FSM {
        let mut root = StateEdge::new_root();
        let mut ids = StateEdge::id_gen();
        self.paths
            .iter()
            .map(|path| StateEdge::merge_path(Rc::clone(&root), path, &mut ids))
            .for_each(|status| {
                match status {
                    MergeStatus::Failed => panic!("building failed..."), // TODO: make this more proper into a Result<>
                    _ => ()
                }
            });
        FSM { root }
    }
}